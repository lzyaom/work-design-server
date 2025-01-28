use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;
use tracing::error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("JWT error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("Email error: {0}")]
    Email(#[from] lettre::error::Error),

    #[error("Email address error: {0}")]
    EmailAddress(#[from] lettre::address::AddressError),

    #[error("Email send error: {0}")]
    EmailSend(#[from] lettre::transport::smtp::Error),

    #[error("Python execution error: {0}")]
    Python(String),

    #[error("Rate limit exceeded")]
    RateLimit,

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Invalid value: {0}")]
    InvalidValue(String),

    #[error("Invalid log level: {0}")]
    InvalidLogLevel(String),

    #[error("Invalid user role: {0}")]
    InvalidUserRole(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Python error: {0}")]
    Py(#[from] pyo3::PyErr),

    #[error("Job Scheduler error: {0}")]
    JobScheduler(#[from] tokio_cron_scheduler::JobSchedulerError),

    #[error("Json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Multipart error: {0}")]
    Multipart(#[from] axum::extract::multipart::MultipartError),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Auth(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Database(ref e) => {
                error!(error = ?e, "Database error occurred");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "A database error occurred".to_string(),
                )
            }
            AppError::Jwt(ref e) => {
                error!(error = ?e, "JWT error occurred");
                (StatusCode::UNAUTHORIZED, "Invalid token".to_string())
            }
            AppError::Email(ref e) => {
                error!(error = ?e, "Email error occurred");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to send email".to_string(),
                )
            }
            AppError::EmailAddress(ref e) => {
                error!(error = ?e, "Email address error occurred");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to parse email address".to_string(),
                )
            }
            AppError::Python(msg) => {
                error!(error = %msg, "Python execution error occurred");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Python execution failed".to_string(),
                )
            }
            AppError::RateLimit => (
                StatusCode::TOO_MANY_REQUESTS,
                "Rate limit exceeded".to_string(),
            ),
            AppError::Internal(ref msg) => {
                error!(error = %msg, "Internal server error occurred");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An internal server error occurred".to_string(),
                )
            }
            AppError::BadRequest(ref msg) => (StatusCode::BAD_REQUEST, msg.to_string()),
            AppError::InvalidValue(ref msg) => (StatusCode::BAD_REQUEST, msg.to_string()),
            AppError::InvalidLogLevel(ref msg) => (StatusCode::BAD_REQUEST, msg.to_string()),
            AppError::InvalidUserRole(ref msg) => (StatusCode::BAD_REQUEST, msg.to_string()),
            AppError::EmailSend(ref e) => {
                error!(error = ?e, "Email send error occurred");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to send email".to_string(),
                )
            }
            AppError::Io(ref e) => {
                error!(error = ?e, "IO error occurred");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An IO error occurred".to_string(),
                )
            }
            AppError::Py(ref e) => {
                error!(error = ?e, "Python error occurred");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "A Python error occurred".to_string(),
                )
            }
            AppError::JobScheduler(ref e) => {
                error!(error =?e, "Job scheduler error occurred");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "A Jog scheduler error occurred".to_string(),
                )
            }
            AppError::Json(ref e) => {
                error!(error = ?e, "Json error occurred");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "A Json error occurred".to_string(),
                )
            }
            AppError::Multipart(ref e) => {
                error!(error = ?e, "Multipart error occurred");
                (
                    StatusCode::BAD_REQUEST,
                    "A Multipart error occurred".to_string(),
                )
            }
        };

        let body = Json(json!({
            "error": {
                "code": status.as_u16(),
                "message": error_message,
                "type": status.canonical_reason()
            }
        }));

        (status, body).into_response()
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
