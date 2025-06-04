use ::types::{Article, FeedTargetDescription, LLMSection};
use spdlog::info;
use tauri::{AppHandle, Runtime};

use crate::search::types::IProvider;
use crate::types::IFetcher;

pub mod baidu;
pub mod bing;
pub(crate) mod selector_extensions;
pub mod types;
pub mod utils;

pub enum ScrapProviderEnums {
    Baidu(baidu::Provider),
    Bing(bing::Provider),
}

impl IProvider for ScrapProviderEnums {
    async fn search_by_words<R: Runtime>(
        &self,
        words: Vec<&str>,
        app_handle: Option<AppHandle<R>>,
    ) -> anyhow::Result<Vec<Article>> {
        match self {
            ScrapProviderEnums::Baidu(p) => p.search_by_words(words, app_handle).await,
            ScrapProviderEnums::Bing(p) => p.search_by_words(words, app_handle).await,
        }
    }
}

impl IFetcher for ScrapProviderEnums {
    async fn fetch<R: Runtime>(
        &self,
        app_handle: Option<AppHandle<R>>,
        _llm_section: &LLMSection,
        ftd: FeedTargetDescription,
    ) -> anyhow::Result<Vec<Article>> {
        let words: Vec<&str> = ftd.data.iter().map(|x| x.as_str()).collect();
        info!("scraping, via the words...{:?}", words);
        let articles = self.search_by_words(words, app_handle).await?;
        info!(
            "found {} articles for the feed_id = {}, feed_name = {}",
            articles.len(),
            ftd.id,
            ftd.name
        );
        Ok(articles)
    }
}
