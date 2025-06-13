// For pagination
#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedResult<T> {
    pub items: Vec<T>,
    pub total: i64,
    pub page: u32,
    pub per_page: u32,
}

// For reaction counts
#[derive(Debug, Serialize, Deserialize)]
pub struct ReactionCount {
    pub reaction_type: String,
    pub count: i64,
}