use crate::{
    error::AppError,
    models::{CreateTaskRequest, ListTasksQuery, ScheduledTask, UpdateTaskRequest},
};
use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::sqlite::SqlitePool;
use uuid::Uuid;

pub async fn create_task(pool: &SqlitePool, task: CreateTaskRequest) -> Result<(), AppError> {
    let task_type = task.task_type.to_string();

    sqlx::query!(
        r#"INSERT INTO tasks (
            id, name, description, task_type, cron_expression, one_time, priority, timeout_seconds, max_retries, retry_delay_seconds, parameters, status, is_active, created_by)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        task.id,
        task.name,
        task.description,
        task_type,
        task.cron_expression,
        task.one_time,
        task.priority,
        task.timeout_seconds,
        task.max_retries,
        task.retry_delay_seconds,
        task.parameters,
        task.status,
        task.is_active,
        task.created_by,
    )
    .execute(pool)
    .await
    .map_err(AppError::Database)?;

    Ok(())
}

pub async fn update_task(
    pool: &SqlitePool,
    id: Uuid,
    req: UpdateTaskRequest,
) -> Result<(), AppError> {
    let _ = sqlx::query_as!(
        ScheduledTask,
        r#"UPDATE tasks
        SET name = COALESCE(?, name),
            description = COALESCE(?, description),
            cron_expression = COALESCE(?, cron_expression),
            one_time = COALESCE(?, one_time),
            priority = COALESCE(?, priority),
            timeout_seconds = COALESCE(?, timeout_seconds),
            max_retries = COALESCE(?, max_retries),
            retry_delay_seconds = COALESCE(?, retry_delay_seconds),
            parameters = COALESCE(?, parameters),
            is_active = COALESCE(?, is_active),
            status = COALESCE(?, status)
        WHERE id = ?;"#,
        req.name,
        req.description,
        req.cron_expression,
        req.one_time,
        req.priority,
        req.timeout_seconds,
        req.max_retries,
        req.retry_delay_seconds,
        req.parameters,
        req.is_active,
        req.status,
        id
    )
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)?
    .ok_or_else(|| AppError::NotFound("Task not found".to_string()));
    Ok(())
}

pub async fn get_task(pool: &SqlitePool, id: Uuid) -> Result<ScheduledTask, AppError> {
    sqlx::query_as!(
        ScheduledTask,
        r#"SELECT
            id as "id: Uuid", name, description, task_type as "task_type: String", cron_expression,
            one_time, priority as "priority: String", timeout_seconds, max_retries, retry_delay_seconds, parameters as "parameters: Value", status as "status: String", is_active, created_by as "created_by: Uuid", next_run_at as "next_run_at: DateTime<Utc>", last_run_at as "last_run_at: DateTime<Utc>", created_at as "created_at: DateTime<Utc>", updated_at as "updated_at: DateTime<Utc>"
        FROM tasks WHERE id = ?"#,
        id
    )
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)?
    .ok_or_else(|| AppError::NotFound("Task not found".to_string()))
}

pub async fn list_tasks(
    pool: &SqlitePool,
    query: ListTasksQuery,
) -> Result<Vec<ScheduledTask>, AppError> {
    let limit = query.size;
    let offset = (query.page - 1) * limit;

    sqlx::query_as!(
        ScheduledTask,
        r#"SELECT
                id as "id: Uuid", name, description, task_type, cron_expression,
                one_time, priority as "priority: String", timeout_seconds, max_retries, retry_delay_seconds,
                parameters as "parameters: Value", status as "status: String", is_active, created_by as "created_by: Uuid",
                next_run_at as "next_run_at: DateTime<Utc>", last_run_at as "last_run_at: DateTime<Utc>", created_at as "created_at: DateTime<Utc>", updated_at as "updated_at: DateTime<Utc>"
           FROM tasks
           WHERE (? IS NULL OR status = ?)
           AND (? IS NULL OR priority = ?)
           AND is_active = ?
           ORDER BY created_at DESC
           LIMIT ? OFFSET ?"#,
        query.status,
        query.status,
        query.priority,
        query.priority,
        query.is_active,
        limit,
        offset
    )
    .fetch_all(pool)
    .await
    .map_err(AppError::Database)
}

pub async fn delete_task(pool: &SqlitePool, id: Uuid) -> Result<(), AppError> {
    let result = sqlx::query!("DELETE FROM tasks WHERE id = ? ", id)
        .execute(pool)
        .await
        .map_err(AppError::Database)?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Task not found".to_string()));
    }

    Ok(())
}
