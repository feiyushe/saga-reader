use std::{sync::Arc, time::Duration};

use feed_api_rs::features::api::FeaturesAPI;
use fslock::LockFile;
use spdlog::{error, info, warn};
use tauri::{async_runtime, AppHandle, EventLoopMessage, Runtime};
use tauri_plugin_feed_api::state::HybridRuntimeState;
use tokio::time;

use crate::daemon::locks::{get_lock_path, LOCK_FEEDS_SCHEDULE_UPDATE};

pub(crate) fn launch_feeds_schedule_update<R: Runtime>(
    app_handle: &AppHandle<R>,
    state: Arc<HybridRuntimeState>,
) -> anyhow::Result<()> {
    // 防止重复运行更新实例
    let lock_path = get_lock_path(LOCK_FEEDS_SCHEDULE_UPDATE);
    let mut updater_lock = LockFile::open(&lock_path)?;
    if !updater_lock.try_lock()? {
        warn!(
            "launch_feeds_schedule_update...lock failured, may be there already a updater running."
        );
        return Ok(());
    }
    let ah = app_handle.clone();
    async_runtime::spawn(async move {
        schedule_loop(ah, state)
            .await
            .expect("schedule_loop occurs error");
    });
    Ok(())
}

async fn schedule_loop<R: Runtime>(
    app_handle: AppHandle<R>,
    state: Arc<HybridRuntimeState>,
) -> anyhow::Result<()> {
    let features = &state.features_api;
    let app_config = &features.context.read().await.app_config;
    // 在所有权转移前读取需要的配置
    let update_interval = match &app_config.daemon.frequency_feeds_update {
        true => 60 * 60 * 1,
        false => 60 * 60 * 3,
    };
    let mut interval = time::interval(Duration::from_secs(update_interval));
    let mut cold_start_delay = time::interval(Duration::from_secs(120));

    // 定时任务
    loop {
        cold_start_delay.tick().await;
        info!("scheduled feeds update begin");
        let feeds_packages = features.get_feeds_packages().await;
        for feed_package in feeds_packages {
            for feed in feed_package.feeds {
                match features
                    .update_feed_contents(&feed_package.id, &feed.id, Some(app_handle.clone()))
                    .await
                {
                    Ok(_) => (),
                    Err(e) => error!(
                        "update_feed_contents failure, package_id = {}, feed_id = {}, error = {}",
                        &feed_package.id, &feed.id, e
                    ),
                }
            }
        }
        info!("scheduled feeds update end");
        interval.tick().await;
    }
}
