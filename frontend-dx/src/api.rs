use dioxus::signals::{Signal, WritableExt};
use std::{fmt::Display, sync::LazyLock};

static BASE_URL: LazyLock<String> = LazyLock::new(|| {
    // https://github.com/seanmonstar/reqwest/issues/1433
    web_sys::window().unwrap().location().origin().unwrap()
});

pub async fn add_subscriber(
    name: String,
    email: String,
    error_message: &mut Signal<Option<Message>>,
    info_message: &mut Signal<Option<Message>>,
) {
    #[derive(serde::Serialize)]
    struct SubscribeApiParams {
        name: String,
        email: String,
    }

    match call_api("/api/subscriptions", SubscribeApiParams { name, email }).await {
        Ok(()) => {
            tracing::info!("call_api returned Ok");
            error_message.set(None);
            info_message.set(Some(Message::AddSubscriberSucceeded));
        }
        Err(error) => {
            tracing::info!("call_api returned Err");
            let message = if let ApiError::ServerError(m) = error {
                m
            } else {
                Message::InternalError
            };
            error_message.set(Some(message));
            info_message.set(None);
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
                if let Ok(text) = response.text().await {
                    tracing::info!("Response text: \"{text}\"");
                } else {
                    tracing::info!("Couldn't read response text");
                }

                //let output = response.json::<Output>().await?;
                //Ok(output)
                Ok(())
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
enum ApiError {
    #[error("Server returned error: '{0}'")]
    ServerError(Message),

    #[error("Error making HTTP request error")]
    ReqwestError(#[from] reqwest::Error),

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
    AddSubscriberSucceeded,
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
            Self::AddSubscriberSucceeded => "You have subscribed to our newsletter. Stay tuned, you're going to love it!",
            //Self::PublishNewsletterSucceeded => "Your newsletter publish request has been accepted, and emails will go out shortly.",
        }
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
