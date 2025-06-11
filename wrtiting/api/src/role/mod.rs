#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Role {
    pub id: i32,
    pub name: String, // e.g. "admin", "user", "moderator"
}
