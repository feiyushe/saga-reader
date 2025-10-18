use std::sync::Arc;

use spdlog::error;
use tauri::{AppHandle, Runtime, State};

use feed_api_rs::features::api::FeaturesAPI;
use ollama::ProgramStatus;
use recorder::entity::article_record::Model;
use types::{AppConfig, ConversationMessage, FeedTargetDescription, FeedsPackage};

use crate::scrap_host;
use crate::state::HybridRuntimeState;

// #[tauri::command(rename_all = "snake_case")]
// async fn template(state: State<'_, Mutex<HybridRuntimeState>>) -> Result<(), ()> {
//     let features_api = &mut state.lock().await.features_api;
//     todo!()
// }

#[tauri::command(rename_all = "snake_case")]
pub(crate) async fn add_feeds_package(
    state: State<'_, Arc<HybridRuntimeState>>,
    feeds_package: FeedsPackage,
) -> Result<(), ()> {
    let features_api = &state.features_api;
    convert_result(features_api.add_feeds_package(feeds_package).await)
}

#[tauri::command(rename_all = "snake_case")]
pub(crate) async fn remove_feeds_package(
    state: State<'_, Arc<HybridRuntimeState>>,
    package_id: &str,
) -> Result<(), ()> {
    let features_api = &state.features_api;
    convert_result(features_api.remove_feeds_package(package_id).await)
}

#[tauri::command(rename_all = "snake_case")]
pub(crate) async fn rename_feeds_package(
    state: State<'_, Arc<HybridRuntimeState>>,
    package_id: &str,
    new_name: &str,
) -> Result<(), ()> {
    let features_api = &state.features_api;
    convert_result(
        features_api
            .rename_feeds_package(package_id, new_name)
            .await,
    )
}

#[tauri::command(rename_all = "snake_case")]
pub(crate) async fn add_feed(
    state: State<'_, Arc<HybridRuntimeState>>,
    package_id: &str,
    ftd: FeedTargetDescription,
) -> Result<(), ()> {
    let features_api = &state.features_api;
    convert_result(features_api.add_feed(package_id, ftd).await)
}

#[tauri::command(rename_all = "snake_case")]
pub(crate) async fn remove_feed(
    state: State<'_, Arc<HybridRuntimeState>>,
    package_id: &str,
    feed_id: &str,
) -> Result<(), ()> {
    let features_api = &state.features_api;
    convert_result(features_api.remove_feed(package_id, feed_id).await)
}

#[tauri::command(rename_all = "snake_case")]
pub(crate) async fn rename_feed(
    state: State<'_, Arc<HybridRuntimeState>>,
    package_id: &str,
    feed_id: &str,
    new_name: &str,
) -> Result<(), ()> {
    let features_api = &state.features_api;
    convert_result(features_api.rename_feed(package_id, feed_id, new_name).await)
}

#[tauri::command(rename_all = "snake_case")]
pub(crate) async fn change_feed_data(
    state: State<'_, Arc<HybridRuntimeState>>,
    package_id: &str,
    feed_id: &str,
    data: Vec<String>,
) -> Result<(), ()> {
    let features_api = &state.features_api;
    convert_result(features_api.change_feed_data(package_id, feed_id, data).await)
}

#[tauri::command(rename_all = "snake_case")]
pub(crate) async fn get_feeds_packages(
    state: State<'_, Arc<HybridRuntimeState>>,
) -> Result<Vec<FeedsPackage>, ()> {
    let features_api = &state.features_api;
    Ok(features_api.get_feeds_packages().await)
}

#[tauri::command(rename_all = "snake_case")]
pub(crate) async fn get_feeds_by_package(
    state: State<'_, Arc<HybridRuntimeState>>,
    package_id: &str,
) -> Result<Option<FeedsPackage>, ()> {
    let features_api = &state.features_api;
    Ok(features_api.get_feeds_by_package(package_id).await)
}

#[tauri::command(rename_all = "snake_case")]
pub(crate) async fn read_feed_contents(
    state: State<'_, Arc<HybridRuntimeState>>,
    feed_id: &str,
    offset: u64,
    count: u64,
) -> Result<Vec<Model>, ()> {
    let features_api = &state.features_api;
    convert_result(features_api.read_feed_contents(feed_id, offset, count).await)
}

#[tauri::command(rename_all = "snake_case")]
pub(crate) async fn query_by_id(
    state: State<'_, Arc<HybridRuntimeState>>,
    id: i32,
) -> Result<Option<Model>, ()> {
    let features_api = &state.features_api;
    convert_result(features_api.query_by_id(id).await)
}

#[tauri::command(rename_all = "snake_case")]
pub(crate) async fn mark_as_read(
    state: State<'_, Arc<HybridRuntimeState>>,
    id: i32,
) -> Result<(), ()> {
    let features_api = &state.features_api;
    convert_result(features_api.mark_as_read(id).await)
}

