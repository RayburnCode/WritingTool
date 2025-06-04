// client/components/auth/login.rs
use dioxus::prelude::*;
use crate::auth::AuthRoute;
use crate::auth::common::{AuthFormContainer, AuthInputField, AuthButton, AuthLink};

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
                println!("Logged in with email: {}", form.read().email);
                navigator.push("/");
            }

            loading.set(false);
        });
    };

    rsx! {
        AuthFormContainer {
            title: "Sign in to your account",
            footer: rsx! {
                AuthLink {
                    to: AuthRoute::Register {},
                    text: "Don't have an account?",
                    link_text: "Sign up",
                }
            },

            form { class: "mt-8 space-y-6", onsubmit: handle_submit,
                if let Some(err) = error.read().as_ref() {
                    div { class: "p-4 text-sm text-red-600 bg-red-50 rounded-md",
                        p { "{err}" }
                    }
                }

                div { class: "space-y-4",
                    AuthInputField {
                        id: "email",
                        label: "Email address",
                        r#type: "email",
                        value: form.read().email.clone(),
                        disabled: loading(),
                        oninput: move |e: dioxus::events::FormEvent| form.with_mut(|f| f.email = e.value()),
                        error: None,
                    }

                    AuthInputField {
                        id: "password",
                        label: "Password",
                        r#type: "password",
                        value: form.read().password.clone(),
                        disabled: loading(),
                        oninput: move |e: dioxus::events::FormEvent| form.with_mut(|f| f.password = e.value()),
                        error: None,
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
                            "Forgot password?"
                        }
                    }
                }

                AuthButton {
                    loading: loading(),
                    label: "Sign in",
                    loading_label: "Signing in...",
                }
            }
        }
    }
}