use axum::{
    extract::{Path, State},
    Extension, Json,
};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    api::AppState,
    error::AppError,
    middleware::auth::AuthUser,
    models::{
        document::{Document, PermissionType},
        user::UserRole,
    },
    services::document,
};

#[derive(Debug, Deserialize)]
pub struct CreateDocumentRequest {
    title: String,
    content: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateDocumentRequest {
    title: Option<String>,
    content: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePermissionRequest {
    user_id: Uuid,
    permission_type: PermissionType,
}

pub async fn create_document(
    auth: AuthUser,
    Extension(state): Extension<Arc<AppState>>,
    Json(req): Json<CreateDocumentRequest>,
) -> Result<Json<Document>, AppError> {
    let document =
        document::create_document(&state.pool, req.title, req.content, auth.user_id).await?;
    Ok(Json(document))
}

pub async fn get_document(
    auth: AuthUser,
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Document>, AppError> {
    let (document, _) =
        document::get_document_with_permission(&state.pool, id, auth.user_id).await?;
    Ok(Json(document))
}

pub async fn update_document(
    auth: AuthUser,
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateDocumentRequest>,
) -> Result<Json<Document>, AppError> {
    // 检查权限
    let (_, permission) =
        document::get_document_with_permission(&state.pool, id, auth.user_id).await?;

    if permission.is_none() {
        return Err(AppError::Auth("Insufficient permissions".to_string()));
    }
    let permission = permission.unwrap_or("read".to_string());
    if permission == String::from("read") {
        return Err(AppError::Auth("Insufficient permissions".to_string()));
    }

    let document = document::update_document(&state.pool, id, req.title, req.content).await?;
    Ok(Json(document))
}

pub async fn update_permissions(
    auth: AuthUser,
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdatePermissionRequest>,
) -> Result<Json<()>, AppError> {
    // 检查权限
    let (document, permission) =
        document::get_document_with_permission(&state.pool, id, auth.user_id).await?;
    if document.owner_id != auth.user_id && permission.is_none() {
        return Err(AppError::Auth("Insufficient permissions".to_string()));
    }

    document::add_document_permission(&state.pool, id, req.user_id, req.permission_type, None)
        .await?;
    Ok(Json(()))
}

pub async fn delete_document(
    auth: AuthUser,
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<()>, AppError> {
    // 检查权限
    let role = UserRole::from(auth.role);
    if role != UserRole::Admin {
        return Err(AppError::Auth("Insufficient permissions".to_string()));
    }

    document::delete_document(&state.pool, id).await?;
    Ok(Json(()))
}

pub async fn list_documents(
    auth: AuthUser,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<Vec<Document>>, AppError> {
    let role = UserRole::from(auth.role);
    if role != UserRole::Admin {
        return Err(AppError::Auth("Insufficient permissions".to_string()));
    }

    let documents = document::list_documents(&state.pool).await?;
    Ok(Json(documents))
}
