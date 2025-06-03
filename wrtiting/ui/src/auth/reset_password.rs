// client/components/auth/reset_password.rs
use dioxus::prelude::*;
use crate::auth::AuthRoute;
use crate::auth::common::{AuthFormContainer, AuthInputField, AuthButton, AuthLink};

#[derive(Default, Clone)]
struct ResetPasswordForm {
    email: String,
}

#[component]
pub fn ResetPassword() -> Element {
    let mut form = use_signal(ResetPasswordForm::default);
    let mut error = use_signal(|| None::<String>);
    let mut success = use_signal(|| false);
    let mut loading = use_signal(|| false);

    let handle_submit = move |evt: FormEvent| {
        evt.prevent_default();
        loading.set(true);

        spawn(async move {
            // Simulate API call
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            
            // Validation
            if form.read().email.is_empty() {
                error.set(Some("Please enter your email".to_string()));
            } else if !form.read().email.contains('@') {
                error.set(Some("Please enter a valid email".to_string()));
            } else {
                // Successful submission
                success.set(true);
            }
            
            loading.set(false);
        });
    };

    rsx! {
        AuthFormContainer {
            title: "Reset your password",
            footer: rsx! {
                AuthLink {
                    to: AuthRoute::Login {},
                    text: "Remember your password?",
                    link_text: "Sign in",
                }
            },

            if *success.read() {
                div { class: "p-4 text-sm text-green-600 bg-green-50 rounded-md",
                    p {
                        "We've sent a password reset link to {form.read().email}. "
                        "Please check your email."
                    }
                }
            } else {
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
                    }

                    div { class: "text-sm text-gray-600",
                        "Enter your email and we'll send you a link to reset your password."
                    }

                    AuthButton {
                        loading: loading(),
                        label: "Send reset link",
                        loading_label: "Sending...",
                    }
                }
            }
        }
    }
}