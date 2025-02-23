use crate::{
    error::AppError,
    models::{ScheduledTask, TaskType},
};
use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::SqlitePool;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_cron_scheduler::{Job, JobScheduler};
use uuid::Uuid;
pub struct Scheduler {
    scheduler: JobScheduler,
    pool: SqlitePool,
}

impl Scheduler {
    pub async fn new(pool: SqlitePool) -> Result<Arc<Mutex<Self>>, AppError> {
        let scheduler = JobScheduler::new().await?;
        Ok(Arc::new(Mutex::new(Self { scheduler, pool })))
    }

    pub async fn start(&mut self) -> Result<(), AppError> {
        // 加载所有活跃的任务
        let tasks = sqlx::query_as!(
            ScheduledTask,
            r#"SELECT
                id as 'id: Uuid', name, description, one_time, retry_delay_seconds, timeout_seconds, cron_expression, task_type as 'task_type: String', parameters as 'parameters: Value', priority as "priority: String", status as "status: String", max_retries, is_active, created_by as "created_by: Uuid", created_at as 'created_at: DateTime<Utc>', updated_at as 'updated_at: DateTime<Utc>', next_run_at as 'next_run_at: DateTime<Utc>', last_run_at as 'last_run_at: DateTime<Utc>'
            FROM tasks WHERE is_active = 1"#,
        )
        .fetch_all(&self.pool)
        .await?;

        // 为每个任务创建调度
        for task in tasks {
            self.add_task(task).await?;
        }

        self.scheduler.start().await?;
        Ok(())
    }

    pub async fn add_task(&mut self, task: ScheduledTask) -> Result<(), AppError> {
        let pool = self.pool.clone();
        let cron_expression = task.clone().cron_expression.unwrap_or_default();
        let job = Job::new_async(cron_expression.as_str(), move |_uuid, _l| {
            let _pool = pool.clone();
            let task = task.clone();
            Box::pin(async move {
                match task.task_type {
                    TaskType::EmailNotification => {
                        // 处理邮件通知任务
                    }
                    TaskType::DataBackup => {
                        // 处理数据备份任务
                    }
                    TaskType::SystemCleanup => {
                        // 处理系统清理任务
                    }
                    TaskType::Custom(ref _name) => {
                        // 处理自定义任务
                    }
                }
            })
        })?;

        self.scheduler.add(job).await?;
        Ok(())
    }
}
