mod api;

use dioxus::prelude::*;
use dioxus_primitives::dropdown_menu::{
    DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger,
};

use crate::api::{add_subscriber, get_username, Message};

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

#[component]
fn App() -> Element {
    use_effect(move || {
        spawn(async move {
            *USERNAME.write() = get_username().await;
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

    let mut error_message: Signal<Option<Message>> = use_signal(|| None);
    let mut info_message: Signal<Option<Message>> = use_signal(|| None);

    rsx! {
        UserMenu { }
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
fn LoginForm() -> Element {
    rsx! {
        "TODO: Login form"
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
fn UserMenu() -> Element {
    rsx! {
        if let Some(_username) = USERNAME() {
            UserMenuLoggedIn {  }
        } else {
            a {
                onclick: |_| { navigator().push(Route::LoginForm {}); },
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
        p { {USERNAME()} }
        DropdownMenu {
            on_open_change: move |value| {
                is_open.set(value);
            },
            class: "relative inline-block",
            DropdownMenuTrigger {
                class: "flex rounded-md px-1 py-1 font-bold hover:bg-gray-400",
                "Log in"
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
            }
        }
    }
}
