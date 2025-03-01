// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::sync::Arc;
use std::time::{Duration, Instant};
use tracing::{debug, error, info, warn};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::container::{ContainerConfig, ContainerError, ContainerManager, NetworkMode};
use crate::function::{FunctionContext, FunctionResult};
use crate::sandbox::{SandboxConfig, SandboxExecutor};
use crate::sandbox_executor::SandboxExecutor;

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

/// Executor configuration
#[derive(Debug, Clone)]
pub struct ExecutorConfig {
    /// Sandbox configuration
    pub sandbox_config: SandboxConfig,

    /// Use container-based isolation
    pub use_container_isolation: bool,

    /// Container configuration
    pub container_config: Option<ContainerConfig>,

    /// Maximum execution time in milliseconds
    pub max_execution_time_ms: u64,

    /// Maximum memory usage in bytes
    pub max_memory_bytes: u64,
}

impl Default for ExecutorConfig {
    fn default() -> Self {
        Self {
            sandbox_config: SandboxConfig::default(),
            use_container_isolation: false,
            container_config: None,
            max_execution_time_ms: 5000,         // 5 seconds
            max_memory_bytes: 128 * 1024 * 1024, // 128 MB
        }
    }
}

/// Function executor
pub struct FunctionExecutor {
    /// Executor configuration
    config: ExecutorConfig,

    /// Sandbox executor
    sandbox: Option<SandboxExecutor>,

    /// Container manager for isolation
    container_manager: Option<ContainerManager>,
}

impl FunctionExecutor {
    /// Create a new function executor
    pub fn new(config: ExecutorConfig) -> Self {
        // Initialize sandbox executor
        let sandbox = Some(SandboxExecutor::new(config.sandbox_config.clone()));

        // Initialize container manager if enabled
        let container_manager = if config.use_container_isolation {
            let container_config = config.container_config.unwrap_or_default();
            Some(ContainerManager::new(container_config))
        } else {
            None
        };

        Self {
            config,
            sandbox,
            container_manager,
        }
    }

    /// Execute a function with simple interface
    pub fn execute_simple(
        &self,
        function_id: &str,
        code: &str,
        context: &FunctionContext,
    ) -> FunctionResult {
        debug!("Executing function {}", function_id);

        // Set up execution context
        if let Some(sandbox) = &self.sandbox {
            let execution_context = sandbox.create_context(function_id, context);

            // Execute the function
            match sandbox.execute(code, execution_context) {
                Ok(result) => {
                    debug!("Function {} executed successfully", function_id);
                    FunctionResult::Success(result)
                }
                Err(err) => {
                    error!("Function {} execution failed: {}", function_id, err);
                    FunctionResult::Error(format!("Execution failed: {}", err))
                }
            }
        } else {
            error!("No sandbox executor available");
            FunctionResult::Error("No sandbox executor available".to_string())
        }
    }

