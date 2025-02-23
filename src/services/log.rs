use crate::{
    error::AppError,
    models::{ListLogsQuery, Log},
};
use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn create_log(pool: &SqlitePool, log: Log) -> Result<Log, AppError> {
    let level = log.level.to_string();
    let log = sqlx::query_as!(
        Log,
        r#"INSERT INTO logs (id, level, message, source, metadata)
        VALUES (?, ?, ?, ?, ?)
        RETURNING id as "id: Uuid", level as "level: String", message, source, metadata as "metadata: Value", created_at as "created_at: DateTime<Utc>""#,
        log.id,
        level,
        log.message,
        log.source,
        log.metadata
    )
    .fetch_one(pool)
    .await?;

    Ok(log)
}

pub async fn list_logs(pool: &SqlitePool, query: ListLogsQuery) -> Result<Vec<Log>, AppError> {
    let limit = query.size.unwrap_or(10);
    let page = query.page.unwrap_or(1);
    let offset = (page - 1) * limit;

    let logs = match query.level {
        Some(level) => {
            let level = level.to_string();
            sqlx::query_as!(
                Log,
                r#"SELECT 
                    id as "id: Uuid",
                    level as "level: String",
                    message,
                    source,
                    metadata as "metadata: Value",
                    created_at as "created_at: DateTime<Utc>"
                FROM logs 
                WHERE level = ?
                ORDER BY created_at DESC
                LIMIT ? OFFSET ?"#,
                level,
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
                    level as "level: String",
                    message,
                    source,
                    metadata as "metadata: Value",
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
            level as "level: String",
            message,
            source,
            metadata as "metadata: Value",
            created_at as "created_at: DateTime<Utc>"
        FROM logs WHERE id = ?"#,
        id
    )
    .fetch_one(pool)
    .await?;
    Ok(log)
}
