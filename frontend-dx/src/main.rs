use std::{fmt::Display, sync::LazyLock};

use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    SubscribeForm {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus_logger::init(dioxus_logger::tracing::Level::INFO).expect("failed to init logger");
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS } document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

#[component]
fn SubscribeForm() -> Element {
    let name = use_signal(|| "".to_string());
    let email = use_signal(|| "".to_string());

    let mut error_message: Signal<Option<Message>> = use_signal(|| None);
    let mut info_message: Signal<Option<Message>> = use_signal(|| None);

    rsx! {
        AppForm {
            heading: "Welcome to our newsletter",
            onsubmit: move || async move {
                add_subscriber(name(), email(), &mut error_message, &mut info_message).await;
            },
            p {
                "To subscribe to our newsletter, enter your information here."
            }
            FormTextField {
                value: name,
                name: "name",
                label: "Name",
                autocomplete: "name",
                placeholder: "Enter your name"
            }
            FormTextField {
                value: email,
                name: "email",
                label: "Email address",
                autocomplete: "email",
                placeholder: "Enter your email address"
            }
            SubmitButton {
                "Subscribe"
            }
            MessageDisplay {
                error: error_message,
                info: info_message
            }
        }
    }
}

static BASE_URL: LazyLock<String> = LazyLock::new(|| {
    // https://github.com/seanmonstar/reqwest/issues/1433
    web_sys::window().unwrap().location().origin().unwrap()
});

async fn add_subscriber(
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
enum Message {
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

#[component]
fn AppForm(heading: String, children: Element, onsubmit: EventHandler<()>) -> Element {
    rsx! {
        div {
            class: "mx-auto max-w-xl py-12 px-6",
            AppHeading {
                {heading}
            }
            form {
                onsubmit: move |event| {
                    event.prevent_default();
                    onsubmit(());
                },
                div {
                    class: "grid grid-cols-1 gap-6 mt-8",
                    {children}
                }
            }
        }
    }
}

#[component]
fn AppHeading(children: Element) -> Element {
    rsx! {
        h1 {
            class: "text-4xl font-bold",
            {children}
        }
    }
}

#[component]
fn FormTextField(
    value: Signal<String>,
    label: String,
    name: String,
    autocomplete: String,
    placeholder: String,
) -> Element {
    rsx! {
        div {
            label {
                for: name.clone(),
                class: "text-gray-700",
                "{label}",
            }
            input {
                id: name.clone(),
                name,
                value: "{value}",
                oninput: move |e| value.set(e.value()),
                placeholder,
                autocomplete,
                class: "rounded mt-1 block w-full"
            }
        }
    }
}

#[component]
fn SubmitButton(children: Element) -> Element {
    rsx! {
        div {
            button {
                type: "submit",
                class: "text-white bg-blue-500 hover:bg-blue-800 font-medium text-sm rounded-lg px-5 py-2.5 mt-8",
                {children}
            }
        }
    }
}

#[component]
fn MessageDisplay(
    #[props(!optional)] error: ReadSignal<Option<Message>>,
    #[props(!optional)] info: ReadSignal<Option<Message>>,
) -> Element {
    rsx! {
        if let Some(ref message) = *error.read() {
            div {
                class: "text-red-900 bg-red-200 border border-solid border-red-900 p-2 mt-2",
                {message.to_string()}
            }
        }

        if let Some(ref message) = *info.read() {
            div {
                class: "text-green-900 bg-green-200 border border-solid border-green-900 p-2 mt-2",
                {message.to_string()}
            }
        }
    }
}

/// Shared navbar component.
#[component]
fn Navbar() -> Element {
    rsx! {
        div {
            id: "navbar",
            Link {
                to: Route::SubscribeForm {},
                "Home"
            }
        }

        Outlet::<Route> {}
    }
}
