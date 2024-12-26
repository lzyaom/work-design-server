use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::SqlitePool;

use crate::{error::AppError, middleware::auth::AuthUser, utils::python::PythonExecutor};

#[derive(Debug, Deserialize)]
pub struct ExecutePythonRequest {
    code: Value,
    input_data: Option<Value>,
}

#[derive(Debug, Serialize)]
pub struct ExecutePythonResponse {
    result: Value,
}

pub async fn execute_python(
    _auth: AuthUser,
    State((_pool, executor)): State<(SqlitePool, PythonExecutor)>,
    Json(req): Json<ExecutePythonRequest>,
) -> Result<Json<ExecutePythonResponse>, AppError> {
    // 执行 Python 代码
    let result = executor
        .execute_json_as_python(req.code, req.input_data)
        .await?;

    Ok(Json(ExecutePythonResponse { result }))
}
