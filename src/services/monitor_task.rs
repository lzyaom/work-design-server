use crate::{
    error::AppError,
    models::log::LogLevel,
    services::{log::create_log, monitor::get_system_status},
};
use serde_json::json;
use sqlx::SqlitePool;

pub async fn monitor_system(pool: &SqlitePool) -> Result<(), AppError> {
    let status = get_system_status();

    // 检查系统资源使用情况
    if status.cpu_usage > 90.0 {
        create_log(
            pool,
            LogLevel::Warning,
            "High CPU usage detected".to_string(),
            Some(json!({
                "cpu_usage": status.cpu_usage,
            })),
        )
        .await?;
    }

    if (status.memory_used as f64 / status.memory_total as f64) > 0.9 {
        create_log(
            pool,
            LogLevel::Warning,
            "High memory usage detected".to_string(),
            Some(json!({
                "memory_used": status.memory_used,
                "memory_total": status.memory_total,
            })),
        )
        .await?;
    }

    if (status.disk_used as f64 / status.disk_total as f64) > 0.9 {
        create_log(
            pool,
            LogLevel::Warning,
            "High disk usage detected".to_string(),
            Some(json!({
                "disk_used": status.disk_used,
                "disk_total": status.disk_total,
            })),
        )
        .await?;
    }

    Ok(())
}
