use crate::{
    error::AppError, middleware::auth::AuthUser, models::user::UserRole, services::monitor::{get_system_status, SystemStatus}
};
use axum::{extract::State, Json};
use sqlx::SqlitePool;

pub async fn get_status(
    auth: AuthUser,
    State(_pool): State<SqlitePool>,
) -> Result<Json<SystemStatus>, AppError> {
    // 检查权限
    let role = UserRole::from(auth.role);
    if role != UserRole::Admin {
        return Err(AppError::Auth("Unauthorized".to_string()));
    }

    let status = get_system_status();
    Ok(Json(status))
}
