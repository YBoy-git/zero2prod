mod common;

use common::spawn_app;
use sqlx::query;

#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;

    let app_url = format!("http://{}", app.config.connection.to_string());

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{app_url}/health_check",))
        .send()
        .await
        .unwrap();

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form() {
    let app = spawn_app().await;

    let app_url = format!("http://{}", app.config.connection.to_string());
    let pool = app.pool;

    let client = reqwest::Client::new();

    let name = "le%20guin";
    let email = "ursula_le_guin%40gmail.com";
    let body = format!("name={name}&email={email}");

    query!("DELETE FROM subscriptions WHERE email = $1", email)
        .execute(&pool)
        .await
        .unwrap();

    let response = client
        .post(format!("{app_url}/subscriptions"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);

    let saved = query!("SELECT email, name FROM subscriptions")
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(saved.name, "le guin");
    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app = spawn_app().await;

    let app_url = format!("http://{}", app.config.connection.to_string());

    let client = reqwest::Client::new();

    let test_cases = [
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (body, error_message) in test_cases {
        let response = client
            .post(format!("{app_url}/subscriptions",))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .unwrap();

        assert_eq!(
            response.status(),
            400,
            "No 400 when the payload is: {error_message}"
        );
    }
}
