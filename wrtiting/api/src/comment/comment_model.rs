#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, sqlx::FromRow, Validate)]
pub struct Comment {
    pub id: i32,
    pub post_id: i32,
    pub user_id: Uuid,
    
    #[validate(length(min = 1, max = 1000))]
    pub content: String,
    
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
    
    #[serde(skip_serializing)]
    pub user: Option<User>,
}