use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use std::time::Duration;

/// 初始化 SQLite 数据库
pub async fn init_db_sqlite(database_url: String) -> Result<SqlitePool, sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&database_url)
        .await?;

    let migrator = sqlx::migrate!("./migrations");
    migrator.run(&pool).await?;

    Ok(pool)
}
