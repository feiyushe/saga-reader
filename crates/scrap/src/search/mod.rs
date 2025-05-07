use ::types::Article;
use tauri::{AppHandle, Runtime};

use crate::search::types::IProvider;

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
