use reqwest::Client;
use scraper::{Html, Selector};
use spdlog::{debug, error, info};
use urlencoding::encode;

use types::{Article, LLMSection};

use crate::connector::ClientOption;
use crate::search::selector_extensions::ElementSelector;
use crate::search::types::IProvider;
use crate::search::utils::{trim_head_read_days_ago, trim_html_with_script_and_style};
use crate::{article_reader as article, connector};

const SEARCH_HOST: &str = "www.bing.com";

pub struct Provider {
    client: Client,
    llm_section: LLMSection,
    selector_item_layout: Selector,
    selector_item_title: Selector,
    selector_item_head_read: Selector,
    selector_item_link: Selector,
}

impl Provider {
    pub fn new(llm_section: LLMSection) -> anyhow::Result<Self> {
        let client = connector::new(ClientOption {
            host: String::from(SEARCH_HOST),
            user_agent: None,
        })?;
        Ok(Provider {
            client,
            llm_section,
            selector_item_layout: Selector::parse(".b_algo").unwrap(),
            selector_item_title: Selector::parse("h2").unwrap(),
            selector_item_head_read: Selector::parse(".b_caption").unwrap(),
            selector_item_link: Selector::parse("h2 > a").unwrap(),
        })
    }

    fn prepare_target_sources(&self, html_text: &str) -> anyhow::Result<Vec<Article>> {
        let sharked_html = trim_html_with_script_and_style(html_text);
        let document = Html::parse_document(sharked_html.as_str());

        let mut pending_result: Vec<Article> = Vec::new();
        for element in document.select(&self.selector_item_layout) {
            let title = element.select_text(&self.selector_item_title)?;
            let head_read = trim_head_read_days_ago(
                element
                    .select_text(&self.selector_item_head_read)
                    .unwrap_or_default(),
            );
            if let Ok(source_link) = element.select_attr_text(&self.selector_item_link, "href") {
                pending_result.push(Article {
                    title,
                    head_read: Some(head_read),
                    date_created: String::new(),
                    source_link,
                    summary: None,
                    content: None,
                    date_read: None,
                });
            } else {
                debug!("the tag a or the attribute href not found when execute bing::prepare_target_sources, title = {}, head_read = {}", title, head_read);
            }
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
            match article::read(
                &source_link,
                Some(String::from("bing.com")),
                self.llm_section.clone(),
            )
            .await
            {
                Ok(c) => {
                    result.push(Article {
                        title,
                        head_read,
                        date_created: String::new(),
                        source_link: c.1,
                        summary: None,
                        content: Some(c.0),
                        date_read: None,
                    });
                }
                Err(e) => error!(
                    "read article failure...title = {}, source_link = {}, error = {}",
                    title, source_link, e
                ),
            };
        }
        Ok(result)
    }
}

impl IProvider for Provider {
    async fn search_by_words(&self, words: Vec<&str>) -> anyhow::Result<Vec<Article>> {
        info!("内容清单搜索中...{:?}", words);
        let search_word = words
            .iter()
            .map(|word| encode(word).to_string())
            .collect::<Vec<String>>()
            .join("+")
            .to_string();
        let url = reqwest::Url::parse(
            format!(
                r#"https://www.bing.com/search?ensearch=1&q={}&filters=ex1:%22ez2%22&rdr=1"#,
                search_word
            )
            .as_str(),
        )?;
        let response = self.client.get(url).send().await?;
        let html_text = response.text().await?;
        info!("获得内容清单，深取内容中");
        self.convert(html_text).await
    }
}
