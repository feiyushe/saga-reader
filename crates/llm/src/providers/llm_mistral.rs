use reqwest::Client;
use serde::Serialize;
use types::OpenAILLMProvider;

use crate::connector;
use crate::providers::llm_openaibase_like::OpenAILikeCompletionService;
use crate::providers::types::{AITargetOption, CompletionService};

const API_AGENT_QINO_COMPLETIONS_FULL: &str = "https://api.mistral.ai/v1/chat/completions";
const MISTRAL_AGENT_QINO_ID: &str = "";
const MISTRAL_AGENT_QINO_KEY: &str = "";

#[derive(Serialize)]
pub struct RequestParameters {
    messages: Vec<Message>,
    agent_id: String,
}

#[derive(Serialize)]
pub struct Message {
    role: String,
    content: String,
}

pub struct MistralQinoAgentService {
    inner: OpenAILikeCompletionService
}

impl MistralQinoAgentService {
    pub fn new(system_prompt: String, _options: AITargetOption) -> anyhow::Result<MistralQinoAgentService> {
        Ok(
            MistralQinoAgentService {
                inner: OpenAILikeCompletionService {
                    config: OpenAILLMProvider {
                        model_name: MISTRAL_AGENT_QINO_ID.to_string(),
                        api_base_url: API_AGENT_QINO_COMPLETIONS_FULL.to_string(),
                        api_key: MISTRAL_AGENT_QINO_KEY.to_string(),
                    },
                    system_prompt,
                    client: connector::new()?,
                }
            }
        )
    }
}

impl CompletionService for MistralQinoAgentService {
    async fn completion(&self, content: String) -> anyhow::Result<String> {
        self.inner.completion(content).await
    }
}