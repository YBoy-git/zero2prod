mod common;

use common::spawn_app;

#[tokio::test]
async fn health_check_works() {
    let url = spawn_app();
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{url}/health_check"))
        .send()
        .await
        .unwrap();

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}
