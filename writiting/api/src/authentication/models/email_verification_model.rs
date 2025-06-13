#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct EmailVerificationToken {
    pub token: Uuid,
    pub user_id: Uuid,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub expires_at: DateTime<Utc>,
}

impl EmailVerificationToken {
    pub fn new(user_id: Uuid, expires_at: DateTime<Utc>) -> Self {
        Self {
            token: Uuid::new_v4(),
            user_id,
            expires_at,
        }
    }

    pub fn is_valid(&self) -> bool {
        Utc::now() < self.expires_at
    }
}