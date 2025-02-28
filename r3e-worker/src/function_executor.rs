// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::sync::Arc;
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::sandbox_executor::SandboxExecutor;
use crate::sandbox::SandboxConfig;

/// Function execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionExecutionResult {
    /// Function ID
    pub function_id: Uuid,
    
    /// Execution ID
    pub execution_id: Uuid,
    
    /// Result
    pub result: Value,
    
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
    
    /// Status
    pub status: String,
    
    /// Error message
    pub error: Option<String>,
    
    /// Logs
    pub logs: Vec<String>,
}

/// Function execution context
#[derive(Debug)]
pub struct FunctionExecutionContext {
    /// Function ID
    pub function_id: Uuid,
    
    /// User ID
    pub user_id: u64,
    
    /// Function code
    pub code: String,
    
    /// Input
    pub input: Value,
    
    /// Sandbox configuration
    pub sandbox_config: SandboxConfig,
    
    /// Execution start time
    pub start_time: Instant,
    
    /// Execution ID
    pub execution_id: Uuid,
    
    /// Logs
    pub logs: Arc<RwLock<Vec<String>>>,
}

impl FunctionExecutionContext {
    /// Create a new function execution context
    pub fn new(
        function_id: Uuid,
        user_id: u64,
        code: String,
        input: Value,
        sandbox_config: SandboxConfig,
    ) -> Self {
        Self {
            function_id,
            user_id,
            code,
            input,
            sandbox_config,
            start_time: Instant::now(),
            execution_id: Uuid::new_v4(),
            logs: Arc::new(RwLock::new(Vec::new())),
        }
    }
    
    /// Add a log message
    pub async fn add_log(&self, message: &str) {
        let mut logs = self.logs.write().await;
        logs.push(message.to_string());
    }
    
    /// Get execution time in milliseconds
    pub fn execution_time_ms(&self) -> u64 {
        self.start_time.elapsed().as_millis() as u64
    }
    
    /// Create a successful execution result
    pub async fn success(&self, result: Value) -> FunctionExecutionResult {
        FunctionExecutionResult {
            function_id: self.function_id,
            execution_id: self.execution_id,
            result,
            execution_time_ms: self.execution_time_ms(),
            status: "success".to_string(),
            error: None,
            logs: self.logs.read().await.clone(),
        }
    }
    
    /// Create a failed execution result
    pub async fn failure(&self, error: String) -> FunctionExecutionResult {
        FunctionExecutionResult {
            function_id: self.function_id,
            execution_id: self.execution_id,
            result: Value::Null,
            execution_time_ms: self.execution_time_ms(),
            status: "error".to_string(),
            error: Some(error),
            logs: self.logs.read().await.clone(),
        }
    }
}

/// Function executor
pub struct FunctionExecutor {
    /// Sandbox configuration
    sandbox_config: SandboxConfig,
}

impl FunctionExecutor {
    /// Create a new function executor
    pub fn new(sandbox_config: SandboxConfig) -> Self {
        Self { sandbox_config }
    }
    
    /// Execute a function
    pub async fn execute(
        &self,
        function_id: Uuid,
        user_id: u64,
        code: String,
        input: Value,
    ) -> FunctionExecutionResult {
        // Create execution context
        let context = FunctionExecutionContext::new(
            function_id,
            user_id,
            code,
            input.clone(),
            self.sandbox_config.clone(),
        );
        
        // Log execution start
        context.add_log(&format!(
            "Executing function {} for user {} with input: {}",
            function_id, user_id, input
        )).await;
        
        // Create sandbox executor
        let sandbox_executor = SandboxExecutor::new(self.sandbox_config.clone());
        
        // Prepare the code with input
        let code_with_input = format!(
            r#"
            // Function code
            {}
            
            // Execute the function with input
            const input = {};
            
            // Get the default export
            const fn = (() => {{
                if (typeof exports.default === 'function') {{
                    return exports.default;
                }}
                
                // Find the first exported function
                for (const key in exports) {{
                    if (typeof exports[key] === 'function') {{
                        return exports[key];
                    }}
                }}
                
                throw new Error('No exported function found');
            }})();
            
            // Execute the function
            const result = fn(input);
            
            // Return the result
            JSON.stringify(result);
            "#,
            context.code,
            serde_json::to_string(&context.input).unwrap_or_else(|_| "null".to_string())
        );
        
        // Execute the code
        match sandbox_executor.execute(&code_with_input).await {
            Ok(result) => {
                // Log execution success
                context.add_log(&format!(
                    "Function executed successfully in {}ms with result: {}",
                    context.execution_time_ms(), result
                )).await;
                
                // Parse the result
                match serde_json::from_str::<Value>(&result) {
                    Ok(parsed_result) => {
                        context.success(parsed_result).await
                    },
                    Err(e) => {
                        // Log parsing error
                        context.add_log(&format!(
                            "Failed to parse result: {}", e
                        )).await;
                        
                        context.failure(format!("Failed to parse result: {}", e)).await
                    }
                }
            },
            Err(e) => {
                // Log execution error
                context.add_log(&format!(
                    "Function execution failed: {}", e
                )).await;
                
                context.failure(format!("Function execution failed: {}", e)).await
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_execute_success() {
        let config = SandboxConfig::default();
        let executor = FunctionExecutor::new(config);
        
        let code = r#"
        export default function(input) {
            return { message: "Hello, " + input.name };
        }
        "#;
        
        let input = serde_json::json!({
            "name": "World"
        });
        
        let result = executor.execute(
            Uuid::new_v4(),
            1,
            code.to_string(),
            input,
        ).await;
        
        assert_eq!(result.status, "success");
        assert!(result.error.is_none());
        
        let expected = serde_json::json!({
            "message": "Hello, World"
        });
        
        assert_eq!(result.result, expected);
    }
    
    #[tokio::test]
    async fn test_execute_error() {
        let config = SandboxConfig::default();
        let executor = FunctionExecutor::new(config);
        
        let code = r#"
        export default function(input) {
            throw new Error("Test error");
        }
        "#;
        
        let input = serde_json::json!({
            "name": "World"
        });
        
        let result = executor.execute(
            Uuid::new_v4(),
            1,
            code.to_string(),
            input,
        ).await;
        
        assert_eq!(result.status, "error");
        assert!(result.error.is_some());
    }
}
