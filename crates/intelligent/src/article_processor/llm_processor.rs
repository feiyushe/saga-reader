use llm::llm_agent::CompletionAgent;
use llm::providers::types::AITargetOption;
use types::{Article, LLMSection};

use crate::article_processor::types::IArticleProcessor;

/// 基于LLM的文章处理器。
pub struct ArticleLLMProcessor {
    /// Agent化的生成式服务实例。
    agent: CompletionAgent,
    /// 用于与Agent交互的user prompt。
    user_prompt_command: String,
}

/// 预设文章处理器创建trait，所有文章预处理器都通过impl此trait创建[ArticleLLMProcessor][ArticleLLMProcessor]的实例以支持框架层面调度。
pub trait IPresetArticleLLMProcessor {
    fn new_processor(llm_section: LLMSection) -> anyhow::Result<ArticleLLMProcessor>;
}

impl IArticleProcessor for ArticleLLMProcessor {
    async fn process(&self, input: &Article) -> anyhow::Result<Article> {
        let mut output = input.clone();
        let content = output.content.as_ref().unwrap();
        let mut chat = format!(r#"## 原内容\n"{}"\n"#, content);
        chat.push_str(self.user_prompt_command.as_str());
        let content = self.agent.completion(chat).await?;
        output.content.replace(content);
        Ok(output)
    }
}

impl ArticleLLMProcessor {
    pub fn new(
        llm_section: LLMSection,
        system_prompt: String,
        user_prompt_command: String,
        options: AITargetOption,
    ) -> anyhow::Result<Self> {
        Ok(
            ArticleLLMProcessor {
                agent: CompletionAgent::new(llm_section, system_prompt, options)?,
                user_prompt_command,
            }
        )
    }
}
