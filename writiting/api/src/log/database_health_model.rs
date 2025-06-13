use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DatabaseConnectionStats {
    pub id: i64,
    pub application_name: String,
    pub connection_count: i32,
    pub max_connections: i32,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl DatabaseConnectionStats {
    /// Creates a new stats snapshot from current database state
    pub async fn capture(pool: &sqlx::PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as!(
            Self,
            r#"
            INSERT INTO database_connections
            SELECT 
                nextval('database_connections_id_seq'),
                application_name,
                count(*),
                (SELECT setting::int FROM pg_settings WHERE name='max_connections'),
                NOW()
            FROM pg_stat_activity 
            GROUP BY application_name
            RETURNING *
            "#
        )
        .fetch_all(pool)
        .await
    }

    /// Calculates connection utilization percentage
    pub fn utilization_pct(&self) -> f64 {
        if self.max_connections > 0 {
            (self.connection_count as f64 / self.max_connections as f64) * 100.0
        } else {
            0.0
        }
    }

    /// Checks if connections are over warning threshold (default: 75%)
    pub fn is_over_warning(&self, threshold: Option<f64>) -> bool {
        self.utilization_pct() > threshold.unwrap_or(75.0)
    }

    /// Checks if connections are over critical threshold (default: 90%)
    pub fn is_over_critical(&self, threshold: Option<f64>) -> bool {
        self.utilization_pct() > threshold.unwrap_or(90.0)
    }
}

impl fmt::Display for DatabaseConnectionStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {}/{} connections ({:.1}%)",
            self.application_name,
            self.connection_count,
            self.max_connections,
            self.utilization_pct()
        )
    }
}

/// Health status derived from connection stats
#[derive(Debug, Serialize, PartialEq)]
pub enum DatabaseHealthStatus {
    Normal,
    Warning(String),
    Critical(String),
}

/// Analysis of database health
pub struct DatabaseHealthReport {
    pub stats: Vec<DatabaseConnectionStats>,
    pub status: DatabaseHealthStatus,
    pub timestamp: DateTime<Utc>,
}

impl DatabaseHealthReport {
    pub async fn generate(pool: &sqlx::PgPool) -> Result<Self, sqlx::Error> {
        let stats = DatabaseConnectionStats::capture(pool).await?;
        let status = Self::analyze_health(&stats);
        Ok(Self {
            stats,
            status,
            timestamp: Utc::now(),
        })
    }

    fn analyze_health(stats: &[DatabaseConnectionStats]) -> DatabaseHealthStatus {
        let mut messages = Vec::new();

        for stat in stats {
            if stat.is_over_critical(None) {
                messages.push(format!(
                    "CRITICAL: {} at {:.1}% utilization",
                    stat.application_name,
                    stat.utilization_pct()
                ));
            } else if stat.is_over_warning(None) {
                messages.push(format!(
                    "WARNING: {} at {:.1}% utilization",
                    stat.application_name,
                    stat.utilization_pct()
                ));
            }
        }

        match messages.len() {
            0 => DatabaseHealthStatus::Normal,
            _ if messages.iter().any(|m| m.contains("CRITICAL")) => {
                DatabaseHealthStatus::Critical(messages.join("\n"))
            }
            _ => DatabaseHealthStatus::Warning(messages.join("\n")),
        }
    }
}

/// Configuration for connection monitoring
#[derive(Debug, Clone)]
pub struct DatabaseHealthConfig {
    pub warning_threshold: f64,
    pub critical_threshold: f64,
    pub check_interval_seconds: u64,
}

impl Default for DatabaseHealthConfig {
    fn default() -> Self {
        Self {
            warning_threshold: 75.0,
            critical_threshold: 90.0,
            check_interval_seconds: 300, // 5 minutes
        }
    }
}

/// Background monitor service
pub struct DatabaseHealthMonitor {
    pool: sqlx::PgPool,
    config: DatabaseHealthConfig,
}

impl DatabaseHealthMonitor {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self {
            pool,
            config: DatabaseHealthConfig::default(),
        }
    }

    pub fn with_config(mut self, config: DatabaseHealthConfig) -> Self {
        self.config = config;
        self
    }

    pub async fn run(&self) -> ! {
        let mut interval = tokio::time::interval(
            std::time::Duration::from_secs(self.config.check_interval_seconds)
        );

        loop {
            interval.tick().await;
            
            match DatabaseHealthReport::generate(&self.pool).await {
                Ok(report) => self.handle_report(report).await,
                Err(e) => log::error!("Failed to generate health report: {}", e),
            }
        }
    }

    async fn handle_report(&self, report: DatabaseHealthReport) {
        match report.status {
            DatabaseHealthStatus::Critical(msg) => {
                log::error!("Database health critical:\n{}", msg);
                // TODO: Trigger alerts
            }
            DatabaseHealthStatus::Warning(msg) => {
                log::warn!("Database health warning:\n{}", msg);
            }
            DatabaseHealthStatus::Normal => {
                log::debug!("Database health normal");
            }
        }

        // Log all stats at appropriate level
        for stat in report.stats {
            if stat.is_over_critical(Some(self.config.critical_threshold)) {
                log::error!("{}", stat);
            } else if stat.is_over_warning(Some(self.config.warning_threshold)) {
                log::warn!("{}", stat);
            } else {
                log::debug!("{}", stat);
            }
        }
    }
}