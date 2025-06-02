use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;

// Notification types
#[derive(Clone, PartialEq)]
pub enum NotificationType {
    Success,
    Danger,
    Info,
    Warn,
}

#[derive(Clone, PartialEq)]
pub struct Notification {
    pub mode: NotificationType,
    pub message: String,
    pub lifetime: Option<u32>,
}

// Form data structure
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
    pub remember: bool,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct FormErrors {
    pub email: Option<String>,
    pub password: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct LoginState {
    pub data: LoginForm,
    pub errors: FormErrors,
    pub not_verified: bool,
    pub loading: bool,
}

#[component]
pub fn Login() -> Element {
    let form_state = use_signal(LoginState::default);
    let navigator = use_navigator();
    let notifications = use_signal::<Vec<Notification>>(Vec::new);

    // Notification messages
    let messages = json!({
        "success": "Login successful!",
        "invalid": "Invalid credentials",
        "error": "Something went wrong!",
        "notVerified": "You must verify your email before you can login."
    });

    // Show notification
    let show_notification = |mode: NotificationType, message: String| {
        notifications.with_mut(|notifs| {
            notifs.push(Notification {
                mode,
                message,
                lifetime: Some(3000),
            });
        });
    };

    // Handle form submission
    let handle_submit = move |event: FormEvent| {
        event.prevent_default();

        form_state.with_mut(|state| state.loading = true);

        // Clone the form data before entering the async block
        let form_data = form_state.get().data.clone();

        // In a real app, you would make an async call to your backend here
        spawn(async move {
            // Simulate API call delay
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;

            // This would be your actual API response handling
            let result = simulate_login_api(&form_data).await;

            match result {
                LoginResult::Success { redirect } => {
                    show_notification(NotificationType::Success, messages["success"].to_string());
                    if let Some(redirect) = redirect {
                        navigator.push(redirect);
                    }
                }
                LoginResult::InvalidCredentials => {
                    show_notification(NotificationType::Danger, messages["invalid"].to_string());
                }
                LoginResult::NotVerified => {
                    form_state.with_mut(|state| state.not_verified = true);
                    show_notification(NotificationType::Info, messages["notVerified"].to_string());
                }
                LoginResult::Error(_) => {
                    show_notification(NotificationType::Danger, messages["error"].to_string());
                }
            }

            form_state.with_mut(|state| state.loading = false);
        });
    };

    // Handle input changes
    let handle_input_change = move |field: &str, value: String| {
        form_state.with_mut(|state| match field {
            "email" => state.data.email = value,
            "password" => state.data.password = value,
            _ => (),
        });
    };

    // Handle remember me checkbox
    let handle_remember_change = move |checked: bool| {
        form_state.with_mut(|state| state.data.remember = checked);
    };

    rsx! {
    div { 
        class: "w-full bg-gray-50 dark:bg-gray-900 min-h-screen",
        div { 
            class: "pt:mt-0 mx-auto flex flex-col items-center justify-center px-6 pt-8 md:h-screen dark:bg-gray-900",
            Link { 
                to: "/", 
                class: "my-2 flex items-center justify-center pt-5 text-2xl font-semibold lg:mb-10 dark:text-white",
                img { 
                    class: "mr-4 h-11", 
                    src: "/static/logo.svg", 
                    alt: "Logo" 
                }
                span { "The Title" }
            }
            
            div { 
                class: "flex w-full max-w-xl flex-col divide-gray-200 rounded-lg border-gray-200 bg-white p-4 text-gray-500 shadow-md sm:p-6 dark:divide-gray-700 dark:border-gray-700 dark:bg-gray-800 dark:text-gray-400",
                h1 { 
                    class: "text-2xl font-bold text-gray-900 dark:text-white", 
                    "Sign in to platform" 
                }
                
                form {
                    class: "flex w-full flex-col items-center space-y-2 pt-4",
                    onsubmit: handle_submit,
                    
                    // Email field
                    div {
                        label {
                            r#for: "email",
                            class: "mb-2 block text-sm font-medium text-gray-900 rtl:text-right dark:text-white",
                            "Your email"
                        }
                        div { 
                            class: "relative w-full",
                            input {
                                r#type: "email",
                                id: "email",
                                name: "email",
                                placeholder: "name@company.com",
                                required: true,
                                value: "{form_state.get().data.email}",
                                disabled: form_state.get().loading,
                                oninput: move |e| handle_input_change("email", e.value.clone()),
                                class: "w-full rounded border border-gray-300 bg-gray-50 p-2.5 text-gray-900 focus:border-primary-600 focus:ring-primary-600 dark:border-gray-600 dark:bg-gray-700 dark:text-white dark:placeholder-gray-400 dark:focus:border-primary-500 dark:focus:ring-primary-500 sm:text-sm"
                            }
                            if let Some(err) = &form_state.get().errors.email {
                                p { 
                                    class: "mt-2 text-sm text-red-600 dark:text-red-500", 
                                    "{err}" 
                                }
                            }
                        }
                    }
                    
                    // Password field
                    div {
                        label {
                            r#for: "password",
                            class: "mb-2 block text-sm font-medium text-gray-900 rtl:text-right dark:text-white",
                            "Your password"
                        }
                        div { 
                            class: "relative w-full",
                            input {
                                r#type: "password",
                                id: "password",
                                name: "password",
                                placeholder: "password",
                                required: true,
                                value: "{form_state.get().data.password}",
                                disabled: form_state.get().loading,
                                oninput: move |e| handle_input_change("password", e.value.clone()),
                                class: "w-full rounded border border-gray-300 bg-gray-50 p-2.5 text-gray-900 focus:border-primary-600 focus:ring-primary-600 dark:border-gray-600 dark:bg-gray-700 dark:text-white dark:placeholder-gray-400 dark:focus:border-primary-500 dark:focus:ring-primary-500 sm:text-sm"
                            }
                            if let Some(err) = &form_state.get().errors.password {
                                p { 
                                    class: "mt-2 text-sm text-red-600 dark:text-red-500", 
                                    "{err}" 
                                }
                            }
                        }
                    }
                    
                    // Remember me checkbox
                    div { 
                        class: "flex items-start",
                        label { 
                            class: "flex items-center text-sm font-medium text-gray-900 accent-primary-600 rtl:text-right dark:text-gray-300",
                            input {
                                r#type: "checkbox",
                                value: "on",
                                checked: form_state.get().data.remember,
                                onchange: move |e| handle_remember_change(e.value.parse().unwrap_or(false)),
                                class: "me-2 h-4 w-4 rounded border-gray-300 bg-gray-100 text-primary-600 focus:ring-2 focus:ring-primary-500 dark:border-gray-500 dark:bg-gray-600 dark:ring-offset-gray-800 dark:focus:ring-primary-600"
                            }
                            "Remember me"
                        }
                    }
                    
                    // Lost password link
                    div {
                        Link {
                            to: "/reset-password",
                            class: "ml-auto text-sm text-primary-600 dark:text-primary-500",
                            "Lost Password?"
                        }
                    }
                    
                    // Submit button
                    button {
                        r#type: "submit",
                        disabled: form_state.get().loading,
                        class: "inline-flex items-center justify-center rounded-lg bg-primary-700 px-5 py-3 text-center text-base font-medium text-white focus-within:outline-none focus-within:ring-4 focus-within:ring-primary-300 hover:bg-primary-800 dark:bg-primary-600 dark:focus-within:ring-primary-800 dark:hover:bg-primary-700",
                        
                        if form_state.get().loading {
                            "Logging in..."
                        } else {
                            "Login to your account"
                        }
                    }
                    
                    // Registration link
                    div { 
                        class: "text-sm font-medium text-gray-500 dark:text-gray-400",
                        "Not registered? "
                        Link {
                            to: "/register",
                            class: "inline-flex items-center text-primary-600 hover:underline dark:text-primary-500",
                            "Create account"
                        }
                    }
                }
            }
        }
        
        // Not verified alert
        if form_state.get().not_verified {
            div { 
                class: "alert alert-error w-full max-w-lg shadow-lg mx-auto mt-4",
                div { 
                    class: "flex items-center",
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
                    span { 
                        class: "ml-2", 
                        "{messages["notVerified"]}" 
                    }
                }
            }
        }
        
        // Notification container
        div { 
            class: "fixed bottom-4 right-4 z-50 space-y-2",
            notifications.get().iter().map(|notification| {
                let bg_color = match notification.mode {
                    NotificationType::Success => "bg-green-500",
                    NotificationType::Danger => "bg-red-500",
                    NotificationType::Info => "bg-blue-500",
                    NotificationType::Warn => "bg-yellow-500",
                };
                
                rsx! {
                    div {
                        key: notification.message.clone(),
                        class: format!("rounded-md p-4 text-white {} shadow-lg", bg_color),
                        onmounted: move |_| {
                            // Auto-dismiss after lifetime
                            if let Some(lifetime) = notification.lifetime {
                                let notifs = notifications.clone();
                                let message = notification.message.clone();
                                spawn(async move {
                                    tokio::time::sleep(std::time::Duration::from_millis(lifetime.into())).await;
                                    notifs.with_mut(|n| n.retain(|n| n.message != message));
                                });
                            }
                        },
                        p { "{notification.message}" }
                    }
                }
            })
        }
    }
}
}

// Simulated API response types
enum LoginResult {
    Success { redirect: Option<String> },
    InvalidCredentials,
    NotVerified,
    Error(String),
}

// Simulated login API
async fn simulate_login_api(form: &LoginForm) -> LoginResult {
    // In a real app, this would be an actual API call
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    
    // Simple validation for demo purposes
    if form.email.is_empty() || form.password.is_empty() {
        return LoginResult::InvalidCredentials;
    }
    
    if form.email.contains("unverified") {
        return LoginResult::NotVerified;
    }
    
    if form.email.contains("error") {
        return LoginResult::Error("Simulated error".to_string());
    }
    
    LoginResult::Success { redirect: Some("/dashboard".to_string()) }
}