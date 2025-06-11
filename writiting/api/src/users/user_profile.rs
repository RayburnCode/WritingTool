pub struct UserProfile {
    pub id: i32,
    pub user_id: i32,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
}
