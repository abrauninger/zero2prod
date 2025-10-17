mod api;

use dioxus::prelude::*;
use dioxus_primitives::dropdown_menu::{
    self, DropdownMenu, DropdownMenuContent, DropdownMenuContentProps, DropdownMenuItem,
    DropdownMenuItemProps, DropdownMenuProps, DropdownMenuTrigger, DropdownMenuTriggerProps,
};

use crate::api::{add_subscriber, Message};

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
        UserMenu {}
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

#[component]
#[allow(clippy::unnecessary_cast)]
fn UserMenu() -> Element {
    rsx! {
        DropdownMenu {
            class: "relative inline-block",
            DropdownMenuTrigger {
                class: "flex rounded-md px-1 py-1 font-bold hover:bg-gray-400",
                "Log in"
            }
            DropdownMenuContent {
                class: "absolute left-0 w-56 origin-top-right bg-white rounded-md px-1 py-1 shadow-lg ring-1 ring-black/5 focus:outline-none",
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
            }
        }
    }
}
