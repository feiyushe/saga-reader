mod constrant;
mod daemon;
mod env;
mod monitor;
#[cfg(desktop)]
mod tray;

use std::sync::Arc;

use constrant::WINDOW_MAIN_LABEL;
use daemon::{args::DAEMON_FEEDS_SCHEDULE_UPDATE, feeds_update::launch_feeds_schedule_update};
use tauri::{Manager, State};
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_feed_api::state::HybridRuntimeState;
use tray::open_main_window;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    monitor::start();

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            open_main_window(&app);
        }))
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_feed_api::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec![DAEMON_FEEDS_SCHEDULE_UPDATE]),
        ))
        .invoke_handler(tauri::generate_handler![])
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                if window.label() != WINDOW_MAIN_LABEL {
                    return;
                }
                #[cfg(target_os = "macos")]
                {
                    tauri::AppHandle::hide(window.app_handle()).unwrap();
                    api.prevent_close();
                }
            }
            _ => {}
        })
        .setup(|app| {
            let handle = app.handle();
            #[cfg(all(desktop))]
            {
                tray::create_tray(handle)?;
            }

            let is_daemon = env::is_daemon();
            if !is_daemon {
                if let Some(window) = app.get_window(WINDOW_MAIN_LABEL) {
                    window.show().unwrap();
                }
            }
            let state: State<'_, Arc<HybridRuntimeState>> = app.state();
            let state_clone = Arc::clone(&state);
            launch_feeds_schedule_update(handle, state_clone).unwrap();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Saga Reader Desktop Application");
}
