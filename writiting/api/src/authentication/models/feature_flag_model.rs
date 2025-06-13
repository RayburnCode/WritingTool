use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Validate)]
pub struct FeatureFlag {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub is_enabled: bool,
    #[validate(range(min = 0, max = 100))]
    pub rollout_percentage: i32,
    pub target_users: Vec<Uuid>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

impl FeatureFlag {
    /// Creates a new feature flag builder
    pub fn builder(name: impl Into<String>) -> FeatureFlagBuilder {
        FeatureFlagBuilder::new(name.into())
    }

    /// Evaluates if the feature is active for a specific user
    pub fn is_active_for_user(&self, user_id: Option<Uuid>) -> bool {
        if !self.is_enabled {
            return false;
        }

        // Check explicit allowlist
        if let Some(user_id) = user_id {
            if self.target_users.contains(&user_id) {
                return true;
            }
        }

        // Check percentage rollout
        if self.rollout_percentage >= 100 {
            true
        } else if self.rollout_percentage <= 0 {
            false
        } else {
            // Stable rollout based on user ID or random fallback
            user_id.map_or_else(
                || rand::random::<i32>() % 100 < self.rollout_percentage,
                |uid| (uid.as_u128() % 100) < self.rollout_percentage as u128,
            )
        }
    }

    /// Checks if the flag has any targeting rules
    pub fn has_targeting(&self) -> bool {
        self.rollout_percentage != 100 || !self.target_users.is_empty()
    }
}

/// Builder pattern for feature flags
#[derive(Debug)]
pub struct FeatureFlagBuilder {
    name: String,
    is_enabled: bool,
    rollout_percentage: i32,
    target_users: Vec<Uuid>,
}

impl FeatureFlagBuilder {
    pub fn new(name: String) -> Self {
        Self {
            name,
            is_enabled: false,
            rollout_percentage: 0,
            target_users: Vec::new(),
        }
    }

    pub fn enabled(mut self) -> Self {
        self.is_enabled = true;
        self
    }

    pub fn disabled(mut self) -> Self {
        self.is_enabled = false;
        self
    }

    pub fn rollout_percentage(mut self, percentage: i32) -> Self {
        self.rollout_percentage = percentage.clamp(0, 100);
        self
    }

    pub fn target_user(mut self, user_id: Uuid) -> Self {
        self.target_users.push(user_id);
        self
    }

    pub fn target_users(mut self, users: impl IntoIterator<Item = Uuid>) -> Self {
        self.target_users.extend(users);
        self
    }

    pub fn build(self) -> FeatureFlag {
        let now = Utc::now();
        FeatureFlag {
            name: self.name,
            is_enabled: self.is_enabled,
            rollout_percentage: self.rollout_percentage,
            target_users: self.target_users,
            created_at: now,
            updated_at: now,
        }
    }
}

/// DTO for API responses
#[derive(Debug, Serialize)]
pub struct FeatureFlagResponse {
    pub name: String,
    pub is_enabled: bool,
    pub rollout_percentage: i32,
    pub has_targeting: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<FeatureFlag> for FeatureFlagResponse {
    fn from(flag: FeatureFlag) -> Self {
        Self {
            name: flag.name,
            is_enabled: flag.is_enabled,
            rollout_percentage: flag.rollout_percentage,
            has_targeting: flag.has_targeting(),
            created_at: flag.created_at,
            updated_at: flag.updated_at,
        }
    }
}

/// Rollout strategy enums
#[derive(Debug, Serialize, Deserialize)]
pub enum RolloutStrategy {
    Disabled,
    Enabled,
    Percentage(i32),
    Targeted(Vec<Uuid>),
    Gradual {
        initial_percentage: i32,
        increment_per_day: i32,
    },
}