use actix_web::{HttpResponse, web};
use actix_web_flash_messages::FlashMessage;
use sqlx::PgPool;
use uuid::Uuid;

use crate::utils::e500;

#[derive(serde::Deserialize)]
pub struct Parameters {
    subscription_token: String,
}

#[tracing::instrument(name = "Confirm a pending subscriber", skip(parameters, pool))]
pub async fn confirm(
    parameters: web::Query<Parameters>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let id = get_subscriber_id_from_token(&pool, &parameters.subscription_token)
        .await
        .map_err(e500)?;

    let Some(subscriber_id) = id else {
        return Err(e500(anyhow::anyhow!("Invalid subscriber token")));
    };

    confirm_subscriber(&pool, subscriber_id)
        .await
        .map_err(e500)?;

    FlashMessage::info(
        "You have confirmed your newsletter subscription. Stay tuned for exciting newsletters!",
    )
    .send();
    Ok(HttpResponse::Ok().finish())
}

#[tracing::instrument(name = "Mark subscriber as confirmed", skip(subscriber_id, pool))]
pub async fn confirm_subscriber(pool: &PgPool, subscriber_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"UPDATE subscriptions SET status = 'confirmed' WHERE id = $1"#,
        subscriber_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

#[tracing::instrument(name = "Get subscriber_id from token", skip(subscription_token, pool))]
pub async fn get_subscriber_id_from_token(
    pool: &PgPool,
    subscription_token: &str,
) -> Result<Option<Uuid>, sqlx::Error> {
    let result = sqlx::query!(
        r#"SELECT subscriber_id FROM subscription_tokens
        WHERE subscription_token = $1"#,
        subscription_token
    )
    .fetch_optional(pool)
    .await?;

    Ok(result.map(|r| r.subscriber_id))
}
