use axum::{
    extract::{Path, Query},
    Extension, Json,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    error::AppError,
    models::{CreateTaskRequest, ListTasksQuery, ResponseResult, ScheduledTask, UpdateTaskRequest},
    services::task,
};

use super::AppState;

pub async fn task_list(
    Extension(state): Extension<Arc<AppState>>,
    Query(query): Query<ListTasksQuery>,
) -> Result<Json<ResponseResult<Vec<ScheduledTask>>>, AppError> {
    let tasks = task::list_tasks(&state.db.sqlite, query).await?;

    Ok(Json(ResponseResult {
        code: 0,
        message: None,
        result: Some(tasks),
    }))
}

pub async fn create_task(
    Extension(state): Extension<Arc<AppState>>,
    Json(req): Json<CreateTaskRequest>,
) -> Result<Json<ResponseResult<()>>, AppError> {
    task::create_task(&state.db.sqlite, req).await?;
    Ok(Json(ResponseResult {
        code: 0,
        message: Some("Task created successfully".to_string()),
        result: None,
    }))
}

pub async fn get_task(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ResponseResult<ScheduledTask>>, AppError> {
    let task = task::get_task(&state.db.sqlite, id).await?;
    Ok(Json(ResponseResult {
        code: 0,
        message: None,
        result: Some(task),
    }))
}

pub async fn update_task(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateTaskRequest>,
) -> Result<Json<ResponseResult<()>>, AppError> {
    task::update_task(&state.db.sqlite, id, req).await?;
    Ok(Json(ResponseResult {
        code: 0,
        message: Some("Task updated successfully".to_string()),
        result: None,
    }))
}

pub async fn delete_task(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ResponseResult<()>>, AppError> {
    task::delete_task(&state.db.sqlite, id).await?;
    Ok(Json(ResponseResult {
        code: 0,
        message: Some("Task deleted successfully".to_string()),
        result: None,
    }))
}
