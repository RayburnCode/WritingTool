use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};
use argon2::{Argon2, password_hash::{PasswordHasher, SaltString}};
use rand::Rng;
use argon2::password_hash::rand_core::OsRng;
use sqlx::Type;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, sqlx::FromRow, Validate)]
pub struct User {
    pub id: Uuid,
    
    #[validate(email(message = "Must be a valid email"))]
    pub email: String,
    
    #[validate(length(min = 3, max = 50, message = "Username must be 3-50 characters"))]
    pub username: String,
    
    #[serde(skip_serializing)]
    pub password_hash: String,
    
    #[serde(skip_serializing)]
    pub salt: String,
    
    pub role_id: i32,
    
    #[serde(default = "default_active")]
    pub is_active: bool,
    
    #[serde(default = "default_email_verified")]
    pub email_verified: bool,
    
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
    
    #[serde(skip_serializing)]
    pub role: Option<Role>,
}

fn default_active() -> bool { true }
fn default_email_verified() -> bool { false }

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, sqlx::FromRow, Type)]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub permissions: Vec<String>,
}

impl User {
    pub fn new(
        email: String,
        username: String,
        password: String,
        role_id: i32,
    ) -> Result<Self, ValidationErrors> {
let salt = SaltString::generate(&mut OsRng);  
        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password(password.as_bytes(), &salt)
            .unwrap()
            .to_string();
            
        let now = Utc::now();
        let user = Self {
            id: Uuid::new_v4(),
            email,
            username,
            password_hash,
            salt: salt.to_string(),
            role_id,
            is_active: true,
            email_verified: false,
            created_at: now,
            updated_at: now,
            role: None,
        };
        
        user.validate()?;
        Ok(user)
    }
    
    pub fn verify_password(&self, password: &str) -> bool {
        use argon2::{Argon2, PasswordHash, PasswordVerifier};
        if let Ok(parsed_hash) = PasswordHash::new(&self.password_hash) {
            Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok()
        } else {
            false
        }
    }
    
    pub fn has_permission(&self, permission: &str) -> bool {
        self.role.as_ref()
            .map(|r| r.permissions.contains(&permission.to_string()))
            .unwrap_or(false)
    }

    pub fn set_password(&mut self, new_password: &str) {
let salt = SaltString::generate(&mut OsRng);  
        let argon2 = Argon2::default();
        self.password_hash = argon2.hash_password(new_password.as_bytes(), &salt)
            .unwrap()
            .to_string();
        self.salt = salt.to_string();
        self.updated_at = Utc::now();
    }
} 