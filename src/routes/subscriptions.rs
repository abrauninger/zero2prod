use actix_web::{HttpResponse, web};
use chrono::Utc;
use sqlx::PgConnection;
use uuid::Uuid;

pub async fn subscribe(
    _form: web::Form<FormData>,
    connection: web::Data<PgConnection>,
) -> HttpResponse {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now(),
    )
    .execute(connection.get_ref())
    .await;

    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
pub struct FormData {
    _email: String,
    _name: String,
}
