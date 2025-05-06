use llm::providers::types::AITargetOption;
use types::LLMSection;

use crate::article_processor::llm_processor::{ArticleLLMProcessor, IPresetArticleLLMProcessor};

const SYSTEM_PROMPT: &str = include_str!("prompts/optimizer_sys.prompt");
const USER_PROMPT_COMMAND_OPTIMIZE: &str = include_str!("prompts/optimizer_suffix.prompt");

pub struct Optimizer {}

impl IPresetArticleLLMProcessor for Optimizer {
    fn new_processor(llm_section: LLMSection) -> anyhow::Result<ArticleLLMProcessor> {
        let options = AITargetOption {
            temperature: Some(0.1),
            ..Default::default()
        };
        ArticleLLMProcessor::new(llm_section, SYSTEM_PROMPT.into(), USER_PROMPT_COMMAND_OPTIMIZE.into(), options)
    }
}
