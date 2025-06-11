#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, sqlx::FromRow)]
pub struct SessionToken {
    pub id: i32,
    pub user_id: i32,
    pub token: String, // Secure random string

    #[serde(with = "chrono::serde::ts_seconds")]
    pub expires_at: DateTime<Utc>,
}
