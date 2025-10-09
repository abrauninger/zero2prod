use actix_web::http::StatusCode;
use actix_web::{HttpResponse, HttpResponseBuilder, ResponseError, web};
use chrono::Utc;
use rand::distributions::Alphanumeric;
use rand::{Rng, thread_rng};
use sqlx::{Executor, PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::domain::{NewSubscriber, SubscriberEmail, SubscriberName};
use crate::email_client::EmailClient;
use crate::startup::ApplicationBaseUrl;
use crate::utils::error_chain_fmt;

#[derive(serde::Deserialize)]
pub struct SubscribeFormData {
    email: String,
    name: String,
}

#[tracing::instrument(
    name = "Adding a new subscriber.",
    skip(form, pool, email_client, base_url),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
pub async fn subscribe(
    form: web::Json<SubscribeFormData>,
    pool: web::Data<PgPool>,
    email_client: web::Data<EmailClient>,
    base_url: web::Data<ApplicationBaseUrl>,
) -> Result<HttpResponse, SubscribeError> {
    let new_subscriber = form.0.try_into().map_err(SubscribeError::BadFormData)?;

    // TODO: Should we check within the context of a transaction in order to get transaction isolation?
    // TODO: What if multiple 'subscribe' requests come in simultaneously for the same user?
    if already_subscribed(&new_subscriber, &pool).await? {
        return Ok(HttpResponse::new(StatusCode::OK));
    }

    let mut transaction = pool.begin().await?;

    let subscriber_id = insert_subscriber(&mut transaction, &new_subscriber)
        .await
        .map_err(SubscribeError::DatabaseError)?;

    let subscription_token = generate_subscription_token();
    store_token(&mut transaction, subscriber_id, &subscription_token).await?;

    send_confirmation_email(
        &email_client,
        new_subscriber,
        &base_url.0,
        &subscription_token,
    )
    .await
    .map_err(SubscribeError::SendConfirmationEmailError)?;

    transaction.commit().await?;

    Ok(HttpResponse::new(StatusCode::OK))
}

impl TryFrom<SubscribeFormData> for NewSubscriber {
    type Error = String;

    fn try_from(value: SubscribeFormData) -> Result<NewSubscriber, Self::Error> {
        let name = SubscriberName::parse(value.name)?;
        let email = SubscriberEmail::parse(value.email)?;
        Ok(Self { email, name })
    }
}

#[tracing::instrument(
    name = "Store subscription token in the database",
    skip(subscription_token, transaction)
)]
async fn store_token(
    transaction: &mut Transaction<'_, Postgres>,
    subscriber_id: Uuid,
    subscription_token: &str,
) -> Result<(), sqlx::Error> {
    let query = sqlx::query!(
        r#"INSERT INTO subscription_tokens (subscription_token, subscriber_id)
        VALUES ($1, $2)"#,
        subscription_token,
        subscriber_id,
    );

    transaction.execute(query).await?;

    Ok(())
}

#[tracing::instrument(
    name = "Send a confirmation email to a new subscriber",
    skip(email_client, new_subscriber, base_url, subscription_token)
)]
async fn send_confirmation_email(
    email_client: &EmailClient,
    new_subscriber: NewSubscriber,
    base_url: &str,
    subscription_token: &str,
) -> Result<(), reqwest::Error> {
    let confirmation_link =
        format!("{base_url}/subscriptions/confirm?subscription_token={subscription_token}");

    let html_body = format!(
        "Welcome to our newsletter!<br />\
        Click <a href=\"{confirmation_link}\">here</a> to confirm your subscription."
    );

    let plain_body = format!(
        "Welcome to our newsletter!\nVisit {confirmation_link} to confirm your subscription."
    );

    email_client
        .send_email(&new_subscriber.email, "Welcome!", &html_body, &plain_body)
        .await
}

#[tracing::instrument(
    name = "Checking whether subscriber already exists",
    skip(new_subscriber, pool)
)]
async fn already_subscribed(
    new_subscriber: &NewSubscriber,
    pool: &PgPool,
) -> Result<bool, sqlx::Error> {
    Ok(sqlx::query!(
        r#"
        SELECT id
        FROM subscriptions
        WHERE email = $1
        "#,
        new_subscriber.email.as_ref()
    )
    .fetch_optional(pool)
    .await?
    .is_some())
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(new_subscriber, transaction)
)]
async fn insert_subscriber(
    transaction: &mut Transaction<'_, Postgres>,
    new_subscriber: &NewSubscriber,
) -> Result<Uuid, sqlx::Error> {
    let subscriber_id = Uuid::new_v4();

    let query = sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at, status)
        VALUES ($1, $2, $3, $4, 'pending_confirmation')
        "#,
        subscriber_id,
        new_subscriber.email.as_ref(),
        new_subscriber.name.as_ref(),
        Utc::now(),
    );

    transaction.execute(query).await?;

    Ok(subscriber_id)
}

/// Generate a random 25-character-long case-sensitive subscription token
fn generate_subscription_token() -> String {
    let mut rng = thread_rng();
    std::iter::repeat_with(|| rng.sample(Alphanumeric))
        .map(char::from)
        .take(25)
        .collect()
}

// We use thiserror #[error] attributes for better diagnostics in internal logging.
#[derive(thiserror::Error)]
pub enum SubscribeError {
    #[error("Invalid form data: '{0}'")]
    BadFormData(String),

    // TODO: Call this 'UnexpectedError' and use 'anyhow::Error' as 'from'?
    #[error("Database error")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Unable to send confirmation email")]
    SendConfirmationEmailError(#[from] reqwest::Error),
}

impl SubscribeError {
    fn response_builder(&self) -> HttpResponseBuilder {
        match self {
            SubscribeError::BadFormData(_) => HttpResponse::BadRequest(),
            SubscribeError::DatabaseError(_) => HttpResponse::InternalServerError(),
            SubscribeError::SendConfirmationEmailError(_) => HttpResponse::InternalServerError(),
        }
    }
    fn error_id(&self) -> &str {
        match self {
            SubscribeError::BadFormData(_) => "invalid_data",
            SubscribeError::DatabaseError(_) => "internal",
            SubscribeError::SendConfirmationEmailError(_) => "send_confirmation_email",
        }
    }
}

impl std::fmt::Debug for SubscribeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for SubscribeError {
    fn error_response(&self) -> HttpResponse {
        self.response_builder().json(serde_json::json!({
            "error_id": self.error_id()
        }))
    }
}
