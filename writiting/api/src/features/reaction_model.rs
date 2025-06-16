#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Reaction {
    pub id: i32,
    pub user_id: Uuid,
    pub content_type: String, // "post" or "comment"
    pub content_id: i32,
    pub reaction_type: String, // "like", "upvote", etc.
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, strum::EnumString, strum::Display)]
pub enum ContentType {
    #[strum(serialize = "post")]
    Post,
    #[strum(serialize = "comment")]
    Comment,
}

#[derive(Debug, strum::EnumString, strum::Display)]
pub enum ReactionType {
    #[strum(serialize = "like")]
    Like,
    #[strum(serialize = "upvote")]
    Upvote,
    #[strum(serialize = "downvote")]
    Downvote,
}