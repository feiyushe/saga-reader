use std::time::Duration;

use reqwest::Client;

pub(crate) fn new() -> anyhow::Result<Client> {
    Ok(
        Client::builder()
            .timeout(Duration::from_secs(60))
            .gzip(true)
            .deflate(true)
            .build()?
    )
}