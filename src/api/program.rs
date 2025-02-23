use axum::extract::{ws::Message, Path, Query, WebSocketUpgrade};
use axum::response::IntoResponse;
use axum::{Extension, Json};
use futures::{SinkExt, StreamExt};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    api::AppState,
    error::AppError,
    middleware::auth::AuthUser,
    models::{
        CreateProgramRequest, ListProgramQuery, ListProgramResponse, Program,
        ProgramCompileResponse, ResponseResult, UpdateProgram,
    },
    services::program::{self, ProgramService},
};

pub async fn list_programs(
    _auth: AuthUser,
    Extension(state): Extension<Arc<AppState>>,
    Query(query): Query<ListProgramQuery>,
) -> Result<Json<ResponseResult<Vec<ListProgramResponse>>>, AppError> {
    let programs = program::list_programs(&state.db.sqlite, query).await?;
    Ok(Json(ResponseResult {
        code: 0,
        message: None,
        result: Some(programs),
    }))
}

pub async fn create_program(
    _auth: AuthUser,
    Extension(state): Extension<Arc<AppState>>,
    Json(req): Json<CreateProgramRequest>,
) -> Result<Json<ResponseResult<()>>, AppError> {
    if req.name.is_empty() {
        return Err(AppError::BadRequest("Name is required".to_string()));
    }
    let program = program::create_program(&state.db.sqlite, req).await?;

    Ok(Json(ResponseResult {
        code: 0,
        message: Some("Program created successfully".to_string()),
        result: Some(program),
    }))
}

pub async fn get_program(
    _auth: AuthUser,
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ResponseResult<Program>>, AppError> {
    if id.is_nil() {
        return Err(AppError::BadRequest("Id is required".to_string()));
    }
    let program = program::get_program(&state.db.sqlite, id).await?;
    Ok(Json(ResponseResult {
        code: 0,
        message: None,
        result: Some(program),
    }))
}

pub async fn update_program(
    _auth: AuthUser,
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateProgram>,
) -> Result<Json<ResponseResult<()>>, AppError> {
    if id.is_nil() {
        return Err(AppError::BadRequest("Id is required".to_string()));
    }
    program::update_program(&state.db.sqlite, id, req).await?;
    Ok(Json(ResponseResult {
        code: 0,
        message: Some("Program updated successfully".to_string()),
        result: None,
    }))
}

pub async fn delete_program(
    _auth: AuthUser,
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ResponseResult<()>>, AppError> {
    if id.is_nil() {
        return Err(AppError::BadRequest("Id is required".to_string()));
    }
    program::delete_program(&state.db.sqlite, id).await?;
    Ok(Json(ResponseResult {
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
    let program_service =
        ProgramService::new(state.db.sqlite.clone(), state.python_executor.clone());
    let result = program_service.compile_program(id).await?;
    Ok(Json(result))
}

pub async fn run_program(
    ws: WebSocketUpgrade,
    _auth: AuthUser,
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let program_service =
        ProgramService::new(state.db.sqlite.clone(), state.python_executor.clone());

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
