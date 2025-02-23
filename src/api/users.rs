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
    models::{ListUsersQuery, ResponseResult, UpdateUserPasswordRequest, UpdateUserRequest, User},
    services::user,
    utils::password,
};

pub async fn list_users(
    auth: AuthUser,
    Extension(state): Extension<Arc<AppState>>,
    Query(query): Query<ListUsersQuery>,
) -> Result<Json<ResponseResult<Vec<User>>>, AppError> {
    // 检查权限
    if !auth.is_admin() {
        return Err(AppError::Auth("Unauthorized".to_string()));
    }

    let users = user::list_users(&state.db.sqlite, query).await?;

    Ok(Json(ResponseResult {
        code: 0,
        message: None,
        result: Some(users),
    }))
}

pub async fn get_user(
    auth: AuthUser,
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ResponseResult<User>>, AppError> {
    // 检查权限
    if auth.user_id != id {
        return Err(AppError::Auth("Unauthorized".to_string()));
    }
    let user = user::get_user_by_id(&state.db.sqlite, id).await?;
    Ok(Json(ResponseResult {
        code: 0,
        message: Some("User found".to_string()),
        result: Some(user),
    }))
}

pub async fn update_user(
    auth: AuthUser,
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateUserRequest>,
) -> Result<Json<ResponseResult<()>>, AppError> {
    // 检查权限
    if auth.user_id != id {
        return Err(AppError::Auth("Unauthorized".to_string()));
    }

    user::update_user(&state.db.sqlite, req).await?;
    Ok(Json(ResponseResult {
        code: 0,
        message: Some("User updated successfully".to_string()),
        result: Some(()),
    }))
}

pub async fn delete_user(
    auth: AuthUser,
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ResponseResult<()>>, AppError> {
    // 检查权限
    if auth.user_id != id && !auth.is_admin() {
        return Err(AppError::Auth(
            "Not allowed to delete users, only admin can do this".to_string(),
        ));
    }

    user::delete_user(&state.db.sqlite, id).await?;
    Ok(Json(ResponseResult {
        code: 0,
        message: Some("User deleted successfully".to_string()),
        result: Some(()),
    }))
}

pub async fn create_user(
    auth: AuthUser,
    Extension(state): Extension<Arc<AppState>>,
    Json(user): Json<User>,
) -> Result<Json<User>, AppError> {
    // 检查权限
    if !auth.is_admin() {
        return Err(AppError::Auth(
            "Not allowed to create users, only admin can do this".to_string(),
        ));
    }

    // 检查邮箱是否已存在
    let exists = user::check_email_exists(&state.db.sqlite, &user.email).await?;
    if exists {
        return Err(AppError::Auth("Email already exists".to_string()));
    }

    let (password_hash, salt) = password::generate(&user.password.unwrap(), None)?;
    let user_id = Uuid::new_v4();

    let user = user::create_user(
        &state.db.sqlite,
        User {
            id: user_id,
            password: Some(password_hash),
            salt: Some(salt),
            is_active: true,
            is_online: false,
            created_at: Some(chrono::Utc::now()),
            ..user
        },
    )
    .await?;
    Ok(Json(user))
}

pub async fn update_user_password(
    auth: AuthUser,
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateUserPasswordRequest>,
) -> Result<Json<ResponseResult<()>>, AppError> {
    // 检查权限
    if auth.user_id != id {
        return Err(AppError::Auth("Unauthorized".to_string()));
    }

    let user = user::get_user_by_id(&state.db.sqlite, id).await?;
    let (password_hash, _) = password::generate(&req.password, user.salt.as_deref())?;

    user::update_user_password(&state.db.sqlite, id, password_hash).await?;
    Ok(Json(ResponseResult {
        code: 0,
        message: Some("Password updated successfully".to_string()),
        result: Some(()),
    }))
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
        let user = user::update_user_avatar(&state.db.sqlite, id, &file_path).await?;
        return Ok(Json(user));
    }

    Err(AppError::BadRequest("No file uploaded".to_string()))
}
