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
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
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
        div {
            p {
                "To subscribe to our newsletter, enter your information here."
            }
            FormTextField {
                id: "name",
                label: "Name",
                placeholder: "Enter your name"
            }
        }
    }
}

#[derive(Clone, PartialEq, Props)]
struct FormTextFieldProps {
    id: String,
    label: String,
    placeholder: String,
}

#[component]
fn FormTextField(props: FormTextFieldProps) -> Element {
    rsx! {
        div {
            label {
                class: "text-gray-700",
                "{props.label}",
            }
            input {
                id: props.id,
                placeholder: props.placeholder,
                class: "rounded mt-1 block w-full"
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