#[tauri::command(rename_all = "snake_case")]
pub(crate) async fn set_favorite(
    state: State<'_, Arc<HybridRuntimeState>>,
    id: i32,
    favorite: bool,
) -> Result<(), ()> {
    let features_api = &state.features_api;
    convert_result(features_api.set_favorite(id, favorite).await)
}

#[tauri::command(rename_all = "snake_case")]
pub(crate) async fn get_app_config(
    state: State<'_, Arc<HybridRuntimeState>>,
) -> Result<AppConfig, ()> {
    let features_api = &state.features_api;
    convert_result(features_api.get_app_config().await)
}

#[tauri::command(rename_all = "snake_case")]
pub(crate) async fn set_app_config(
    state: State<'_, Arc<HybridRuntimeState>>,
    app_config: AppConfig,
) -> Result<(), ()> {
    let features_api = &state.features_api;
    convert_result(features_api.set_app_config(app_config).await)
}

#[tauri::command(rename_all = "snake_case")]
pub(crate) async fn get_ollama_status(
    state: State<'_, Arc<HybridRuntimeState>>,
) -> Result<ProgramStatus, ()> {
    let features_api = &state.features_api;
    convert_result(features_api.get_ollama_status().await)
}

#[tauri::command(rename_all = "snake_case")]
pub(crate) async fn download_ollama(state: State<'_, Arc<HybridRuntimeState>>) -> Result<(), ()> {
    let features_api = &state.features_api;
    convert_result(features_api.download_ollama().await)
}

#[tauri::command(rename_all = "snake_case")]
pub(crate) async fn launch_ollama(state: State<'_, Arc<HybridRuntimeState>>) -> Result<(), ()> {
    let features_api = &state.features_api;
    convert_result(features_api.launch_ollama().await)
}

#[tauri::command(rename_all = "snake_case")]
pub(crate) async fn open_article_external(
    state: State<'_, Arc<HybridRuntimeState>>,
    url: &str,
) -> Result<(), ()> {
    let features_api = &state.features_api;
    convert_result(features_api.open_article_external(url).await)
}

#[tauri::command(rename_all = "snake_case")]
pub(crate) async fn scrap_text_by_url<R: Runtime>(
    app_handle: AppHandle<R>,
    state: State<'_, Arc<HybridRuntimeState>>,
    url: &str,
) -> Result<String, ()> {
    convert_result(scrap_host::scrap_text_by_url(app_handle, url).await)
}

#[tauri::command(rename_all = "snake_case")]
pub(crate) async fn update_article_by_source(
    state: State<'_, Arc<HybridRuntimeState>>,
    article_id: i32,
    source_text: String,
) -> Result<bool, ()> {
    let features_api = &state.features_api;
    convert_result(
        features_api
            .update_article_by_source(article_id, source_text)
            .await,
    )
}

#[tauri::command(rename_all = "snake_case")]
pub(crate) async fn chat_with_article_assistant<R: Runtime>(
    app_handle: AppHandle<R>,
    state: State<'_, Arc<HybridRuntimeState>>,
    article_id: i32,
    user_prompt: &str,
    history: Vec<ConversationMessage>,
) -> Result<String, ()> {
    let features_api = &state.features_api;
    convert_result(
        features_api
            .chat_with_article_assistant(article_id, user_prompt, history)
            .await,
    )
}

#[tauri::command(rename_all = "snake_case")]
pub(crate) async fn update_feed_contents<R: Runtime>(
    app_handle: AppHandle<R>,
    state: State<'_, Arc<HybridRuntimeState>>,
    package_id: &str,
    feed_id: &str,
) -> Result<usize, ()> {
    let features_api = &state.features_api;
    convert_result(
        features_api
            .update_feed_contents(package_id, feed_id, Some(app_handle))
            .await,
    )
}

#[tauri::command(rename_all = "snake_case")]
pub(crate) async fn search_contents_by_keyword(
    state: State<'_, Arc<HybridRuntimeState>>,
    keyword: &str,
    offset: u64,
    count: u64,
) -> Result<Vec<Model>, ()> {
    let features_api = &state.features_api;
    match features_api.search_contents_by_keyword(keyword, offset, count).await {
        Ok(mut articles) => {
            // 获取所有feeds信息用于匹配feed名称
            let feeds_packages = features_api.get_feeds_packages().await;
            let mut feed_map = std::collections::HashMap::new();
            
            for package in feeds_packages {
                for feed in package.feeds {
                    feed_map.insert(feed.id, feed.name);
                }
            }
            
            // 为每个文章添加feed名称
            for article in &mut articles {
                article.feed_name = feed_map.get(&article.group_id).cloned();
            }
            
            Ok(articles)
        }
        Err(e) => {
            error!("search_contents_by_keyword error: {}", e);
            Err(())
        }
    }
}

fn convert_result<T>(result: anyhow::Result<T>) -> Result<T, ()> {
    match result {
        Ok(value) => Ok(value),
        Err(e) => {
            error!("command execution error...{}", e);
            Err(())
        }
    }
}