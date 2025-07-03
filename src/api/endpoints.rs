use axum::http::{HeaderMap, HeaderValue};
use axum::response::IntoResponse;
use rss::ChannelBuilder;
use rss::validation::Validate;
use crate::utils::posts::load_posts;

pub async fn hello_world() -> &'static str {
    "Hello, World!"
}

pub async fn get_rss_feed() -> impl IntoResponse {
    let posts = match load_posts("src/items").await {
        Ok(posts) => posts,
        Err(e) => {
            eprintln!("Error loading posts: {}", e);
            vec![]
        }
    };

    let base_url = "https://rss.davispalmie.com";
    let mut rss_items = Vec::new();
    
    for post in posts {
        match post.to_rss_item(base_url) {
            Ok(item) => rss_items.push(item),
            Err(e) => eprintln!("Error creating RSS item for {}: {}", post.filename, e),
        }
    }

    let channel = ChannelBuilder::default()
        .title("DPRSS")
        .link(base_url)
        .description("RSS feed for my posts")
        .items(rss_items)
        .build();

    if let Err(e) = channel.validate() {
        panic!("Invalid RSS feed: {}", e);
    }

    let rss_string = channel.to_string();

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("application/rss+xml"));

    (headers, rss_string)
}