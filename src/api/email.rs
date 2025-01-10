use crate::api::AppState;
use crate::error::AppError;
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct SendEmailRequest {
    pub email: String,
    pub subject: String,
    pub body: String,
}

#[derive(Deserialize)]
pub struct VerifyEmailRequest {
    pub email: String,
    pub code: String,
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
) -> Result<Json<SendEmailResponse>, AppError> {
    // 实现发送邮件的逻辑
    state
        .email_service
        .send_email(&payload.email, &payload.subject, &payload.body)
        .await?;
    Ok(Json(SendEmailResponse {
        message: "Email sent successfully".to_string(),
    }))
}

pub async fn verify_email(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<VerifyEmailRequest>,
) -> Result<Json<VerifyEmailResponse>, AppError> {
    // 实现验证邮件的逻辑
    state
        .email_service
        .verify_email(&payload.email, &payload.code)
        .await?;
    Ok(Json(VerifyEmailResponse {
        message: "Email verified successfully".to_string(),
    }))
}
