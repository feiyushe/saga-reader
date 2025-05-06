use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct AITargetOption {
    pub temperature: Option<f32>,
    pub seed: Option<u32>,
    pub top_k: Option<u32>,
    pub top_p: Option<f32>,
    pub num_ctx: Option<u32>,
}

impl Default for AITargetOption {
    fn default() -> Self {
        AITargetOption {
            temperature: Some(0.0),
            seed: Some(0),
            top_k: Some(40),
            top_p: Some(0.9),
            num_ctx: Some(4096),
        }
    }
}

/// LLM Generate服务代理
pub trait CompletionService {
    /// 调用LLM Completion能力，参数`message`会被作为user prompt传递给LLM。
    fn completion(&self, message: String) -> impl std::future::Future<Output=anyhow::Result<String>>;
}
