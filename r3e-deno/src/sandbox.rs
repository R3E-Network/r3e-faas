// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::time::Duration;

/// Sandbox configuration for JavaScript runtime
#[derive(Debug, Clone)]
pub struct SandboxConfig {
    /// Initial heap size in bytes
    pub initial_heap_size: usize,
    
    /// Maximum heap size in bytes
    pub max_heap_size: usize,
    
    /// Maximum execution time
    pub max_execution_time: Duration,
    
    /// Enable JIT compilation
    pub enable_jit: bool,
    
    /// Allow network access
    pub allow_net: bool,
    
    /// Allow file system access
    pub allow_fs: bool,
    
    /// Allow environment variables access
    pub allow_env: bool,
    
    /// Allow running subprocesses
    pub allow_run: bool,
    
    /// Allow high-resolution time
    pub allow_hrtime: bool,
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self {
            initial_heap_size: 1 * 1024 * 1024, // 1MB
            max_heap_size: 128 * 1024 * 1024,   // 128MB
            max_execution_time: Duration::from_secs(10),
            enable_jit: false,
            allow_net: false,
            allow_fs: false,
            allow_env: false,
            allow_run: false,
            allow_hrtime: false,
        }
    }
}

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
        // Create a new deno runtime with sandbox configuration
        let mut runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
            module_loader: Some(std::rc::Rc::new(deno_core::FsModuleLoader)),
            startup_snapshot: None,
            v8_platform: None,
            get_error_class_fn: None,
            extensions: vec![],
            extensions_with_js: vec![],
            startup_script: None,
            create_params: None,
            js_error_create_fn: None,
            source_map_getter: None,
            should_break_on_first_statement: false,
            should_wait_for_inspector_session: false,
            heap_limits: deno_core::HeapLimits {
                initial_heap_size_in_bytes: Some(self.config.initial_heap_size as _),
                maximum_heap_size_in_bytes: Some(self.config.max_heap_size as _),
            },
            create_web_worker_cb: None,
            maybe_inspector_server: None,
            should_create_inspector: false,
            stdio: Default::default(),
        });

        // Set up execution timeout
        let start_time = std::time::Instant::now();
        let timeout = tokio::time::sleep(self.config.max_execution_time);
        
        // Execute the code in the sandbox
        let execution = async {
            let result = runtime.execute_script("[sandbox]", code)
                .map_err(|e| format!("Execution error: {}", e))?;
            
            // Get the result value
            let result = runtime.get_value_from_slot(result)
                .map_err(|e| format!("Failed to get result: {}", e))?;
            
            // Convert the result to a string
            let result = result.to_string(&mut runtime)
                .map_err(|e| format!("Failed to convert result: {}", e))?;
            
            Ok::<String, String>(result)
        };

        // Run with timeout
        tokio::select! {
            result = execution => result,
            _ = timeout => Err("Execution time exceeded".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_execute() {
        let config = SandboxConfig::default();
        let executor = SandboxExecutor::new(config);
        
        let result = executor.execute("console.log('Hello, world!');").await;
        assert!(result.is_ok());
    }
}
