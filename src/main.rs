use std::sync::Arc;
use tracing::info;
use work_designer_server::{
    api::{init_router, AppState},
    db::init_databases,
    services::{broadcast::MessageBroadcast, scheduler::Scheduler},
    utils::{email::EmailService, python::PythonExecutor},
    AppError, Config,
};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Load configuration
    let config = Config::new()?;
    info!("Configuration loaded");

    // Initialize database connections
    let db = init_databases(&config.database_url, &config.redis_url).await?;
    info!("Database connections initialized");

    // Initialize  services
    let email_service = EmailService::new(
        &config.smtp_host,
        &config.smtp_username,
        &config.smtp_password,
        &format!("noreply@{}", &config.smtp_host),
    )?;
    info!("Email service initialized");

    let python_executor = PythonExecutor::new();
    info!("Python executor initialized");

    let broadcaster = Arc::new(MessageBroadcast::new(100));
    info!("Websocket broadcaster initialized");

    let scheduler = Scheduler::new(db.sqlite.clone()).await?;
    info!("Scheduler initialized");

    // Create shared application state
    let state = Arc::new(AppState {
        config: config.clone(),
        db,
        email_service,
        python_executor,
        broadcaster: broadcaster.clone(),
        scheduler,
    });

    // Initialize router
    let app = init_router(state);
    info!("Router initialized");

    // Start server
    let addr = format!("127.0.0.1:{}", config.server_port);
    info!("Server starting on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
