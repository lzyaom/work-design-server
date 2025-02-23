use chrono::{DateTime, Utc};
use rand::Rng;
use sqlx::SqlitePool;
use sqlx::Transaction;
use uuid::Uuid;

use crate::models::ListUsersQuery;
use crate::{error::AppError, models::user::{User, UpdateUserRequest}};

pub async fn get_user_by_id(pool: &SqlitePool, id: Uuid) -> Result<User, AppError> {
    let user = sqlx::query_as!(
        User,
        r#"SELECT 
            id as "id: Uuid",
            email,
            password,
            salt,
            username,
            role as "role: String",
            is_active,
            avatar,
            is_online,
            gender,
            last_ip,
            last_login as "last_login: DateTime<Utc>",
            created_at as "created_at: DateTime<Utc>"
        FROM users 
        WHERE id = ?"#,
        id
    )
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    Ok(user)
}

pub async fn list_users(pool: &SqlitePool, query: ListUsersQuery) -> Result<Vec<User>, AppError> {
    let limit = query.size.unwrap_or(10);
    let page = query.page.unwrap_or(0);
    let offset = (page - 1) * limit;
    let users = sqlx::query_as!(
        User,
        r#"SELECT 
            id as "id: Uuid",
            email,
            password,
            salt,
            username,
            role as "role: String",
            is_active,
            avatar,
            is_online,
            gender,
            last_ip,
            last_login as "last_login: DateTime<Utc>",
            created_at as "created_at: DateTime<Utc>"
        FROM users 
        ORDER BY created_at DESC 
        LIMIT ? OFFSET ?"#,
        limit,
        offset
    )
    .fetch_all(pool)
    .await?;

    Ok(users)
}

pub async fn update_user(
    pool: &SqlitePool,
    user: UpdateUserRequest
) -> Result<User, AppError> {
    let mut transaction: Transaction<'_, sqlx::Sqlite> = pool.begin().await?;
    sqlx::query!(
        "UPDATE users SET username = COALESCE(?, username), is_online = COALESCE(?, is_online), gender = COALESCE(?, gender), is_active = COALESCE(?, is_active) WHERE email = ?",
        user.username,
        user.is_online,
        user.gender,
        user.is_active,
        user.email
    )
    .execute(&mut *transaction)
    .await?;

    let user = sqlx::query_as!(
        User,
        r#"SELECT id as "id: Uuid", password, salt, email, username, role, is_active, avatar, is_online, gender, last_ip, last_login as "last_login: DateTime<Utc>", created_at as "created_at: DateTime<Utc>" FROM users WHERE email = ?"#,
        user.email
    )
    .fetch_one(&mut *transaction)
    .await?;

    transaction.commit().await?;
    Ok(user)
}

pub async fn delete_user(pool: &SqlitePool, id: Uuid) -> Result<(), AppError> {
    let result = sqlx::query!("DELETE FROM users WHERE id = ?", id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("User not found".to_string()));
    }

    Ok(())
}

pub async fn create_user(pool: &SqlitePool, user: User) -> Result<User, AppError> {
    let role_str = user.role.to_string();
    let user = sqlx::query_as!(
        User,
        r#"INSERT INTO users (id, email, password, salt, username, role, avatar, gender, last_ip, last_login) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?) RETURNING id as "id: Uuid", email, password, salt, username, role as "role: String", is_active, avatar, is_online, gender, last_ip, last_login as "last_login: DateTime<Utc>", created_at as "created_at: DateTime<Utc>""#,
        user.id,
        user.email,
        user.password,
        user.salt,  
        user.username,
        role_str,
        user.avatar,
        user.gender,
        user.last_ip,
        user.last_login
    )
    .fetch_one(pool)
    .await?;

    Ok(User {
        password: None,
        salt: None,
        ..user
    })
}

pub async fn check_email_exists(pool: &SqlitePool, email: &str) -> Result<bool, AppError> {
    let result = sqlx::query!("SELECT COUNT(*) as count FROM users WHERE email = ?", email)
        .fetch_one(pool)
        .await?;

    Ok(result.count > 0)
}

pub async fn get_user_by_email(pool: &SqlitePool, email: &str) -> Result<User, AppError> {
    let user = sqlx::query_as!(
        User,
        r#"SELECT id as "id: Uuid", email, password, salt, username, role as "role: String", is_active, avatar, is_online, gender, last_ip, last_login as "last_login: DateTime<Utc>", created_at as "created_at: DateTime<Utc>" FROM users WHERE email = ?"#,
        email
    )
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    Ok(user)
}

pub async fn create_verification_code(pool: &SqlitePool, email: &str) -> Result<String, AppError> {
    // 生成6位随机验证码
    let code: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Uniform::new(0, 10))
        .take(6)
        .map(|d| d.to_string())
        .collect();
    
    sqlx::query!(
        r#"
        INSERT INTO verification_codes (email, code, expires_at)
        VALUES (?, ?, datetime('now', '+15 minutes'))
        ON CONFLICT(email) DO UPDATE SET code = ?
        "#,
        email,
        code,
        code,
    )
    .execute(pool)
    .await?;

    Ok(code)
}

pub async fn verify_code(pool: &SqlitePool, email: &str, code: &str) -> Result<bool, AppError> {
    let result = sqlx::query!(
        r#"
        SELECT * FROM verification_codes 
        WHERE email = ? AND code = ? AND expires_at > CURRENT_TIMESTAMP
        ORDER BY created_at DESC LIMIT 1
        "#,
        email,
        code
    )
    .fetch_optional(pool)
    .await?;

    Ok(result.is_some())
}

pub async fn update_user_password(pool: &SqlitePool, id: Uuid, password: String) -> Result<User, AppError> {
    let mut transaction: Transaction<'_, sqlx::Sqlite> = pool.begin().await?;
    sqlx::query!(
        "UPDATE users SET password = COALESCE(?, password) WHERE id = ?",
        password,
        id
    )
    .execute(&mut *transaction)
    .await?;

    let user = sqlx::query_as!(
        User,
        r#"SELECT id as "id: Uuid", password, salt, email, username, role as "role: String", is_active, avatar, is_online, gender, last_ip, last_login as "last_login: DateTime<Utc>", created_at as "created_at: DateTime<Utc>" FROM users WHERE id = ?"#,
        id
    )
    .fetch_one(&mut *transaction)
    .await?;

    transaction.commit().await?;
    Ok(User {
        password: None,
        salt: None,
        ..user
    })
}


pub async fn update_user_avatar(pool: &SqlitePool, id: Uuid, avatar: &str) -> Result<User, AppError> {
    let mut transaction: Transaction<'_, sqlx::Sqlite> = pool.begin().await?;
    sqlx::query!(
        "UPDATE users SET avatar = COALESCE(?, avatar) WHERE id = ?",
        avatar,
        id
    )
    .execute(&mut *transaction)
    .await?;

    let user = sqlx::query_as!(
        User,
        r#"SELECT id as "id: Uuid", password, salt, email, username, role, is_active, avatar, is_online, gender, last_ip, last_login as "last_login: DateTime<Utc>", created_at as "created_at: DateTime<Utc>" FROM users WHERE id = ?"#,
        id
    )
    .fetch_one(&mut *transaction)
    .await?;

    transaction.commit().await?;
    Ok(user)
}
