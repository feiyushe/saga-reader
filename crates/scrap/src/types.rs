use tauri::{AppHandle, Runtime};
use types::{Article, FeedTargetDescription, LLMSection};

pub trait IFetcher {
    async fn fetch<R: Runtime>(
        &self,
        app_handle: Option<AppHandle<R>>,
        llm_section: &LLMSection,
        ftd: FeedTargetDescription,
    ) -> anyhow::Result<Vec<Article>>;
}
