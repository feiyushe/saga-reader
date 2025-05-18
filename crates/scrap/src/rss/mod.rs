use crate::types::IFetcher;
use rss::Channel;
use tauri::{AppHandle, Runtime};
use types::{Article, FeedTargetDescription, LLMSection};
use crate::article_reader;

#[derive(Default)]
pub struct RSSFetcher {}

impl IFetcher for RSSFetcher {
    async fn fetch<R: Runtime>(
        &self,
        _app_handle: Option<AppHandle<R>>,
        llm_section: &LLMSection,
        ftd: FeedTargetDescription,
    ) -> anyhow::Result<Vec<Article>> {
        let llm_section = llm_section.clone();
        match ftd.data.get(0) {
            Some(url) => {
                let content = reqwest::get(url).await?.bytes().await?;
                let channel = Channel::read_from(&content[..])?;

                let mut articles = vec![];

                for item in channel.items().iter() {
                    let title = item.title().unwrap_or("").to_string();
                    let source_link = item.link().unwrap_or("").to_string();
                    let head_read = item.description().unwrap_or("").to_string();

                    match article_reader::read(
                        &source_link,
                        None,
                        llm_section.clone(),
                    ).await {
                        Ok(c) => {
                            articles.push(Article {
                                title,
                                source_link,
                                head_read: Some(head_read),
                                content: Some(c.0),
                                date_created: "".to_string(),
                                summary: None,
                                date_read: None,
                            });
                        }
                        Err(_) => {}
                    };
                }
                Ok(articles)
            }
            None => Err(anyhow::Error::msg("bad rss feed, the url is missing")),
        }
    }
}
