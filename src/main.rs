use axum::{
    routing::get,
    Router,
};
use tokio::net::TcpListener;
use utils::constants::PORT;

mod utils;

async fn hello_world() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() {
    println!("Starting server on port {}", PORT);

    let app = Router::new()
        .route("/", get(hello_world));

    let address = format!("0.0.0.0:{}", PORT);
    let listener = TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}