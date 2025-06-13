#[derive(Debug, Deserialize)]
pub struct PasswordResetRequest {
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct EmailVerificationRequest {
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct TokenValidationRequest {
    pub token: Uuid,
}