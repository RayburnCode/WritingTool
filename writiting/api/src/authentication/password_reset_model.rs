use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PasswordResetToken {
    pub token: Uuid,
    pub user_id: Uuid,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub expires_at: DateTime<Utc>,
    pub used: bool,
}

impl PasswordResetToken {
    pub fn new(user_id: Uuid, expires_at: DateTime<Utc>) -> Self {
        Self {
            token: Uuid::new_v4(),
            user_id,
            expires_at,
            used: false,
        }
    }

    pub fn is_valid(&self) -> bool {
        !self.used && Utc::now() < self.expires_at
    }

    pub fn mark_as_used(&mut self) {
        self.used = true;
    }
}