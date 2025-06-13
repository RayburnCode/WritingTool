#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Profile {
    pub user_id: Uuid,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub website_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileUpdate {
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub website_url: Option<String>,
}