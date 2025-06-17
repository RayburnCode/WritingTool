use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;
use sqlx::FromRow;
use crate::users::users_model::User;
use crate::utils::validation::{URL_REGEX_STR, validate_url, validate_http_url};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, FromRow, Validate)]
pub struct UserProfile {
    #[serde(skip_serializing)]
    pub user_id: Uuid,
    
    #[validate(length(max = 100, message = "First name must be 100 characters or less"))]
    pub first_name: Option<String>,
    
    #[validate(length(max = 100, message = "Last name must be 100 characters or less"))]
    pub last_name: Option<String>,
    
    #[validate(length(max = 2000, message = "Bio must be 2000 characters or less"))]
    pub bio: Option<String>,
     
    // Option 1: Using the regex string directly
    #[validate(url(message = "Avatar URL must be a valid URL"))]
    #[validate(regex(
        path = "validate_http_url",
        message = "Avatar URL must start with http:// or https://"
    ))]
    pub avatar_url: Option<String>,
    
    // Option 2: Using the validation function (alternative)
    #[validate(url(message = "Website URL must be a valid URL"))]
    #[validate(custom(
        function = "validate_http_url",
        message = "Website URL must start with http:// or https://"
    ))]
    pub website_url: Option<String>,
    
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

impl UserProfile {
    pub fn new(user_id: Uuid) -> Self {
        let now = Utc::now();
        Self {
            user_id,
            first_name: None,
            last_name: None,
            bio: None,
            avatar_url: None,
            website_url: None,
            created_at: now,
            updated_at: now,
            user: None,
        }
    }

    pub fn update(&mut self, update: ProfileUpdate) -> Result<(), validator::ValidationErrors> {
        if let Some(first_name) = update.first_name {
            self.first_name = Some(first_name);
        }
        if let Some(last_name) = update.last_name {
            self.last_name = Some(last_name);
        }
        if let Some(bio) = update.bio {
            self.bio = Some(bio);
        }
        if let Some(avatar_url) = update.avatar_url {
            self.avatar_url = Some(avatar_url);
        }
        if let Some(website_url) = update.website_url {
            self.website_url = Some(website_url);
        }
        
        self.updated_at = Utc::now();
        self.validate()?;
        Ok(())
    }
}

#[derive(Debug, Deserialize,Serialize, Validate, Clone)]
pub struct ProfileUpdate {
    #[validate(length(max = 100))]
    pub first_name: Option<String>,
    
    #[validate(length(max = 100))]
    pub last_name: Option<String>,
    
    #[validate(length(max = 2000))]
    pub bio: Option<String>,
    
    #[validate(url)]    
    #[validate(regex(
        path = "crate::utils::validation::URL_REGEX",
        message = "Avatar URL must start with http:// or https://"
    ))]
    pub avatar_url: Option<String>,

    #[validate(url)]
    #[validate(regex(path = "crate::utils::validation::URL_REGEX"))]
    pub website_url: Option<String>,
}
