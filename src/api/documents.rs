use axum::{extract::Path, Extension, Json};
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
        CreateDocumentRequest, ResponseResult, UpdateDocumentRequest,
    },
    services::document,
};

#[derive(Debug, Deserialize)]
pub struct UpdatePermissionRequest {
    user_id: Uuid,
    permission_type: PermissionType,
}

pub async fn create_document(
    _auth: AuthUser,
    Extension(state): Extension<Arc<AppState>>,
    Json(req): Json<CreateDocumentRequest>,
) -> Result<Json<ResponseResult<()>>, AppError> {
    document::create_document(&state.db.sqlite, req).await?;
    Ok(Json(ResponseResult {
        code: 0,
        message: Some("Document created successfully".to_string()),
        result: None,
    }))
}

pub async fn get_document(
    auth: AuthUser,
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ResponseResult<Document>>, AppError> {
    let (document, _) =
        document::get_document_with_permission(&state.db.sqlite, id, auth.user_id).await?;
    Ok(Json(ResponseResult {
        code: 0,
        message: None,
        result: Some(document),
    }))
}

pub async fn update_document(
    auth: AuthUser,
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateDocumentRequest>,
) -> Result<Json<ResponseResult<()>>, AppError> {
    // 检查权限
    let (_, permission) =
        document::get_document_with_permission(&state.db.sqlite, id, auth.user_id).await?;

    if permission.is_none() {
        return Err(AppError::Auth("Insufficient permissions".to_string()));
    }
    let permission = permission.unwrap_or("read".to_string());
    if permission == String::from("read") {
        return Err(AppError::Auth("Insufficient permissions".to_string()));
    }

    document::update_document(&state.db.sqlite, id, req).await?;
    Ok(Json(ResponseResult {
        code: 0,
        message: Some("Document updated successfully".to_string()),
        result: None,
    }))
}

pub async fn update_permissions(
    auth: AuthUser,
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdatePermissionRequest>,
) -> Result<Json<ResponseResult<()>>, AppError> {
    // 检查权限
    let (document, permission) =
        document::get_document_with_permission(&state.db.sqlite, id, auth.user_id).await?;
    if document.user_id != auth.user_id && permission.is_none() {
        return Err(AppError::Auth("Insufficient permissions".to_string()));
    }

    document::add_document_permission(&state.db.sqlite, id, req.user_id, req.permission_type, None)
        .await?;
    Ok(Json(ResponseResult {
        code: 0,
        message: Some("Permissions updated successfully".to_string()),
        result: None,
    }))
}

pub async fn delete_document(
    auth: AuthUser,
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ResponseResult<()>>, AppError> {
    // 检查权限
    let role = UserRole::from(auth.role);
    if role != UserRole::Admin {
        return Err(AppError::Auth("Insufficient permissions".to_string()));
    }

    document::delete_document(&state.db.sqlite, id).await?;
    Ok(Json(ResponseResult {
        code: 0,
        message: Some("Document deleted successfully".to_string()),
        result: None,
    }))
}

pub async fn list_documents(
    auth: AuthUser,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<ResponseResult<Vec<Document>>>, AppError> {
    let role = UserRole::from(auth.role);
    if role != UserRole::Admin {
        return Err(AppError::Auth("Insufficient permissions".to_string()));
    }

    let documents = document::list_documents(&state.db.sqlite).await?;
    Ok(Json(ResponseResult {
        code: 0,
        message: None,
        result: Some(documents),
    }))
}
