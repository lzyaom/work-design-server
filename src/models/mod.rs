pub mod document;
pub mod log;
pub mod message;
pub mod program;
pub mod task;
pub mod user;

use serde::{Deserialize, Serialize};

pub use self::document::{
    CreateDocumentRequest, Document, DocumentPermission, DocumentResponse, DocumentType,
    DocumentUpdateMessage, PermissionType, UpdateDocumentRequest, UpdatePermissionRequest,
};
pub use self::log::{ListLogsQuery, Log, LogLevel};
pub use self::program::{
    CreateProgramRequest, ListProgramQuery, ListProgramResponse, Program, ProgramCompileResponse,
    ProgramExecution, ProgramStatus, UpdateProgram,
};
pub use self::task::{
    CreateTaskRequest, ListTasksQuery, ScheduledTask, TaskAuditLog, TaskDependency, TaskExecution,
    TaskPriority, TaskStatus, TaskType, UpdateTaskRequest,
};
pub use self::user::{
    CreateUserRequest, ListUsersQuery, UpdateUserPasswordRequest, UpdateUserRequest, User,
    UserRole, VerificationCode,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseResult<T> {
    pub code: i32,
    pub message: Option<String>,
    pub result: Option<T>,
}
