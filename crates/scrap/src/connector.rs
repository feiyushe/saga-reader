use std::time::Duration;

use reqwest::{Client, ClientBuilder, header};

pub struct ClientOption {
    pub user_agent: Option<String>,
    pub host: String,
}

const DEFAULT_USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/127.0.0.0 Safari/537.36 Edg/127.0.0.0";
const DEFAULT_ACCEPT: &str = "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7";
const DEFAULT_ACCEPT_ENCODING: &str = "gzip, deflate";

const DEFAULT_ACCEPT_LANGUAGE: &str = "en-US,en;q=0.9,zh-CN;q=0.8,zh;q=0.7";
const DEFAULT_CACHE_CONTROL: &str = "no-cache";
const DEFAULT_CONNECTION: &str = "keep-alive";

pub(crate) fn new_builder(option: ClientOption) -> anyhow::Result<ClientBuilder> {
    Ok(
        Client::builder()
            .cookie_store(true)
            .timeout(Duration::from_secs(20))
            .gzip(true)
            .deflate(true)
            .default_headers({
                let mut headers = header::HeaderMap::new();
                headers.insert(header::USER_AGENT, option.user_agent.unwrap_or(DEFAULT_USER_AGENT.to_string()).parse()?);
                headers.insert(header::ACCEPT, DEFAULT_ACCEPT.to_string().parse()?);
                headers.insert(header::ACCEPT_ENCODING, DEFAULT_ACCEPT_ENCODING.to_string().parse()?);
                headers.insert(header::ACCEPT_LANGUAGE, DEFAULT_ACCEPT_LANGUAGE.to_string().parse()?);
                headers.insert(header::CACHE_CONTROL, DEFAULT_CACHE_CONTROL.to_string().parse()?);
                headers.insert(header::CONNECTION, DEFAULT_CONNECTION.to_string().parse()?);
                headers.insert(header::HOST, option.host.parse()?);
                headers.insert(header::DNT, "1".parse()?);
                headers
            })
    )
}

pub(crate) fn new(option: ClientOption) -> anyhow::Result<Client> {
    let builder = new_builder(option)?;
    Ok(builder.build().unwrap())
}
