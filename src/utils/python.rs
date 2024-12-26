use pyo3::{prelude::*, types::PyDict};
use serde_json::Value;
use std::fs;
use tempfile::NamedTempFile;

use crate::error::AppError;

pub struct PythonExecutor;

impl PythonExecutor {
    pub fn new() -> Self {
        Self
    }

    pub async fn execute_json_as_python(
        &self,
        json_code: Value,
        input_data: Option<Value>,
    ) -> Result<Value, AppError> {
        // 将 JSON 转换为 Python 代码
        let python_code = self.convert_json_to_python(&json_code)?;

        // 创建临时文件
        let mut temp_file = NamedTempFile::new()?;
        fs::write(temp_file.path(), python_code)?;

        // 执行 Python 代码
        Python::with_gil(|py| {
            let locals = PyDict::new(py);
            
            // 如果有输入数据，添加到 Python 上下文
            if let Some(data) = input_data {
                locals.set_item("input_data", data.to_string())?;
            }

            // 执行代码并捕获输出
            match py.run_path(temp_file.path().to_str().unwrap(), Some(locals), None) {
                Ok(_) => {
                    // 获取输出结果
                    if let Ok(result) = locals.get_item("result") {
                        let result_str = result.to_string();
                        Ok(serde_json::from_str(&result_str)?)
                    } else {
                        Ok(Value::Null)
                    }
                }
                Err(e) => Err(AppError::Internal(format!("Python execution error: {}", e))),
            }
        })
    }

    fn convert_json_to_python(&self, json_code: &Value) -> Result<String, AppError> {
        // 这里实现 JSON 到 Python 代码的转换逻辑
        // 这是一个简单的示例，实际实现需要更复杂的转换逻辑
        Ok(format!(
            "import json\n\
             input_data = json.loads(input_data) if 'input_data' in locals() else {{}}\n\
             {}\n\
             result = json.dumps(locals().get('result', None))",
            json_code.as_str().unwrap_or("")
        ))
    }
} 