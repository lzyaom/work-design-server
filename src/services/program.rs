use crate::{
    error::AppError,
    models::{
        CreateProgramRequest, ListProgramQuery, ListProgramResponse, Program,
        ProgramCompileResponse, ProgramExecution, ProgramStatus,
    },
    utils::python::PythonExecutor,
};
use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::SqlitePool;
use tokio::sync::broadcast;
use uuid::Uuid;

pub struct ProgramService {
    pool: SqlitePool,
    python_executor: PythonExecutor,
    execution_updates: broadcast::Sender<ProgramExecution>,
}

impl ProgramService {
    pub fn new(pool: SqlitePool, python_executor: PythonExecutor) -> Self {
        let (execution_updates, _) = broadcast::channel(100);
        Self {
            pool,
            python_executor,
            execution_updates,
        }
    }
    pub async fn compile_program(&self, id: Uuid) -> Result<ProgramCompileResponse, AppError> {
        let mut program = get_program(&self.pool, id).await?;

        // 更新状态为编译中
        program.status = ProgramStatus::Compiling;
        update_program(&self.pool, id, None, None, Some(program.status.to_string())).await?;

        // 执行编译
        match self.python_executor.compile(&program.source_code).await {
            Ok(_) => {
                program.status = ProgramStatus::Compiled;
                update_program(&self.pool, id, None, None, Some(program.status.to_string()))
                    .await?;

                Ok(ProgramCompileResponse {
                    status: program.status,
                    time: Utc::now(),
                    error_file: None,
                    error_type: None,
                    error_line: None,
                    error_message: None,
                    error_suggestions: None,
                })
            }
            Err(e) => {
                program.status = ProgramStatus::Failed;
                update_program(&self.pool, id, None, None, Some(program.status.to_string()))
                    .await?;

                Ok(ProgramCompileResponse {
                    status: program.status,
                    time: Utc::now(),
                    error_file: e.error_file,
                    error_type: e.error_type,
                    error_line: e.error_line,
                    error_message: e.error_message,
                    error_suggestions: e.error_suggestions,
                })
            }
        }
    }

    pub async fn run_program(
        &self,
        id: Uuid,
    ) -> Result<broadcast::Receiver<ProgramExecution>, AppError> {
        let program = get_program(&self.pool, id).await?;

        if program.status != ProgramStatus::Compiled {
            return Err(AppError::BadRequest(
                "Program must be compiled first".to_string(),
            ));
        }

        // 创建一个新的接收器
        let receiver = self.execution_updates.subscribe();

        // 启动程序执行
        let executor = self.python_executor.clone();
        let sender = self.execution_updates.clone();
        let program_id = program.id;

        tokio::spawn(async move {
            let sender_clone = sender.clone();

            let result = executor
                .execute_with_updates(&program.source_code, move |line, output, error| {
                    let update = ProgramExecution {
                        id: program_id.clone(),
                        line: line as i32,
                        input: None,
                        output: output.clone(),
                        error: error.clone(),
                        timestamp: Utc::now(),
                    };
                    let _ = sender_clone.send(update);
                })
                .await;

            if let Err(e) = result {
                let _ = sender.send(ProgramExecution {
                    id: program_id.clone(),
                    line: -1,
                    input: None,
                    output: None,
                    error: Some(e.to_string()),
                    timestamp: Utc::now(),
                });
            }
        });

        Ok(receiver)
    }
}

pub async fn create_program(
    pool: &SqlitePool,
    program: CreateProgramRequest,
) -> Result<(), AppError> {
    let status = program.status.to_string();
    sqlx::query_as!(
        CreateProgramRequest,
        r#"
        INSERT INTO programs (user_id, name, description, source_code, is_active, status, metadata)
        VALUES (?, ?, ?, ?, ?, ?, ?)
        RETURNING user_id as "user_id: Uuid", name, description, source_code, status as "status: String", is_active, metadata as "metadata: Value"
        "#,
        program.user_id,
        program.name,
        program.description,
        program.source_code,
        program.is_active,
        status,
        program.metadata
    )
    .fetch_one(pool)
    .await?;

    Ok(())
}

pub async fn get_program(pool: &SqlitePool, id: Uuid) -> Result<Program, AppError> {
    let program = sqlx::query_as!(
        Program,
        r#"SELECT 
            id as "id: Uuid",
            user_id as "user_id: Uuid",
            name,
            description,
            source_code,
            compiled_code,
            is_active,
            metadata as "metadata: Value",
            status as "status: String",
            created_at as "created_at: DateTime<Utc>",
            updated_at as "updated_at: DateTime<Utc>"
        FROM programs WHERE id = ?"#,
        id
    )
    .fetch_one(pool)
    .await?;
    Ok(program)
}

pub async fn list_programs(
    pool: &SqlitePool,
    query: ListProgramQuery,
) -> Result<Vec<ListProgramResponse>, AppError> {
    let page_size = query.size.unwrap_or(10);
    let page = query.page.unwrap_or(1);
    let offset = page_size * (page - 1);
    let programs = sqlx::query_as!(
        ListProgramResponse,
        r#"SELECT 
            id as "id: Uuid",
            name,
            description,
            is_active,
            status as "status: String",
            created_at as "created_at: DateTime<Utc>",
            updated_at as "updated_at: DateTime<Utc>"
         FROM programs
         ORDER BY created_at DESC
         LIMIT ? OFFSET ?
        "#,
        page_size,
        offset
    )
    .fetch_all(pool)
    .await?;
    Ok(programs)
}

pub async fn update_program(
    pool: &SqlitePool,
    id: Uuid,
    name: Option<String>,
    content: Option<String>,
    status: Option<String>,
) -> Result<(), AppError> {
    sqlx::query!(
        r#"UPDATE programs SET name = COALESCE(?, name), source_code = COALESCE(?, source_code), status = COALESCE(?, status) WHERE id = ?"#,
        name,
        content,
        status,
        id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn delete_program(pool: &SqlitePool, id: Uuid) -> Result<(), AppError> {
    let result = sqlx::query!(r#"DELETE FROM programs WHERE id = ?"#, id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Program not found".to_string()));
    }

    Ok(())
}
