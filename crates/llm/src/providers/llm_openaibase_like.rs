use reqwest::Client;
use serde::{Deserialize, Serialize};
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
        let response = self
            .client
            .post(self.config.api_base_url.clone())
            .header(
                "Authorization",
                format!("Bearer {}", self.config.api_key.clone()),
            )
            .json(&parameter)
            .send()
            .await?;
        let resp: Response = response.json().await?;
        let content = match &resp.choices[0].message {
            Some(m) => m.content.clone(),
            None => String::new(),
        };
        Ok(content)
    }
}
