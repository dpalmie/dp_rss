use tokio::net::TcpListener;
use dp_rss::utils::{constants::PORT, art::print_dp_rss};
use dp_rss::app;

#[tokio::main]
async fn main() {
    print_dp_rss();
    println!("Starting server on port {}", PORT);

    let app = app();

    let address = format!("0.0.0.0:{}", PORT);
    let listener = TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}