use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use serde_json::Value;
use std::net::IpAddr;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AuditLog {
    pub id: i64,
    pub user_id: Option<Uuid>,
    pub action: String,
    pub entity_type: Option<String>,
    pub entity_id: Option<String>, // Flexible ID (UUID or other)
    pub ip_address: Option<IpAddr>,
    pub user_agent: Option<String>,
    pub metadata: Option<Value>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl AuditLog {
    /// Creates a new audit log entry builder
    pub fn builder(action: impl Into<String>) -> AuditLogBuilder {
        AuditLogBuilder::new(action.into())
    }

    /// Checks if this log entry relates to a specific entity
    pub fn relates_to(&self, entity_type: &str, entity_id: &str) -> bool {
        self.entity_type.as_deref() == Some(entity_type) && 
        self.entity_id.as_deref() == Some(entity_id)
    }

    /// Checks if the action matches (with optional wildcard)
    pub fn is_action(&self, action: &str) -> bool {
        // Support wildcard matching like "user.*"
        if action.ends_with(".*") {
            let prefix = &action[..action.len() - 2];
            self.action.starts_with(prefix)
        } else {
            self.action == action
        }
    }
}

/// Builder pattern for audit logs
#[derive(Debug)]
pub struct AuditLogBuilder { 
    user_id: Option<Uuid>,
    action: String,
    entity_type: Option<String>,
    entity_id: Option<String>,
    ip_address: Option<IpAddr>,
    user_agent: Option<String>,
    metadata: Option<Value>,
}

impl AuditLogBuilder {
    pub fn new(action: String) -> Self {
        Self {
            user_id: None,
            action,
            entity_type: None,
            entity_id: None,
            ip_address: None,
            user_agent: None,
            metadata: None,
        }
    }

    pub fn user_id(mut self, user_id: Uuid) -> Self {
        self.user_id = Some(user_id);
        self
    }

    pub fn entity<T: ToString>(mut self, entity_type: &str, entity_id: T) -> Self {
        self.entity_type = Some(entity_type.to_string());
        self.entity_id = Some(entity_id.to_string());
        self
    }

    pub fn ip(mut self, ip: IpAddr) -> Self {
        self.ip_address = Some(ip);
        self
    }

    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }

    pub fn metadata(mut self, metadata: Value) -> Self {
        self.metadata = Some(metadata);
        self
    }

    /// Consumes the builder and returns an unpersisted AuditLog
    pub fn build(self) -> AuditLog {
        AuditLog {
            id: 0, // Will be set by database
            user_id: self.user_id,
            action: self.action,
            entity_type: self.entity_type,
            entity_id: self.entity_id,
            ip_address: self.ip_address,
            user_agent: self.user_agent,
            metadata: self.metadata,
            created_at: Utc::now(),
        }
    }
}

/// Example action types
pub mod audit_actions {
    pub const USER_LOGIN: &str = "user.login";
    pub const USER_LOGOUT: &str = "user.logout";
    pub const USER_PASSWORD_CHANGE: &str = "user.password_change";
    
    pub fn entity_action(entity: &str, action: &str) -> String {
        format!("{}.{}", entity, action)
    }
}

/// Safe DTO for API responses
#[derive(Debug, Serialize)]
pub struct AuditLogResponse {
    pub id: i64,
    pub user_id: Option<Uuid>,
    pub action: String,
    pub entity_type: Option<String>,
    pub entity_id: Option<String>,
    pub created_at: DateTime<Utc>,
}