use dioxus::{html::p, prelude::*};
use crate::auth::AuthRoute;
// Import or define your Route type here





#[derive(Default, Clone)]
struct LoginForm {
    email: String,
    password: String,
    remember: bool,
}

#[component]
pub fn Login() -> Element {
    let mut form = use_signal(LoginForm::default);
    let mut error = use_signal(|| None::<String>);
    let mut loading = use_signal(|| false);
    let navigator = use_navigator();

    let handle_submit = move |evt: FormEvent| {
        evt.prevent_default();
        loading.set(true);

        spawn(async move {
            // Simulate API call
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            
            // Simple validation
            if form.read().email.is_empty() || form.read().password.is_empty() {
                error.set(Some("Please fill in all fields".to_string()));
            } else if !form.read().email.contains('@') {
                error.set(Some("Please enter a valid email".to_string()));
            } else {
                // Successful login
                // navigator.push(Route::Home {});
                /// WILL NEED TO CHANGE THIS TO YOUR ACTUAL ROUTE
                println!("Logged in with email: {}", form.read().email);
            }
            
            loading.set(false);
        });
    };

    rsx! {
        div { class: "min-h-screen flex items-center justify-center bg-gray-50",
            div { class: "w-full max-w-md p-8 space-y-8 bg-white rounded-lg shadow",
                div { class: "text-center",
                    if let Some(err) = &*error.read() {
                        div { class: "p-4 text-sm text-red-600 bg-red-50 rounded-md",
                            p { "{err}" }
                        }
                    }
                }

                form { class: "mt-8 space-y-6", onsubmit: handle_submit,
                    input { r#type: "hidden", name: "remember", value: "true" }

                    div { class: "space-y-4",
                        div {
                            label {
                                r#for: "email",
                                class: "block text-sm font-medium text-gray-700",
                                "Email address"
                            }
                            input {
                                id: "email",
                                value: "{form.read().email}",
                                disabled: *loading.read(),
                                oninput: move |e| form.with_mut(|f| f.email = e.value().to_string()),
                                class: "w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500",
                            }
                        }

                        div {
                            label {
                                r#for: "password",
                                class: "block text-sm font-medium text-gray-700",
                                "Password"
                            }
                            input {
                                id: "password",
                                value: "{form.read().password}",
                                disabled: *loading.read(),
                                oninput: move |e| form.with_mut(|f| f.password = e.value().to_string()),
                                class: "w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500",
                            }
                        }
                    }

                    div { class: "flex items-center justify-between",
                        div { class: "flex items-center",
                            input {
                                r#type: "checkbox",
                                id: "remember-me",
                                checked: form.read().remember,
                                onchange: move |e| form.with_mut(|f| f.remember = e.value().parse().unwrap_or(false)),
                                class: "h-4 w-4 text-indigo-600 focus:ring-indigo-500 border-gray-300 rounded",
                            }
                            label {
                                r#for: "remember-me",
                                class: "ml-2 block text-sm text-gray-900",
                                "Remember me"
                            }
                        }

                        div { class: "text-sm",
                            Link {
                                to: AuthRoute::ResetPassword {},
                                class: "font-medium text-indigo-600 hover:text-indigo-500",
                                "Forgot your password?"
                            }
                        }
                    }

                    button {
                        r#type: "submit",
                        disabled: *loading.read(),
                        class: "w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500",
                        if *loading.read() {
                            "Signing in..."
                        } else {
                            "Sign in"
                        }
                    }
                }

                div { class: "text-center text-sm",
                    Link {
                        to: AuthRoute::Register {},
                        class: "font-medium text-indigo-600 hover:text-indigo-500",
                        "Don't have an account? Sign up"
                    }
                }
            }
        }
    }
}