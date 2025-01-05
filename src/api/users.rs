use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::{
    error::AppError,
    middleware::auth::AuthUser,
    models::user::{User, UserRole},
    services::user,
};

#[derive(Debug, Deserialize)]
pub struct ListUsersQuery {
    limit: Option<i64>,
    offset: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    name: Option<String>,
    email: Option<String>,
    avatar: Option<String>,
}

pub async fn list_users(
    auth: AuthUser,
    State(pool): State<SqlitePool>,
    Query(query): Query<ListUsersQuery>,
) -> Result<Json<Vec<User>>, AppError> {
    // 检查权限
    let role = UserRole::from(auth.role);
    if role != UserRole::Admin {
        return Err(AppError::Auth("Unauthorized".to_string()));
    }

    let users =
        user::list_users(&pool, query.limit.unwrap_or(10), query.offset.unwrap_or(0)).await?;

    Ok(Json(users))
}

pub async fn get_user(
    auth: AuthUser,
    State(pool): State<SqlitePool>,
    Path(id): Path<Uuid>,
) -> Result<Json<User>, AppError> {
    // 检查权限
    if auth.user_id != id {
        return Err(AppError::Auth("Unauthorized".to_string()));
    }
    let user = user::get_user_by_id(&pool, id).await?;
    Ok(Json(user))
}

pub async fn update_user(
    auth: AuthUser,
    State(pool): State<SqlitePool>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateUserRequest>,
) -> Result<Json<User>, AppError> {
    // 检查权限
    if auth.user_id != id {
        return Err(AppError::Auth("Unauthorized".to_string()));
    }

    let user = user::update_user(&pool, id, req.name, req.email, req.avatar).await?;
    Ok(Json(user))
}

pub async fn delete_user(
    auth: AuthUser,
    State(pool): State<SqlitePool>,
    Path(id): Path<Uuid>,
) -> Result<(), AppError> {
    // 检查权限
    let role = UserRole::from(auth.role);
    if auth.user_id != id && role != UserRole::Admin {
        return Err(AppError::Auth("Unauthorized".to_string()));
    }

    user::delete_user(&pool, id).await?;
    Ok(())
}
