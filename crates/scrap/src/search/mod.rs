use ::types::Article;

use crate::search::types::IProvider;

pub mod baidu;
pub mod types;
pub mod bing;
pub(crate) mod selector_extensions;
pub mod utils;

pub enum ScrapProviderEnums {
    Baidu(baidu::Provider),
    Bing(bing::Provider),
}

impl IProvider for ScrapProviderEnums {
    async fn search_by_words(&self, words: Vec<&str>) -> anyhow::Result<Vec<Article>> {
        match self {
            ScrapProviderEnums::Baidu(p) => p.search_by_words(words).await,
            ScrapProviderEnums::Bing(p) => p.search_by_words(words).await
        }
    }
}