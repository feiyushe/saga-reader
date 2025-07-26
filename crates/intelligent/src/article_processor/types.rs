use types::{Article, LLMInstructOption};

/// 文章处理器trait，所有文章处理器都应该impl这个trait以形成责任链调度。
pub trait IArticleProcessor {
    /// 处理文章，输入为[Article][types::Article]，输出为`Article`的[future][std::future::Future]。
    /// 函数本身不会修改输入的`Article`，且参数本身为不可变引用，在流水线中作为先前的副本。
    fn process(
        &self,
        input: &Article,
        opt: LLMInstructOption,
    ) -> impl std::future::Future<Output = anyhow::Result<Article>>;
}
