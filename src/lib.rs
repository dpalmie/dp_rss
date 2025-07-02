pub mod api;
pub mod utils;

use axum::{
    routing::get,
    Router,
};
use crate::api::endpoints::{hello_world, get_rss_feed};

pub fn app() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/rss", get(get_rss_feed))
}
