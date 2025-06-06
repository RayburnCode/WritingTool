// pg_app/shared/src/models/post_models.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};
// use super::role_models::Permission;
// use super::user_models::User;



// ===== Post Model =====

/// Represents a blog post or content entry in the system
///
/// # Fields
/// - `id`: Unique database identifier (auto-incremented)
/// - `title`: Human-readable title of the post
/// - `body`: Main content in markdown or HTML format  
/// - `created_at`: Timestamp of initial creation
/// - `updated_at`: Timestamp of last modification
/// - `is_published`: Visibility status
/// - `author_id`: User who created the post
///
/// # Validation Rules
/// - Title: 1-100 characters
/// - Body: 1-5000 characters
///
/// # Example
/// ```rust
/// let post = Post {
///     id: 1,
///     title: "My First Post".into(),
///     body: "This is the content...".into(),
///     created_at: Some(Utc::now()),
///     updated_at: None,

/// };
/// ```
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, sqlx::FromRow, Validate)]
pub struct Post {
    /// Unique database identifier (auto-incremented)
    pub id: i32,
    
    /// Human-readable title of the post
    ///
    /// # Validation
    /// - Minimum length: 1 character
    /// - Maximum length: 100 characters
    #[validate(length(min = 1, max = 100, message = "Title must be 1-100 characters"))]
    pub title: String,
    
    /// Main content in markdown or HTML format
    ///
    /// # Validation  
    /// - Minimum length: 1 character
    /// - Maximum length: 5000 characters
    #[validate(length(min = 1, max = 5000, message = "Content must be 1-5000 characters"))]
    pub body: String,
    
    /// Timestamp of when the post was initially created
    ///
    /// # Serialization
    /// Stored as UNIX timestamp in seconds
    /// #[serde_as]
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    
    /// Timestamp of when the post was last modified  
    ///
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
    
}



impl Post {
    pub(super) fn new(
        title: String,
        body: String,
    ) -> Result<Self, ValidationErrors> {
        let post = Self {
            id: 0, // Temporary ID before DB insertion
            title,
            body,
            created_at: Utc::now(),
            updated_at: Utc::now(),

        };
        post.validate()?;
        Ok(post)
    }
    

    
    /// Updates the post content
    pub(super) fn update_content(&mut self, title: String, body: String) -> Result<(), ValidationErrors> {
        self.title = title;
        self.body = body;
        self.updated_at = Utc::now();
        self.validate()
    }
    

}