use deadpool_redis::{Config as RedisConfig, Pool as RedisPool};
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use std::time::Duration;

use tracing::info;

use crate::error::AppError;

#[derive(Clone)]
pub struct DatabasePools {
    pub sqlite: SqlitePool,
    pub redis: RedisPool,
}

pub async fn init_databases(
    database_url: &str,
    redis_url: &str,
) -> Result<DatabasePools, AppError> {
    // Initialize SQLite
    info!("Initializing Sqlite connection...");
    let sqlite_pool = SqlitePoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(30))
        .connect(database_url)
        .await?;

    // Run migrations if enabled
    #[cfg(feature = "migrate")]
    {
        info!("Running database migrations...");
        sqlx::migrate!().run(&sqlite_pool).await?;
    }

    // Initialize Redis
    info!("Initializing Redis connection...");
    let redis_cfg = RedisConfig::from_url(redis_url);
    let redis_pool = redis_cfg
        .create_pool(Some(deadpool_redis::Runtime::Tokio1))
        .map_err(|e| AppError::Configuration(format!("Redis pool error: {}", e)))?;

    Ok(DatabasePools {
        sqlite: sqlite_pool,
        redis: redis_pool,
    })
}
