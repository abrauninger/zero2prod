use crate::helpers::{assert_error_response, assert_successful_response, spawn_app};

#[tokio::test]
async fn username_without_login_returns_401() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app.get_username().await;

    // Assert
    assert_error_response(response, 401, "not_logged_in").await;
}

// #[derive(serde::Deserialize)]
// struct Username {
//     username: String,
// }

#[tokio::test]
async fn you_must_be_logged_in_to_access_the_admin_dashboard() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app.get_admin_dashboard().await;

    // Assert
    assert_error_response(response, 401, "not_logged_in").await;
}

#[tokio::test]
async fn logout_clears_session_state() {
    // Arrange
    let app = spawn_app().await;

    app.login().await;

    // Act - Part 1 - Follow the redirect from login
    let html_page = app.get_admin_dashboard_html().await;
    assert!(html_page.contains(&format!("Welcome {}", app.test_user.username)));

    // Act - Part 2 - Logout
    let response = app.get_logout().await;
    assert_successful_response(&response);

    // Act - Part 3 - Attempt to load admin panel
    let response = app.get_admin_dashboard().await;
    assert_error_response(response, 401, "not_logged_in").await;
}
