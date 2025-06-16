#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Follow {
    pub follower_id: Uuid,
    pub following_id: Uuid,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FollowStats {
    pub followers_count: i64,
    pub following_count: i64,
}