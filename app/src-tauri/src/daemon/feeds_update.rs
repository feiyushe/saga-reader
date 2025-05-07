use std::time::Duration;

use feed_api_rs::{
    application_context::ContextHost,
    features::{api::FeaturesAPI, impl_default::FeaturesAPIImpl},
    startup::{init_logger, Startup},
};
use fslock::LockFile;
use spdlog::{error, info, warn};
use tokio::{runtime::Runtime, time};

use crate::daemon::locks::{get_lock_path, LOCK_FEEDS_SCHEDULE_UPDATE};

pub(crate) fn launch_feeds_schedule_update() -> anyhow::Result<()> {
    // 防止重复运行更新实例
    let lock_path = get_lock_path(LOCK_FEEDS_SCHEDULE_UPDATE);
    let mut updater_lock = LockFile::open(&lock_path)?;
    if !updater_lock.try_lock()? {
        warn!(
            "launch_feeds_schedule_update...lock failured, may be there already a updater running."
        );
        return Ok(());
    }

    // 初始化异步运行时
    info!("async runtime initializing");
    let rt = Runtime::new().expect("Failed to create Tokio runtime");
    rt.block_on(async {
        info!("async runtime initialized");
        schedule_loop().await.expect("schedule_loop occurs error");
    });
    Ok(())
}

async fn schedule_loop() -> anyhow::Result<()> {
    // 初始化业务模块
    info!("features module initializing");
    let context_host = Startup::launch().await?;
    let mut context = context_host.copy_context();
    context.app_config.log.log_name_tail = String::from("_feeds_schedule_update");

    let app_config = &context.app_config;
    init_logger::init_by(&app_config.log).expect("日志初始化失败");

    // 在所有权转移前读取需要的配置
    let update_interval = match &app_config.daemon.frequency_feeds_update {
        true => 60 * 60 * 6,
        false => 60 * 60 * 1,
    };
    let mut interval = time::interval(Duration::from_secs(update_interval));

    let features = FeaturesAPIImpl::new(context).await?;
    info!("features module initialized");

    // 定时任务
    loop {
        info!("scheduled feeds update begin");
        // let feeds_packages = features.get_feeds_packages().await;
        // for feed_package in feeds_packages {
        //     for feed in feed_package.feeds {
        //         match features
        //             .update_feed_contents(&feed_package.id, &feed.id, None)
        //             .await
        //         {
        //             Ok(_) => (),
        //             Err(e) => error!(
        //                 "update_feed_contents failure, package_id = {}, feed_id = {}, error = {}",
        //                 &feed_package.id, &feed.id, e
        //             ),
        //         }
        //     }
        // }
        info!("scheduled feeds update end");
        interval.tick().await;
    }
}
