use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, PartialEq)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    User,
    Guest,
}

impl fmt::Display for UserRole {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UserRole::Admin => write!(f, "admin"),
            UserRole::User => write!(f, "user"),
            UserRole::Guest => write!(f, "guest"),
        }
    }
}

impl From<&str> for UserRole {
    fn from(s: &str) -> Self {
        match s {
            "admin" => UserRole::Admin,
            "guest" => UserRole::Guest,
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
    pub last_ip: Option<String>,
    pub last_login: Option<DateTime<Utc>>,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct ListUsersQuery {
    pub role: Option<UserRole>,
    pub is_active: Option<i64>,
    pub is_online: Option<i64>,
    pub search: Option<String>,
    pub page: Option<i64>,
    pub size: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub username: Option<String>,
    pub email: String,
    pub password: String,
    pub role: Option<UserRole>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub email: String,
    pub username: Option<String>,
    pub is_online: Option<i64>,
    pub gender: Option<i64>,
    pub role: Option<UserRole>,
    pub is_active: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserPasswordRequest {
    pub old_password: String,
    pub new_password: String,
}
