use axum::{body::Body, http::Request, middleware::Next, response::Response};
use metrics::{counter, histogram};
use std::time::Instant;

pub async fn track_metrics(req: Request<Body>, next: Next) -> Response {
    let start = Instant::now();
    let path = req.uri().path().to_owned();
    let method = req.method().clone();

    // 增加请求计数
    counter!("http_requests_total", "path" => path.clone(), "method" => method.to_string());

    let response = next.run(req).await;

    // 记录响应时间
    let duration = start.elapsed();

    histogram!(
        "http_request",
        "duration" => duration.as_secs_f64().to_string(),
        "path" => path,
        "method" => method.to_string(),
        "status" => response.status().as_u16().to_string()
    );

    response
}
