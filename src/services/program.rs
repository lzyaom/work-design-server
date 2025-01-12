use crate::{
    error::AppError,
    models::program::{Program, ProgramCompileResponse, ProgramExecutionUpdate, ProgramStatus},
    utils::python::PythonExecutor,
};
use chrono::{DateTime, Utc};
use sqlx::SqlitePool;
use tokio::sync::broadcast;
use uuid::Uuid;

pub struct ProgramService {
    pool: SqlitePool,
    python_executor: PythonExecutor,
    execution_updates: broadcast::Sender<ProgramExecutionUpdate>,
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
        match self.python_executor.compile(&program.content).await {
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
    ) -> Result<broadcast::Receiver<ProgramExecutionUpdate>, AppError> {
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
            let result = executor
                .execute_with_updates(&program.content, |line, output, error| {
                    let update = ProgramExecutionUpdate {
                        program_id,
                        line_number: line as i32,
                        output,
                        error,
                        timestamp: Utc::now(),
                    };
                    let _ = sender.send(update);
                })
                .await;

            if let Err(e) = result {
                let _ = sender.send(ProgramExecutionUpdate {
                    program_id,
                    line_number: -1,
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
    user_id: Uuid,
    name: String,
    content: String,
) -> Result<Program, AppError> {
    let id = Uuid::new_v4();
    let program = sqlx::query_as!(
        Program,
        r#"
        INSERT INTO programs (id, user_id, name, content, status)
        VALUES (?, ?, ?, ?, ?)
        RETURNING id as "id: Uuid", user_id as "user_id: Uuid", name, content, status as "status: String    ",created_at as "created_at: DateTime<Utc>",updated_at as "updated_at: DateTime<Utc>"
        "#,
        id,
        user_id,
        name,
        content,
        "pending"
    )
    .fetch_one(pool)
    .await?;

    Ok(program)
}

pub async fn get_program(pool: &SqlitePool, id: Uuid) -> Result<Program, AppError> {
    let program = sqlx::query_as!(
        Program,
        r#"SELECT 
            id as "id: Uuid",
            user_id as "user_id: Uuid",
            name,
            content,
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
    limit: i64,
    offset: i64,
) -> Result<Vec<Program>, AppError> {
    let programs = sqlx::query_as!(
        Program,
        r#"SELECT 
            id as "id: Uuid",
            user_id as "user_id: Uuid",
            name,
            content,
            status as "status: String",
            created_at as "created_at: DateTime<Utc>",
            updated_at as "updated_at: DateTime<Utc>"
         FROM programs
         ORDER BY created_at DESC
         LIMIT ? OFFSET ?
        "#,
        limit,
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
        r#"UPDATE programs SET name = COALESCE(?, name), content = COALESCE(?, content), status = COALESCE(?, status) WHERE id = ?"#,
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
