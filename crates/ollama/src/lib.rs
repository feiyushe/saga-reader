use serde::{Deserialize, Serialize};
use tokio::process::Command;
use types::llm_endpoint::LLMEndPoint;

#[derive(Serialize, Deserialize, Clone)]
pub enum ProgramStatus {
    Uninstall,
    InstallButNotRunning,
    Running,
}

pub struct Information {
    pub version: String,
    pub status: ProgramStatus,
    pub extra: Option<String>,
}

#[derive(Deserialize)]
struct APIVersionResponse {
    version: String,
}

#[cfg(target_os = "windows")]
static PATH_TO_OLLAMA: &str = "ollama";
#[cfg(target_os = "macos")]
static PATH_TO_OLLAMA: &str = "/usr/local/bin/ollama";
#[cfg(target_os = "linux")]
static PATH_TO_OLLAMA: &str = "/usr/local/bin/ollama";

pub async fn install() {}

pub async fn launch() -> anyhow::Result<()> {
    create_shell_command()
        .arg(format!("{PATH_TO_OLLAMA} list"))
        .output()
        .await?;
    Ok(())
}

#[cfg(target_family = "unix")]
fn create_shell_command() -> Command {
    let mut cmd = Command::new("sh");
    cmd.arg("-c");
    cmd
}

#[cfg(target_family = "windows")]
fn create_shell_command() -> Command {
    // use std::os::windows::process::CommandExt;
    let mut cmd = Command::new("cmd");
    cmd.arg("/C").creation_flags(0x08000000);
    cmd
}

pub async fn request_running(llm_endpoint: &LLMEndPoint) -> anyhow::Result<bool> {
    let raw_str = reqwest::get(&llm_endpoint.api_base_url)
        .await?
        .text()
        .await?;
    Ok(raw_str.eq("Ollama is running"))
}

pub async fn request_version(llm_endpoint: &LLMEndPoint) -> anyhow::Result<String> {
    let url = [llm_endpoint.api_base_url.as_str(), "/api/version"].join("");
    let body: APIVersionResponse = reqwest::get(url).await?.json().await?;
    Ok(body.version)
}

pub async fn query_platform(llm_endpoint: &LLMEndPoint) -> anyhow::Result<Information> {
    match query_platform_by_process().await {
        Ok(information) => match information.status {
            ProgramStatus::Uninstall => query_platform_by_remote(llm_endpoint).await,
            ProgramStatus::InstallButNotRunning => Ok(information),
            ProgramStatus::Running => Ok(information),
        },
        Err(_) => query_platform_by_remote(&LLMEndPoint::default()).await,
    }
}

async fn query_platform_by_remote(llm_endpoint: &LLMEndPoint) -> anyhow::Result<Information> {
    let has_running = request_running(llm_endpoint).await?;
    if !has_running {
        return Ok(Information {
            version: "-".into(),
            status: ProgramStatus::Uninstall,
            extra: None,
        });
    }
    let version = request_version(llm_endpoint).await?;
    Ok(Information {
        version,
        status: ProgramStatus::Running,
        extra: None,
    })
}

async fn query_platform_by_process() -> anyhow::Result<Information> {
    match create_shell_command()
        .arg(format!("{PATH_TO_OLLAMA} -v"))
        .output()
        .await
    {
        Ok(output) => {
            if output.status.success() {
                let stdout_str = std::str::from_utf8(&output.stdout)?;
                let status = match parse_is_running_from_version(stdout_str) {
                    true => ProgramStatus::Running,
                    false => ProgramStatus::InstallButNotRunning,
                };
                return Ok(Information {
                    version: parse_version(stdout_str).into(),
                    status,
                    extra: None,
                });
            }
            let stderr_str = std::str::from_utf8(&output.stderr)?;
            Ok(Information {
                version: "-".into(),
                status: ProgramStatus::Uninstall,
                extra: Some(stderr_str.to_owned()),
            })
        }
        Err(err) => Ok(Information {
            version: "-".into(),
            status: ProgramStatus::Uninstall,
            extra: Some(err.to_string()),
        }),
    }
}

fn parse_version(version_output: &str) -> &str {
    match version_output.find(" version is ") {
        Some(index) => version_output.split_at(index + 12).1,
        None => "unknown",
    }
}

fn parse_is_running_from_version(version_output: &str) -> bool {
    !version_output.contains("could not connect to a running Ollama instance")
}
