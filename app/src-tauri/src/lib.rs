mod constrant;
mod daemon;
mod monitor;
#[cfg(desktop)]
mod tray;

use constrant::WINDOW_MAIN_LABEL;
use daemon::{args::DAEMON_FEEDS_SCHEDULE_UPDATE, launcher, locks::LOCK_FEEDS_SCHEDULE_UPDATE};
use tauri::Manager;
use tauri_plugin_autostart::MacosLauncher;
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
            Some(vec!["--feeds-schedule-update"]),
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
            #[cfg(all(desktop))]
            {
                let handle = app.handle();
                tray::create_tray(handle)?;
            }
            launcher::launch_ignore_error(DAEMON_FEEDS_SCHEDULE_UPDATE, LOCK_FEEDS_SCHEDULE_UPDATE);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Qino Feed Desktop Application");
}
