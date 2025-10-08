use crate::helpers::{assert_error_response, assert_is_redirect_to, spawn_app};

#[tokio::test]
async fn username_without_login_returns_403() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app.get_username().await;

    // Assert
    assert_error_response(response, 403, "not_logged_in").await;
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
    assert_is_redirect_to(&response, "/login");
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
    let response = app.post_logout().await;
    assert_is_redirect_to(&response, "/login");

    // Act - Part 3 - Follow the redirect
    let html_page = app.get_login_html().await;
    assert!(html_page.contains("You have successfully logged out"));

    // Act - Part 4 - Attempt to load admin panel
    let response = app.get_admin_dashboard().await;
    assert_is_redirect_to(&response, "/login");
}
