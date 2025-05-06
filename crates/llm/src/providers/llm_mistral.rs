use reqwest::Client;
use serde::Serialize;

use crate::connector;
use crate::providers::types::{AITargetOption, CompletionService};

const API_AGENT_QINO_COMPLETIONS_FULL: &str = "https://api.mistral.ai/v1/agents/completions";
const MISTRAL_AGENT_QINO_ID: &str = "ag:52270968:20250416:universal-chat:db393378";
const MISTRAL_AGENT_QINO_KEY: &str = "Bearer vpWsC31ZMuTLmMWCBkQ3oJE095IrWkWe";

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
    system_prompt: String,
    client: Client,
}

impl MistralQinoAgentService {
    pub fn new(system_prompt: String, _options: AITargetOption) -> anyhow::Result<MistralQinoAgentService> {
        Ok(
            MistralQinoAgentService {
                system_prompt,
                client: connector::new()?,
            }
        )
    }
}

impl CompletionService for MistralQinoAgentService {
    async fn completion(&self, content: String) -> anyhow::Result<String> {
        let parameter = RequestParameters {
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: self.system_prompt.to_owned(),
                },
                Message {
                    role: "user".to_string(),
                    content,
                },
            ],
            agent_id: MISTRAL_AGENT_QINO_ID.to_owned(),
        };
        let response =
            self.client
                .post(API_AGENT_QINO_COMPLETIONS_FULL)
                .header("Authorization", MISTRAL_AGENT_QINO_KEY)
                .json(&parameter)
                .send().await?;
        let text = response.text().await?;
        Ok(text)
    }
}