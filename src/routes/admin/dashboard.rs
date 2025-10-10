use actix_web::{HttpResponse, web};
use anyhow::Context;
use sqlx::PgPool;
use uuid::Uuid;

use crate::authentication::UserId;
use crate::utils::e500;

pub async fn user_metadata(pool: web::Data<PgPool>, user_id: web::ReqData<UserId>) -> HttpResponse {
    let user_id = user_id.into_inner();
    // TODO: No unwrap
    let username = get_username(*user_id, &pool).await.map_err(e500).unwrap();
    HttpResponse::Ok().json(serde_json::json!({
        "username": username
    }))
}

#[tracing::instrument(name = "Get username", skip(pool))]
pub async fn get_username(user_id: Uuid, pool: &PgPool) -> Result<String, anyhow::Error> {
    let row = sqlx::query!(
        r#"
        SELECT username
        FROM users
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_one(pool)
    .await
    .context("Failed to perform a query to retrieve a username")?;

    Ok(row.username)
}
