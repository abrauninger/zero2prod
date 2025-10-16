use crate::helpers::spawn_app;

#[tokio::test]
async fn index_html_file_served_by_app() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response_html = app.get_index_html().await;

    dbg!(&response_html);

    // Assert
    assert!(response_html.contains("<body"));
}
