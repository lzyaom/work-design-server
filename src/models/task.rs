use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "task_status", rename_all = "lowercase")]
pub enum TaskStatus {
    Pending,
    Scheduled,
    Running,
    Completed,
    Failed,
    Paused,
    Canceled,
}

impl ToString for TaskStatus {
    fn to_string(&self) -> String {
        match self {
            TaskStatus::Pending => "pending".to_string(),
            TaskStatus::Scheduled => "scheduled".to_string(),
            TaskStatus::Running => "running".to_string(),
            TaskStatus::Completed => "completed".to_string(),
            TaskStatus::Failed => "failed".to_string(),
            TaskStatus::Paused => "paused".to_string(),
            TaskStatus::Canceled => "canceled".to_string(),
        }
    }
}

impl From<String> for TaskStatus {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "scheduled" => TaskStatus::Scheduled,
            "running" => TaskStatus::Running,
            "completed" => TaskStatus::Completed,
            "failed" => TaskStatus::Failed,
            "paused" => TaskStatus::Paused,
            "canceled" => TaskStatus::Canceled,
            _ => TaskStatus::Pending,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "task_priority", rename_all = "lowercase")]
pub enum TaskPriority {
    Low,
    Medium,
    High,
    Critical,
}

impl ToString for TaskPriority {
    fn to_string(&self) -> String {
        match self {
            TaskPriority::Low => "low".to_string(),
            TaskPriority::Medium => "medium".to_string(),
            TaskPriority::High => "high".to_string(),
            TaskPriority::Critical => "critical".to_string(),
        }
    }
}

impl From<String> for TaskPriority {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "medium" => TaskPriority::Medium,
            "high" => TaskPriority::High,
            "critical" => TaskPriority::Critical,
            _ => TaskPriority::Low,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskType {
    EmailNotification,
    DataBackup,
    SystemCleanup,
    Custom(String),
}

impl ToString for TaskType {
    fn to_string(&self) -> String {
        match self {
            TaskType::EmailNotification => "email_notification".to_string(),
            TaskType::DataBackup => "data_backup".to_string(),
            TaskType::SystemCleanup => "system_cleanup".to_string(),
            TaskType::Custom(name) => format!("custom_{}", name),
        }
    }
}

impl From<String> for TaskType {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "email_notification" => TaskType::EmailNotification,
            "data_backup" => TaskType::DataBackup,
            "system_cleanup" => TaskType::SystemCleanup,
            _ => TaskType::Custom(s),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScheduledTask {
    pub id: Uuid,                           // 任务 ID
    pub name: String,                       // 任务名称
    pub description: Option<String>,        // 任务描述
    pub task_type: TaskType,                // 任务类型
    pub cron_expression: Option<String>,    // cron 表达式
    pub one_time: bool,                     // 是否一次性任务
    pub priority: TaskPriority,             // 任务优先级
    pub timeout_seconds: Option<i64>,       // 超时时间
    pub max_retries: i64,                   // 最大重试次数
    pub retry_delay_seconds: i64,           // 重试延迟时间
    pub parameters: Option<Value>,          // 任务参数
    pub status: TaskStatus,                 // 任务状态
    pub is_active: bool,                    // 是否激活
    pub created_by: Uuid,                   // 创建者 ID
    pub created_at: Option<DateTime<Utc>>,  // 创建时间
    pub updated_at: Option<DateTime<Utc>>,  // 更新时间
    pub next_run_at: Option<DateTime<Utc>>, // 下次运行时间
    pub last_run_at: Option<DateTime<Utc>>, // 上次运行时间
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListTasksQuery {
    pub search: Option<String>,
    pub task_type: Option<TaskType>,
    pub status: Option<TaskStatus>,
    pub priority: Option<TaskPriority>,
    pub is_active: Option<bool>,
    pub page: i32,
    pub size: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTaskRequest {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub task_type: TaskType,
    pub cron_expression: Option<String>,
    pub one_time: Option<bool>,
    pub priority: Option<TaskPriority>,
    pub timeout_seconds: Option<i32>,
    pub max_retries: Option<i32>,
    pub retry_delay_seconds: Option<i32>,
    pub parameters: Option<Value>,
    pub is_active: Option<bool>,
    pub status: TaskStatus,
    pub created_by: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTaskRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub cron_expression: Option<String>,
    pub one_time: Option<bool>,
    pub priority: Option<TaskPriority>,
    pub timeout_seconds: Option<i32>,
    pub max_retries: Option<i32>,
    pub retry_delay_seconds: Option<i32>,
    pub parameters: Option<Value>,
    pub is_active: Option<bool>,
    pub status: Option<TaskStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskExecution {
    pub id: Uuid,
    pub task_id: Uuid,
    pub status: TaskStatus,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub duration_ms: Option<i32>,
    pub error_message: Option<String>,
    pub node_id: String,
    pub attempt_number: i32,
    pub parameters: Option<Value>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskDependency {
    pub id: Uuid,
    pub dependent_task_id: Uuid,
    pub prerequisite_task_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskAuditLog {
    pub id: Uuid,
    pub task_id: Uuid,
    pub action: String,
    pub user_id: Option<Uuid>,
    pub details: Value,
    pub created_at: DateTime<Utc>,
}
