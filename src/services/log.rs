use crate::{
    error::AppError,
    models::log::{Log, LogLevel},
};
use chrono::{DateTime, Utc};
use serde_json::Value;
use serde_json::Value as JsonValue;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn create_log(
    pool: &SqlitePool,
    level: LogLevel,
    message: String,
    metadata: Option<Value>,
) -> Result<Log, AppError> {
    let level_str = level.to_string();
    let id = Uuid::new_v4();
    let log = sqlx::query_as!(
        Log,
        r#"INSERT INTO logs (id, level, message, metadata)
        VALUES (?, ?, ?, ?)
        RETURNING id as "id: Uuid", level, message, metadata as "metadata: JsonValue", created_at as "created_at: DateTime<Utc>""#,
        id,
        level_str,
        message,
        metadata
    )
    .fetch_one(pool)
    .await?;

    Ok(log)
}

pub async fn list_logs(
    pool: &SqlitePool,
    limit: i64,
    offset: i64,
    level: Option<LogLevel>,
) -> Result<Vec<Log>, AppError> {
    let logs = match level {
        Some(level) => {
            let level_str = level.to_string();
            sqlx::query_as!(
                Log,
                r#"SELECT 
                    id as "id: Uuid",
                    level as "level: String",
                    message,
                    metadata as "metadata: JsonValue",
                    created_at as "created_at: DateTime<Utc>"
                FROM logs 
                WHERE level = ?
                ORDER BY created_at DESC
                LIMIT ? OFFSET ?"#,
                level_str,
                limit,
                offset
            )
            .fetch_all(pool)
            .await?
        }
        None => {
            sqlx::query_as!(
                Log,
                r#"SELECT 
                    id as "id: Uuid",
                    level,
                    message,
                    metadata as "metadata: JsonValue",
                    created_at as "created_at: DateTime<Utc>"
                FROM logs
                ORDER BY created_at DESC
                LIMIT ? OFFSET ?"#,
                limit,
                offset
            )
            .fetch_all(pool)
            .await?
        }
    };

    Ok(logs)
}

pub async fn delete_logs_before(
    pool: &SqlitePool,
    before: chrono::DateTime<chrono::Utc>,
) -> Result<u64, AppError> {
    let result = sqlx::query!(
        "DELETE FROM logs
        WHERE created_at < ?",
        before
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}

pub async fn get_log(pool: &SqlitePool, id: Uuid) -> Result<Log, AppError> {
    let log = sqlx::query_as!(
        Log,
        r#"SELECT 
            id as "id: Uuid",
            level,
            message,
            metadata as "metadata: JsonValue",
            created_at as "created_at: DateTime<Utc>"
        FROM logs WHERE id = ?"#,
        id
    )
    .fetch_one(pool)
    .await?;
    Ok(log)
}
