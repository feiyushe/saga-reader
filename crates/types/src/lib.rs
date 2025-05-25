use std::cmp::Ordering;

use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

use crate::llm_endpoint::LLMEndPoint;

pub mod llm_endpoint;

#[derive(Serialize, Deserialize, Clone)]
pub struct Article {
    pub title: String,
    pub head_read: Option<String>,
    pub source_link: String,
    pub summary: Option<String>,
    pub content: Option<String>,
    pub date_created: String,
    pub date_read: Option<String>,
}

/// 日志配置节点，用于指定日志的的开启、输出模式等行为。
/// 该配置的应用详见[init_logger][feed_api_rs::startup::init_logger]
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct AppConfigLogSection {
    /// 是否启用日志功能，仅当未true时才会采纳log section中设定的属性。
    pub enable: bool,
    /// 日志输出类型，包括'stdout' or 'disk'，分别对应stdout和日志文件模式。
    pub output_type: OutputType,

    pub log_name_tail: String,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct DiagnosticSection {
    pub flame_whole: bool,
}

#[derive(Serialize, Deserialize, EnumString, Clone, Default)]
pub enum OutputType {
    #[default]
    #[strum(disabled)]
    UnSpecified,

    #[strum(serialize = "stdout")]
    #[serde(rename = "stdout")]
    Stdout,

    #[strum(serialize = "disk")]
    #[serde(rename = "disk")]
    Disk,
}

#[derive(Serialize, Deserialize, EnumString, Clone, Default)]
pub enum ScrapProviderType {
    #[strum(serialize = "baidu")]
    #[serde(rename = "baidu")]
    Baidu,

    #[default]
    #[strum(serialize = "bing")]
    #[serde(rename = "bing")]
    Bing,
}

#[derive(Serialize, Deserialize, EnumString, Clone, Default)]
pub enum LLMProviderType {
    #[strum(serialize = "ollama")]
    #[serde(rename = "ollama")]
    Ollama,

    #[strum(serialize = "platform")]
    #[serde(rename = "platform")]
    Platform,

    #[default]
    #[strum(serialize = "glm")]
    #[serde(rename = "glm")]
    GLM,

    #[strum(serialize = "openai")]
    #[serde(rename = "openai")]
    OpenAI,

    #[strum(serialize = "mistral")]
    #[serde(rename = "mistral")]
    Mistral,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LLMProvider {
    pub provider_type: LLMProviderType,
    pub template_path: String,
    pub model_path: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PlatformLLMProvider {
    pub template_path: String,
    pub model_path: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GLMLLMProvider {
    pub model_name: String,
    pub api_base_url: String,
    pub api_key: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OpenAILLMProvider {
    pub model_name: String,
    pub api_base_url: String,
    pub api_key: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OllamaLLMProvider {
    pub endpoint: LLMEndPoint,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LLMSection {
    pub provider_ollama: OllamaLLMProvider,
    pub provider_platform: PlatformLLMProvider,
    pub provider_glm: GLMLLMProvider,
    pub provider_openai: OpenAILLMProvider,
    pub active_provider_type: LLMProviderType,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct DaemonSection {
    pub frequency_feeds_update: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ScrapSection {
    pub provider: ScrapProviderType,
}

/// 端侧应用配置节点，包括LLM配置、日志配置等。
/// 应用内会包含一套默认的应用配置作为初始化。
#[derive(Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub llm: LLMSection,
    pub scrap: ScrapSection,
    pub log: AppConfigLogSection,
    pub daemon: DaemonSection,
    pub diagnostic: DiagnosticSection,
}

/// 端侧用户配置节点，订阅包与订阅信息。
/// 应用内会包含一套默认的应用配置作为初始化。
#[derive(Serialize, Deserialize, Clone)]
pub struct UserConfig {
    pub feeds_packages: Vec<FeedsPackage>,
}

impl UserConfig {
    /// 重命名feed
    /// 该操作仅更高内存数据本身。
    /// 如果无法找到对应的订阅包或订阅会返回false。
    pub fn rename_feed(&mut self, package_id: &str, feed_id: &str, new_name: &str) -> bool {
        match self.find_feeds_package_mut(package_id) {
            Some(package) => match search_feed_by_inner(feed_id, &package.feeds) {
                Ok(index) => {
                    let feed = package.feeds.get_mut(index).unwrap();
                    feed.name = new_name.into();
                    true
                }
                Err(_) => false,
            },
            None => false,
        }
    }

    pub fn change_feed_data(&mut self, package_id: &str, feed_id: &str, data: Vec<String>) -> bool {
        match self.find_feeds_package_mut(package_id) {
            Some(package) => match search_feed_by_inner(feed_id, &package.feeds) {
                Ok(index) => {
                    let feed = package.feeds.get_mut(index).unwrap();
                    feed.data = data;
                    true
                }
                Err(_) => false,
            },
            None => false,
        }
    }

    /// 移除feed
    /// 该操作仅更高内存数据本身。
    /// 如果无法找到对应的订阅包或订阅会返回false。
    pub fn remove_feed(&mut self, package_id: &str, feed_id: &str) -> bool {
        match self.find_feeds_package_mut(package_id) {
            Some(package) => match search_feed_by_inner(feed_id, &package.feeds) {
                Ok(index) => {
                    package.feeds.remove(index);
                    true
                }
                Err(_) => false,
            },
            None => false,
        }
    }

    /// 查找订阅包并返回一个副本
    pub fn find_feeds_package(&self, package_id: &str) -> Option<FeedsPackage> {
        match self.search_feeds_package_inner(package_id) {
            Ok(index) => Some(self.feeds_packages.get(index).unwrap().clone()),
            Err(_) => None,
        }
    }

    /// 查找订阅包并返回可变引用
    pub fn find_feeds_package_mut(&mut self, package_id: &str) -> Option<&mut FeedsPackage> {
        match self.search_feeds_package_inner(package_id) {
            Ok(index) => self.feeds_packages.get_mut(index),
            Err(_) => None,
        }
    }

    /// 查找订阅并返回一个副本
    pub fn find_feed(&self, package_id: &str, feed_id: &str) -> Option<FeedTargetDescription> {
        match self.search_feeds_package_inner(package_id) {
            Ok(index) => match self.feeds_packages.get(index) {
                Some(feeds_package) => match search_feed_by_inner(feed_id, &feeds_package.feeds) {
                    Ok(ftd_index) => Some(feeds_package.feeds.get(ftd_index).unwrap().clone()),
                    Err(_) => None,
                },
                None => None,
            },
            Err(_) => None,
        }
    }

    /// 增加一个订阅包，如果订阅包已存在则返回false。
    pub fn add_feeds_packages(&mut self, feeds_package: FeedsPackage) -> bool {
        if self.search_feeds_package_inner(&feeds_package.id).is_ok() {
            return false;
        }
        self.feeds_packages.push(feeds_package);
        true
    }

    /// 移除一个订阅包，如果订阅包已存在则返回false。
    pub fn remove_feeds_package(&mut self, package_id: &str) -> bool {
        match self.search_feeds_package_inner(package_id) {
            Ok(index) => {
                self.feeds_packages.remove(index);
                true
            }
            Err(_) => false,
        }
    }

    /// 移重命名一个订阅包，如果订阅包不存在则返回false。
    pub fn rename_feeds_package(&mut self, package_id: &str, new_name: &str) -> bool {
        match self.search_feeds_package_inner(package_id) {
            Ok(index) => {
                let feeds_package = self.feeds_packages.get_mut(index).unwrap();
                feeds_package.name = new_name.to_owned();
                true
            }
            Err(_) => false,
        }
    }

    /// 查找订阅包的索引辅助函数
    fn search_feeds_package_inner(&self, package_id: &str) -> Result<usize, usize> {
        match self
            .feeds_packages
            .iter()
            .position(|probe| package_id.cmp(&probe.id) == Ordering::Equal)
        {
            None => Err(0),
            Some(idx) => Ok(idx),
        }
    }
}

fn search_feed_by_inner(feed_id: &str, feeds: &[FeedTargetDescription]) -> Result<usize, usize> {
    match feeds
        .iter()
        .position(|prob| feed_id.cmp(&prob.id) == Ordering::Equal)
    {
        None => Err(0),
        Some(idx) => Ok(idx),
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FeedsPackage {
    pub id: String,
    pub name: String,
    pub feeds: Vec<FeedTargetDescription>,
    pub is_flat_on_root: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FeedTargetDescription {
    // 用于做查询用的group id。
    pub id: String,
    pub name: String,
    pub fetcher_id: String,
    pub data: Vec<String>,
}

#[derive(Serialize, Deserialize, EnumString, Clone, Default, Display)]
pub enum ConversationMessageRoleType {
    #[default]
    #[strum(disabled)]
    UnSpecified,

    #[strum(serialize = "system")]
    #[serde(rename = "system")]
    System,

    #[strum(serialize = "user")]
    #[serde(rename = "user")]
    User,

    #[strum(serialize = "assistant")]
    #[serde(rename = "assistant")]
    Assistant,
}

#[derive(Serialize, Deserialize, EnumString, Clone, Default, Display)]
pub enum ConversationMessagePayloadType {
    #[default]
    #[strum(disabled)]
    UnSpecified,

    #[strum(serialize = "text")]
    #[serde(rename = "text")]
    Text,

    #[strum(serialize = "image")]
    #[serde(rename = "image")]
    Image,

    #[strum(serialize = "video")]
    #[serde(rename = "video")]
    Video,

    #[strum(serialize = "audio")]
    #[serde(rename = "audio")]
    Audio,

    #[strum(serialize = "file")]
    #[serde(rename = "file")]
    File,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ConversationMessage {
    pub role: ConversationMessageRoleType,
    pub mtype: ConversationMessagePayloadType,
    pub payload: String,
    pub created_at: String,
}
