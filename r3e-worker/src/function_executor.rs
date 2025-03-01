// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::sync::Arc;
use std::time::{Duration, Instant};

use r3e_core::error::{Error, Result};
use r3e_core::logging;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::RwLock;
use tracing::{info, warn, error, instrument};
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
    
    /// Preloaded function code
    preloaded_function: Option<Arc<FunctionDeployment>>,
}

impl FunctionExecutor {
    /// Create a new function executor
    pub fn new(sandbox_config: SandboxConfig) -> Self {
        Self { 
            sandbox_config,
            preloaded_function: None,
        }
    }
    
    /// Preload a function
    #[instrument(skip(self, function))]
    pub async fn preload_function(&mut self, function: &FunctionDeployment) -> Result<()> {
        info!(
            function_id = %function.id,
            version = %function.version,
            "Preloading function"
        );
        
        // Create sandbox executor
        let sandbox_executor = SandboxExecutor::new(self.sandbox_config.clone());
        
        // Prepare the code for preloading
        let preload_code = format!(
            r#"
            // Function code
            {}
            
            // Verify the code can be parsed
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
            
            // Return success
            "preloaded"
            "#,
            function.code
        );
        
        // Execute the preload code
        match sandbox_executor.execute(&preload_code).await {
            Ok(_) => {
                // Store the preloaded function
                self.preloaded_function = Some(Arc::new(function.clone()));
                
                info!(
                    function_id = %function.id,
                    version = %function.version,
                    "Function preloaded successfully"
                );
                
                Ok(())
            },
            Err(e) => {
                error!(
                    function_id = %function.id,
                    version = %function.version,
                    error = %e,
                    "Failed to preload function"
                );
                
                Err(Error::Execution(format!("Failed to preload function: {}", e)))
            }
        }
    }
    
    /// Execute a function
    #[instrument(
        name = "function_execution",
        skip(self, code, input, metrics_manager),
        fields(
            function_id = %function_id,
            user_id = %user_id,
            execution_id = tracing::field::Empty
        )
    )]
    pub async fn execute(
        &self,
        function_id: Uuid,
        user_id: u64,
        code: String,
        input: Value,
        metrics_manager: Option<&crate::metrics::MetricsManager>,
    ) -> Result<FunctionExecutionResult> {
        // Check if we have a preloaded function
        let actual_code = if let Some(preloaded) = &self.preloaded_function {
            if preloaded.id == function_id.to_string() {
                info!(
                    function_id = %function_id,
                    version = %preloaded.version,
                    "Using preloaded function"
                );
                preloaded.code.clone()
            } else {
                code
            }
        } else {
            code
        };
        
        // Create execution context
        let context = FunctionExecutionContext::new(
            function_id,
            user_id,
            actual_code,
            input.clone(),
            self.sandbox_config.clone(),
        );
        
        // Record execution ID in the span
        tracing::Span::current().record("execution_id", &context.execution_id.to_string());
        
        // Log execution start
        info!(
            input_size = input.to_string().len(),
            code_size = context.code.len(),
            "Starting function execution"
        );
        
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
                let execution_time = context.execution_time_ms();
                let memory_usage = sandbox_executor.get_memory_usage_mb();
                
                info!(
                    execution_time_ms = execution_time,
                    memory_usage_mb = memory_usage,
                    result_size = result.len(),
                    "Function executed successfully"
                );
                
                context.add_log(&format!(
                    "Function executed successfully in {}ms with result: {}",
                    execution_time, result
                )).await;
                
                // Record metrics if metrics manager is provided
                if let Some(metrics) = metrics_manager {
                    metrics.record_function_execution(
                        &function_id.to_string(),
                        &user_id.to_string(),
                        execution_time,
                        memory_usage,
                        true,
                    );
                }
                
                // Parse the result
                match serde_json::from_str::<Value>(&result) {
                    Ok(parsed_result) => {
                        Ok(context.success(parsed_result).await)
                    },
                    Err(e) => {
                        // Log parsing error
                        error!(
                            error = %e,
                            result_preview = %truncate_string(&result, 100),
                            "Failed to parse function result"
                        );
                        
                        context.add_log(&format!(
                            "Failed to parse result: {}", e
                        )).await;
                        
                        // Record error metrics if metrics manager is provided
                        if let Some(metrics) = metrics_manager {
                            metrics.record_function_execution(
                                &function_id.to_string(),
                                &user_id.to_string(),
                                execution_time,
                                memory_usage,
                                false,
                            );
                        }
                        
                        Err(Error::Serialization(e))
                    }
                }
            },
            Err(e) => {
                // Log execution error
                let execution_time = context.execution_time_ms();
                let memory_usage = sandbox_executor.get_memory_usage_mb();
                
                error!(
                    error = %e,
                    execution_time_ms = execution_time,
                    memory_usage_mb = memory_usage,
                    "Function execution failed"
                );
                
                context.add_log(&format!(
                    "Function execution failed: {}", e
                )).await;
                
                // Record error metrics if metrics manager is provided
                if let Some(metrics) = metrics_manager {
                    metrics.record_function_execution(
                        &function_id.to_string(),
                        &user_id.to_string(),
                        execution_time,
                        memory_usage,
                        false,
                    );
                }
                
                Err(Error::Execution(format!("Function execution failed: {}", e)))
            }
        }
    }
}

/// Truncate a string to a maximum length
fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[0..max_len])
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
        ).await.expect("Function execution should succeed");
        
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
        
        assert!(result.is_err(), "Function execution should fail");
        
        if let Ok(result) = result {
            panic!("Expected error, got success: {:?}", result);
        }
    }
    
    #[test]
    fn test_truncate_string() {
        assert_eq!(truncate_string("hello", 10), "hello");
        assert_eq!(truncate_string("hello world", 5), "hello...");
        assert_eq!(truncate_string("", 5), "");
    }
}
