use crate::error::AppError;
use crate::{api::AppState, models::ResponseResult};
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct SendEmailRequest {
    pub email: String,
    pub subject: String,
    pub body: String,
}

#[derive(Debug, Serialize)]
pub struct SendEmailResponse {
    message: String,
}

#[derive(Debug, Serialize)]
pub struct VerifyEmailResponse {
    message: String,
}

pub async fn send_email(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<SendEmailRequest>,
) -> Result<Json<ResponseResult<()>>, AppError> {
    // 实现发送邮件的逻辑
    state
        .email_service
        .send_email(&payload.email, &payload.subject, &payload.body)
        .await?;
    Ok(Json(ResponseResult {
        code: 0,
        message: Some("Email sent successfully".to_string()),
        result: None,
    }))
}
