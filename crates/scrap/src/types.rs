use tauri::{AppHandle, Runtime};
use types::{Article, FeedTargetDescription, LLMSection};

pub trait IFetcher {
    fn fetch<R: Runtime>(
        &self,
        app_handle: Option<AppHandle<R>>,
        llm_section: &LLMSection,
        ftd: FeedTargetDescription,
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<Article>>> + Send;
}
