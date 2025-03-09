use crate::{
    config::Config,
    db::DatabasePools,
    handlers,
    middleware::{monitor::track_metrics, require_auth},
    services::{broadcast::MessageBroadcast, scheduler::Scheduler},
    utils::{email::EmailService, python::PythonExecutor},
};
use axum::{
    middleware::from_fn,
    routing::{delete, get, post, put},
    Extension, Router,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};

mod auth;
mod documents;
mod email;
mod logs;
mod monitor;
mod program;
mod tasks;
mod users;
mod websocket;

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub db: DatabasePools,
    pub email_service: EmailService,
    pub python_executor: PythonExecutor,
    pub broadcaster: Arc<MessageBroadcast>,
    pub scheduler: Arc<Mutex<Scheduler>>,
}

pub fn init_router(state: Arc<AppState>) -> Router {
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
        .fallback(handlers::handle_404)
}

fn api_router() -> Router {
    Router::new()
        // Auth routes
        .route("/auth/login", post(auth::login))
        .route("/auth/register", post(auth::register))
        .route("/auth/code", post(auth::send_verification_code))
        // User routes
        .route("/users", get(users::list_users))
        .route("/users", post(users::create_user))
        .route("/upload/avatar", post(users::upload_user_avatar))
        .route("/users/password/:id", put(users::update_user_password))
        .route("/users/:id", get(users::get_user))
        .route("/users/:id", put(users::update_user))
        .route("/users/:id", delete(users::delete_user))
        // Email routes
        .route("/email/send", post(email::send_email))
        // Log routes
        .route("/logs", get(logs::list_logs))
        .route("/logs/:id", get(logs::get_log))
        .route("/logs/:id", delete(logs::delete_old_logs))
        // Task routes
        .route("/tasks", get(tasks::task_list))
        .route("/tasks", post(tasks::create_task))
        .route("/tasks/:id", get(tasks::get_task))
        .route("/tasks/:id", put(tasks::update_task))
        .route("/tasks/:id", delete(tasks::delete_task))
        // Document routes
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
        // Program routes
        .route("/program", get(program::list_programs))
        .route("/program/:id", get(program::get_program))
        .route("/program", post(program::create_program))
        .route("/program/:id", put(program::update_program))
        .route("/program/:id", delete(program::delete_program))
        .route("/program/compile/:id", post(program::compile_program))
        .route("/program/run/:id", post(program::run_program))
        // Websocket routes
        .route("/ws", get(websocket::ws_handler))
}
