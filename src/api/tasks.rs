use axum::{
    extract::{Path, Query, State},
    Extension, Json,
};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    api::AppState,
    error::AppError,
    middleware::auth::AuthUser,
    models::{
        task::{ScheduledTask, TaskType},
        user::UserRole,
    },
    services::task,
};

#[derive(Debug, Deserialize)]
pub struct CreateTaskRequest {
    name: String,
    cron_expression: String,
    task_type: TaskType,
    parameters: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct ListTasksQuery {
    limit: Option<i64>,
    offset: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTaskRequest {
    name: Option<String>,
    cron_expression: Option<String>,
    parameters: Option<serde_json::Value>,
    is_active: Option<bool>,
}
pub async fn task_list(
    auth: AuthUser,
    Extension(state): Extension<Arc<AppState>>,
    Query(query): Query<ListTasksQuery>,
) -> Result<Json<Vec<ScheduledTask>>, AppError> {
    let role = UserRole::from(auth.role);
    if role != UserRole::Admin {
        return Err(AppError::Auth("Insufficient permissions".to_string()));
    }

    let tasks = task::list_tasks(
        &state.pool,
        query.limit.unwrap_or(100),
        query.offset.unwrap_or(0),
    )
    .await?;

    Ok(Json(tasks))
}

pub async fn create_task(
    auth: AuthUser,
    Extension(state): Extension<Arc<AppState>>,
    Json(req): Json<CreateTaskRequest>,
) -> Result<Json<ScheduledTask>, AppError> {
    // 检查权限
    let role = UserRole::from(auth.role);
    if role != UserRole::Admin {
        return Err(AppError::Auth("Insufficient permissions".to_string()));
    }

    let task = task::create_task(
        &state.pool,
        req.name,
        req.cron_expression,
        req.task_type,
        req.parameters,
    )
    .await?;

    Ok(Json(task))
}

pub async fn update_task(
    auth: AuthUser,
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateTaskRequest>,
) -> Result<Json<ScheduledTask>, AppError> {
    // 检查权限
    let role = UserRole::from(auth.role);
    if role != UserRole::Admin {
        return Err(AppError::Auth("Insufficient permissions".to_string()));
    }

    let task = task::update_task(
        &pool,
        id,
        req.name,
        req.cron_expression,
        req.parameters,
        req.is_active,
    )
    .await?;

    Ok(Json(task))
}

pub async fn delete_task(
    auth: AuthUser,
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<()>, AppError> {
    // 检查权限
    let role = UserRole::from(auth.role);
    if role != UserRole::Admin {
        return Err(AppError::Auth("Insufficient permissions".to_string()));
    }

    task::delete_task(&state.pool, id).await?;
    Ok(Json(()))
}

pub async fn get_task(
    auth: AuthUser,
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ScheduledTask>, AppError> {
    // 检查权限
    let role = UserRole::from(auth.role);
    if role != UserRole::Admin {
        return Err(AppError::Auth("Insufficient permissions".to_string()));
    }

    let task = task::get_task(&state.pool, id).await?;
    Ok(Json(task))
}
