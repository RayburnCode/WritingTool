// client/components/auth/register.rs
use dioxus::prelude::*;
use crate::auth::AuthRoute;
use crate::auth::common::{AuthFormContainer, AuthInputField, AuthButton, AuthLink};

#[derive(Default, Clone)]
struct RegisterForm {
    email: String,
    password: String,
    confirm_password: String,
    name: String,
}

#[component]
pub fn Register() -> Element {
    let mut form = use_signal(RegisterForm::default);
    let mut error = use_signal(|| None::<String>);
    let mut loading = use_signal(|| false);
    let navigator = use_navigator();

    let handle_submit = move |evt: FormEvent| {
        evt.prevent_default();
        loading.set(true);

        spawn(async move {
            // Simulate API call
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            
            // Validation
            if form.read().email.is_empty() 
                || form.read().password.is_empty() 
                || form.read().confirm_password.is_empty() 
                || form.read().name.is_empty() {
                error.set(Some("Please fill in all fields".to_string()));
            } else if !form.read().email.contains('@') {
                error.set(Some("Please enter a valid email".to_string()));
            } else if form.read().password != form.read().confirm_password {
                error.set(Some("Passwords do not match".to_string()));
            } else if form.read().password.len() < 8 {
                error.set(Some("Password must be at least 8 characters".to_string()));
            } else {
                // Successful registration
                println!("Registered with email: {}", form.read().email);
                navigator.push(AuthRoute::Login {});
            }
            
            loading.set(false);
        });
    };

    rsx! {
        AuthFormContainer {
            title: "Create a new account",
            footer: rsx! {
                AuthLink {
                    to: AuthRoute::Login {},
                    text: "Already have an account?",
                    link_text: "Sign in",
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
                        id: "name",
                        label: "Full name",
                        r#type: "text",
                        value: form.read().name.clone(),
                        disabled: loading(),
                        oninput: move |e: dioxus::events::FormEvent| form.with_mut(|f| f.name = e.value()),
                        error: None,
                    }

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

                    AuthInputField {
                        id: "confirm_password",
                        label: "Confirm Password",
                        r#type: "password",
                        value: form.read().confirm_password.clone(),
                        disabled: loading(),
                        oninput: move |e: dioxus::events::FormEvent| {
                            form.with_mut(|f| f.confirm_password = e.value())
                        },
                        error: None,
                    }
                }

                AuthButton {
                    loading: loading(),
                    label: "Register",
                    loading_label: "Creating account...",
                }
            }
        }
    }
}