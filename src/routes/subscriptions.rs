use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError, web};
use actix_web_flash_messages::FlashMessage;
use chrono::Utc;
use rand::distributions::Alphanumeric;
use rand::{Rng, thread_rng};
use sqlx::{Executor, PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::domain::{NewSubscriber, SubscriberEmail, SubscriberName};
use crate::email_client::EmailClient;
use crate::startup::ApplicationBaseUrl;
use crate::utils::{e400, e500, error_chain_fmt, see_other};

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
    // let new_subscriber = match form.0.try_into() {
    //     Ok(subscriber) => subscriber,
    //     Err(_) => {
    //         // TODO: Better error with specific issue in form
    //         FlashMessage::error("Error in processing your subscription request").send();
    //         return Ok(see_other("/"));
    //     }
    // };
    let new_subscriber = form.0.try_into().map_err(SubscribeError::BadFormData)?;

    let mut transaction = pool
        .begin()
        .await
        //.map_err(e500)?;
        .unwrap();

    let subscriber_id = insert_subscriber(&mut transaction, &new_subscriber)
        .await
        .map_err(SubscribeError::InsertSubscriberError)?;

    let subscription_token = generate_subscription_token();
    store_token(&mut transaction, subscriber_id, &subscription_token)
        .await
        //.map_err(e500)?;
        .unwrap();

    send_confirmation_email(
        &email_client,
        new_subscriber,
        &base_url.0,
        &subscription_token,
    )
    .await
    .map_err(SubscribeError::SendConfirmationEmailError)?;

    transaction
        .commit()
        .await
        //.map_err(e500)?;
        .unwrap();

    // FlashMessage::info("Thank you for subscribing to our newsletter. \
    //     To confirm your subscription and start receiving newsletters, check your email and click the link we've sent you.".to_string()).send();
    // Ok(see_other("/"))
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
) -> Result<(), StoreTokenError> {
    let query = sqlx::query!(
        r#"INSERT INTO subscription_tokens (subscription_token, subscriber_id)
        VALUES ($1, $2)"#,
        subscription_token,
        subscriber_id,
    );

    transaction.execute(query).await.map_err(StoreTokenError)?;

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

// TODO: Do we need thiserror?
#[derive(thiserror::Error)]
pub enum SubscribeError {
    #[error("Invalid form data: '{0}'")]
    BadFormData(String),

    #[error("Unable to insert subscriber")]
    InsertSubscriberError(#[from] sqlx::Error),

    #[error("Unable to send confirmation email")]
    SendConfirmationEmailError(#[from] reqwest::Error),

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for SubscribeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for SubscribeError {
    fn error_response(&self) -> HttpResponse {
        match self {
            SubscribeError::BadFormData(_) => HttpResponse::BadRequest().json(serde_json::json!({
                "message": "Bad form data, so sorry!"
            })),
            SubscribeError::InsertSubscriberError(_) => {
                HttpResponse::BadRequest().json(serde_json::json!({
                    "message": "Couldn't insert that mother-blippin' subscriber!"
                }))
            }
            SubscribeError::SendConfirmationEmailError(_) => HttpResponse::InternalServerError()
                .json(serde_json::json!({
                    "message": "Couldn't send a confirmation email!"
                })),
            SubscribeError::UnexpectedError(_) => {
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "message": "Internal server error"
                }))
            }
        }
    }
}

pub struct StoreTokenError(sqlx::Error);

impl std::fmt::Debug for StoreTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl std::fmt::Display for StoreTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "A database error was encountered while trying to store a subscription token."
        )
    }
}

impl std::error::Error for StoreTokenError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}
