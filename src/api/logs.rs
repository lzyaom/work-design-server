use axum::{
    extract::{Path, Query},
    Extension, Json,
};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    api::AppState,
    error::AppError,
    middleware::auth::AuthUser,
    models::{ListLogsQuery, Log, ResponseResult},
    services::log,
};

#[derive(Debug, Deserialize)]
pub struct DeleteLogsQuery {
    before: DateTime<Utc>,
}

pub async fn list_logs(
    auth: AuthUser,
    Extension(state): Extension<Arc<AppState>>,
    Query(query): Query<ListLogsQuery>,
) -> Result<Json<ResponseResult<Vec<Log>>>, AppError> {
    // 检查权限
    if !auth.is_admin() {
        return Err(AppError::Auth("Unauthorized".to_string()));
    }

    let logs = log::list_logs(&state.db.sqlite, query).await?;

    Ok(Json(ResponseResult {
        code: 0,
        message: None,
        result: Some(logs),
    }))
}

pub async fn delete_old_logs(
    auth: AuthUser,
    Extension(state): Extension<Arc<AppState>>,
    Query(query): Query<DeleteLogsQuery>,
) -> Result<Json<ResponseResult<()>>, AppError> {
    // 检查权限
    if !auth.is_admin() {
        return Err(AppError::Auth("Unauthorized".to_string()));
    }

    let deleted_count = log::delete_logs_before(&state.db.sqlite, query.before).await?;
    if deleted_count > 0 {
        Ok(Json(ResponseResult {
            code: 0,
            message: Some(format!("Deleted {} logs", deleted_count)),
            result: None,
        }))
    } else {
        Ok(Json(ResponseResult {
            code: 0,
            message: Some("No logs deleted".to_string()),
            result: None,
        }))
    }
}

pub async fn get_log(
    auth: AuthUser,
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ResponseResult<Log>>, AppError> {
    // 检查权限
    if !auth.is_admin() {
        return Err(AppError::Auth("Insufficient permissions".to_string()));
    }

    let log = log::get_log(&state.db.sqlite, id).await?;
    Ok(Json(ResponseResult {
        code: 0,
        message: None,
        result: Some(log),
    }))
}
