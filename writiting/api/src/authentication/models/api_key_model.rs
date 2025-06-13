use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Validate)]
pub struct ApiKey {
    pub key: Uuid,
    pub user_id: Uuid,
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub scopes: Vec<String>,
    #[serde(with = "chrono::serde::ts_seconds_option", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<Utc>>,
    #[serde(with = "chrono::serde::ts_seconds_option", skip_serializing_if = "Option::is_none")]
    pub last_used_at: Option<DateTime<Utc>>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl ApiKey {
    /// Creates a new API key with default values
    pub fn new(
        user_id: Uuid,
        name: String,
        scopes: Vec<String>,
        expires_in_days: Option<i64>,
    ) -> Result<Self, validator::ValidationErrors> {
        let mut key = Self {
            key: Uuid::new_v4(),
            user_id,
            name,
            scopes,
            expires_at: expires_in_days.map(|days| Utc::now() + chrono::Duration::days(days)),
            last_used_at: None,
            created_at: Utc::now(),
        };

        key.validate()?;
        Ok(key)
    }

    /// Checks if the API key is currently valid
    pub fn is_valid(&self) -> bool {
        match self.expires_at {
            Some(expiry) => Utc::now() < expiry,
            None => true, // No expiration set
        }
    }

    /// Verifies if the key has all required scopes
    pub fn has_scopes(&self, required: &[&str]) -> bool {
        required.iter().all(|scope| self.scopes.contains(&scope.to_string()))
    }

    /// Updates the last used timestamp (requires mutable DB reference)
    pub async fn record_usage(&mut self, pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
        self.last_used_at = Some(Utc::now());
        
        sqlx::query!(
            r#"UPDATE api_keys SET last_used_at = $1 WHERE key = $2"#,
            self.last_used_at,
            self.key
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Repository pattern - Fetch by key
    pub async fn find_by_key(
        pool: &sqlx::PgPool,
        key: Uuid,
    ) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as!(
            Self,
            r#"SELECT * FROM api_keys WHERE key = $1"#,
            key
        )
        .fetch_optional(pool)
        .await
    }

    /// Repository pattern - List keys for user
    pub async fn list_for_user(
        pool: &sqlx::PgPool,
        user_id: Uuid,
        include_expired: bool,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let query = if include_expired {
            "SELECT * FROM api_keys WHERE user_id = $1"
        } else {
            "SELECT * FROM api_keys WHERE user_id = $1 AND (expires_at IS NULL OR expires_at > NOW())"
        };

        sqlx::query_as::<_, Self>(query)
            .bind(user_id)
            .fetch_all(pool)
            .await
    }

    /// Repository pattern - Revoke a key
    pub async fn revoke(
        pool: &sqlx::PgPool,
        key: Uuid,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"DELETE FROM api_keys WHERE key = $1"#,
            key
        )
        .execute(pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}

// DTO for safe serialization (excludes sensitive fields)
#[derive(Debug, Serialize)]
pub struct ApiKeyResponse {
    pub key: Uuid,
    pub name: String,
    pub scopes: Vec<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl From<ApiKey> for ApiKeyResponse {
    fn from(api_key: ApiKey) -> Self {
        Self {
            key: api_key.key,
            name: api_key.name,
            scopes: api_key.scopes,
            expires_at: api_key.expires_at,
            created_at: api_key.created_at,
        }
    }
}