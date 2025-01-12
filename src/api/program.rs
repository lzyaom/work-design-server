use axum::extract::{ws::Message, Path, Query, WebSocketUpgrade};
use axum::response::IntoResponse;
use axum::{Extension, Json};
use futures::{SinkExt, StreamExt};
use std::sync::Arc;
use uuid::Uuid;

use crate::models::program::*;
use crate::services::program::{self, ProgramService};
use crate::{error::AppError, middleware::auth::AuthUser, AppState};

pub async fn list_programs(
    _auth: AuthUser,
    Extension(state): Extension<Arc<AppState>>,
    Query(query): Query<ListProgramsQuery>,
) -> Result<Json<Vec<Program>>, AppError> {
    let programs = program::list_programs(&state.pool, query.limit, query.offset).await?;
    Ok(Json(programs))
}

pub async fn create_program(
    auth: AuthUser,
    Extension(state): Extension<Arc<AppState>>,
    Json(req): Json<ProgramRequest>,
) -> Result<Json<ProgramResponse>, AppError> {
    if req.name.is_empty() {
        return Err(AppError::BadRequest("Name is required".to_string()));
    }
    let program = program::create_program(
        &state.pool,
        auth.user_id,
        req.name,
        req.content.unwrap_or_default(),
    )
    .await?;

    Ok(Json(ProgramResponse {
        code: 0,
        message: Some("Program created successfully".to_string()),
        result: Some(program),
    }))
}

pub async fn get_program(
    _auth: AuthUser,
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Program>, AppError> {
    if id.is_nil() {
        return Err(AppError::BadRequest("Id is required".to_string()));
    }
    let program = program::get_program(&state.pool, id).await?;
    Ok(Json(program))
}

pub async fn update_program(
    _auth: AuthUser,
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(req): Json<ProgramRequest>,
) -> Result<Json<ProgramResponse>, AppError> {
    if id.is_nil() {
        return Err(AppError::BadRequest("Id is required".to_string()));
    }
    program::update_program(&state.pool, id, Some(req.name), req.content, req.status).await?;
    Ok(Json(ProgramResponse {
        code: 0,
        message: Some("Program updated successfully".to_string()),
        result: None,
    }))
}

pub async fn delete_program(
    _auth: AuthUser,
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ProgramResponse>, AppError> {
    if id.is_nil() {
        return Err(AppError::BadRequest("Id is required".to_string()));
    }
    program::delete_program(&state.pool, id).await?;
    Ok(Json(ProgramResponse {
        code: 0,
        message: Some("Program deleted successfully".to_string()),
        result: None,
    }))
}

pub async fn compile_program(
    _auth: AuthUser,
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ProgramCompileResponse>, AppError> {
    let program_service = ProgramService::new(state.pool.clone(), state.python_executor.clone());
    let result = program_service.compile_program(id).await?;
    Ok(Json(result))
}

pub async fn run_program(
    ws: WebSocketUpgrade,
    _auth: AuthUser,
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let program_service = ProgramService::new(state.pool.clone(), state.python_executor.clone());

    ws.on_upgrade(move |socket| async move {
        let (mut sender, _) = socket.split();

        match program_service.run_program(id).await {
            Ok(mut receiver) => {
                while let Ok(update) = receiver.recv().await {
                    if let Ok(msg) = serde_json::to_string(&update) {
                        if sender.send(Message::Text(msg)).await.is_err() {
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                let _ = sender.send(Message::Text(format!("Error: {}", e))).await;
            }
        }
    })
}
