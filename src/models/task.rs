use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

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

impl From<TaskType> for String {
    fn from(value: TaskType) -> Self {
        value.to_string()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScheduledTask {
    pub id: String,
    pub name: String,
    pub cron_expression: String,
    pub task_type: TaskType,
    pub parameters: Option<Value>,
    pub is_active: i64,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
