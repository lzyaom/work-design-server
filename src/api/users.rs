use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::{error::AppError, middleware::auth::AuthUser, models::user::User, services::user};

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
    _auth: AuthUser,
    State(pool): State<SqlitePool>,
    Query(query): Query<ListUsersQuery>,
) -> Result<Json<Vec<User>>, AppError> {
    let users =
        user::list_users(&pool, query.limit.unwrap_or(10), query.offset.unwrap_or(0)).await?;

    Ok(Json(users))
}

pub async fn get_user(
    _auth: AuthUser,
    State(pool): State<SqlitePool>,
    Path(id): Path<Uuid>,
) -> Result<Json<User>, AppError> {
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
    if auth.user_id != id && auth.role != "admin" {
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
    if auth.user_id != id && auth.role != "admin" {
        return Err(AppError::Auth("Unauthorized".to_string()));
    }

    user::delete_user(&pool, id).await?;
    Ok(())
}
