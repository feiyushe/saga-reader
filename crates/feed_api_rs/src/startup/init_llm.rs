use ollama::{ProgramStatus, query_platform};
use types::{LLMProviderType, LLMSection};

use super::task::{InitTask, TaskInitializer};

pub async fn call(llm_section: &LLMSection) -> anyhow::Result<()> {
    match llm_section.active_provider_type {
        LLMProviderType::Ollama => {
            let mut task = InitTask::default();
            task.start("llm_provider_ollama", || async {
                let ollama_information = query_platform(&llm_section.provider_ollama.endpoint).await?;
                match ollama_information.status {
                    // 在启动环节如果未安装Ollama不自动引导安装，只记录状态。
                    ProgramStatus::Uninstall => {}
                    // 在启动环节如果已安装Ollama但未运行则唤起Ollama。
                    ProgramStatus::InstallButNotRunning => ollama::launch().await?,
                    ProgramStatus::Running => {}
                };
                Ok(ollama_information.status)
            })
                .await?;
            Ok(())
        }
        _ => {
            let mut task = InitTask::default();
            task.start("llm_provider_others", || async { Ok(()) }).await?;
            Ok(())
        }
    }
}
