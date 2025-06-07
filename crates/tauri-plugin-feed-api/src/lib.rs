use std::sync::Arc;

use tauri::{
    generate_handler,
    plugin::{Builder, TauriPlugin},
    Manager, RunEvent,
};

use feed_api_rs::application_context::ContextHost;
use feed_api_rs::features::impl_default::FeaturesAPIImpl;
use feed_api_rs::startup::Startup;

use crate::commands::{
    add_feed, add_feeds_package, change_feed_data, chat_with_article_assistant, download_ollama,
    get_app_config, get_feeds_by_package, get_feeds_packages, get_ollama_status, launch_ollama,
    mark_as_read, open_article_external, query_by_id, read_feed_contents, remove_feed,
    remove_feeds_package, rename_feed, rename_feeds_package, scrap_text_by_url,
    search_contents_by_keyword, set_app_config, set_favorite, update_article_by_source,
    update_feed_contents,
};
use crate::state::HybridRuntimeState;

mod commands;
mod scrap_host;
pub mod state;

pub fn init<R>() -> TauriPlugin<R>
where
    R: tauri::Runtime,
{
    Builder::new("feed-api")
        .invoke_handler(generate_handler![
            add_feeds_package,
            remove_feeds_package,
            rename_feeds_package,
            add_feed,
            remove_feed,
            rename_feed,
            change_feed_data,
            get_feeds_packages,
            get_feeds_by_package,
            update_feed_contents,
            read_feed_contents,
            query_by_id,
            mark_as_read,
            set_favorite,
            get_app_config,
            set_app_config,
            get_ollama_status,
            download_ollama,
            launch_ollama,
            open_article_external,
            scrap_text_by_url,
            update_article_by_source,
            chat_with_article_assistant,
            search_contents_by_keyword
        ])
        .setup(|app_handle, _plugin| {
            let features_api = tauri::async_runtime::block_on(async {
                let context_host = Startup::launch().await.unwrap();
                let context = context_host.copy_context();
                FeaturesAPIImpl::new(context)
                    .await
                    .expect("tauri-plugin-feed-api setup the features instance failure")
            });

            app_handle.manage(Arc::new(HybridRuntimeState { features_api }));
            Ok(())
        })
        .on_event(|app, event| match event {
            #[cfg(target_os = "macos")]
            RunEvent::Reopen {
                has_visible_windows,
                ..
            } => {
                if *has_visible_windows {
                    return;
                }
                if let Some(window) = app.get_window("main") {
                    window.show().unwrap();
                }
            }
            _ => {}
        })
        .build()
}
