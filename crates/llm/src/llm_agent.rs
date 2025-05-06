use types::{LLMProviderType, LLMSection};

use crate::providers::llm_glm::GLMCompletionService;
use crate::providers::llm_mistral::MistralQinoAgentService;
use crate::providers::llm_ollama::OllamaCompletionService;
use crate::providers::llm_platform::PlatformAgentService;
use crate::providers::types::{AITargetOption, CompletionService};

enum CompletionServiceEnums {
    Ollama(OllamaCompletionService),
    Mistral(MistralQinoAgentService),
    Platform(PlatformAgentService),
    GLM(GLMCompletionService),
}

/// LLM Generate服务代理
pub struct CompletionAgent {
    provider: CompletionServiceEnums,
    // 还不能用box，box的trait需要对象安全，即函数中不能使用async、self、泛型等。
}

impl CompletionAgent {
    pub fn new(llm_section: LLMSection, system_prompt: String, options: AITargetOption) -> anyhow::Result<CompletionAgent> {
        let provider_type = llm_section.active_provider_type;

        match provider_type {
            LLMProviderType::Ollama => {
                let provider = OllamaCompletionService::new(&llm_section.provider_ollama, system_prompt, options)?;
                Ok(
                    CompletionAgent {
                        provider: CompletionServiceEnums::Ollama(provider),
                    }
                )
            }
            LLMProviderType::Mistral => {
                let provider = MistralQinoAgentService::new(system_prompt, options)?;
                Ok(
                    CompletionAgent {
                        provider: CompletionServiceEnums::Mistral(provider),
                    }
                )
            }
            LLMProviderType::Platform => {
                let provider = PlatformAgentService::new(&llm_section.provider_platform, system_prompt, options)?;
                Ok(
                    CompletionAgent {
                        provider: CompletionServiceEnums::Platform(provider),
                    }
                )
            }
            LLMProviderType::GLM => {
                let provider = GLMCompletionService::new(&llm_section.provider_glm, system_prompt, options)?;
                Ok(
                    CompletionAgent {
                        provider: CompletionServiceEnums::GLM(provider),
                    }
                )
            }
            _ => todo!()
        }
    }

    /// 调用LLM Completion能力，参数`message`会被作为user prompt传递给LLM。
    pub async fn completion(&self, message: String) -> anyhow::Result<String> {
        match &self.provider {
            CompletionServiceEnums::Ollama(p) => p.completion(message).await,
            CompletionServiceEnums::Mistral(p) => p.completion(message).await,
            CompletionServiceEnums::Platform(p) => p.completion(message).await,
            CompletionServiceEnums::GLM(p) => p.completion(message).await,
        }
    }
}
