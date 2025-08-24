use types::{GLMLLMProvider, OpenAILLMProvider};

use crate::connector;
use crate::providers::llm_openaibase_like::OpenAILikeCompletionService;
use crate::providers::types::{AITargetOption, CompletionService};

pub struct GLMCompletionService {
    inner: OpenAILikeCompletionService
}

impl GLMCompletionService {
    pub fn new(config: &GLMLLMProvider, system_prompt: String, _options: AITargetOption) -> anyhow::Result<GLMCompletionService> {
        Ok(
            GLMCompletionService {
                inner: OpenAILikeCompletionService {
                    system_prompt,
                    config: OpenAILLMProvider {
                        model_name: config.model_name.clone(),
                        api_base_url: config.api_base_url.clone(),
                        api_key: config.api_key.clone(),
                    },
                    client: connector::new()?,
                }
            }
        )
    }
}

impl CompletionService for GLMCompletionService {
    async fn completion(&self, content: String) -> anyhow::Result<String> {
        self.inner.completion(content).await
    }
}