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
    models::{
        log::{Log, LogLevel},
        user::UserRole,
    },
    services::log,
};

#[derive(Debug, Deserialize)]
pub struct ListLogsQuery {
    limit: Option<i64>,
    offset: Option<i64>,
    level: Option<LogLevel>,
}

#[derive(Debug, Deserialize)]
pub struct DeleteLogsQuery {
    before: DateTime<Utc>,
}

pub async fn list_logs(
    auth: AuthUser,
    Extension(state): Extension<Arc<AppState>>,
    Query(query): Query<ListLogsQuery>,
) -> Result<Json<Vec<Log>>, AppError> {
    // 检查权限
    let role = UserRole::from(auth.role);
    if role != UserRole::Admin {
        return Err(AppError::Auth("Unauthorized".to_string()));
    }

    let logs = log::list_logs(
        &state.pool,
        query.limit.unwrap_or(10),
        query.offset.unwrap_or(0),
        query.level,
    )
    .await?;

    Ok(Json(logs))
}

pub async fn delete_old_logs(
    auth: AuthUser,
    Extension(state): Extension<Arc<AppState>>,
    Query(query): Query<DeleteLogsQuery>,
) -> Result<Json<u64>, AppError> {
    // 检查权限
    let role = UserRole::from(auth.role);
    if role != UserRole::Admin {
        return Err(AppError::Auth("Unauthorized".to_string()));
    }

    let deleted_count = log::delete_logs_before(&state.pool, query.before).await?;
    Ok(Json(deleted_count))
}

pub async fn get_log(
    auth: AuthUser,
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Log>, AppError> {
    // 检查权限
    let role = UserRole::from(auth.role);
    if role != UserRole::Admin {
        return Err(AppError::Auth("Insufficient permissions".to_string()));
    }

    let log = log::get_log(&state.pool, id).await?;
    Ok(Json(log))
}
