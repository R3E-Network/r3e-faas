// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::sync::Arc;
use tracing::{debug, error, info, warn};

use crate::function::{FunctionContext, FunctionResult};
use crate::sandbox::{SandboxConfig, SandboxExecutor};

/// Function executor configuration
#[derive(Clone)]
pub struct ExecutorConfig {
    /// Sandbox configuration
    pub sandbox_config: SandboxConfig,
    
    /// Maximum execution time in milliseconds
    pub max_execution_time_ms: u64,
    
    /// Maximum memory usage in bytes
    pub max_memory_bytes: u64,
}

impl Default for ExecutorConfig {
    fn default() -> Self {
        Self {
            sandbox_config: SandboxConfig::default(),
            max_execution_time_ms: 5000, // 5 seconds
            max_memory_bytes: 128 * 1024 * 1024, // 128 MB
        }
    }
}

/// Function executor
pub struct FunctionExecutor {
    /// Executor configuration
    config: ExecutorConfig,
    
    /// Sandbox executor
    sandbox: SandboxExecutor,
}

impl FunctionExecutor {
    /// Create a new function executor
    pub fn new(config: ExecutorConfig) -> Self {
        let sandbox = SandboxExecutor::new(config.sandbox_config.clone());
        
        Self {
            config,
            sandbox,
        }
    }
    
    /// Execute a function
    pub fn execute(&self, function_id: &str, code: &str, context: &FunctionContext) -> FunctionResult {
        debug!("Executing function {}", function_id);
        
        // Set up execution context
        let execution_context = self.sandbox.create_context(function_id, context);
        
        // Execute the function
        match self.sandbox.execute(code, execution_context) {
            Ok(result) => {
                debug!("Function {} executed successfully", function_id);
                FunctionResult::Success(result)
            }
            Err(err) => {
                error!("Function {} execution failed: {}", function_id, err);
                FunctionResult::Error(format!("Execution failed: {}", err))
            }
        }
    }
}
