use reqwest::Client;
use serde::{Deserialize, Serialize};
use spdlog::error;
use types::OpenAILLMProvider;

use crate::connector;
use crate::providers::types::{AITargetOption, CompletionService};

#[derive(Serialize)]
pub struct RequestParameters {
    pub(crate) model: String,
    pub(crate) messages: Vec<Message>,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
pub struct Response {
    pub(crate) choices: Vec<ChoiceChunk>,
}

#[derive(Deserialize)]
pub struct ChoiceChunk {
    #[allow(dead_code)]
    finish_reason: Option<String>,
    message: Option<Message>,
}

pub struct OpenAILikeCompletionService {
    pub(crate) config: OpenAILLMProvider,
    pub(crate) system_prompt: String,
    pub(crate) client: Client,
}

impl OpenAILikeCompletionService {
    pub fn new(
        config: &OpenAILLMProvider,
        system_prompt: String,
        _options: AITargetOption,
    ) -> anyhow::Result<OpenAILikeCompletionService> {
        Ok(OpenAILikeCompletionService {
            system_prompt,
            config: config.clone(),
            client: connector::new()?,
        })
    }
}

impl CompletionService for OpenAILikeCompletionService {
    async fn completion(&self, content: String) -> anyhow::Result<String> {
        let sys_prompt = Message {
            role: "system".to_string(),
            content: self.system_prompt.to_string(),
        };

        let message = Message {
            role: "user".to_string(),
            content,
        };
        let parameter = RequestParameters {
            model: self.config.model_name.clone(),
            messages: vec![sys_prompt, message],
        };
        // Auto-append /chat/completions if the base URL doesn't already end with it
        let url = if self.config.api_base_url.ends_with("/chat/completions") {
            self.config.api_base_url.clone()
        } else {
            let base = self.config.api_base_url.trim_end_matches('/');
            format!("{}/chat/completions", base)
        };
        spdlog::debug!("LLM request URL: {}", url);

        let response = self
            .client
            .post(url)
            .header(
                "Authorization",
                format!("Bearer {}", self.config.api_key.clone()),
            )
            .json(&parameter)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            let err_msg = format!("LLM API returned status {}: {}", status, body);
            error!("{}", err_msg);
            return Err(anyhow::Error::msg(err_msg));
        }

        let body_text = response.text().await?;
        let resp: Response = match serde_json::from_str(&body_text) {
            Ok(r) => r,
            Err(e) => {
                error!("LLM API response JSON parse error: {}, body snippet: {:?}", e, &body_text[..body_text.len().min(500)]);
                return Err(anyhow::Error::msg(format!("LLM API response JSON parse error: {}", e)));
            }
        };
        let content = match &resp.choices[0].message {
            Some(m) => m.content.clone(),
            None => String::new(),
        };
        Ok(content)
    }
}
