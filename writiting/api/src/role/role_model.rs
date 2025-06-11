use serde::{Serialize, Deserialize};
use validator::Validate;
use uuid::Uuid;
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, Validate)]
pub struct Role {
    pub id: i32,
    
    #[validate(length(min = 3, max = 50, message = "Name must be 3-50 characters"))]
    pub name: String,
    
    #[validate(length(max = 255, message = "Description too long"))]
    pub description: Option<String>,
    
    pub is_default: bool,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
    
    #[serde(skip_serializing)]
    pub permissions: Vec<Permission>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, PartialEq, Eq, Hash)]
pub struct Permission {
    pub id: i32,
    pub code: String,
    
    #[validate(length(max = 255, message = "Description too long"))]
    pub description: Option<String>,
    
    pub created_at: chrono::DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateRoleRequest {
    #[validate(length(min = 3, max = 50))]
    pub name: String,
    
    #[validate(length(max = 255))]
    pub description: Option<String>,
    
    pub permission_ids: Vec<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateRoleRequest {
    #[validate(length(min = 3, max = 50))]
    pub name: Option<String>,
    
    #[validate(length(max = 255))]
    pub description: Option<String>,
    
    pub permission_ids: Option<Vec<i32>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleWithPermissions {
    #[serde(flatten)]
    pub role: Role,
    pub permissions: Vec<Permission>,
}