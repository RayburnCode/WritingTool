// wrtiting/api/src/db/connection_pool.rs

// use sqlx::PgPool;
// use tokio::sync::OnceCell;
// use dotenv::dotenv;
// use std::env;


// static DB: OnceCell<PgPool> = OnceCell::const_new();

// pub async fn init_db() -> Result<PgPool, sqlx::Error> {
//     dotenv().ok();
//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//     let connection_pool = PgPool::connect(&database_url).await?;
//     sqlx::migrate!("./../migrations").run(&connection_pool).await?;
//     Ok(connection_pool)
// }

// pub async fn get_db() -> &'static PgPool {
//     DB.get_or_init(|| async { init_db().await.expect("Failed to initialize database") }).await
// }

use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Error};
use tokio::sync::OnceCell;
use dotenv::dotenv;
use std::env;
use std::time::Duration;
use tracing::info;

static DB_POOL: OnceCell<PgPool> = OnceCell::const_new();

#[derive(Debug)]
pub struct DbConfig {
    pub max_connections: u32,
    pub min_connections: u32,
    pub acquire_timeout_secs: u64,
    pub idle_timeout_secs: u64,
    pub max_lifetime_secs: u64,
    pub test_before_acquire: bool,
}

impl Default for DbConfig {
    fn default() -> Self {
        Self {
            max_connections: 10,
            min_connections: 2,
            acquire_timeout_secs: 30,
            idle_timeout_secs: 600, // 10 minutes
            max_lifetime_secs: 1800, // 30 minutes
            test_before_acquire: true,
        }
    }
}

pub async fn init_db() -> Result<PgPool, Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file or environment");

    let config = DbConfig::default();

    let pool = PgPoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .acquire_timeout(Duration::from_secs(config.acquire_timeout_secs))
        .idle_timeout(Duration::from_secs(config.idle_timeout_secs))
        .max_lifetime(Duration::from_secs(config.max_lifetime_secs))
        .test_before_acquire(config.test_before_acquire)
        .after_connect(|conn, _meta| Box::pin(async move {
            // Set timezone or other connection defaults
            sqlx::query("SET TIME ZONE 'UTC'").execute(conn).await?;
            Ok(())
        }))
        .connect(&database_url)
        .await?;

    // Verify connection
    sqlx::query("SELECT 1").execute(&pool).await?;
    info!("Successfully connected to database");

    // Run migrations
    sqlx::migrate!("./../migrations")
        .run(&pool)
        .await?;
    info!("Database migrations completed");

    Ok(pool)
}

pub async fn get_db() -> Result<&'static PgPool, Error> {
    DB_POOL.get_or_try_init(|| async {
        init_db().await
    }).await
}

// Health check endpoint
pub async fn check_db_health() -> Result<(), Error> {
    let pool = get_db().await?;
    sqlx::query("SELECT 1")
        .execute(pool)
        .await?;
    Ok(())
}

// For testing purposes
#[cfg(test)]
pub async fn get_test_pool() -> Result<PgPool, Error> {
    dotenv().ok();
    let database_url = env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| env::var("DATABASE_URL").expect("DATABASE_URL must be set"));
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    
    sqlx::migrate!("./../migrations").run(&pool).await?;
    Ok(pool)
} 