use tauri::{AppHandle, Runtime};
use types::Article;

pub trait IProvider {
    fn search_by_words<R: Runtime>(
        &self,
        words: Vec<&str>,
        app_handle: Option<AppHandle<R>>,
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<Article>>>;
}
