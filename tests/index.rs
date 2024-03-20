mod common;

use common::spawn_app;

#[tokio::test]
async fn index_works() {
    let app = spawn_app().await;

    let app_url = format!("http://{}", app.config.connection.to_string());

    let client = reqwest::Client::new();
    let response = client.get(format!("{app_url}/")).send().await.unwrap();

    assert!(response.status().is_success());
    assert_ne!(response.content_length(), Some(0));
}
