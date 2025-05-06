use reqwest::Client;
use serde::Serialize;

use types::PlatformLLMProvider;

use crate::connector;
use crate::providers::types::{AITargetOption, CompletionService};

#[derive(Serialize)]
pub struct RequestParameters {
    prompt: String,
}

pub struct PlatformAgentService {
    config: PlatformLLMProvider,
    client: Client,
}

impl PlatformAgentService {
    pub fn new(config: &PlatformLLMProvider, _system_prompt: String, _options: AITargetOption) -> anyhow::Result<PlatformAgentService> {
        Ok(
            PlatformAgentService {
                config: config.clone(),
                client: connector::new()?,
            }
        )
    }
}

impl CompletionService for PlatformAgentService {
    async fn completion(&self, prompt: String) -> anyhow::Result<String> {
        // TODO: 启动服务
        let parameter = RequestParameters {
            prompt
        };
        let response =
            self.client
                .post("".to_owned())
                .json(&parameter)
                .send().await?;
        let text = response.text().await?;
        Ok(text)
    }
}