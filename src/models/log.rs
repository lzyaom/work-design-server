use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, Copy)]
#[sqlx(type_name = "log_level", rename_all = "lowercase")]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
    Critical,
    Event,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogLevel::Debug => write!(f, "debug"),
            LogLevel::Info => write!(f, "info"),
            LogLevel::Warning => write!(f, "warning"),
            LogLevel::Error => write!(f, "error"),
            LogLevel::Critical => write!(f, "critical"),
            LogLevel::Event => write!(f, "event"),
        }
    }
}

impl From<String> for LogLevel {
    fn from(s: String) -> Self {
        match s.as_str() {
            "debug" => LogLevel::Debug,
            "warning" => LogLevel::Warning,
            "error" => LogLevel::Error,
            "critical" => LogLevel::Critical,
            "event" => LogLevel::Event,
            _ => LogLevel::Info,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Log {
    pub id: Uuid,
    pub level: LogLevel,
    pub message: String,
    pub source: Option<String>,
    pub metadata: Option<Value>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListLogsQuery {
    pub level: Option<LogLevel>,
    pub source: Option<String>,
    pub from: Option<DateTime<Utc>>,
    pub to: Option<DateTime<Utc>>,
    pub page: Option<i64>,
    pub size: Option<i64>,
}
