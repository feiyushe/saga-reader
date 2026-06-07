use reqwest::redirect::Policy;
use scraper::{Html, Selector};
use spdlog::{error, info};
use base64::Engine;
use tauri::Runtime;

use llm::llm_agent::CompletionAgent;
use llm::providers::types::AITargetOption;
use types::LLMSection;

use crate::connector;
use crate::connector::ClientOption;
use crate::search::selector_extensions::ElementSelector;
use crate::search::utils::trim_html_with_script_and_style;
use crate::simulator::scrap_text_by_url;

/// 尝试从 Bing 重定向 URL（/ck/a?...&u=a1XXXX...）中提取实际目标 URL。
/// Bing 的 u 参数前两个字符是协议标记（a1=https, a3=http），后续是 base64 编码的完整 URL。
fn extract_bing_redirect_url(url_str: &str) -> Option<String> {
    let url = reqwest::Url::parse(url_str).ok()?;
    if !url_str.contains("bing.com/ck/") {
        return None;
    }
    let u_param = url.query_pairs().find(|(k, _)| k == "u")?.1;
    // Bing 使用自定义编码：前两个字符表示协议（a1=https, a3=http），后续是 base64 的完整 URL
    if u_param.len() < 2 {
        return None;
    }
    let prefix = &u_param[..2];
    // 协议标记校验，但不需要拼接——base64 内容已包含完整协议
    match prefix {
        "a1" | "a3" => {}
        _ => return None,
    };
    let encoded = &u_param[2..];
    // u 参数使用 URL-safe base64 编码（无 padding）
    let decoded = {
        let mut padded = encoded.to_string();
        while padded.len() % 4 != 0 {
            padded.push('=');
        }
        base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(&padded)
            .or_else(|_| base64::engine::general_purpose::URL_SAFE.decode(&padded))
            .ok()?
    };
    let decoded_str = String::from_utf8(decoded).ok()?;
    // base64 解码结果已包含完整协议（如 https://...），直接返回
    if decoded_str.starts_with("http://") || decoded_str.starts_with("https://") {
        Some(decoded_str)
    } else {
        None
    }
}

pub async fn read<R: Runtime>(
    url_str: &str,
    source_search_host: Option<String>,
    llm_section: LLMSection,
    app_handle: Option<tauri::AppHandle<R>>,
) -> anyhow::Result<(String, String)> {
    read_inner(url_str, true, source_search_host, llm_section, app_handle).await
}

async fn read_inner<R: Runtime>(
    url_str: &str,
    auto_redirect: bool,
    source_search_host: Option<String>,
    llm_section: LLMSection,
    app_handle: Option<tauri::AppHandle<R>>,
) -> anyhow::Result<(String, String)> {
    let url = reqwest::Url::parse(url_str)?;
    if let Some(host) = url.domain() {
        let (html_text, final_url) = acquire_html(url_str, auto_redirect, &source_search_host, &llm_section, host, &app_handle).await?;
        return read_inner_from_response(&final_url,
                                        auto_redirect,
                                        source_search_host,
                                        llm_section,
                                        html_text,
                                        app_handle).await;
    }
    Err(anyhow::Error::msg("article.read_inner occurs error, selector parse error for body"))
}

