use crate::api::AppState;
use crate::error::AppError;
use axum::{extract::State, Extension, Json};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct SendEmailRequest {
    pub email: String,
}

#[derive(Deserialize)]
pub struct VerifyEmailRequest {
    pub email: String,
    pub code: String,
}

pub async fn send_email(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<SendEmailRequest>,
) -> Result<impl IntoResponse, AppError> {
    // 实现发送邮件的逻辑
    state
        .email_service
        .send_email(
            &payload.email,
            "Verify your email",
            "Please click the link to verify your email",
        )
        .await?;
    Ok(Json(SendEmailResponse {
        message: "Email sent successfully".to_string(),
    }))
}

pub async fn verify_email(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<VerifyEmailRequest>,
) -> Result<impl IntoResponse, AppError> {
    // 实现验证邮件的逻辑
    state
        .email_service
        .verify_email(&payload.email, &payload.code)
        .await?;
    Ok(Json(VerifyEmailResponse {
        message: "Email verified successfully".to_string(),
    }))
}
