use llm::providers::types::AITargetOption;
use types::LLMSection;

use crate::article_processor::llm_processor::{ArticleLLMProcessor, IPresetArticleLLMProcessor};

const SYSTEM_PROMPT: &str = include_str!("prompts/melt_sys.prompt");
const USER_PROMPT_COMMAND_PURGE: &str = include_str!("prompts/melt_suffix.prompt");

pub struct Melt {}

impl IPresetArticleLLMProcessor for Melt {
    fn new_processor(llm_section: LLMSection) -> anyhow::Result<ArticleLLMProcessor> {
        let options = AITargetOption {
            temperature: Some(0.7),
            ..Default::default()
        };
        ArticleLLMProcessor::new(llm_section, SYSTEM_PROMPT.into(), USER_PROMPT_COMMAND_PURGE.into(), options)
    }
}
