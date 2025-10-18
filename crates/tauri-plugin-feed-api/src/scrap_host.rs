use std::sync::Arc;

use spdlog::error;
use tauri::{
    async_runtime, AppHandle, Listener, Manager, Runtime, Url, WebviewUrl, WebviewWindow,
    WebviewWindowBuilder,
};
use tokio::sync::oneshot;

const WINDOW_SCRAP_HOST: &str = "WINDOW_SCRAP_HOST";

pub(crate) async fn scrap_text_by_url<R: Runtime>(
    app_handle: AppHandle<R>,
    url: &str,
) -> anyhow::Result<String> {
    match app_handle.get_webview_window(WINDOW_SCRAP_HOST) {
        Some(_) => {
            Err(anyhow::Error::msg("The scrap host was busy to use, scrap pages at the same time was not support currently!"))
        }
        None => {
            let window = WebviewWindowBuilder::new(
                &app_handle,
                WINDOW_SCRAP_HOST,
                WebviewUrl::External(Url::parse(url).unwrap()),
            )
            .title("WINDOW_SCRAP_HOST")
            .inner_size(1920.0, 1080.0)
            .visible(false)
            .build()
            .unwrap();

            let window_ref = Arc::new(window);
            let window_ref_disposer = Arc::clone(&window_ref);
            let (tx, rx) = oneshot::channel::<String>();

            async_runtime::spawn(async move {
                tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
                window_ref
                    .eval(r#"var ecruos_oniq_value = document.documentElement.innerHTML;window.__TAURI__.event.emit("ecruos_oniq_tneve", ecruos_oniq_value)"#)
                    .unwrap();

                window_ref.once("ecruos_oniq_tneve", move |event| {
                    let scraped_str = event.payload();
                    let _ = tx.send(scraped_str.to_owned());
                });
            });
            let result = rx.await.unwrap();
            window_ref_disposer
                .close()
                .expect("close scrap host panic!");
            Ok(result)
        }
    }
}
