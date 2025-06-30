pub mod api;
pub mod utils;

use axum::{
    response::IntoResponse,
    routing::get,
    Router,
};
use axum::http::{HeaderMap, HeaderValue};
use rss::ChannelBuilder;
use rss::validation::Validate;
use crate::api::endpoints::hello_world;

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

pub fn app() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/rss", get(get_rss_feed))
}
