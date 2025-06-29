use axum::{
    response::IntoResponse,
    routing::get,
    Router,
};
use axum::http::{HeaderMap, HeaderValue};
use tokio::net::TcpListener;
use rss::ChannelBuilder;
use rss::validation::Validate;
use utils::constants::PORT;
use utils::art::print_dp_rss;
use api::endpoints::hello_world;

mod api;
mod utils;

async fn get_rss_feed() -> impl IntoResponse {
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

#[tokio::main]
async fn main() {
    print_dp_rss();
    println!("Starting server on port {}", PORT);

    let app = Router::new()
        .route("/", get(hello_world))
        .route("/rss", get(get_rss_feed));

    let address = format!("0.0.0.0:{}", PORT);
    let listener = TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}