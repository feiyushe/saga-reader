// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod daemon;

use daemon::{args::DAEMON_FEEDS_SCHEDULE_UPDATE, feeds_update::launch_feeds_schedule_update};
use spdlog::info;

fn main() {
    let launch_mode = std::env::args().nth(1).unwrap_or_default();
    info!("application started, launch_mode = {}", launch_mode);
    if launch_mode.eq(DAEMON_FEEDS_SCHEDULE_UPDATE) {
        launch_feeds_schedule_update().expect("launch feeds-schedule-update failure.");
        return;
    }
    qino_feed_client_lib::run()
}
