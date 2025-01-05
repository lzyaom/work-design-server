use crate::{
    config::CONFIG,
    error::AppError,
    middleware::auth::Claims,
    models::user::{User, UserRole},
    services::user,
};
use axum::{extract::State, Json};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{DateTime, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
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
    State(pool): State<SqlitePool>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    let user = sqlx::query_as!(
        User,
        r#"SELECT id as "id: Uuid", email, password_salt, username, role, is_active, created_at as "created_at: DateTime<Utc>", updated_at as "updated_at: DateTime<Utc>" FROM users WHERE email = ?"#,
        req.email
    )
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::Auth("Invalid credentials".to_string()))?;

    if !verify(&req.password, &user.password_salt.unwrap())? {
        return Err(AppError::Auth("Invalid credentials".to_string()));
    }

    let token = create_token(&user.id.to_string(), &user.role.to_string())?;

    Ok(Json(LoginResponse {
        token,
        user: User {
            password_salt: None,
            ..user
        },
    }))
}

pub async fn register(
    State(pool): State<SqlitePool>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    // 检查邮箱是否已存在
    let exists = sqlx::query!(
        "SELECT COUNT(*) as count FROM users WHERE email = ?",
        req.email
    )
    .fetch_one(&pool)
    .await?
    .count
        > 0;

    if exists {
        return Err(AppError::Validation("Email already exists".to_string()));
    }

    let password_salt = hash(req.password.as_bytes(), DEFAULT_COST)?;
    let user_id = Uuid::new_v4();
    let user = user::create_user(
        &pool,
        User {
            id: user_id,
            email: req.email,
            password_salt: Some(password_salt),
            username: Some(req.name),
            role: UserRole::User,
            is_active: 1,
            created_at: Some(chrono::Utc::now()),
            updated_at: Some(chrono::Utc::now()),
        },
    )
    .await?;

    let token = create_token(&user_id.to_string(), &user.role.to_string())?;

    Ok(Json(LoginResponse {
        token,
        user: User {
            password_salt: None,
            ..user
        },
    }))
}

fn create_token(user_id: &str, role: &str) -> Result<String, AppError> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        role: role.to_string(),
        exp: expiration,
    };
    let key = &CONFIG.get().unwrap().jwt_secret;
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(key.as_bytes()),
    )?;

    Ok(token)
}
