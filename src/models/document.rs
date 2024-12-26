use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub owner_id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentPermission {
    pub id: Uuid,
    pub document_id: Uuid,
    pub user_id: Uuid,
    pub parameters: Option<Value>,
    pub permission_type: PermissionType,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum PermissionType {
    Read,
    Write,
    Admin,
}

impl ToString for PermissionType {
    fn to_string(&self) -> String {
        match self {
            PermissionType::Read => "read".to_string(),
            PermissionType::Write => "write".to_string(),
            PermissionType::Admin => "admin".to_string(),
        }
    }
}

impl From<String> for PermissionType {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "read" => PermissionType::Read,
            _ => PermissionType::Write,
        }
    }
}

impl From<PermissionType> for String {
    fn from(value: PermissionType) -> Self {
        value.to_string()
    }
}
