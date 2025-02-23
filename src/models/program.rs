use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "program_status", rename_all = "lowercase")]
pub enum ProgramStatus {
    Pending,
    Compiling,
    Compiled,
    Failed,
    Running,
    Stopped,
}

impl ToString for ProgramStatus {
    fn to_string(&self) -> String {
        match self {
            ProgramStatus::Pending => "pending".to_string(),
            ProgramStatus::Compiling => "compiling".to_string(),
            ProgramStatus::Compiled => "compiled".to_string(),
            ProgramStatus::Failed => "failed".to_string(),
            ProgramStatus::Running => "running".to_string(),
            ProgramStatus::Stopped => "stopped".to_string(),
        }
    }
}

impl From<String> for ProgramStatus {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "compiling" => ProgramStatus::Compiling,
            "compiled" => ProgramStatus::Compiled,
            "failed" => ProgramStatus::Failed,
            "running" => ProgramStatus::Running,
            "stopped" => ProgramStatus::Stopped,
            _ => ProgramStatus::Pending,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Program {
    pub id: Uuid,
    pub user_id: Uuid,                       // 用户 ID
    pub name: String,                        // 程序名称
    pub description: Option<String>,         // 程序描述
    pub source_code: String,                 // 源代码
    pub compiled_code: Option<String>,       // 编译后的代码
    pub status: ProgramStatus,               // 程序状态
    pub metadata: Option<serde_json::Value>, // 元数据
    pub is_active: bool,                     // 是否激活
    pub created_at: Option<DateTime<Utc>>,   // 创建时间
    pub updated_at: Option<DateTime<Utc>>,   // 更新时间
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProgramCompileResponse {
    pub status: ProgramStatus,             // 编译状态
    pub time: DateTime<Utc>,               // 编译时间
    pub error_file: Option<String>,        // 错误文件
    pub error_type: Option<String>,        // 错误类型
    pub error_line: Option<i32>,           // 错误行
    pub error_message: Option<String>,     // 错误信息
    pub error_suggestions: Option<String>, // 错误修改建议
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProgramRequest {
    pub name: String,
    pub user_id: Uuid,
    pub description: Option<String>,
    pub source_code: String,
    pub status: ProgramStatus,
    pub metadata: Option<serde_json::Value>,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListProgramResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub status: ProgramStatus,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ProgramExecution {
    pub id: Uuid,
    pub line: i32,
    pub input: Option<String>,
    pub output: Option<String>,
    pub error: Option<String>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListProgramQuery {
    pub search: Option<String>,
    pub page: Option<i64>,
    pub size: Option<i64>,
    pub sort: Option<i8>,
}
