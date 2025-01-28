use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum UserRole {
    Admin,
    User,
}

impl ToString for UserRole {
    fn to_string(&self) -> String {
        match self {
            UserRole::Admin => "admin".to_string(),
            UserRole::User => "user".to_string(),
        }
    }
}

impl From<String> for UserRole {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "admin" => UserRole::Admin,
            "user" => UserRole::User,
            _ => UserRole::User,
        }
    }
}

impl From<UserRole> for String {
    fn from(value: UserRole) -> Self {
        value.to_string()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: Option<String>,
    #[serde(skip_serializing)]
    pub salt: Option<String>,
    pub username: Option<String>,
    pub role: UserRole,
    pub is_active: i64,
    pub is_online: i64,
    pub avatar: Option<String>,
    pub gender: i64,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationCode {
    pub id: i64,
    pub email: String,
    pub code: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub email: String,
    pub username: Option<String>,
    pub is_online: Option<i64>,
    pub gender: Option<i64>,
    pub is_active: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserPasswordRequest {
    pub password: String,
}
