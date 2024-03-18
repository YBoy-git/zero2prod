mod common;

use common::spawn_app;

#[tokio::test]
async fn index_works() {
    let url = spawn_app();
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{url}/"))
        .send()
        .await
        .unwrap();

    assert!(response.status().is_success());
    assert_ne!(response.content_length(), Some(0));
}