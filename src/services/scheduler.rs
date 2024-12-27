use crate::{error::AppError, models::task::ScheduledTask, models::task::TaskType};
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
            "SELECT id as 'id: Uuid', name, cron_expression, task_type as 'task_type: String', parameters as 'parameters: Value', is_active, created_at as 'created_at: DateTime<Utc>', updated_at as 'updated_at: DateTime<Utc>' FROM scheduled_tasks WHERE is_active = 1",
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
        let job = Job::new_async(task.clone().cron_expression.as_str(), move |_uuid, _l| {
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
