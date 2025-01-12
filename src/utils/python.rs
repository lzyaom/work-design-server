use chrono::Utc;

use crate::{
    error::AppError,
    models::program::{ProgramCompileResponse, ProgramStatus},
};

#[derive(Clone)]
pub struct PythonExecutor;

impl PythonExecutor {
    pub fn new() -> Self {
        Self
    }

    pub async fn compile(
        &self,
        _code: &str,
    ) -> Result<ProgramCompileResponse, ProgramCompileResponse> {
        Ok(ProgramCompileResponse {
            status: ProgramStatus::Compiled,
            time: Utc::now(),
            error_file: None,
            error_type: None,
            error_line: None,
            error_message: None,
            error_suggestions: None,
        })
    }

    pub async fn execute_with_updates(
        &self,
        _code: &str,
        _updates: impl Fn(i32, Option<String>, Option<String>),
    ) -> Result<(), AppError> {
        Ok(())
    }
}
