use chrono::{Datelike, Duration, Local, Utc};
use reqwest::Client;
use scraper::{Html, Selector};
use spdlog::error;

use types::{Article, LLMSection};

use crate::connector::ClientOption;
use crate::search::types::IProvider;
use crate::search::utils::{trim_head_read_days_ago, trim_html_with_script_and_style};
use crate::{article_reader as article, connector};

const SEARCH_HOST: &str = "www.baidu.com";

///
/// 目前仍存在的问题：
///     百度的link都是baidu的中间地址，访问时会做redirect，但reqwest在自动redirection时由于host自动沿用使用了baidu导致被目标网站反制。
/// https://www.searchapi.io/docs/baidu
/// wd--查询的关键词
/// ie--查询输入文字的编码
/// tn--提交搜索请求的来源站点。
/// rn--搜索结果显示条数
pub struct Provider {
    client: Client,
    llm_section: LLMSection,
    selector_item_layout: Selector,
    selector_item_title: Selector,
    selector_item_head_read1: Selector,
    selector_item_head_read2: Selector,
    selector_item_date: Selector,
    selector_item_link: Selector,
}

impl Provider {
    pub fn new(llm_section: LLMSection) -> anyhow::Result<Self> {
        let client = connector::new(ClientOption {
            host: String::from(SEARCH_HOST),
            user_agent: None,
        })
        .unwrap();
        Ok(Provider {
            client,
            llm_section,
            selector_item_layout: Selector::parse(".result.c-container.xpath-log.new-pmd").unwrap(),
            selector_item_title: Selector::parse(".c-title").unwrap(),
            selector_item_head_read1: Selector::parse(".content-right_1THTn").unwrap(),
            selector_item_head_read2: Selector::parse(".content-right_2s-H4").unwrap(),
            selector_item_date: Selector::parse(".c-color-gray2").unwrap(),
            selector_item_link: Selector::parse(".c-title > a").unwrap(),
        })
    }

    fn prepare_target_sources(&self, html_text: &str) -> anyhow::Result<Vec<Article>> {
        let mut pending_result: Vec<Article> = Vec::new();
        let sharked_html = trim_html_with_script_and_style(html_text);
        let document = Html::parse_document(sharked_html.as_str());
        for element in document.select(&self.selector_item_layout) {
            let date_created_opt = match element.select(&self.selector_item_date).next() {
                None => None,
                Some(er) => {
                    let lossy_date_str = er.text().collect::<String>();
                    Some(adjust_date_str(lossy_date_str))
                }
            };
            if date_created_opt.is_none() {
                continue;
            }

            let date_created = date_created_opt.unwrap();
            let title = element
                .select(&self.selector_item_title)
                .next()
                .unwrap()
                .text()
                .collect::<String>();
            let mut head_read = String::new();

            {
                if let Some(head_read_block) = element.select(&self.selector_item_head_read1).next()
                {
                    head_read = head_read_block.text().collect::<String>();
                } else if let Some(head_read_block) =
                    element.select(&self.selector_item_head_read2).next()
                {
                    head_read = head_read_block.text().collect::<String>();
                }
                head_read = trim_head_read_days_ago(head_read);
            }
            let a_html = element
                .select(&self.selector_item_link)
                .next()
                .unwrap()
                .html();
            let a_left_index = a_html.find(r#"href=""#).unwrap() + 6;
            let a_sub_string = &a_html[a_left_index..];
            let a_right_index = a_sub_string.find(r#"""#).unwrap();
            let source_link = a_sub_string[0..a_right_index].to_string();
            pending_result.push(Article {
                title,
                head_read: Some(head_read),
                date_created,
                source_link,
                summary: None,
                content: None,
                date_read: None,
            })
        }
        Ok(pending_result)
    }

    async fn convert(&self, html_text: String) -> anyhow::Result<Vec<Article>> {
        let mut result: Vec<Article> = Vec::new();
        let pending_result = self.prepare_target_sources(&html_text)?;
        for pending_article in pending_result {
            let title = pending_article.title;
            let head_read = pending_article.head_read;
            let source_link = pending_article.source_link;
            if let Ok(c) = article::read(
                &source_link,
                Some(String::from("baidu.com")),
                self.llm_section.clone(),
            )
            .await
            {
                result.push(Article {
                    title,
                    head_read,
                    date_created: String::new(),
                    source_link: c.1,
                    summary: None,
                    content: Some(c.0),
                    date_read: None,
                })
            } else {
                error!(
                    "fetch article content failure, source_link = {}",
                    source_link
                );
            };
        }
        Ok(result)
    }
}

impl IProvider for Provider {
    async fn search_by_words(&self, words: Vec<&str>) -> anyhow::Result<Vec<Article>> {
        let search_word = words.join("%20").to_string();
        let date_range_end = Utc::now().timestamp();
        let date_range_begin = Utc::now().timestamp() - 60 * 60 * 24 * 7;
        let url = format!("https://www.baidu.com/s?ie=utf-8&f=8&rsv_bp=1&tn=baidu&wd={}&rn=20&gpc=stf%3D{}%2C{}%7Cstftype%3D1", search_word, date_range_begin, date_range_end);
        let response = self.client.get(url).send().await?;
        let html_text = response.text().await?;
        self.convert(html_text).await
    }
}

fn adjust_date_str(lossy_date_str: String) -> String {
    if !lossy_date_str.ends_with("天前") {
        let sub_days: i64 = lossy_date_str[0..1].parse().unwrap();
        let now = Local::now().naive_local();
        let past = now.checked_sub_signed(Duration::days(sub_days)).unwrap();
        return past.format("%Y年%m月%d日").to_string();
    }
    if !lossy_date_str.contains("年") {
        let mut date = Local::now().year().to_string();
        date.push('年');
        date.push_str(lossy_date_str.as_str());
        return date;
    }
    lossy_date_str
}
