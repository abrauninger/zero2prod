use uuid::Uuid;

use crate::helpers::{assert_error_response, assert_successful_response, spawn_app};

#[tokio::test]
async fn you_must_be_logged_in_to_see_the_change_password_form() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app.post_change_password(&serde_json::json!({})).await;

    // Assert
    assert_error_response(response, 401, "not_logged_in").await;
}

#[tokio::test]
async fn new_password_fields_must_match() {
    // Arrange
    let app = spawn_app().await;
    let new_password = Uuid::new_v4().to_string();
    let another_new_password = Uuid::new_v4().to_string();
    assert_ne!(new_password, another_new_password);

    app.login().await;

    // Act - Part 1 - Try to change password
    let response = app
        .post_change_password(&serde_json::json!({
            "current_password": &app.test_user.password,
            "new_password": &new_password,
            "new_password_check": &another_new_password,
        }))
        .await;

    // Assert
    assert_error_response(response, 400, "password_check_failed").await;
}

#[tokio::test]
async fn current_password_must_be_valid() {
    // Arrange
    let app = spawn_app().await;
    let new_password = Uuid::new_v4().to_string();
    let wrong_password = Uuid::new_v4().to_string();
    assert_ne!(new_password, wrong_password);

    app.login().await;

    // Act
    let response = app
        .post_change_password(&serde_json::json!({
            "current_password": &wrong_password,
            "new_password": &new_password,
            "new_password_check": &new_password,
        }))
        .await;

    // Assert
    assert_error_response(response, 400, "current_password_incorrect").await;
}

#[tokio::test]
async fn new_password_must_be_long_enough() {
    // Arrange
    let app = spawn_app().await;
    let new_password = "short";

    app.login().await;

    // Act - Part 1 - Try to change password
    let response = app
        .post_change_password(&serde_json::json!({
            "current_password": &app.test_user.password,
            "new_password": &new_password,
            "new_password_check": &new_password,
        }))
        .await;

    // Assert
    assert_error_response(response, 400, "new_password_too_short").await;
}

#[tokio::test]
async fn changing_password_works() {
    // Arrange
    let app = spawn_app().await;
    let new_password = Uuid::new_v4().to_string();

    app.login().await;

    // Act - Part 1 - Change password
    let response = app
        .post_change_password(&serde_json::json!({
            "current_password": &app.test_user.password,
            "new_password": &new_password,
            "new_password_check": &new_password,
        }))
        .await;
    assert_successful_response(&response);

    // Act - Part 2 - Logout
    let response = app.get_logout().await;
    assert_successful_response(&response);

    // Act - Part 3 - Log in using the new password
    let response = app
        .post_login(&serde_json::json!({
            "username": &app.test_user.username,
            "password": &new_password,
        }))
        .await;
    assert_successful_response(&response);
}
