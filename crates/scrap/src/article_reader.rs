use reqwest::redirect::Policy;
use scraper::{Html, Selector};
use spdlog::{error, info};

use llm::llm_agent::CompletionAgent;
use llm::providers::types::AITargetOption;
use types::LLMSection;

use crate::connector;
use crate::connector::ClientOption;
use crate::search::selector_extensions::ElementSelector;
use crate::search::utils::trim_html_with_script_and_style;

pub async fn read(
    url_str: &str,
    source_search_host: Option<String>,
    llm_section: LLMSection,
) -> anyhow::Result<(String, String)> {
    read_inner(url_str, true, source_search_host, llm_section).await
}

async fn read_inner(
    url_str: &str,
    auto_redirect: bool,
    source_search_host: Option<String>,
    llm_section: LLMSection,
) -> anyhow::Result<(String, String)> {
    let url = reqwest::Url::parse(url_str)?;
    if let Some(host) = url.domain() {
        let (html_text, final_url) = acquire_html(url_str, auto_redirect, &source_search_host, &llm_section, host).await?;
        return read_inner_from_response(&final_url,
                                        auto_redirect,
                                        source_search_host,
                                        llm_section,
                                        html_text).await;
    }
    Err(anyhow::Error::msg("article.read_inner occurs error, selector parse error for body"))
}

async fn acquire_html(url_str: &str, auto_redirect: bool, source_search_host: &Option<String>, llm_section: &LLMSection, host: &str) -> anyhow::Result<(String, String)> {
    info!("article.read，网页抓取中...{}", url_str);
    let connector_builder = connector::new_builder(ClientOption {
        host: host.into(),
        user_agent: None,
    })?.redirect(Policy::none());
    let client = connector_builder.build()?;
    let response = client.get(url_str).send().await?;
    let response_status = response.status();
    if response_status.is_client_error() || response_status.is_server_error() {
        let err_msg = format!(
            "article.read_inner failure, status code is {}",
            response.status()
        );
        error!("{}, {}", err_msg, url_str);
        return Err(anyhow::Error::msg(err_msg));
    }
    if response_status.is_redirection() {
        let redirect_location = response.headers().get("location").unwrap().to_str()?;
        info!("detect the redirection from {} to {}", url_str, redirect_location);
        let redirected_url = reqwest::Url::parse(redirect_location)?;
        let redirected_host = redirected_url.domain().unwrap();
        return Box::pin(acquire_html(redirect_location, auto_redirect, source_search_host, llm_section, redirected_host)).await;
    }
    Ok((response.text().await?, url_str.into()))
}

async fn read_inner_from_response(url_str: &str, auto_redirect: bool, source_search_host: Option<String>, llm_section: LLMSection, html_text: String) -> anyhow::Result<(String, String)> {
    let gs = CompletionAgent::new(llm_section.clone(), "".into(), AITargetOption::default())?;
    // 如果是auto_redirect进入校准模式。
    //  校准模式下
    //    如果搜索源为空则开始纠正。
    //    如果搜索源不为空则先判定是否来自搜索源，如果不来自搜索源则不修正，来自搜索源则修正。
    if auto_redirect {
        if let Some(source_search_host_str) = source_search_host {
            if url_str.contains(source_search_host_str.as_str()) {
                let mut chat = format!(r#""{}""#, html_text);
                chat.push_str("\n上述代码中是否包括一个通过window.location.href重定向的新页面链接，如果有则只告诉我这个链接地址且不要携带其他说明信息，如果没有和我说“没有”且不要携带其他说明信息。");
                let url_detected = gs.completion(chat).await?;
                let url_detected = url_detected.replace('`', "");
                if url_detected.starts_with("http") {
                    info!("article.read，网页抓取遇到重定向指示并尝试重定向到...{}, 原链接为{}", url_detected, url_str);
                    return Box::pin(read_inner(url_detected.as_str(), false, None, llm_section.clone())).await;
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
