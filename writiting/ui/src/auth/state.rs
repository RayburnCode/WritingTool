use serde::{Deserialize, Serialize};

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

// Simulated API response types
pub enum LoginResult {
    Success { redirect: Option<String> },
    InvalidCredentials,
    NotVerified,
    Error(String),
}

// Simulated login API
pub async fn simulate_login_api(form: &LoginForm) -> LoginResult {
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