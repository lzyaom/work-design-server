use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
mod api;
mod config;
mod db;
mod error;
mod handlers;
mod middleware;
mod models;
mod services;
mod utils;

use crate::{
    services::{broadcast::DocumentBroadcaster, scheduler::Scheduler},
    utils::{email::EmailService, python::PythonExecutor},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "work_designer_server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 加载配置
    let config = config::load_config()?;

    // 初始化数据库连接
    let pool = db::init_db_sqlite(&config.database_url).await?;

    // 初始化邮件服务
    let email_service = EmailService::new(
        &config.smtp_host,
        &config.smtp_username,
        &config.smtp_password,
        format!("noreply@{}", &config.smtp_host),
    )?;

    // 初始化 Python 执行器
    let python_executor = PythonExecutor::new();

    // 初始化文档广播器
    let broadcaster = Arc::new(DocumentBroadcaster::new());

    // 初始化调度器
    let scheduler = Scheduler::new(pool.clone()).await?;
    let mut scheduler_lock = scheduler.lock().await;

    // 添加系统监控任务
    scheduler_lock
        .add_task(models::task::ScheduledTask {
            id: uuid::Uuid::new_v4().to_string(),
            name: "System Monitor".to_string(),
            cron_expression: "*/5 * * * *".to_string(), // 每5分钟执行一次
            task_type: models::task::TaskType::SystemCleanup,
            parameters: None,
            is_active: 1,
            created_at: Some(chrono::Utc::now()),
            updated_at: Some(chrono::Utc::now()),
        })
        .await?;

    // 启动调度器
    scheduler_lock.start().await?;
    drop(scheduler_lock);

    // 创建应用路由
    let app = api::create_router(
        pool.clone(),
        email_service,
        python_executor,
        broadcaster.clone(),
    );

    // 启动服务器
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], config.server_port));
    tracing::info!("Server listening on {}", addr);

    axum::serve(
        tokio::net::TcpListener::bind(&addr).await?,
        app.into_make_service(),
    )
    .await?;

    Ok(())
}
