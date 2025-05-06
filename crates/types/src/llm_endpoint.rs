use serde::{Deserialize, Serialize};

const DEFAULT_API_CHAT_COMPLETION: &str = "/api/chat";
const DEFAULT_API_GENERATE_COMPLETION: &str = "/api/generate";

const DEFAULT_MODEL_NAME: &str = "default_adaptived_llm";

const DEFAULT_LOCAL_BASE_URL: &str = "http://localhost:11434";

/// LLM EndPoint信息，目前只支持Ollama
///
/// 接口定义见<https://github.com/ollama/ollama/blob/main/docs/api.md>
#[derive(Serialize, Deserialize, Clone)]
pub struct LLMEndPoint {
    /// 加一个type
    /// 服务基地址
    pub api_base_url: String,
    /// Completion接口的path
    pub api_path_generate_completion: String,
    /// Chat接口的path
    pub api_path_chat_completion: String,
    /// 模型名称
    pub model: String,
}

impl Default for LLMEndPoint {
    fn default() -> Self {
        LLMEndPoint {
            api_base_url: DEFAULT_LOCAL_BASE_URL.into(),
            api_path_generate_completion: DEFAULT_API_GENERATE_COMPLETION.into(),
            api_path_chat_completion: DEFAULT_API_CHAT_COMPLETION.into(),
            model: DEFAULT_MODEL_NAME.into(),
        }
    }
}

impl LLMEndPoint {
    /// 获得chat接口的全地址
    pub fn get_api_chat_completion(&self) -> String {
        [
            self.api_base_url.as_str(),
            self.api_path_chat_completion.as_str(),
        ]
            .join("")
    }

    /// 获得completion接口的全地址
    pub fn get_api_generate_completion(&self) -> String {
        [
            self.api_base_url.as_str(),
            self.api_path_generate_completion.as_str(),
        ]
            .join("")
    }
}