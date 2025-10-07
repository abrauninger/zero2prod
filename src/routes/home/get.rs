use actix_web::{HttpResponse, http::header::ContentType};
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;

pub async fn home(flash_messages: IncomingFlashMessages) -> HttpResponse {
    let mut msg_html = String::new();
    for m in flash_messages.iter() {
        writeln!(msg_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"<!doctype html>
<html lang="en">
    <head>
        <meta http-equiv="content-type" content="text/html; charset=utf-8" />
        <title>Home</title>
    </head>
    <body>
        {msg_html}

        <p>Welcome to our newsletter!</p>

        <form action="/subscriptions" method="post">
            <label>Name
                <input
                    type="text"
                    placeholder="Enter your name"
                    name="name"
                >
            </label>
            <br>
            <label>Email address
                <input
                    type="text"
                    placeholder="Enter your email address"
                    name="email"
                >
            </label>
            <br>
            <button type="submit">Subscribe</button>
        </form>
    </body>
</html>
"#,
        ))
}
