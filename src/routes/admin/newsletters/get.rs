use actix_web::{HttpResponse, http::header::ContentType};
use actix_web_flash_messages::IncomingFlashMessages;

use std::fmt::Write;

pub async fn publish_newsletter_form(
    flash_messages: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    let mut msg_html = String::new();
    for m in flash_messages.iter() {
        writeln!(msg_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"<!doctype html>
<html lang="en">
<head>
    <meta http-equiv="content-type" content="text/html; charset=utf-8">
    <title>Send a newsletter issue</title>
</head>
<body>
    {msg_html}
    <form action="/admin/newsletters" method="post">
        <label>Title
            <input
                type="text"
                placeholder="Enter newsletter title"
                name="title"
            >
        </label>
        <br>
        <label>Plain-text content
            <input
                type="text"
                placeholder="Enter plain-text content"
                name="content_text"
            >
        </label>
        <br>
        <label>HTML content
            <input
                type="text"
                placeholder="Enter HTML content"
                name="content_html"
            >
        </label>
        <br>
        <button type="submit">Send</button>
    </form>
    <p><a href="/admin/dashboard">&lt; - Back</a></p>
</body>
</html>"#,
        )))
}
