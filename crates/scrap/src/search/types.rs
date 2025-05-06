use types::Article;

pub trait IProvider {
    fn search_by_words(
        &self,
        words: Vec<&str>,
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<Article>>>;
}
