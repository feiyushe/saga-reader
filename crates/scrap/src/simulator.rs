use once_cell::sync::Lazy;
use spdlog::error;
use std::sync::Arc;
use tauri::{
    async_runtime, AppHandle, Listener, Manager, Runtime, Url, WebviewUrl, WebviewWindowBuilder,
};
use tokio::{
    sync::{oneshot, Mutex},
    time::{sleep, Duration},
};

const WINDOW_SCRAP_HOST: &str = "WINDOW_SCRAP_HOST";
static MUTEX: Lazy<Arc<Mutex<()>>> = Lazy::new(|| Arc::new(Mutex::new(())));

pub async fn scrap_text_by_url<R: Runtime>(
    app_handle: AppHandle<R>,
    url: &str,
) -> anyhow::Result<String> {
    let _lock = MUTEX.lock().await;
    match app_handle.get_webview_window(WINDOW_SCRAP_HOST) {
        Some(_) => {
            error!("The scrap host for simulator was busy to use, scrap pages at the same time was not support currently!");
            Err(anyhow::anyhow!("Scrap host is busy"))
        }
        None => {
            let window = WebviewWindowBuilder::new(
                &app_handle,
                WINDOW_SCRAP_HOST,
                WebviewUrl::External(Url::parse(url)?),
            )
            .title("WINDOW_SCRAP_HOST")
            .inner_size(1920.0, 1080.0)
            .visible(false)
            .build()?;

            let window_ref = Arc::new(window);
            let window_ref_disposer = Arc::clone(&window_ref);
            let (tx, rx) = oneshot::channel::<String>();

            async_runtime::spawn(async move {
                sleep(Duration::from_secs(3)).await;
                window_ref
                    .eval(r#"var ecruos_oniq_value = document.documentElement.innerHTML;window.__TAURI__.event.emit("ecruos_oniq_tneve", ecruos_oniq_value)"#)
                    .map_err(|e| error!("Failed to inspect: {}", e))
                    .ok();

                window_ref.once("ecruos_oniq_tneve", move |event| {
                    let payload = event.payload();
                    let mut scraped_str = payload
                        .chars()
                        .skip(1)
                        .take(payload.len() - 2)
                        .collect::<String>();
                    scraped_str = scraped_str.replace(r#"\""#, r#"""#);
                    let _ = tx.send(scraped_str);
                });
            });
            let result = rx.await?;
            window_ref_disposer
                .close()
                .expect("close scrap host panic!");
            Ok(result)
        }
    }
}
