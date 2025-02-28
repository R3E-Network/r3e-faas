// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::sync::Arc;
use std::time::Duration;

use deno_core::{error::AnyError, op, Extension, JsRuntime, RuntimeOptions};
use deno_core::v8::{self, Isolate, IsolateCreateParams};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

use crate::sandbox::SandboxConfig;

/// Sandbox executor for JavaScript runtime
pub struct SandboxExecutor {
    /// Sandbox configuration
    config: SandboxConfig,
}

impl SandboxExecutor {
    /// Create a new sandbox executor
    pub fn new(config: SandboxConfig) -> Self {
        Self { config }
    }
    
    /// Get the sandbox configuration
    pub fn config(&self) -> &SandboxConfig {
        &self.config
    }
    
    /// Execute JavaScript code in the sandbox
    pub async fn execute(&self, code: &str) -> Result<String, String> {
        // Create a channel for communication between the sandbox and the executor
        let (tx, mut rx) = mpsc::channel::<Result<String, String>>(1);
        
        // Clone the code and configuration for the sandbox task
        let code = code.to_string();
        let config = self.config.clone();
        
        // Spawn a task to execute the code in a sandbox
        let handle = tokio::spawn(async move {
            // Create a new runtime with the sandbox configuration
            let mut runtime = match Self::create_runtime(&config) {
                Ok(runtime) => runtime,
                Err(e) => {
                    let _ = tx.send(Err(format!("Failed to create runtime: {}", e))).await;
                    return;
                }
            };
            
            // Execute the code
            let result = runtime.execute_script("<anon>", &code);
            
            // Send the result back to the executor
            match result {
                Ok(global) => {
                    // Get the result from the global scope
                    let scope = &mut runtime.handle_scope();
                    let local = v8::Local::new(scope, global);
                    let result = local.to_string(scope).unwrap();
                    let result_str = result.to_rust_string_lossy(scope);
                    
                    let _ = tx.send(Ok(result_str)).await;
                },
                Err(e) => {
                    let _ = tx.send(Err(format!("Execution error: {}", e))).await;
                }
            }
        });
        
        // Wait for the result with a timeout
        let timeout = tokio::time::timeout(self.config.max_execution_time, rx.recv()).await;
        
        // Handle the result
        match timeout {
            Ok(Some(result)) => {
                // Abort the task if it's still running
                handle.abort();
                result
            },
            Ok(None) => {
                // Channel closed without a result
                handle.abort();
                Err("Execution failed: channel closed".to_string())
            },
            Err(_) => {
                // Timeout
                handle.abort();
                Err(format!("Execution timed out after {:?}", self.config.max_execution_time))
            }
        }
    }
    
    /// Create a new JavaScript runtime with the sandbox configuration
    fn create_runtime(config: &SandboxConfig) -> Result<JsRuntime, AnyError> {
        // Create isolate parameters
        let mut params = IsolateCreateParams::default();
        
        // Set memory limits
        params.set_memory_limits(
            config.initial_heap_size,
            config.max_heap_size,
        );
        
        // Create extensions
        let extensions = Self::create_extensions(config);
        
        // Create runtime options
        let options = RuntimeOptions {
            v8_platform: None,
            isolate_create_params: Some(params),
            extensions,
            ..Default::default()
        };
        
        // Create the runtime
        let runtime = JsRuntime::new(options);
        
        Ok(runtime)
    }
    
    /// Create extensions for the JavaScript runtime
    fn create_extensions(config: &SandboxConfig) -> Vec<Extension> {
        let mut extensions = Vec::new();
        
        // Add console extension
        extensions.push(Self::create_console_extension());
        
        // Add timer extension if allowed
        if config.allow_hrtime {
            extensions.push(Self::create_timer_extension());
        }
        
        extensions
    }
    
    /// Create console extension
    fn create_console_extension() -> Extension {
        Extension::builder("console")
            .ops(vec![
                op_console_log::decl(),
                op_console_error::decl(),
                op_console_warn::decl(),
                op_console_info::decl(),
            ])
            .build()
    }
    
    /// Create timer extension
    fn create_timer_extension() -> Extension {
        Extension::builder("timer")
            .ops(vec![
                op_set_timeout::decl(),
                op_clear_timeout::decl(),
            ])
            .build()
    }
}

/// Console log operation
#[op]
fn op_console_log(message: String) -> Result<(), AnyError> {
    println!("[console.log] {}", message);
    Ok(())
}

/// Console error operation
#[op]
fn op_console_error(message: String) -> Result<(), AnyError> {
    eprintln!("[console.error] {}", message);
    Ok(())
}

/// Console warn operation
#[op]
fn op_console_warn(message: String) -> Result<(), AnyError> {
    println!("[console.warn] {}", message);
    Ok(())
}

/// Console info operation
#[op]
fn op_console_info(message: String) -> Result<(), AnyError> {
    println!("[console.info] {}", message);
    Ok(())
}

/// Set timeout operation
#[op]
async fn op_set_timeout(delay_ms: u64) -> Result<(), AnyError> {
    tokio::time::sleep(Duration::from_millis(delay_ms)).await;
    Ok(())
}

/// Clear timeout operation
#[op]
fn op_clear_timeout(_timeout_id: u32) -> Result<(), AnyError> {
    // Cancel the timeout using the runtime's timeout handler
    let runtime = deno_core::JsRuntime::current();
    if let Some(handler) = runtime.timeout_handler() {
        handler.cancel_timeout(_timeout_id);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_execute() {
        let config = SandboxConfig::default();
        let executor = SandboxExecutor::new(config);
        
        let result = executor.execute("console.log('Hello, world!'); 'test result'").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test result");
    }
    
    #[tokio::test]
    async fn test_execute_timeout() {
        let mut config = SandboxConfig::default();
        config.max_execution_time = Duration::from_millis(100);
        let executor = SandboxExecutor::new(config);
        
        let result = executor.execute("while(true) {}").await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("timed out"));
    }
}
