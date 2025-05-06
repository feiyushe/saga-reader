use reqwest::Client;
use serde::{Deserialize, Serialize};

use types::GLMLLMProvider;

use crate::connector;
use crate::providers::types::{AITargetOption, CompletionService};

// const GLM_API_KEY: &str = "bbffaa065195e72e243c22cd574de0d2.WNJdTbrkP2xHkihK";

#[derive(Serialize)]
pub struct RequestParameters {
    model: String,
    messages: Vec<Message>,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
pub struct Response {
    choices: Vec<ChoiceChunk>,
}

#[derive(Deserialize)]
pub struct ChoiceChunk {
    finish_reason: String,
    message: Message,
}

pub struct GLMCompletionService {
    config: GLMLLMProvider,
    system_prompt: String,
    client: Client,
}

impl GLMCompletionService {
    pub fn new(config: &GLMLLMProvider, system_prompt: String, _options: AITargetOption) -> anyhow::Result<GLMCompletionService> {
        Ok(
            GLMCompletionService {
                system_prompt,
                config: config.clone(),
                client: connector::new()?,
            }
        )
    }
}

impl CompletionService for GLMCompletionService {
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
        let response =
            self.client
                .post(self.config.api_base_url.clone())
                .header("Authorization", format!("Bearer {}", self.config.api_key.clone()))
                .json(&parameter)
                .send().await?;
        let resp: Response = response.json().await?;
        Ok(resp.choices[0].message.content.clone())
    }
}