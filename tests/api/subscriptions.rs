use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

use crate::helpers::{assert_is_redirect_to, spawn_app};

#[tokio::test]
async fn subscribe_shows_confirmation_for_valid_form_data() {
    // Arrange
    let app = spawn_app().await;
    let body = serde_json::json!({
        "name": "le guin",
        "email": "ursula_le_guin@gmail.com",
    });

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    // Act
    let response = app.post_subscriptions(body).await;

    // Assert
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn subscribe_persists_the_new_subscriber() {
    // Arrange
    let app = spawn_app().await;
    let body = serde_json::json!({
        "name": "le guin",
        "email": "ursula_le_guin@gmail.com",
    });

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    // Act
    app.post_subscriptions(body).await;

    // Assert
    let saved = sqlx::query!("SELECT email, name, status FROM subscriptions")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
    assert_eq!(saved.status, "pending_confirmation");
}

#[tokio::test]
async fn subscribe_shows_error_when_fields_are_present_but_empty() {
    // Arrange
    let app = spawn_app().await;
    let test_cases = vec![
        (
            serde_json::json!({
                "name": "",
                "email": "ursula_le_guin@gmail.com",
            }),
            "empty name",
        ),
        (
            serde_json::json!({
                "name": "Ursula",
                "email": "",
            }),
            "empty email",
        ),
        (
            serde_json::json!({
                "name": "Ursula",
                "email": "definitely-not-an-email",
            }),
            "empty email",
        ),
    ];

    // Act
    for (body, description) in test_cases {
        // Act
        let response = app.post_subscriptions(body).await;

        // Assert
        let response_status = response.status().as_u16();
        assert_eq!(
            response_status, 400,
            "Error case returned {response_status} instead of 400: '{description}'"
        );
    }
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let app = spawn_app().await;
    let test_cases = vec![
        (
            serde_json::json!({
                "name": "le guin",
            }),
            "missing the email",
        ),
        (
            serde_json::json!({
                "email": "ursula_le_guin@gmail.com",
            }),
            "missing the name",
        ),
        (serde_json::json!({}), "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = app.post_subscriptions(invalid_body).await;

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {error_message}."
        );
    }
}

#[tokio::test]
async fn subscribe_sends_a_confirmation_email_for_valid_data() {
    // Arrange
    let app = spawn_app().await;
    let body = serde_json::json!({
        "name": "le guin",
        "email": "ursula_le_guin@gmail.com",
    });

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    // Act
    app.post_subscriptions(body).await;

    // Assert
    // Mock asserts on drop
}

#[tokio::test]
async fn subscribe_sends_a_confirmation_email_with_a_link() {
    // Arrange
    let app = spawn_app().await;
    let body = serde_json::json!({
        "name": "le guin",
        "email": "ursula_le_guin@gmail.com",
    });

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        // We are not setting an expectation here; this test is focused on another aspect of the ap pbehavior.
        //.expect(1)
        .mount(&app.email_server)
        .await;

    // Act
    app.post_subscriptions(body).await;

    // Assert
    // Get the first intercepted request
    let email_request = &app.email_server.received_requests().await.unwrap()[0];
    let confirmation_links = app.get_confirmation_links(email_request);

    assert_eq!(confirmation_links.html, confirmation_links.plain_text);
}

#[tokio::test]
async fn subscribe_fails_if_there_is_a_fatal_database_error() {
    // Arrange
    let app = spawn_app().await;
    let body = serde_json::json!({
        "name": "le guin",
        "email": "ursula_le_guin@gmail.com",
    });

    // Sabotage the database
    sqlx::query!("ALTER TABLE subscriptions DROP COLUMN email;")
        .execute(&app.db_pool)
        .await
        .unwrap();

    // Act
    let response = app.post_subscriptions(body).await;

    // Assert
    // Right now this is reported as a 400 BAD_REQUEST.  Maybe 500 would be better in this case?  But we don't know the difference between a primary-key violation (input error)
    // and a more catastrophic internal error (table column is missing), so we err on the side of reporting it as an input error.
    assert_eq!(response.status().as_u16(), 400);
}