    /// Execute a function with full context
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
            code.clone(),
            input.clone(),
            self.config.sandbox_config.clone(),
        );

        // Log execution start
        context
            .add_log(&format!(
                "Executing function {} for user {} with input: {}",
                function_id, user_id, input
            ))
            .await;

        // Check if container-based isolation is enabled
        if let Some(container_manager) = &self.container_manager {
            // Prepare the code with input for container execution
            let container_code = format!(
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
                
                // Print the result for container output
                console.log(JSON.stringify(result));
                "#,
                code,
                serde_json::to_string(&input).unwrap_or_else(|_| "null".to_string())
            );

            // Execute the function in a container
            match container_manager.run_function(&function_id.to_string(), &container_code) {
                Ok(output) => {
                    // Log execution success
                    context
                        .add_log(&format!(
                            "Function executed successfully in container in {}ms with output: {}",
                            context.execution_time_ms(),
                            output
                        ))
                        .await;

                    // Parse the output as JSON
                    match serde_json::from_str::<Value>(&output) {
                        Ok(parsed_result) => context.success(parsed_result).await,
                        Err(e) => {
                            // Log parsing error
                            context
                                .add_log(&format!("Failed to parse container output: {}", e))
                                .await;

                            context
                                .failure(format!("Failed to parse container output: {}", e))
                                .await
                        }
                    }
                }
                Err(e) => {
                    // Log execution error
                    context
                        .add_log(&format!("Container execution failed: {}", e))
                        .await;

                    context
                        .failure(format!("Container execution failed: {}", e))
                        .await
                }
            }
        } else {
            // Create sandbox executor for V8 isolation
            let sandbox_executor = SandboxExecutor::new(self.config.sandbox_config.clone());

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

            // Execute the code in the V8 sandbox
            match sandbox_executor.execute(&code_with_input).await {
                Ok(result) => {
                    // Log execution success
                    context
                        .add_log(&format!(
                            "Function executed successfully in V8 sandbox in {}ms with result: {}",
                            context.execution_time_ms(),
                            result
                        ))
                        .await;

                    // Parse the result
                    match serde_json::from_str::<Value>(&result) {
                        Ok(parsed_result) => context.success(parsed_result).await,
                        Err(e) => {
                            // Log parsing error
                            context
                                .add_log(&format!("Failed to parse result: {}", e))
                                .await;

                            context
                                .failure(format!("Failed to parse result: {}", e))
                                .await
                        }
                    }
                }
                Err(e) => {
                    // Log execution error
                    context
                        .add_log(&format!("Function execution failed: {}", e))
                        .await;

                    context
                        .failure(format!("Function execution failed: {}", e))
                        .await
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_execute_success() {
        let config = ExecutorConfig {
            sandbox_config: SandboxConfig::default(),
            use_container_isolation: false,
            container_config: None,
            max_execution_time_ms: 5000,
            max_memory_bytes: 128 * 1024 * 1024,
        };
        let executor = FunctionExecutor::new(config);

        let code = r#"
        export default function(input) {
            return { message: "Hello, " + input.name };
        }
        "#;

        let input = serde_json::json!({
            "name": "World"
        });

        let result = executor
            .execute(Uuid::new_v4(), 1, code.to_string(), input)
            .await;

        assert_eq!(result.status, "success");
        assert!(result.error.is_none());

        let expected = serde_json::json!({
            "message": "Hello, World"
        });

        assert_eq!(result.result, expected);
    }

    #[tokio::test]
    async fn test_execute_error() {
        let config = ExecutorConfig {
            sandbox_config: SandboxConfig::default(),
            use_container_isolation: false,
            container_config: None,
            max_execution_time_ms: 5000,
            max_memory_bytes: 128 * 1024 * 1024,
        };
        let executor = FunctionExecutor::new(config);

        let code = r#"
        export default function(input) {
            throw new Error("Test error");
        }
        "#;

        let input = serde_json::json!({
            "name": "World"
        });

        let result = executor
            .execute(Uuid::new_v4(), 1, code.to_string(), input)
            .await;

        assert_eq!(result.status, "error");
        assert!(result.error.is_some());
    }

    #[tokio::test]
    #[ignore] // Requires Docker to be installed and running
    async fn test_container_execution() {
        let container_config = ContainerConfig {
            base_image: "node:18-alpine".to_string(),
            memory_limit: 256 * 1024 * 1024, // 256MB
            cpu_limit: 0.5,                  // Half a core
            network_mode: NetworkMode::None,
            max_execution_time: Duration::from_secs(10),
            allow_fs: false,
            env_vars: Vec::new(),
        };

        let config = ExecutorConfig {
            sandbox_config: SandboxConfig::default(),
            use_container_isolation: true,
            container_config: Some(container_config),
            max_execution_time_ms: 5000,
            max_memory_bytes: 128 * 1024 * 1024,
        };

        let executor = FunctionExecutor::new(config);

        let code = r#"
        export default function(input) {
            return { message: "Hello from container, " + input.name };
        }
        "#;

        let input = serde_json::json!({
            "name": "World"
        });

        let result = executor
            .execute(Uuid::new_v4(), 1, code.to_string(), input)
            .await;

        assert_eq!(result.status, "success");
        assert!(result.error.is_none());

        let expected = serde_json::json!({
            "message": "Hello from container, World"
        });

        assert_eq!(result.result, expected);
    }
}