async fn acquire_html<R: Runtime>(
    url_str: &str,
    auto_redirect: bool,
    source_search_host: &Option<String>,
    llm_section: &LLMSection,
    host: &str,
    app_handle: &Option<tauri::AppHandle<R>>,
) -> anyhow::Result<(String, String)> {
    info!("article.read，网页抓取中...{}", url_str);

    // 优先处理 Bing 重定向 URL：直接提取实际目标地址，避免请求中间页面
    if let Some(real_url) = extract_bing_redirect_url(url_str) {
        info!("article.read，Bing重定向URL解析，{} -> {}", url_str, real_url);
        let real_parsed = reqwest::Url::parse(&real_url)?;
        let real_host = real_parsed.domain().unwrap_or(host);
        return Box::pin(acquire_html(&real_url, auto_redirect, source_search_host, llm_section, real_host, app_handle)).await;
    }

    let connector_builder = connector::new_builder(ClientOption {
        host: host.into(),
        user_agent: None,
    })?.redirect(Policy::none());
    let client = connector_builder.build()?;
    let response = client.get(url_str).send().await?;
    let response_status = response.status();

    if response_status.is_client_error() {
        // 403 等客户端错误：如果有 app_handle，回退到 simulator（WebView）抓取
        if let Some(ap) = app_handle {
            info!("article.read，reqwest收到{}，回退到WebView抓取...{}", response_status, url_str);
            match scrap_text_by_url(ap.clone(), url_str).await {
                Ok(html) => return Ok((html, url_str.into())),
                Err(e) => {
                    error!("article.read，WebView抓取也失败...{}, error = {}", url_str, e);
                }
            }
        }
        let err_msg = format!(
            "article.read_inner failure, status code is {}",
            response_status
        );
        error!("{}, {}", err_msg, url_str);
        return Err(anyhow::Error::msg(err_msg));
    }

    if response_status.is_server_error() {
        let err_msg = format!(
            "article.read_inner failure, status code is {}",
            response_status
        );
        error!("{}, {}", err_msg, url_str);
        return Err(anyhow::Error::msg(err_msg));
    }

    if response_status.is_redirection() {
        let redirect_location = response.headers()
            .get("location")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");
        if redirect_location.is_empty() {
            let err_msg = format!("article.read_inner received redirect (status {}) but no location header", response_status);
            error!("{}, {}", err_msg, url_str);
            return Err(anyhow::Error::msg(err_msg));
        }
        info!("detect the redirection from {} to {}", url_str, redirect_location);
        let redirected_url = reqwest::Url::parse(redirect_location)?;
        let redirected_host = redirected_url.domain().unwrap_or(host);
        return Box::pin(acquire_html(redirect_location, auto_redirect, source_search_host, llm_section, redirected_host, app_handle)).await;
    }
    Ok((response.text().await?, url_str.into()))
}

async fn read_inner_from_response<R: Runtime>(
    url_str: &str,
    auto_redirect: bool,
    source_search_host: Option<String>,
    llm_section: LLMSection,
    html_text: String,
    app_handle: Option<tauri::AppHandle<R>>,
) -> anyhow::Result<(String, String)> {
    let gs = CompletionAgent::new(llm_section.clone(), "".into(), AITargetOption::default())?;
    // 如果是auto_redirect进入校准模式。
    //  校准模式下
    //    如果搜索源为空则开始纠正。
    //    如果搜索源不为空则先判定是否来自搜索源，如果不来自搜索源则不修正，来自搜索源则修正。
    if auto_redirect {
        if let Some(source_search_host_str) = source_search_host {
            if url_str.contains(source_search_host_str.as_str()) {
                let mut chat = format!(r#""{}""#, html_text);
                chat.push_str("\n上述代码中是否包括一个通过window.location.href重定向的新页面链接，如果有则只告诉我这个链接地址且不要携带其他说明信息，如果没有和我说\u{201C}没有\u{201D}且不要携带其他说明信息。");
                let url_detected = gs.completion(chat).await?;
                let url_detected = url_detected.replace('`', "");
                if url_detected.starts_with("http") {
                    info!("article.read，网页抓取遇到重定向指示并尝试重定向到...{}, 原链接为{}", url_detected, url_str);
                    return Box::pin(read_inner(url_detected.as_str(), false, None, llm_section.clone(), app_handle)).await;
                }
            }
        }
    }

    return if let Ok(selector) = Selector::parse("body") {
        let sharked_html = trim_html_with_script_and_style(html_text.as_str());
        let document = Html::parse_document(&sharked_html);
        let scraped_content = document.select_text(&selector)?;
        info!("article.read，成功抓取网页内容...{}, length = {}", url_str, scraped_content.len());
        Ok((scraped_content, url_str.into()))
    } else {
        Err(anyhow::Error::msg("article.read_inner occurs error, selector parse error for body"))
    };
}
