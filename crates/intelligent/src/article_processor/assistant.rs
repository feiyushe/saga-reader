use llm::llm_agent::CompletionAgent;
use llm::providers::types::AITargetOption;
use types::{ConversationMessage, LLMSection};

const SYSTEM_PROMPT: &str = include_str!("prompts/assistant_sys.prompt");
const USER_PROMPT_COMMAND_PURGE: &str = include_str!("prompts/assistant_suffix.prompt");

pub struct Assistant {
    agent: CompletionAgent,
    user_prompt_command: String,
}

impl Assistant {
    pub fn new(llm_section: LLMSection) -> Assistant {
        let options = AITargetOption {
            temperature: Some(0.7),
            ..Default::default()
        };
        Assistant {
            agent: CompletionAgent::new(llm_section.clone(), SYSTEM_PROMPT.into(), options).unwrap(),
            user_prompt_command: USER_PROMPT_COMMAND_PURGE.into(),
        }
    }

    pub async fn chat(&self, article: String, user_prompt: &str, history: Vec<ConversationMessage>) -> anyhow::Result<String> {
        let conversation_description = history.iter().map(|message| {
            format!("{}说：{}", message.role, message.payload)
        }).collect::<Vec<String>>().join("\n");
        let mut chat = format!(
            "### 文章正文：\
            {} \
            ### 用户会话历史记录\
            {} \
            ### 用户本次的提问 \
            {} \
            ",
            article, conversation_description, user_prompt
        );
        chat.push_str(self.user_prompt_command.as_str());
        let output = self.agent.completion(chat).await?;
        Ok(output)
    }
}
