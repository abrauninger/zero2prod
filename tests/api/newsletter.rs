use std::time::Duration;

use fake::{
    Fake,
    faker::{internet::en::SafeEmail, name::en::Name},
};
use uuid::Uuid;
use wiremock::{
    Mock, ResponseTemplate,
    matchers::{any, method, path},
};

use crate::helpers::{
    ConfirmationLinks, TestApp, assert_error_response, assert_is_redirect_to, spawn_app,
};

#[tokio::test]
async fn you_must_be_logged_in_to_publish_a_newsletter() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let newsletter_request_body = serde_json::json!({
        "title": "Newsletter title",
        "content_text": "Newsletter body as plain text",
        "content_html": "<p>Newsletter boddy as HTML</p>",
        "idempotency_key": Uuid::new_v4().to_string(),
    });

    let response = app.post_publish_newsletter(&newsletter_request_body).await;

    // Assert
    assert_error_response(response, 401, "not_logged_in").await;
}

#[tokio::test]
async fn newsletters_are_not_delivered_to_unconfirmed_subscribers() {
    // Arrange
    let app = spawn_app().await;
    create_unconfirmed_subscriber(&app).await;

    Mock::given(any())
        .respond_with(ResponseTemplate::new(200))
        // We assert that no request sent to the email server
        .expect(0)
        .mount(&app.email_server)
        .await;

    app.login().await;

    // Act
    let newsletter_request_body = serde_json::json!({
        "title": "Newsletter title",
        "content_text": "Newsletter body as plain text",
        "content_html": "<p>Newsletter boddy as HTML</p>",
        "idempotency_key": Uuid::new_v4().to_string(),
    });

    let response = app.post_publish_newsletter(&newsletter_request_body).await;

    // Assert
    assert_is_redirect_to(&response, "/admin/newsletters");

    app.dispatch_all_pending_emails().await;

    // Mock verifies on Drop that we haven't sent the newsletter email
}

#[tokio::test]
async fn newsletters_are_delivered_to_confirmed_subscribers() {
    // Arrange
    let app = spawn_app().await;
    create_confirmed_subscriber(&app).await;

    Mock::given(any())
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    app.login().await;

    // Act
    let newsletter_request_body = serde_json::json!({
        "title": "Newsletter title",
        "content_text": "Newsletter body as plain text",
        "content_html": "<p>Newsletter boddy as HTML</p>",
        "idempotency_key": Uuid::new_v4().to_string(),
    });

    let response = app.post_publish_newsletter(&newsletter_request_body).await;

    // Assert
    assert_is_redirect_to(&response, "/admin/newsletters");

    app.dispatch_all_pending_emails().await;

    // Mock verifies on Drop that we have sent the newsletter email
}

#[tokio::test]
async fn newsletters_returns_400_for_invalid_data() {
    // Arrange
    let app = spawn_app().await;
    let test_cases = vec![
        (
            serde_json::json!({
                "content_text": "Newsletter body as plain text",
                "content_html": "<p>Newsletter body as HTML</p>",
                "idempotency_key": Uuid::new_v4().to_string(),
            }),
            "missing title",
        ),
        (
            serde_json::json!({"title": "Newsletter!"}),
            "missing content",
        ),
    ];

    app.login().await;

    // Act
    for (invalid_body, error_message) in test_cases {
        let response = app.post_publish_newsletter(&invalid_body).await;

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 when the payload was: {}",
            error_message
        );
    }
}

#[tokio::test]
async fn publish_newsletter_form_works() {
    // Arrange
    let app = spawn_app().await;

    app.login().await;

    // Act - Part 1 - Publish the newsletter
    let newsletter_request_body = serde_json::json!({
        "title": "Newsletter title",
        "content_text": "Newsletter body as plain text",
        "content_html": "<p>Newsletter body as HTML</p>",
        "idempotency_key": Uuid::new_v4().to_string(),
    });

    let response = app.post_publish_newsletter(&newsletter_request_body).await;

    // Assert
    assert_is_redirect_to(&response, "/admin/newsletters");
}

#[tokio::test]
async fn newsletter_creation_is_idempotent() {
    // Arrange
    let app = spawn_app().await;
    create_confirmed_subscriber(&app).await;

    Mock::given(any())
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    app.login().await;

    // Act - Part 1 - Submit newsletter form
    let newsletter_request_body = serde_json::json!({
        "title": "Newsletter title",
        "content_text": "Newsletter body as plain text",
        "content_html": "<p>Newsletter body as HTML</p>",
        "idempotency_key": Uuid::new_v4().to_string(),
    });

    let response = app.post_publish_newsletter(&newsletter_request_body).await;
    assert_is_redirect_to(&response, "/admin/newsletters");

    // Act - Part 2 - Submit newsletter form *again*
    let response = app.post_publish_newsletter(&newsletter_request_body).await;
    assert_is_redirect_to(&response, "/admin/newsletters");

    app.dispatch_all_pending_emails().await;

    // Assert
    // Mock verifies on Drop that we have sent the newsletter email only once.
}

#[tokio::test]
async fn concurrent_form_submission_is_handled_gracefully() {
    // Arrange
    let app = spawn_app().await;
    create_confirmed_subscriber(&app).await;

    Mock::given(any())
        // Set a long delay to ensure that the second request arrives before the first one completes
        .respond_with(ResponseTemplate::new(200).set_delay(Duration::from_secs(2)))
        .expect(1)
        .mount(&app.email_server)
        .await;

    app.login().await;

    // Act - Submit two newsletter forms concurrently
    let newsletter_request_body = serde_json::json!({
        "title": "Newsletter title",
        "content_text": "Newsletter body as plain text",
        "content_html": "<p>Newsletter body as HTML</p>",
        "idempotency_key": Uuid::new_v4().to_string(),
    });

    let response1 = app.post_publish_newsletter(&newsletter_request_body);
    let response2 = app.post_publish_newsletter(&newsletter_request_body);
    let (response1, response2) = tokio::join!(response1, response2);

    assert_eq!(response1.status(), response2.status());
    assert_eq!(
        response1.text().await.unwrap(),
        response2.text().await.unwrap()
    );

    // Assert
    app.dispatch_all_pending_emails().await;

    // Mock verifies on Drop that we have sent the newsletter email only once.
}

async fn create_unconfirmed_subscriber(app: &TestApp) -> ConfirmationLinks {
    // Since we might have multiple subscribers in one test, randomize their metadata to avoid conflicts.
    let name: String = Name().fake();
    let email: String = SafeEmail().fake();
    let body = serde_json::json!({
        "name": name,
        "email": email,
    });

    let _mock_guard = Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .named("Create unconfirmed subscriber")
        .expect(1)
        .mount_as_scoped(&app.email_server)
        .await;

    app.post_subscriptions(body)
        .await
        .error_for_status()
        .unwrap();

    let email_request = &app
        .email_server
        .received_requests()
        .await
        .unwrap()
        .pop()
        .unwrap();

    app.get_confirmation_links(email_request)
}

async fn create_confirmed_subscriber(app: &TestApp) {
    let confirmation_links = create_unconfirmed_subscriber(app).await;
    reqwest::get(confirmation_links.html)
        .await
        .unwrap()
        .error_for_status()
        .unwrap();
}
