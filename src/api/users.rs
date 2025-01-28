use axum::{
    extract::{Multipart, Path, Query},
    Extension, Json,
};
use std::fs;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    api::AppState,
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
    Extension(state): Extension<Arc<AppState>>,
    Query(query): Query<ListUsersQuery>,
) -> Result<Json<Vec<User>>, AppError> {
    // 检查权限
    let role = UserRole::from(auth.role);
    if role != UserRole::Admin {
        return Err(AppError::Auth("Unauthorized".to_string()));
    }

    let users = user::list_users(
        &state.pool,
        query.limit.unwrap_or(10),
        query.offset.unwrap_or(0),
    )
    .await?;

    Ok(Json(users))
}

pub async fn get_user(
    auth: AuthUser,
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<User>, AppError> {
    // 检查权限
    if auth.user_id != id {
        return Err(AppError::Auth("Unauthorized".to_string()));
    }
    let user = user::get_user_by_id(&state.pool, id).await?;
    Ok(Json(user))
}

pub async fn update_user(
    auth: AuthUser,
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateUserRequest>,
) -> Result<Json<User>, AppError> {
    // 检查权限
    if auth.user_id != id {
        return Err(AppError::Auth("Unauthorized".to_string()));
    }

    let user = user::update_user(&state.pool, id, req.name, req.email, req.avatar).await?;
    Ok(Json(user))
}

pub async fn delete_user(
    auth: AuthUser,
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<(), AppError> {
    // 检查权限
    let role = UserRole::from(auth.role);
    if auth.user_id != id && role != UserRole::Admin {
        return Err(AppError::Auth("Unauthorized".to_string()));
    }

    user::delete_user(&state.pool, id).await?;
    Ok(())
}
pub async fn upload_user_avatar(
    auth: AuthUser,
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<Uuid>,
    mut multipart: Multipart,
) -> Result<Json<User>, AppError> {
    // 检查权限
    if auth.user_id != id {
        return Err(AppError::Auth("Unauthorized".to_string()));
    }

    // 处理上传的文件
    while let Some(field) = multipart.next_field().await? {
        let file_name = field.file_name().unwrap_or("avatar.png").to_string();
        let file_path = format!("./uploads/{}", file_name);

        // 保存文件
        let data = field.bytes().await?;
        fs::write(&file_path, &data)?;

        // 更新用户头像路径
        let user = user::update_user_avatar(&state.pool, id, &file_path).await?;
        return Ok(Json(user));
    }

    Err(AppError::BadRequest("No file uploaded".to_string()))
}
