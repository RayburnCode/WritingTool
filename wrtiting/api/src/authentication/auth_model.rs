#[derive(Debug, Serialize, Deserialize)]
pub struct AuthToken {
    pub token: String,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub user: User,
    pub token: AuthToken,
}

#[derive(Debug, Validate, Deserialize)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    
    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct RegisterRequest {
    #[validate(email)]
    pub email: String,
    
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    
    #[validate(length(min = 8))]
    pub password: String,
    
    #[validate(must_match(other = "password", message = "Passwords must match"))]
    pub password_confirmation: String,
}