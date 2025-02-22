use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use sqlx::migrate::MigrateError;
use thiserror::Error;
use tracing::error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Database migration error: {0}")]
    Migration(#[from] MigrateError),

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("Server error: {0}")]
    Server(String),

    #[error("Email error: {0}")]
    Email(String),

    #[error("External service error: {0}")]
    External(String),

    #[error("Job scheduler error: {0}")]
    JobScheduler(#[from] tokio_cron_scheduler::JobSchedulerError),

    #[error("JWT error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Python error: {0}")]
    Python(#[from] pyo3::PyErr),

    #[error("Multipart error: {0}")]
    Multipart(#[from] axum::extract::multipart::MultipartError),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Database(ref e) => {
                error!(error = ?e, "Database error occurred");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "A database error occurred".to_string(),
                )
            }
            AppError::Migration(ref e) => {
                error!(error = ?e, "Database migration error occurred");
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }
            AppError::Auth(ref msg) => {
                error!(error = ?msg, "Authentication error occurred");
                (StatusCode::UNAUTHORIZED, msg.clone())
            }
            AppError::Validation(ref msg) => {
                error!(error = ?msg, "Validation error occurred");
                (StatusCode::BAD_REQUEST, msg.clone())
            }
            AppError::NotFound(ref msg) => {
                error!(error = ?msg, "Not found error occurred");
                (StatusCode::NOT_FOUND, msg.clone())
            }
            AppError::BadRequest(ref msg) => {
                error!(error = ?msg, "Bad request error occurred");
                (StatusCode::BAD_REQUEST, msg.clone())
            }
            AppError::InvalidInput(ref msg) => {
                error!(error = ?msg, "Invalid input error occurred");
                (StatusCode::BAD_REQUEST, msg.clone())
            }
            AppError::Configuration(ref msg) => {
                error!(error = ?msg, "Configuration error occurred");
                (StatusCode::INTERNAL_SERVER_ERROR, msg.clone())
            }
            AppError::Server(ref msg) => {
                error!(error = ?msg, "Server error occurred");
                (StatusCode::INTERNAL_SERVER_ERROR, msg.clone())
            }
            AppError::Email(ref msg) => {
                error!(error = ?msg, "Email error occurred");
                (StatusCode::INTERNAL_SERVER_ERROR, msg.clone())
            }
            AppError::External(ref msg) => {
                error!(error = ?msg, "External service error occurred");
                (StatusCode::BAD_GATEWAY, msg.clone())
            }
            AppError::JobScheduler(ref e) => {
                error!(error = ?e, "Job scheduler error occurred");
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }
            AppError::Jwt(ref e) => {
                error!(error = ?e, "JWT error occurred");
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }
            AppError::Io(ref e) => {
                error!(error = ?e, "IO error occurred");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An IO error occurred".to_string(),
                )
            }
            AppError::Python(ref e) => {
                error!(error = ?e, "Python error occurred");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "A Python error occurred".to_string(),
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

impl From<String> for AppError {
    fn from(err: String) -> Self {
        Self::Server(err)
    }
}

impl From<&str> for AppError {
    fn from(err: &str) -> Self {
        Self::Server(err.to_string())
    }
}
