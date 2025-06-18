use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;
use sqlx::FromRow;
use crate::users::users_model::User;
use crate::utils::validation::validate_http_url;



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

    #[validate(custom(function = "validate_http_url"))]
    pub avatar_url: Option<String>,
    
    #[validate(custom(function = "validate_http_url"))]
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


    pub fn update(&mut self, first_name: String, last_name: String, bio: String, avatar_url: String, website_url: String) -> Result<(), validator::ValidationErrors> {
        self.first_name = Some(first_name);
        self.last_name = Some(last_name);
        self.bio = Some(bio);
        self.avatar_url = Some(avatar_url);
        self.website_url = Some(website_url);
        self.updated_at = Utc::now();
        self.validate()
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
    #[validate(custom(function = "validate_http_url"))]
    pub avatar_url: Option<String>,

    #[validate(url)]
    #[validate(custom(function = "validate_http_url"))]
    pub website_url: Option<String>,
}