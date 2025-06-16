use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Validate)]
pub struct UserPreferences {
    #[validate(required)]
    pub user_id: Option<Uuid>,
    
    #[validate(length(min = 2, max = 10))]
    pub language: String,
    
    #[validate(length(min = 3, max = 20))]
    pub theme: String,
    
    pub email_notifications: bool,
    
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

impl UserPreferences {
    /// Creates default preferences for a user
    pub fn new(user_id: Uuid) -> Self {
        Self {
            user_id: Some(user_id),
            language: "en".to_string(),
            theme: "light".to_string(),
            email_notifications: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    /// Builder pattern for preferences
    pub fn builder(user_id: Uuid) -> UserPreferencesBuilder {
        UserPreferencesBuilder::new(user_id)
    }

    /// Checks if dark mode is enabled
    pub fn is_dark_mode(&self) -> bool {
        self.theme.to_lowercase() == "dark"
    }

    /// Validates the language code format
    pub fn validate_language(&self) -> Result<(), validator::ValidationError> {
        // Basic ISO 639-1 validation
        if self.language.len() != 2 || !self.language.chars().all(|c| c.is_ascii_lowercase()) {
            return Err(validator::ValidationError::new("invalid_language_code"));
        }
        Ok(())
    }
}

/// Builder for user preferences
pub struct UserPreferencesBuilder {
    user_id: Uuid,
    language: Option<String>,
    theme: Option<String>,
    email_notifications: Option<bool>,
}

impl UserPreferencesBuilder {
    pub fn new(user_id: Uuid) -> Self {
        Self {
            user_id,
            language: None,
            theme: None,
            email_notifications: None,
        }
    }

    pub fn language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }

    pub fn theme(mut self, theme: impl Into<String>) -> Self {
        self.theme = Some(theme.into());
        self
    }

    pub fn email_notifications(mut self, enabled: bool) -> Self {
        self.email_notifications = Some(enabled);
        self
    }

    pub fn build(self) -> UserPreferences {
        UserPreferences {
            user_id: Some(self.user_id),
            language: self.language.unwrap_or_else(|| "en".to_string()),
            theme: self.theme.unwrap_or_else(|| "light".to_string()),
            email_notifications: self.email_notifications.unwrap_or(true),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

/// Supported languages
#[derive(Debug, Serialize, Deserialize)]
pub enum Language {
    #[serde(rename = "en")]
    English,
    #[serde(rename = "es")]
    Spanish,
}

/// Supported themes
#[derive(Debug, Serialize, Deserialize)]
pub enum Theme {
    #[serde(rename = "light")]
    Light,
    #[serde(rename = "dark")]
    Dark,
    #[serde(rename = "system")]
    System,
}

/// DTO for updating preferences
#[derive(Debug, Deserialize, Validate)]
pub struct UpdatePreferences {
    #[validate(length(min = 2, max = 10))]
    pub language: Option<String>,
    
    #[validate(length(min = 3, max = 20))]
    pub theme: Option<String>,
    
    pub email_notifications: Option<bool>,
}

/// API response DTO
#[derive(Debug, Serialize)]
pub struct PreferencesResponse {
    pub language: String,
    pub theme: String,
    pub email_notifications: bool,
    pub updated_at: DateTime<Utc>,
}

impl From<UserPreferences> for PreferencesResponse {
    fn from(prefs: UserPreferences) -> Self {
        Self {
            language: prefs.language,
            theme: prefs.theme,
            email_notifications: prefs.email_notifications,
            updated_at: prefs.updated_at,
        }
    }
}