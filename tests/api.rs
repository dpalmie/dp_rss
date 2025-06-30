use dp_rss::api::endpoints::hello_world;

#[tokio::test]
async fn test_hello_world() {
    assert_eq!(hello_world().await, "Hello, World!");
}

