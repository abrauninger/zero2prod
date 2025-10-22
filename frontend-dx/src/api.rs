use std::{fmt::Debug, fmt::Display, sync::LazyLock};

use uuid::Uuid;

use crate::USERNAME;

static BASE_URL: LazyLock<String> = LazyLock::new(|| {
    // https://github.com/seanmonstar/reqwest/issues/1433
    web_sys::window().unwrap().location().origin().unwrap()
});

pub async fn get_username() -> Result<String, ApiError> {
    #[derive(serde::Deserialize, Debug)]
    struct GetUsernameApiResponse {
        username: String,
    }

    let response = reqwest::get(format!("{}/api/admin/user", *BASE_URL)).await?;

    // When the user is not logged in, the API returns HTTP 401 (Unauthorized)
    if response.status().is_success() {
        // TODO: We might need a more comprehensive approach for how to bubble up error messages.
        // Do we want a way to show an unexpected error to the user anywhere in the app, or only
        // in certain places (like form entry)?
        let response = response.json::<GetUsernameApiResponse>().await?;
        Ok(response.username)
    } else {
        Err(ApiError::Unexpected)
    }
}

pub async fn add_subscriber(name: String, email: String) -> Result<(), ApiError> {
    #[derive(serde::Serialize)]
    struct SubscribeApiParams {
        name: String,
        email: String,
    }

    call_api("/api/subscriptions", SubscribeApiParams { name, email }).await
}

pub async fn login(username: String, password: String) -> Result<(), ApiError> {
    #[derive(serde::Serialize)]
    struct LoginApiParams {
        username: String,
        password: String,
    }

    call_api("/api/login", LoginApiParams { username, password }).await
}

pub async fn change_password(
    current_password: String,
    new_password: String,
    new_password_check: String,
) -> Result<(), ApiError> {
    #[derive(serde::Serialize)]
    struct ChangePasswordApiParams {
        current_password: String,
        new_password: String,
        new_password_check: String,
    }

    call_api(
        "/api/admin/password",
        ChangePasswordApiParams {
            current_password,
            new_password,
            new_password_check,
        },
    )
    .await
}

pub async fn publish_newsletter(
    title: String,
    content_text: String,
    content_html: String,
    idempotency_key: Uuid,
) -> Result<(), ApiError> {
    #[derive(serde::Serialize)]
    struct PublishNewsletterApiParams {
        title: String,
        content_text: String,
        content_html: String,
        idempotency_key: String,
    }

    call_api(
        "/api/admin/newsletters",
        PublishNewsletterApiParams {
            title,
            content_text,
            content_html,
            idempotency_key: idempotency_key.to_string(),
        },
    )
    .await
}

// TODO: Any reason to return bool?
pub async fn logout() -> bool {
    match reqwest::get(format!("{}/api/admin/logout", *BASE_URL)).await {
        Ok(_) => {
            *USERNAME.write() = None;
            true
        }
        Err(e) => {
            tracing::error!("/api/admin/logout returned an error: {e:?}");
            false
        }
    }
}

async fn call_api(relative_url: &str, input: impl serde::Serialize) -> Result<(), ApiError> {
    let url = format!("{}{}", *BASE_URL, relative_url);

    // TODO: Use a tracing span
    tracing::info!("Making request to URL: {url}");

    let response = reqwest::Client::new().post(url).json(&input).send().await;

    tracing::info!("Request completed: {response:?}");

    match response {
        Ok(response) => {
            tracing::info!("Response was success");

            let status = response.status();
            tracing::info!("Response status: {status}");

            if status.is_success() {
                match response.text().await {
                    Ok(text) => {
                        tracing::info!("Response text: {text}");
                        Ok(())
                    }
                    Err(e) => {
                        tracing::error!("Unable to read output object from response");
                        Err(e.into())
                    }
                }
            } else {
                let error_response = response.json::<ApiErrorResponse>().await?;
                tracing::info!("Error response: {error_response:?}");

                let message = Message::from_error_id(&error_response.error_id);
                match message {
                    Some(message) => Err(ApiError::ServerError(message)),
                    None => Err(ApiError::Unexpected),
                }
            }
        }
        Err(e) => {
            tracing::error!("Error from request: {e:?}");
            Err(e.into())
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Server returned error: '{0}'")]
    ServerError(Message),

    #[error("Error making HTTP request error")]
    ReqwestError(#[from] reqwest::Error),

    #[error("Error decoding HTTP response")]
    JsonError(#[from] serde_json::Error),

    #[error("Unexpected error")]
    Unexpected,
}

#[derive(Debug, serde::Deserialize)]
struct ApiErrorResponse {
    error_id: String,
}

#[derive(Debug)]
pub enum Message {
    InternalError,
    InvalidCredentials,
    InvalidData,
    UnableToSendConfirmationEmail,
    PasswordCheckFailed,
    NewPasswordToShort,
    PasswordChangeSucceeded,
    AddSubscriberSucceeded,
    PublishNewsletterSucceeded,
}

impl Message {
    fn from_error_id(error_id: &str) -> Option<Self> {
        match error_id {
            "invalid_credentials" => Some(Self::InvalidCredentials),
            "invalid_data" => Some(Self::InvalidData),
            "send_confirmation_email" => Some(Self::UnableToSendConfirmationEmail),
            "password_check_failed" => Some(Self::PasswordCheckFailed),
            // TODO: Get the character-count from the error response.
            "new_password_too_short" => Some(Self::NewPasswordToShort),
            "internal_error" => Some(Self::InternalError),
            _ => {
                tracing::error!("Unrecognized error ID: {error_id}");
                None
            }
        }
    }

    fn as_str(&self) -> &'static str {
        match self {
            Self::InternalError => "An internal error occurred. Apologies for the inconvenience.",
            Self::InvalidCredentials => "The username and password that you entered did not work. Try again with different credentials.",
            Self::InvalidData => "There was a problem with the form data you entered. Please try again.",
            Self::UnableToSendConfirmationEmail => "We were unable to send a confirmation email to that email address.",
            Self::PasswordCheckFailed => "The new passwords you entered do not match each other.",
            Self::NewPasswordToShort => "The new password you have chosen is too short. Your new password must be at least 12 characters long.",
            Self::PasswordChangeSucceeded => "Your password has been changed.",
            Self::AddSubscriberSucceeded => "You have subscribed to our newsletter. Stay tuned, you're going to love it!",
            Self::PublishNewsletterSucceeded => "Your newsletter publish request has been accepted, and emails will go out shortly.",
        }
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
