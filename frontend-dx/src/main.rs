mod api;

use std::future::Future;

use dioxus::prelude::*;
use dioxus_primitives::dropdown_menu::{
    DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger,
};

use crate::api::{add_subscriber, get_username, login, logout, ApiError, Message};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    SubscribeForm {},

    #[route("/login")]
    LoginForm {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus_logger::init(dioxus_logger::tracing::Level::INFO).expect("failed to init logger");
    dioxus::launch(App);
}

static USERNAME: GlobalSignal<Option<String>> = Global::new(|| None);
static MESSAGES: GlobalSignal<Messages> = Global::new(|| Messages {
    error: None,
    info: None,
});

#[component]
fn App() -> Element {
    use_effect(move || {
        spawn(async move {
            *USERNAME.write() = get_username().await.ok();
        });
    });

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

    rsx! {
        UserMenu { }
        AppForm {
            heading: "Welcome to our newsletter",
            onsubmit: move || async move {
                call_api(async || {
                    let result = add_subscriber(name(), email()).await;

                    MESSAGES.write().info = if result.is_ok() {
                        Some(Message::AddSubscriberSucceeded)
                    } else {
                        None
                    };

                    result
                }).await;
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
            MessageDisplay {}
        }
    }
}

#[component]
fn LoginForm() -> Element {
    let username = use_signal(|| "".to_string());
    let password = use_signal(|| "".to_string());

    rsx! {
        AppForm {
            heading: "Log in",
            onsubmit: move || async move {
                call_api(async || {
                    login(username(), password()).await?;

                    // Even though we already know the username, fetch it again after we've successfully
                    // logged in.  (In the future we'll fetch the user's name or initials here.)
                    *USERNAME.write() = Some(get_username().await?);

                    // TODO: Navigate to a better place after successful login!
                    navigate_to(Route::SubscribeForm {});

                    Ok(())
                }).await;
            },

            FormTextField {
                value: username,
                name: "username",
                label: "Username",
                autocomplete: "username",
                placeholder: "Enter your username",
            }

            FormTextField {
                value: password,
                name: "password",
                field_type: "password",
                label: "Password",
                autocomplete: "password",
                placeholder: "Enter your password",
            }

            SubmitButton {
                "Log in"
            }

            MessageDisplay {}
        }
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
    #[props(default = "text".to_string())] field_type: String,
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
                type: field_type,
                placeholder,
                autocomplete,
                oninput: move |e| value.set(e.value()),
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
fn MessageDisplay() -> Element {
    rsx! {
        if let Some(ref message) = MESSAGES.read().error {
            div {
                class: "text-red-900 bg-red-200 border border-solid border-red-900 p-2 mt-2",
                {message.to_string()}
            }
        }

        if let Some(ref message) = MESSAGES.read().info {
            div {
                class: "text-green-900 bg-green-200 border border-solid border-green-900 p-2 mt-2",
                {message.to_string()}
            }
        }
    }
}

struct Messages {
    // TODO: Should we have a list of messages of various types?
    error: Option<Message>,
    info: Option<Message>,
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

#[component]
fn UserMenu() -> Element {
    rsx! {
        if let Some(_username) = USERNAME() {
            UserMenuLoggedIn {  }
        } else {
            a {
                onclick: |_| { navigate_to(Route::LoginForm {}); },
                class: "text-gray-900 hover:bg-gray-400 rounded-md px-2 py-2 cursor-default",
                "Log in"
            }
        }
    }
}

#[component]
#[allow(clippy::unnecessary_cast)]
fn UserMenuLoggedIn() -> Element {
    let mut is_open = use_signal(|| false);

    const BUTTON_HOVER_OPEN_BACKGROUND: &str = "bg-gray-400";

    let button_dynamic_style = use_memo(move || {
        if is_open() {
            BUTTON_HOVER_OPEN_BACKGROUND
        } else {
            ""
        }
    });

    // The entrance/exit animations for the menu are triggered by the 'is_open' signal, but if we use
    // 'is_open' directly we'll only run the exit animation; the entrance animation doesn't run
    // because the menu contents would be added to the DOM immediately when 'is_open' becomes true
    // which means that the browser doesn't see the menu contents in the DOM in their pre-entrance state.
    // To get the entrance animation to run properly, we wait for the menu contents to appear in the DOM
    // (with their pre-entrance styling), then after a short delay we update the styling.
    //
    // This hack is related to this code in Dioxus Components:
    // https://github.com/DioxusLabs/components/blob/723d43cd1b4e433599881139bafad4b07cdbae46/primitives/src/dropdown_menu.rs#L320C9-L320C22
    // Specifically the `if render()` check:
    //
    // rsx! {
    //     if render() {
    //         div {
    //
    let is_delayed_open = use_resource(move || async move {
        if is_open() {
            // Yield execution to give time for the menu contents to appear in the DOM.
            // Right now we do this with 'eval' to call into JavaScript and yield back.
            // TODO: Some better way?  Do we need JavaScript here?
            // Note that a timeout of 0 works on Chrome but not Safari, so we use a timeout of 10ms.
            let _ = dioxus::document::eval("await new Promise(r => setTimeout(r, 10));").await;
            true
        } else {
            false
        }
    });

    let animated_classes = use_memo(move || {
        if let Some(true) = is_delayed_open() {
            "opacity-100 scale-100"
        } else {
            "opacity-0 scale-95"
        }
    });

    // The easing function is also dependent on whether the menu is opening or closing, but we don't need
    // to delay setting it.
    let animated_styles = use_memo(move || {
        if is_open() {
            "transition-timing-function: ease-out;"
        } else {
            "transition-timing-function: ease-in;"
        }
    });

    rsx! {
        DropdownMenu {
            on_open_change: move |value| {
                is_open.set(value);
            },
            class: "relative inline-block",
            DropdownMenuTrigger {
                class: format!("flex rounded-md px-1 py-1 font-bold hover:bg-gray-400 {button_dynamic_style}"),
                {USERNAME()}
            }
            DropdownMenuContent {
                class: format!("{animated_classes} absolute left-0 w-56 origin-top-right bg-white rounded-md px-1 py-1 shadow-lg ring-1 ring-black/5 focus:outline-none"),
                style: format!("{animated_styles} transition-property: all; transition-duration: 100ms;"),
                DropdownMenuItem {
                    class: "px-2 py-2 text-md rounded-md text-gray-900 hover:bg-blue-500 hover:text-white",
                    index: 0 as usize,
                    value: "",
                    on_select: |_value: String| {
                        tracing::info!("First menu item clicked");
                    },
                    "First item"
                }
                DropdownMenuItem {
                    class: "px-2 py-2 text-md rounded-md text-gray-900 hover:bg-blue-500 hover:text-white",
                    index: 1 as usize,
                    value: "",
                    on_select: |_value: String| {
                        tracing::info!("Second menu item clicked");
                    },
                    "Second item"
                }
                DropdownMenuItem {
                    class: "px-2 py-2 text-md rounded-md text-gray-900 hover:bg-blue-500 hover:text-white",
                    index: 1 as usize,
                    value: "",
                    on_select: async |_value: String| {
                        logout().await;
                    },
                    "Log out"
                }
            }
        }
    }
}

async fn call_api<Output, F: Future<Output = Result<Output, ApiError>>>(
    api: impl FnOnce() -> F,
) -> Option<Output> {
    MESSAGES.write().info = None;
    MESSAGES.write().error = None;

    match api().await {
        Ok(output) => {
            tracing::info!("'call_api' sees an 'Ok' result");
            Some(output)
        }
        Err(api_error) => {
            tracing::info!("'call_api' sees an 'Err' result; setting MESSAGES.error");
            let message = if let ApiError::ServerError(message) = api_error {
                message
            } else {
                Message::InternalError
            };

            MESSAGES.write().error = Some(message);
            None
        }
    }
}

fn navigate_to(route: Route) {
    // Clear the displayed messages any time we go somewhere else.
    let mut messages = MESSAGES.write();
    messages.error = None;
    messages.info = None;
    navigator().push(route);
}
