use chrono::{DateTime, Utc};
use sqlx::SqlitePool;
use sqlx::Transaction;
use uuid::Uuid;

use crate::{error::AppError, models::user::User};

pub async fn get_user_by_id(pool: &SqlitePool, id: Uuid) -> Result<User, AppError> {
    let user = sqlx::query_as!(
        User,
        r#"SELECT 
            id as "id: Uuid",
            email,
            password_salt,
            username,
            role as "role: String",
            is_active,
            created_at as "created_at: DateTime<Utc>",
            updated_at as "updated_at: DateTime<Utc>"
        FROM users 
        WHERE id = ?"#,
        id
    )
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    Ok(user)
}

pub async fn list_users(pool: &SqlitePool, limit: i64, offset: i64) -> Result<Vec<User>, AppError> {
    let users = sqlx::query_as!(
        User,
        r#"SELECT 
            id as "id: Uuid",
            email,
            password_salt,
            username,
            role,
            is_active,
            created_at as "created_at: DateTime<Utc>",
            updated_at as "updated_at: DateTime<Utc>"
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
    id: Uuid,
    username: Option<String>,
    email: Option<String>,
) -> Result<User, AppError> {
    let mut transaction: Transaction<'_, sqlx::Sqlite> = pool.begin().await?;
    sqlx::query!(
        "UPDATE users SET username = COALESCE(?, username), email = COALESCE(?, email),updated_at = CURRENT_TIMESTAMP WHERE id = ?",
        username,
        email,
        id
    )
    .execute(&mut *transaction)
    .await?;

    let user = sqlx::query_as!(
        User,
        r#"SELECT id as "id: Uuid", password_salt, email, username, role, is_active, created_at as "created_at: DateTime<Utc>", updated_at as "updated_at: DateTime<Utc>" FROM users WHERE id = ?"#,
        id
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
        r#"INSERT INTO users (id, email, password_salt, username, role) VALUES (?, ?, ?, ?, ?) RETURNING id as "id: Uuid", email, password_salt, username, role as "role: String", is_active, created_at as "created_at: DateTime<Utc>", updated_at as "updated_at: DateTime<Utc>" "#,
        user.id,
        user.email,
        user.password_salt,
        user.username,
        role_str
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}
