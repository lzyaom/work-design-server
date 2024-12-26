use crate::{
    error::AppError,
    models::task::{ScheduledTask, TaskType},
    utils::cron::validate_cron,
};
use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::SqlitePool;
use uuid::Uuid;
pub async fn create_task(
    pool: &SqlitePool,
    name: String,
    cron_expression: String,
    task_type: TaskType,
    parameters: Option<Value>,
) -> Result<ScheduledTask, AppError> {
    // 验证 cron 表达式
    if !validate_cron(&cron_expression) {
        return Err(AppError::Validation("Invalid cron expression".to_string()));
    }
    let id = Uuid::new_v4();
    let task_type_str = task_type.to_string();
    let task = sqlx::query_as!(
        ScheduledTask,
        r#"
        INSERT INTO scheduled_tasks (id, name, cron_expression, task_type, parameters)
        VALUES (?, ?, ?, ?, ?)
        RETURNING id as "id: Uuid", name, cron_expression, task_type as "task_type: String", parameters as "parameters: Value", is_active, created_at as "created_at: DateTime<Utc>", updated_at as "updated_at: DateTime<Utc>"
        "#,
        id,
        name,
        cron_expression,
        task_type_str,
        parameters
    )
    .fetch_one(pool)
    .await?;

    Ok(task)
}

pub async fn update_task(
    pool: &SqlitePool,
    id: Uuid,
    name: Option<String>,
    cron_expression: Option<String>,
    parameters: Option<Value>,
    is_active: Option<bool>,
) -> Result<ScheduledTask, AppError> {
    if let Some(cron) = &cron_expression {
        if !validate_cron(cron) {
            return Err(AppError::Validation("Invalid cron expression".to_string()));
        }
    }

    let task = sqlx::query_as!(
        ScheduledTask,
        r#"
        UPDATE scheduled_tasks
        SET 
            name = COALESCE(?, name),
            cron_expression = COALESCE(?, cron_expression),
            parameters = COALESCE(?, parameters),
            is_active = COALESCE(?, is_active),
            updated_at = CURRENT_TIMESTAMP
        WHERE id = ?
        RETURNING id as "id: Uuid", name, cron_expression, task_type as "task_type: String", parameters as "parameters: Value", is_active, created_at as "created_at: DateTime<Utc>", updated_at as "updated_at: DateTime<Utc>"
        "#,
        name,
        cron_expression,
        parameters,
        is_active,
        id
    )
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Task not found".to_string()))?;

    Ok(task)
}

pub async fn list_tasks(
    pool: &SqlitePool,
    limit: i64,
    offset: i64,
) -> Result<Vec<ScheduledTask>, AppError> {
    let tasks = sqlx::query_as!(
        ScheduledTask,
        r#"
        SELECT id as "id: Uuid", name, cron_expression, task_type as "task_type: String", parameters as "parameters: Value", is_active, created_at as "created_at: DateTime<Utc>", updated_at as "updated_at: DateTime<Utc>" FROM scheduled_tasks
        ORDER BY created_at DESC
        LIMIT ? OFFSET ?
        "#,
        limit,
        offset
    )
    .fetch_all(pool)
    .await?;

    Ok(tasks)
}

pub async fn delete_task(pool: &SqlitePool, id: Uuid) -> Result<(), AppError> {
    let result = sqlx::query!(
        r#"
        DELETE FROM scheduled_tasks WHERE id = ?
        "#,
        id
    )
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Task not found".to_string()));
    }

    Ok(())
}

pub async fn get_task(pool: &SqlitePool, id: Uuid) -> Result<ScheduledTask, AppError> {
    let task = sqlx::query_as!(
        ScheduledTask,
        r#"SELECT id as "id: Uuid", name, cron_expression, task_type as "task_type: String", parameters as "parameters: Value", is_active, created_at as "created_at: DateTime<Utc>", updated_at as "updated_at: DateTime<Utc>" FROM scheduled_tasks WHERE id = ?"#,
        id
    )
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Task not found".to_string()))?;

    Ok(task)
}
