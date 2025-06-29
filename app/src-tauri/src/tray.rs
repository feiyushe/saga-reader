use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    Manager, Runtime, Url, WebviewUrl, WebviewWindow, WebviewWindowBuilder,
};

use crate::constrant::{
    WINDOW_ABOUT_LABEL, WINDOW_ABOUT_TITLE, WINDOW_ABOUT_URL,
    WINDOW_EXTERNAL_ENDPOINT_INFORMATION_LABEL, WINDOW_EXTERNAL_ENDPOINT_INFORMATION_TITLE,
    WINDOW_EXTERNAL_ENDPOINT_INFORMATION_URL, WINDOW_MAIN_LABEL, WINDOW_MAIN_TITLE,
    WINDOW_MAIN_URL,
};

pub fn create_tray<R: Runtime>(app: &tauri::AppHandle<R>) -> tauri::Result<()> {
    let show_main_i = MenuItem::with_id(app, "main_show", "显示主窗口", true, None::<&str>)?;
    let show_endpoint_information =
        MenuItem::with_id(app, "endpoint_information", "网络信息", true, None::<&str>)?;
    let about_i = MenuItem::with_id(app, "about", "关于", true, None::<&str>)?;
    let quit_i = MenuItem::with_id(app, "quit", "退出程序", true, None::<&str>)?;
    let menu = Menu::with_items(
        app,
        &[&show_main_i, &show_endpoint_information, &about_i, &quit_i],
    )?;

    #[cfg(target_os = "macos")]
    let menu_click_by_left = true;
    #[cfg(not(target_os = "macos"))]
    let menu_click_by_left = false;

    let _ = TrayIconBuilder::with_id("tray")
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .show_menu_on_left_click(menu_click_by_left)
        .on_menu_event(move |app, event| match event.id.as_ref() {
            "main_show" => open_main_window(app),
            "endpoint_information" => {
                const WINDOW_LABEL: &str = WINDOW_EXTERNAL_ENDPOINT_INFORMATION_LABEL;
                const URL: &str = WINDOW_EXTERNAL_ENDPOINT_INFORMATION_URL;
                if let Some(window) = app.get_webview_window(WINDOW_LABEL) {
                    bring_to_front(window);
                } else {
                    WebviewWindowBuilder::new(
                        app,
                        WINDOW_LABEL,
                        WebviewUrl::External(Url::parse(URL).expect("url parse error...ip77")),
                    )
                    .title(WINDOW_EXTERNAL_ENDPOINT_INFORMATION_TITLE)
                    .inner_size(1280 as f64, 720 as f64)
                    .center()
                    .build()
                    .expect("build window failure for endpoint_information")
                    .show()
                    .expect("show window failure for endpoint_information");
                }
            }
            "about" => {
                if let Some(window) = app.get_webview_window(WINDOW_ABOUT_LABEL) {
                    bring_to_front(window);
                } else {
                    let window = WebviewWindowBuilder::new(
                        app,
                        WINDOW_ABOUT_LABEL,
                        WebviewUrl::App(WINDOW_ABOUT_URL.into()),
                    )
                    .title(WINDOW_ABOUT_TITLE)
                    .inner_size(480 as f64, 360 as f64)
                    .resizable(false)
                    .maximizable(false)
                    .minimizable(false)
                    .always_on_top(true)
                    .center()
                    .build()
                    .expect("build about window failure");
                    window.show().unwrap();
                }
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(|icon, event| match event {
            tauri::tray::TrayIconEvent::DoubleClick {
                id: _,
                position: _,
                rect: _,
                button: _,
            } => open_main_window(icon.app_handle()),
            _ => (),
        })
        .build(app);

    Ok(())
}

pub fn open_main_window<R: Runtime>(app: &tauri::AppHandle<R>) {
    if let Some(window) = app.get_webview_window(WINDOW_MAIN_LABEL) {
        bring_to_front(window);
    } else {
        let window = WebviewWindowBuilder::new(
            app,
            WINDOW_MAIN_LABEL,
            WebviewUrl::App(WINDOW_MAIN_URL.into()),
        )
        .title(WINDOW_MAIN_TITLE)
        .center()
        .min_inner_size(1440.0, 750.0)
        .maximized(true)
        .build()
        .expect("rebuild main window failure");
        window.show().unwrap();
    }
}

fn bring_to_front<R: Runtime>(window: WebviewWindow<R>) {
    let _ = window.unminimize();
    let _ = window.show();
    let _ = window.set_focus();
}
