use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PostComment {
    pub id: i32,
    pub post_id: i32,
    pub user_id: Uuid,
    pub parent_id: Option<i32>,
    pub content: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

impl PostComment {
    pub fn new(
        post_id: i32,
        user_id: Uuid,
        parent_id: Option<i32>,
        content: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: 0, // Will be set by database
            post_id,
            user_id,
            parent_id,
            content,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommentWithAuthor {
    #[serde(flatten)]
    pub comment: PostComment,
    pub author: Option<User>, // From your previous User model
}