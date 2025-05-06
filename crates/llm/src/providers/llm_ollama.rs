use reqwest::Client;
use serde::{Deserialize, Serialize};

use types::llm_endpoint::LLMEndPoint;
use types::OllamaLLMProvider;

use crate::connector;
use crate::providers::types::{AITargetOption, CompletionService};

#[derive(Serialize, Deserialize)]
struct RequestParameter {
    model: String,
    system: String,
    prompt: String,
    options: AITargetOption,
    images: Option<String>,
    format: Option<String>,
    keep_alive: String,
    stream: bool,
}

#[derive(Deserialize)]
struct CompletionReply {
    response: String,
}

pub struct OllamaCompletionService {
    endpoint: LLMEndPoint,
    options: AITargetOption,
    client: Client,
    system_prompt: String,
}

impl OllamaCompletionService {
    pub fn new(config: &OllamaLLMProvider, system_prompt: String, options: AITargetOption) -> anyhow::Result<OllamaCompletionService> {
        let endpoint = config.endpoint.clone();
        Ok(
            OllamaCompletionService {
                endpoint,
                options,
                system_prompt,
                client: connector::new()?,
            }
        )
    }
}

impl CompletionService for OllamaCompletionService {
    async fn completion(&self, content: String) -> anyhow::Result<String> {
        let parameter = RequestParameter {
            model: self.endpoint.model.to_string(),
            system: self.system_prompt.to_string(),
            prompt: content,
            options: self.options.clone(),
            images: None,
            format: None,
            keep_alive: "5m".to_string(),
            stream: false,
        };
        let url = self.endpoint.get_api_generate_completion();
        let response = self.client.post(url).json(&parameter).send().await?;
        let reply: CompletionReply = response.json().await?;
        Ok(reply.response)
    }
}