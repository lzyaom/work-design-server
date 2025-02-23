use axum::{body::Body, http::Request, middleware::Next, response::Response};
use chrono::Utc;
use metrics::{counter, histogram};
use serde_json::json;
// use sqlx::SqlitePool;
use std::time::Instant;
use uuid::Uuid;

use crate::{
    error::AppError,
    models::{Log, LogLevel},
    // services::log::create_log,
};

/// Monitor middleware for tracking request metrics
pub async fn track_metrics(
    req: Request<Body>,
    next: Next,
    // pool: SqlitePool,
) -> Result<Response, AppError> {
    let start = Instant::now();
    let path = req.uri().path().to_string();
    let method = req.method().clone();

    // 增加请求计数
    counter!("http.request", "path" => path.clone(), "method" => method.to_string());

    let response = next.run(req).await;

    // 记录响应时间
    let duration = start.elapsed().as_millis().to_string();

    // 记录响应状态
    let status = response.status().as_u16();

    histogram!(
        "http.request",
        "duration" => duration.clone(),
        "path" => path.clone(),
        "method" => method.to_string(),
        "status" => status.to_string()
    );

    // Create log entry
    let _log = Log {
        id: Uuid::new_v4(),
        level: if status >= 400 {
            LogLevel::Error
        } else {
            LogLevel::Info
        },
        message: format!("{} {} {} {}ms", method, path, status, duration.clone()),
        source: Some("monitor middleware".to_string()),
        metadata: Some(json!({
            "method": method.to_string(),
            "path": path,
            "status": status,
            "duration_ms": duration
        })),
        created_at: Some(Utc::now()),
    };
    // create_log(&pool, log).await?;

    // Return the response
    Ok(response)
}
