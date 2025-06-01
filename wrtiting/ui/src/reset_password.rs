use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

// Form data structure
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct ResetPasswordForm {
    pub email: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct ResetPasswordState {
    pub data: ResetPasswordForm,
    pub success: bool,
    pub loading: bool,
    pub error: Option<String>,
}

#[component]
pub fn ResetPasswordPage(cx: Scope) -> Element {
    let form_state = use_state(cx, ResetPasswordState::default);

    // Handle form submission
    let handle_submit = move |event: FormEvent| {
        event.prevent_default();
        
        form_state.with_mut(|state| {
            state.loading = true;
            state.error = None;
        });
        
        // In a real app, you would make an async call to your backend here
        async {
            // Simulate API call delay
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            
            // This would be your actual API response handling
            let result = simulate_reset_password_api(&form_state.get().data).await;
            
            match result {
                ResetResult::Success => {
                    form_state.with_mut(|state| {
                        state.success = true;
                        state.loading = false;
                    });
                }
                ResetResult::Error(message) => {
                    form_state.with_mut(|state| {
                        state.error = Some(message);
                        state.loading = false;
                    });
                }
            }
        }
    };

    // Handle input changes
    let handle_email_change = move |value: String| {
        form_state.with_mut(|state| {
            state.data.email = value;
            state.error = None;
        });
    };

    cx.render(rsx! {
        div { class: "flex h-full w-full flex-col items-center",
            h2 { class: "text-base-content mt-2 text-center text-3xl font-bold tracking-tight",
                "Reset Your Password"
            }
            p { class: "mt-1 text-center",
                "We'll send you an email with a link to reset your password."
            }
            
            form {
                action: "/resetPassword",
                method: "POST",
                class: "flex w-full flex-col items-center space-y-2 pt-4",
                onsubmit: handle_submit,
                
                // Email field
                div { class: "form-control w-full max-w-md",
                    label { 
                        r#for: "email", 
                        class: "label pb-1 font-medium",
                        span { class: "label-text", "Email" }
                    }
                    input {
                        r#type: "email",
                        name: "email",
                        id: "email",
                        value: "{form_state.get().data.email}",
                        oninput: move |e| handle_email_change(e.value.clone()),
                        class: "input input-bordered w-full max-w-md",
                        required: true,
                        disabled: form_state.get().loading || form_state.get().success,
                    }
                }
                
                // Submit button
                div { class: "w-full max-w-md pt-2",
                    button {
                        r#type: "submit",
                        disabled: form_state.get().loading || form_state.get().success,
                        class: "btn btn-primary w-full",
                        
                        if form_state.get().loading {
                            rsx! { "Sending request..." }
                        } else if form_state.get().success {
                            rsx! { "Request Sent" }
                        } else {
                            rsx! { "Request Password Reset" }
                        }
                    }
                }
                
                // Success message
                if form_state.get().success {
                    div { class: "alert alert-success w-full max-w-md shadow-lg",
                        div { class: "flex items-center",
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                class: "h-6 w-6 flex-shrink-0 stroke-current",
                                fill: "none",
                                view_box: "0 0 24 24",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    d: "M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
                                }
                            }
                            span { class: "ml-2", "An email has been sent to reset your password!" }
                        }
                    }
                }
                
                // Error message
                if let Some(error) = &form_state.get().error {
                    div { class: "alert alert-error w-full max-w-md shadow-lg",
                        div { class: "flex items-center",
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                class: "h-6 w-6 flex-shrink-0 stroke-current",
                                fill: "none",
                                view_box: "0 0 24 24",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    d: "M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
                                }
                            }
                            span { class: "ml-2", "{error}" }
                        }
                    }
                }
            }
        }
    })
}

// Simulated API response types
enum ResetResult {
    Success,
    Error(String),
}

// Simulated reset password API
async fn simulate_reset_password_api(form: &ResetPasswordForm) -> ResetResult {
    // In a real app, this would be an actual API call
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    
    // Simple validation for demo purposes
    if form.email.is_empty() || !form.email.contains('@') {
        return ResetResult::Error("Please enter a valid email address".to_string());
    }
    
    // Simulate success after validation
    ResetResult::Success
}