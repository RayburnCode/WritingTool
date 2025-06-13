#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub token: Uuid,
    pub expires_at: DateTime<Utc>,
}

impl From<PasswordResetToken> for TokenResponse {
    fn from(token: PasswordResetToken) -> Self {
        Self {
            token: token.token,
            expires_at: token.expires_at,
        }
    }
}

impl From<EmailVerificationToken> for TokenResponse {
    fn from(token: EmailVerificationToken) -> Self {
        Self {
            token: token.token,
            expires_at: token.expires_at,
        }
    }
}