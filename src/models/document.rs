use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, Copy)]
#[sqlx(type_name = "document_type", rename_all = "lowercase")]
pub enum DocumentType {
    Text,
    Markdown,
    Html,
    Code,
    Json,
}

impl fmt::Display for DocumentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DocumentType::Text => write!(f, "text"),
            DocumentType::Markdown => write!(f, "markdown"),
            DocumentType::Html => write!(f, "html"),
            DocumentType::Code => write!(f, "code"),
            DocumentType::Json => write!(f, "json"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub owner_id: Uuid,
    pub doc_type: DocumentType,
    pub metadata: Option<serde_json::Value>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDocumentRequest {
    pub title: String,
    pub content: String,
    pub doc_type: DocumentType,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDocumentRequest {
    pub title: Option<String>,
    pub content: Option<String>,
    pub doc_type: Option<DocumentType>,
    pub metadata: Option<serde_json::Value>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentResponse {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub doc_type: DocumentType,
    pub metadata: Option<serde_json::Value>,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_active: bool,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePermissionRequest {
    pub permission_type: PermissionType,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, Copy)]
#[sqlx(type_name = "permission_type", rename_all = "lowercase")]
pub enum PermissionType {
    Read,
    Write,
    Admin,
}

impl fmt::Display for PermissionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PermissionType::Read => write!(f, "read"),
            PermissionType::Write => write!(f, "write"),
            PermissionType::Admin => write!(f, "admin"),
        }
    }
}

impl From<&str> for PermissionType {
    fn from(value: &str) -> Self {
        match value {
            "write" => PermissionType::Write,
            "admin" => PermissionType::Admin,
            _ => PermissionType::Read,
        }
    }
}
