const COMMANDS: &[&str] = &[
    "add_feeds_package",
    "remove_feeds_package",
    "rename_feeds_package",
    "add_feed",
    "remove_feed",
    "rename_feed",
    "change_feed_data",
    "get_feeds_packages",
    "get_feeds_by_package",
    "update_feed_contents",
    "read_feed_contents",
    "query_by_id",
    "mark_as_read",
    "set_favorite",
    "get_app_config",
    "set_app_config",
    "get_ollama_status",
    "download_ollama",
    "launch_ollama",
    "open_article_external",
    "scrap_text_by_url",
    "update_article_by_source",
    "chat_with_article_assistant",
    "search_contents_by_keyword"
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        // .android_path("android")
        // .ios_path("ios")
        .build();
}
