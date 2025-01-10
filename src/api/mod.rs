use axum::{
    middleware::from_fn,
    routing::{delete, get, post, put},
    Extension, Router,
};
use sqlx::SqlitePool;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

use crate::{
    config::Config,
    middleware::{monitor::track_metrics, require_auth},
    services::broadcast::DocumentBroadcaster,
    utils::{email::EmailService, python::PythonExecutor},
};

mod auth;
mod documents;
mod email;
mod logs;
mod monitor;
mod tasks;
mod users;
mod websocket;

pub struct AppState {
    pub config: Config,
    pub pool: SqlitePool,
    pub email_service: EmailService,
    pub python_executor: PythonExecutor,
    pub broadcaster: Arc<DocumentBroadcaster>,
}

pub fn create_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .nest("/api/v1", api_router())
        .layer(cors)
        .layer(from_fn(require_auth))
        .layer(from_fn(track_metrics))
        .layer(Extension(Arc::new(state)))
        .fallback(crate::handlers::handle_404)
}

fn api_router() -> Router {
    Router::new()
        // 认证路由
        .route("/auth/login", post(auth::login))
        .route("/auth/register", post(auth::register))
        // 用户路由
        .route("/users", get(users::list_users))
        .route("/users/:id", get(users::get_user))
        .route("/users/:id", put(users::update_user))
        .route("/users/:id", delete(users::delete_user))
        // 邮件路由
        .route("/email/send", post(email::send_email))
        .route("/email/verify", post(email::verify_email))
        // 日志路由
        .route("/logs", get(logs::list_logs))
        .route("/logs/:id", get(logs::get_log))
        .route("/logs/:id", delete(logs::delete_old_logs))
        // 任务路由
        .route("/tasks", get(tasks::task_list))
        .route("/tasks", post(tasks::create_task))
        .route("/tasks/:id", get(tasks::get_task))
        .route("/tasks/:id", put(tasks::update_task))
        .route("/tasks/:id", delete(tasks::delete_task))
        // 文档路由
        .route("/documents", get(documents::list_documents))
        .route("/documents", post(documents::create_document))
        .route("/documents/:id", get(documents::get_document))
        .route("/documents/:id", put(documents::update_document))
        .route("/documents/:id", delete(documents::delete_document))
        .route(
            "/documents/:id/permissions",
            post(documents::update_permissions),
        )
        .route("/monitor", get(monitor::get_status))
}
