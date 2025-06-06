use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, sqlx::FromRow, Validate)]
pub struct Post {
    pub id: i32,
    
    #[validate(length(min = 1, max = 100, message = "Title must be 1-100 characters"))]
    pub title: String,

    #[validate(length(min = 1, max = 5000, message = "Content must be 1-5000 characters"))]
    pub body: String,

    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    

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