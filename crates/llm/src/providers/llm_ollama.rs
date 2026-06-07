use reqwest::Client;
use serde::{Deserialize, Serialize};
use spdlog::error;

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

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            let err_msg = format!("Ollama API returned status {}: {}", status, body);
            error!("{}", err_msg);
            return Err(anyhow::Error::msg(err_msg));
        }

        let body_text = response.text().await?;
        let reply: CompletionReply = match serde_json::from_str(&body_text) {
            Ok(r) => r,
            Err(e) => {
                error!("Ollama API response JSON parse error: {}, body snippet: {:?}", e, &body_text[..body_text.len().min(500)]);
                return Err(anyhow::Error::msg(format!("Ollama API response JSON parse error: {}", e)));
            }
        };
        Ok(reply.response)
    }
}