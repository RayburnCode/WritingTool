

use sqlx::PgPool;
use tokio::sync::OnceCell;
use dotenv::dotenv;
use std::env;


static DB: OnceCell<PgPool> = OnceCell::const_new();

pub async fn init_db() -> Result<PgPool, sqlx::Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection_pool = PgPool::connect(&database_url).await?;
    sqlx::migrate!("./../migrations").run(&connection_pool).await?;
    Ok(connection_pool)
}

pub async fn get_db() -> &'static PgPool {
    DB.get_or_init(|| async { init_db().await.expect("Failed to initialize database") }).await
}