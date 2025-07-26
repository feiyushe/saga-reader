use llm::llm_agent::CompletionAgent;
use llm::providers::types::AITargetOption;
use types::{Article, LLMInstructOption, LLMSection};

use crate::article_processor::types::IArticleProcessor;
use sys_locale::get_locale;

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
    async fn process(&self, input: &Article, opt: LLMInstructOption) -> anyhow::Result<Article> {
        let mut output = input.clone();
        let content = output.content.as_ref().unwrap();
        let lang = {
            if opt.lang.as_str() == "system" {
                get_locale().unwrap_or_else(|| String::from("en-US"))
            } else {
                opt.lang.to_owned()
            }
        };

        let prompt_spec_lang = format!(
            "## 语言要求：\n请使用{}语种输出内容，如果原文中存在其他语言则同样翻译为这个语种，代码块、姓名、英文简写除外。",
            lang
        );
        let chat = format!(
            r#"## 原内容\n"{}"\n{}\n{}"#,
            content,
            self.user_prompt_command.as_str(),
            prompt_spec_lang
        );
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
        Ok(ArticleLLMProcessor {
            agent: CompletionAgent::new(llm_section, system_prompt, options)?,
            user_prompt_command,
        })
    }
}
