#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Bookmark {
    pub user_id: Uuid,
    pub post_id: i32,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BookmarkCreate {
    pub post_id: i32,
}