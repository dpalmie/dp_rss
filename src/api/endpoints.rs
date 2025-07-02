use axum::http::{HeaderMap, HeaderValue};
use axum::response::IntoResponse;
use rss::ChannelBuilder;
use rss::validation::Validate;

pub async fn hello_world() -> &'static str {
    "Hello, World!"
}

pub async fn get_rss_feed() -> impl IntoResponse {
    let channel = ChannelBuilder::default()
        .title("DPRSS")
        .link("https://rss.davispalmie.com")
        .description("RSS feed for my posts")
        .build();

    if let Err(e) = channel.validate() {
        panic!("Invalid RSS feed: {}", e);
    }

    let rss_string = channel.to_string();

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("application/rss+xml"));

    (headers, rss_string)
}