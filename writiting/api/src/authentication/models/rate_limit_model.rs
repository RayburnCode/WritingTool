use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use std::net::IpAddr;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Validate)]
pub struct RateLimit {
    #[validate(length(min = 1, max = 255))]
    pub bucket: String,
    #[validate(range(min = 0))]
    pub tokens: f64,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub last_refill: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub expires_at: DateTime<Utc>,
}

impl RateLimit {
    /// Creates a new rate limit bucket
    pub fn new(
        scope: &str,
        identifier: &str,
        capacity: f64,
        refill_rate: f64, // tokens per second
        expires_in: chrono::Duration,
    ) -> Self {
        let now = Utc::now();
        Self {
            bucket: format!("{}:{}", scope, identifier),
            tokens: capacity,
            last_refill: now,
            expires_at: now + expires_in,
        }
    }

    /// Creates a bucket for IP-based rate limiting
    pub fn for_ip(
        scope: &str,
        ip: IpAddr,
        capacity: f64,
        refill_rate: f64,
        expires_in: chrono::Duration,
    ) -> Self {
        Self::new(scope, &ip.to_string(), capacity, refill_rate, expires_in)
    }

    /// Checks if the request should be rate limited
    pub fn check_request(&mut self, cost: f64) -> RateLimitResult {
        self.refill_tokens();

        if self.tokens >= cost {
            self.tokens -= cost;
            RateLimitResult::Allowed {
                remaining: self.tokens,
                reset_seconds: self.reset_in().num_seconds(),
            }
        } else {
            RateLimitResult::Denied {
                retry_after_seconds: self.reset_in().num_seconds(),
            }
        }
    }

    /// Refills tokens based on time elapsed
    fn refill_tokens(&mut self) {
        let now = Utc::now();
        let elapsed = (now - self.last_refill).to_std().unwrap().as_secs_f64();
        self.tokens = (self.tokens + elapsed * self.refill_rate()).min(self.capacity());
        self.last_refill = now;
    }

    /// Calculates time until next refill to full capacity
    pub fn reset_in(&self) -> chrono::Duration {
        let missing = self.capacity() - self.tokens;
        chrono::Duration::seconds((missing / self.refill_rate()).ceil() as i64)
    }

    /// Extracts the refill rate from bucket name (tokens/second)
    fn refill_rate(&self) -> f64 {
        // Parse from bucket name format "scope:identifier:rate"
        self.bucket.split(':')
            .nth(2)
            .and_then(|s| s.parse().ok())
            .unwrap_or(1.0) // default 1 token/sec
    }

    /// Extracts the capacity from bucket name
    fn capacity(&self) -> f64 {
        // Parse from bucket name format "scope:identifier:rate:capacity"
        self.bucket.split(':')
            .nth(3)
            .and_then(|s| s.parse().ok())
            .unwrap_or(10.0) // default 10 tokens
    }
}

/// Result of a rate limit check
#[derive(Debug, Serialize)]
pub enum RateLimitResult {
    Allowed {
        remaining: f64,
        reset_seconds: i64,
    },
    Denied {
        retry_after_seconds: i64,
    },
}

/// Rate limit headers for HTTP responses
#[derive(Debug)]
pub struct RateLimitHeaders {
    pub remaining: i64,
    pub reset: i64,
    pub limit: i64,
}

impl From<&RateLimit> for RateLimitHeaders {
    fn from(rl: &RateLimit) -> Self {
        Self {
            remaining: rl.tokens.floor() as i64,
            reset: rl.reset_in().num_seconds(),
            limit: rl.capacity().floor() as i64,
        }
    }
}

/// Predefined rate limit configurations
pub mod rate_limits {
    pub const LOGIN: &str = "login:1:10"; // 1 token/sec, burst 10
    pub const API: &str = "api:10:100";   // 10 tokens/sec, burst 100
    pub const EMAIL: &str = "email:0.2:5"; // 1 email every 5 sec, burst 5
}