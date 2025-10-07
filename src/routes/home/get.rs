use actix_web::{HttpResponse, http::header::ContentType};

pub async fn home() -> HttpResponse {
    HttpResponse::Ok().content_type(ContentType::html()).body(
        r#"<!doctype html>
<html lang="en">
    <head>
        <meta http-equiv="content-type" content="text/html; charset=utf-8" />
        <title>Home</title>
    </head>
    <body>
        <p>Welcome to our newsletter!</p>
    </body>
</html>
"#,
    )
}
