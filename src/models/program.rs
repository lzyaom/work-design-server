use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
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
    pub id: Uuid,                          // 主键
    pub user_id: Uuid,                     // 用户ID
    pub name: String,                      // 名称
    pub content: String,                   // 代码内容
    pub status: ProgramStatus,             // 运行状态
    pub created_at: Option<DateTime<Utc>>, // 创建时间
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
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

#[derive(Debug, Deserialize)]
pub struct ProgramRequest {
    pub name: String,
    pub content: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ProgramResponse {
    pub code: i32,
    pub message: Option<String>,
    pub result: Option<Program>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ProgramExecutionUpdate {
    pub program_id: Uuid,
    pub line_number: i32,
    pub output: Option<String>,
    pub error: Option<String>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct ListProgramsQuery {
    pub limit: i64,
    pub offset: i64,
}
