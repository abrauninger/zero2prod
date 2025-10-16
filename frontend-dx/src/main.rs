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

#[derive(serde::Serialize)]
struct SubscribeApiParams {
    name: String,
    email: String,
}

#[derive(Debug, serde::Deserialize)]
struct SubscribeApiResponse {
    error_id: String,
}

#[component]
fn SubscribeForm() -> Element {
    let name = use_signal(|| "".to_string());
    let email = use_signal(|| "".to_string());

    rsx! {
        AppForm {
            heading: "Welcome to our newsletter",
            onsubmit: move || async move {
                tracing::info!("Form submitted!");

                let params = SubscribeApiParams {
                    name: name(),
                    email: email(),
                };

                // https://github.com/seanmonstar/reqwest/issues/1433
                let url_base = web_sys::window().unwrap().location().origin().unwrap();

                tracing::info!("url_base: {url_base}");

                let response = reqwest::Client::new()
                    .post(format!("{url_base}/api/subscriptions"))
                    .json(&params)
                    .send()
                    .await;

                tracing::info!("Request completed: {response:?}");

                // TODO: Check response and show errors
                match response {
                    Ok(response) => {
                        tracing::info!("Response was success");

                        // TODO: Don't use hard-coded constants for status codes
                        let status = response.status();
                        if response.status() != 200 {
                            tracing::info!("Response was a non-successful status code: {status}");

                            let api_response = response.json::<SubscribeApiResponse>().await;
                            tracing::info!("api_response: {api_response:?}");
                        }
                    }
                    Err(e) => {
                        tracing::error!("Error: {e:?}")
                    }
                }
            },
            p {
                "To subscribe to our newsletter, enter your information here."
            }
            FormTextField {
                value: name,
                label: "Name",
                placeholder: "Enter your name"
            }
            FormTextField {
                value: email,
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
fn FormTextField(value: Signal<String>, label: String, placeholder: String) -> Element {
    rsx! {
        div {
            label {
                class: "text-gray-700",
                "{label}",
            }
            input {
                value: "{value}",
                oninput: move |e| value.set(e.value()),
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
