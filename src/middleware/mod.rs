pub mod auth;
pub mod monitor;

use axum::{body::Body, http::Request, middleware::Next, response::Response};

use crate::error::AppError;

pub async fn require_auth(req: Request<Body>, next: Next) -> Result<Response, AppError> {
    // 跳过不需要认证的路由
    if !needs_auth(req.uri().path()) {
        return Ok(next.run(req).await);
    }

    // 验证 token
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                // token 验证在 AuthUser extractor 中处理
                return Ok(next.run(req).await);
            }
        }
    }

    Err(AppError::Auth(
        "Missing or invalid authorization".to_string(),
    ))
}

fn needs_auth(path: &str) -> bool {
    !path.starts_with("/api/v1/auth/")
}
