use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
};

use recorder::path::get_appdata_file;
use types::{
    AppConfig, DaemonSection, GLMLLMProvider, LLMInstructOption, LLMSection, OllamaLLMProvider, OpenAILLMProvider,
    PlatformLLMProvider, ScrapSection,
};

use super::task::{InitTask, TaskInitializer};

const FILE_NAME_APP_CONFIG: &str = "app_config.toml";

pub async fn call() -> anyhow::Result<InitTask<AppConfig>> {
    let mut task = InitTask::default();
    task.start("app_config", || async {
        let app_config_path = get_appdata_file(FILE_NAME_APP_CONFIG);
        Ok(match File::open(app_config_path).await {
            Ok(mut file) => {
                let mut data_raw = String::new();
                file.read_to_string(&mut data_raw).await?;
                toml::from_str(data_raw.as_str())?
            }
            Err(_) => {
                let app_config = default_app_config();
                sync_to(&app_config).await?;
                app_config
            }
        })
    })
    .await?;
    Ok(task)
}

pub async fn sync_to(app_config: &AppConfig) -> anyhow::Result<()> {
    let user_config_path = get_appdata_file(FILE_NAME_APP_CONFIG);
    let mut file = File::create(user_config_path).await?;
    let json_raw = toml::to_string(app_config)?;
    file.write_all(json_raw.as_bytes()).await?;
    Ok(())
}

fn default_app_config() -> AppConfig {
    AppConfig {
        llm: LLMSection {
            provider_ollama: OllamaLLMProvider {
                endpoint: Default::default(),
            },
            provider_platform: PlatformLLMProvider {
                template_path: "".to_string(),
                model_path: "".to_string(),
            },
            provider_glm: GLMLLMProvider {
                model_name: "GLM-4.5-Flash".to_string(),
                api_base_url: "https://open.bigmodel.cn/api/paas/v4/chat/completions".to_string(),
                api_key: "".to_string(),
            },
            provider_openai: OpenAILLMProvider {
                model_name: "".to_string(),
                api_base_url: "".to_string(),
                api_key: "".to_string(),
            },
            active_provider_type: Default::default(),
            instruct: LLMInstructOption::default(),
            max_parallel: Some(5),
        },
        scrap: ScrapSection {
            provider: Default::default(),
        },
        log: Default::default(),
        daemon: DaemonSection {
            frequency_feeds_update: Default::default(),
            enable_notification: true, // 默认开启通知
        },
        diagnostic: Default::default(),
    }
}
