use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, sqlx::FromRow, Validate)]
pub struct Session {
    pub id: Uuid,
    
    pub user_id: Uuid,
    
    #[validate(length(min = 32, message = "Token must be at least 32 characters"))]
    pub token: String,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    
    #[serde(with = "chrono::serde::ts_seconds")]
    pub expires_at: DateTime<Utc>,
    
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

impl Session {
    pub fn new(
        user_id: Uuid,
        token: String,
        user_agent: Option<String>,
        ip_address: Option<String>,
        expires_at: DateTime<Utc>,
    ) -> Result<Self, validator::ValidationErrors> {
        let now = Utc::now();
        let session = Self {
            id: Uuid::new_v4(),
            user_id,
            token,
            user_agent,
            ip_address,
            expires_at,
            created_at: now,
            updated_at: now,
        };

        session.validate()?;
        Ok(session)
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() >= self.expires_at
    }

    pub fn update_token(&mut self, new_token: String, new_expiry: DateTime<Utc>) {
        self.token = new_token;
        self.expires_at = new_expiry;
        self.updated_at = Utc::now();
    }
}

#[derive(Serialize, Deserialize)]
pub struct SessionResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

impl From<Session> for SessionResponse {
    fn from(session: Session) -> Self {
        Self {
            id: session.id,
            user_id: session.user_id,
            user_agent: session.user_agent,
            created_at: session.created_at,
            expires_at: session.expires_at,
        }
    }
}