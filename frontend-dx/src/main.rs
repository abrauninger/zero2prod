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
    rsx! {
        AppForm {
            heading: "Welcome to our newsletter",
            p {
                "To subscribe to our newsletter, enter your information here."
            }
            FormTextField {
                id: "name",
                label: "Name",
                placeholder: "Enter your name"
            }
            FormTextField {
                id: "email",
                label: "Email address",
                placeholder: "Enter your email address"
            }
            SubmitButton {
                "Subscribe"
            }
        }
    }
}

#[component]
fn AppForm(heading: String, children: Element) -> Element {
    rsx! {
        div {
            class: "mx-auto max-w-xl py-12 px-6",
            AppHeading {
                {heading}
            }
            form {
                onsubmit: move |event| {
                    event.prevent_default();
                    tracing::info!("Submitted! {event:?}");
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
fn FormTextField(id: String, label: String, placeholder: String) -> Element {
    rsx! {
        div {
            label {
                class: "text-gray-700",
                "{label}",
            }
            input {
                id,
                placeholder,
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
