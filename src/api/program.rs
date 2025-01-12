use crate::models::program::*;
use crate::{error::AppError, middleware::auth::AuthUser, AppState};
use axum::extract::Path;
use axum::{Extension, Json};
use std::sync::Arc;
use uuid::Uuid;

pub async fn list_programs(
    _auth: AuthUser,
    Extension(_state): Extension<Arc<AppState>>,
) -> Result<Json<Vec<Program>>, AppError> {
    Ok(Json(vec![]))
}

pub async fn create_program(
    auth: AuthUser,
    Extension(_state): Extension<Arc<AppState>>,
    Json(req): Json<ProgramRequest>,
) -> Result<Json<ProgramResponse>, AppError> {
    if req.name.is_empty() {
        return Err(AppError::BadRequest("Name is required".to_string()));
    }

    let _program = Program {
        id: Uuid::new_v4(),
        user_id: auth.user_id,
        name: req.name,
        content: req.content.unwrap_or_default(),
        status: ProgramStatus::Pending,
        created_at: Some(chrono::Utc::now().into()),
        updated_at: Some(chrono::Utc::now().into()),
    };

    Ok(Json(ProgramResponse {
        code: 0,
        message: "Program created successfully".to_string(),
    }))
}

pub async fn get_program(
    _auth: AuthUser,
    Extension(_state): Extension<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Program>, AppError> {
    if id.is_nil() {
        return Err(AppError::BadRequest("Id is required".to_string()));
    }
    Ok(Json(Program {
        id: Uuid::new_v4(),
        user_id: Uuid::new_v4(),
        name: "test".to_string(),
        content: "print('Hello, World!')".to_string(),
        status: ProgramStatus::Pending,
        created_at: Some(chrono::Utc::now().into()),
        updated_at: Some(chrono::Utc::now().into()),
    }))
}

pub async fn update_program(
    _auth: AuthUser,
    Extension(_state): Extension<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(_req): Json<ProgramRequest>,
) -> Result<Json<ProgramResponse>, AppError> {
    if id.is_nil() {
        return Err(AppError::BadRequest("Id is required".to_string()));
    }
    Ok(Json(ProgramResponse {
        code: 0,
        message: "Program updated successfully".to_string(),
    }))
}

pub async fn delete_program(
    _auth: AuthUser,
    Extension(_state): Extension<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ProgramResponse>, AppError> {
    if id.is_nil() {
        return Err(AppError::BadRequest("Id is required".to_string()));
    }
    Ok(Json(ProgramResponse {
        code: 0,
        message: "Program deleted successfully".to_string(),
    }))
}

pub async fn compile_program(
    _auth: AuthUser,
    Extension(_state): Extension<Arc<AppState>>,
    Path(id): Path<Uuid>,
    // Json(req): Json<CompilePythonRequest>,
) -> Result<Json<ProgramCompileResponse>, AppError> {
    if id.is_nil() {
        return Err(AppError::BadRequest("Id is required".to_string()));
    }
    // if req.content.is_empty() {
    //     return Err(AppError::BadRequest("Code is required".to_string()));
    // }
    Ok(Json(ProgramCompileResponse {
        status: ProgramStatus::Compiled,
        time: chrono::Utc::now().into(),
        error_file: None,
        error_type: None,
        error_line: None,
        error_message: None,
        error_suggestions: None,
    }))
}

pub async fn run_program(
    _auth: AuthUser,
    Extension(_state): Extension<Arc<AppState>>,
    Path(id): Path<Uuid>,
    // Json(req): Json<ExecutePythonRequest>,
) -> Result<Json<ProgramCompileResponse>, AppError> {
    if id.is_nil() {
        return Err(AppError::BadRequest("Id is required".to_string()));
    }
    Ok(Json(ProgramCompileResponse {
        status: ProgramStatus::Compiled,
        time: chrono::Utc::now().into(),
        error_file: None,
        error_type: None,
        error_line: None,
        error_message: None,
        error_suggestions: None,
    }))
}
