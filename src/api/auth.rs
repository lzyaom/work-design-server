use crate::{
    api::AppState,
    error::AppError,
    middleware::auth::Claims,
    models::user::{User, UserRole},
    services::user,
};
use axum::{Extension, Json};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    token: String,
    user: User,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    email: String,
    password: String,
    name: String,
}

pub async fn login(
    Extension(state): Extension<Arc<AppState>>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    let mut user = user::get_user_by_email(&state.pool, &req.email).await?;

    if !verify(&req.password, &user.password_salt.unwrap())? {
        return Err(AppError::Auth("Invalid credentials".to_string()));
    }
    user.is_active = 1; // 修改用户在线状态
    let token = create_token(&state, &user.id.to_string(), &user.role.to_string())?;

    Ok(Json(LoginResponse {
        token,
        user: User {
            password_salt: None,
            ..user
        },
    }))
}

pub async fn register(
    Extension(state): Extension<Arc<AppState>>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    // 检查邮箱是否已存在
    let exists = user::check_email_exists(&state.pool, &req.email).await?;

    if exists {
        return Err(AppError::Validation("Email already exists".to_string()));
    }

    let password_salt = hash(req.password.as_bytes(), DEFAULT_COST)?;
    let user_id = Uuid::new_v4();
    let user = user::create_user(
        &state.pool,
        User {
            id: user_id,
            email: req.email,
            password_salt: Some(password_salt),
            username: Some(req.name),
            role: UserRole::User,
            is_active: 1,
            avatar: None,
            created_at: Some(chrono::Utc::now()),
            updated_at: Some(chrono::Utc::now()),
        },
    )
    .await?;

    let token = create_token(&state, &user_id.to_string(), &user.role.to_string())?;

    Ok(Json(LoginResponse {
        token,
        user: User {
            password_salt: None,
            ..user
        },
    }))
}

fn create_token(state: &AppState, user_id: &str, role: &str) -> Result<String, AppError> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        role: role.to_string(),
        exp: expiration,
    };
    let key = &state.config.jwt_secret;
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(key.as_bytes()),
    )?;

    Ok(token)
}
