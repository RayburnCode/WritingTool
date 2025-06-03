use dioxus::prelude::*;
use ui::auth::{Login, Register, ResetPassword, AuthRoute};

#[component]
pub fn AuthPages() -> Element {
    // Track the current auth page
    let nav = use_navigator();
    let mut current_page = use_signal(|| "login".to_string());

    // Handle page changes
    let mut handle_page_change = move |page: &str| {
        current_page.set(page.to_string());
        match page {
            "login" => nav.push(AuthRoute::Login {}),
            "register" => nav.push(AuthRoute::Register {}),
            "reset" => nav.push(AuthRoute::ResetPassword {}),
            _ => nav.push(AuthRoute::Login {}),
        }
    };

    rsx! {
        div { class: "min-h-screen bg-gray-50 flex flex-col",
            // Auth navigation tabs
            div { class: "bg-white shadow-sm",
                div { class: "max-w-md mx-auto px-4",
                    div { class: "flex border-b border-gray-200",
                        AuthTabButton {
                            active: current_page() == "login",
                            onclick: EventHandler::new(move |_| {
                                handle_page_change("login");
                            }),
                            label: "Sign In",
                        }
                        AuthTabButton {
                            active: current_page() == "register",
                            onclick: EventHandler::new(move |_| {
                                handle_page_change("register");
                            }),
                            label: "Register",
                        }
                        AuthTabButton {
                            active: current_page() == "reset",
                            onclick: EventHandler::new(move |_| {
                                handle_page_change("reset");
                            }),
                            label: "Reset Password",
                        }
                    }
                }
            }

            // Main content area
            main { class: "flex-1 flex items-center justify-center p-4",
                div { class: "w-full max-w-md bg-white rounded-lg shadow-md p-8",
                    match current_page().as_str() {
                        "login" => rsx! {
                            Login {}
                        },
                        "register" => rsx! {
                            Register {}
                        },
                        "reset" => rsx! {
                            ResetPassword {}
                        },
                        _ => rsx! {
                            Login {}
                        },
                    }
                }
            }
        }
    }
}

#[component]
fn AuthTabButton(active: bool, onclick: EventHandler<()>, label: &'static str) -> Element {
    rsx! {
        button {
            class: if active { "flex-1 py-4 px-1 text-center border-b-2 border-indigo-500 text-sm font-medium text-indigo-600" } else { "flex-1 py-4 px-1 text-center border-b-2 border-transparent text-sm font-medium text-gray-500 hover:text-gray-700 hover:border-gray-300" },
            onclick: move |_| onclick.call(()),
            "{label}"
        }
    }
}