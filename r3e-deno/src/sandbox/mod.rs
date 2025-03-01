// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use deno_core::v8;
use std::time::Duration;

mod threat_monitor;
pub use threat_monitor::ThreatMonitor;

use crate::security::threat_detection::{ThreatDetectionService, ThreatDetectionConfig};

/// Sandbox configuration for JavaScript runtime
#[derive(Debug, Clone)]
pub struct SandboxConfig {
    /// Initial heap size in bytes
    pub initial_heap_size: usize,
    
    /// Maximum heap size in bytes
    pub max_heap_size: usize,
    
    /// Maximum execution time
    pub max_execution_time: Duration,
    
    /// Enable JIT compilation (false for more security)
    pub enable_jit: bool,
    
    /// Allow network access
    pub allow_net: bool,
    
    /// Allow file system access
    pub allow_fs: bool,
    
    /// Allow environment variables access
    pub allow_env: bool,
    
    /// Allow process spawning
    pub allow_run: bool,
    
    /// Allow high resolution time
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

/// Create V8 flags based on sandbox configuration
pub fn create_v8_flags(config: &SandboxConfig) -> String {
    let mut flags = Vec::new();
    
    // Disable JIT for security
    if !config.enable_jit {
        flags.push("--jitless");
    }
    
    // Set heap limits
    flags.push("--expose-gc");
    
    // Disable WebAssembly for security
    flags.push("--no-wasm");
    
    // Disable shared array buffer for security
    flags.push("--no-harmony-sharedarraybuffer");
    
    // Disable web snapshots for security
    flags.push("--no-web-snapshot");
    
    // Disable eval for security
    flags.push("--disallow-code-generation-from-strings");
    
    flags.join(" ")
}

/// Create V8 parameters based on sandbox configuration
pub fn create_v8_params(config: &SandboxConfig) -> v8::CreateParams {
    v8::CreateParams::default().heap_limits(config.initial_heap_size, config.max_heap_size)
}

/// Sandbox execution context
pub struct SandboxContext {
    /// Execution timeout handle
    timeout_handle: Option<std::thread::JoinHandle<()>>,
    
    /// Sandbox configuration
    config: SandboxConfig,
}

impl SandboxContext {
    /// Create a new sandbox context
    pub fn new(config: SandboxConfig, isolate: &mut v8::Isolate) -> Self {
        // Set up timeout
        let timeout_handle = if config.max_execution_time.as_millis() > 0 {
            let duration = config.max_execution_time;
            let isolate_ptr = isolate as *mut v8::Isolate;
            
            let handle = std::thread::spawn(move || {
                std::thread::sleep(duration);
                unsafe {
                    // This is safe because we're only terminating execution, not accessing data
                    (*isolate_ptr).terminate_execution();
                }
            });
            
            Some(handle)
        } else {
            None
        };
        
        Self {
            timeout_handle,
            config,
        }
    }
}

impl Drop for SandboxContext {
    fn drop(&mut self) {
        // Clean up timeout thread if it exists
        if let Some(handle) = self.timeout_handle.take() {
            // We don't care about the result, just want to make sure it's cleaned up
            let _ = handle.join();
        }
    }
}

/// Permission checker for sandbox operations
pub fn check_permission(
    operation: &str,
    config: &SandboxConfig,
) -> Result<(), String> {
    match operation {
        "net" if !config.allow_net => Err("Network access is not allowed in this sandbox".to_string()),
        "fs" if !config.allow_fs => Err("File system access is not allowed in this sandbox".to_string()),
        "env" if !config.allow_env => Err("Environment access is not allowed in this sandbox".to_string()),
        "run" if !config.allow_run => Err("Process spawning is not allowed in this sandbox".to_string()),
        "hrtime" if !config.allow_hrtime => Err("High resolution time is not allowed in this sandbox".to_string()),
        _ => Ok(()),
    }
}
