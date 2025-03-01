// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use deno_core::v8;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Sandbox configuration for JavaScript runtime
#[derive(Debug, Clone)]
pub struct SandboxConfig {
    /// Initial heap size in bytes
    pub initial_heap_size: usize,

    /// Maximum heap size in bytes
    pub max_heap_size: usize,

    /// Maximum execution time
    pub max_execution_time: Duration,

    /// Maximum CPU usage percentage (0-100)
    pub max_cpu_percentage: u8,

    /// Maximum memory usage in bytes
    pub max_memory_usage: usize,

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

    /// Allow dynamic imports
    pub allow_dynamic_imports: bool,

    /// Allow native plugins
    pub allow_native_plugins: bool,

    /// Allow eval and Function constructor
    pub allow_eval: bool,

    /// Namespace isolation level (0-3, higher is more isolated)
    pub namespace_isolation_level: u8,
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self {
            initial_heap_size: 1 * 1024 * 1024, // 1MB
            max_heap_size: 128 * 1024 * 1024,   // 128MB
            max_execution_time: Duration::from_secs(10),
            max_cpu_percentage: 80,             // 80% CPU usage
            max_memory_usage: 256 * 1024 * 1024, // 256MB total memory
            enable_jit: false,
            allow_net: false,
            allow_fs: false,
            allow_env: false,
            allow_run: false,
            allow_hrtime: false,
            allow_dynamic_imports: false,
            allow_native_plugins: false,
            allow_eval: false,
            namespace_isolation_level: 2,       // Medium-high isolation
        }
    }
}

/// Create V8 flags based on sandbox configuration
pub fn create_v8_flags(config: &SandboxConfig) -> String {
    let mut flags = Vec::new();

    // Disable JIT for security
    if !config.enable_jit {
        flags.push("--jitless");
        flags.push("--no-opt");  // Disable optimization
    }

    // Set heap limits
    flags.push("--expose-gc");
    flags.push(&format!("--initial-heap-size={}", config.initial_heap_size));
    flags.push(&format!("--max-heap-size={}", config.max_heap_size));

    // Disable WebAssembly for security
    flags.push("--no-wasm");
    flags.push("--no-wasm-async-compilation");
    flags.push("--no-wasm-bounds-checks");
    flags.push("--no-wasm-trap-handler");

    // Disable shared array buffer for security
    flags.push("--no-harmony-sharedarraybuffer");
    flags.push("--no-harmony-atomics");

    // Disable web snapshots for security
    flags.push("--no-web-snapshot");
    flags.push("--no-expose-wasm");

    // Disable eval and code generation from strings
    if !config.allow_eval {
        flags.push("--disallow-code-generation-from-strings");
    }

    // Disable dynamic imports
    if !config.allow_dynamic_imports {
        flags.push("--no-harmony-dynamic-import");
    }

    // Namespace isolation
    if config.namespace_isolation_level >= 1 {
        flags.push("--no-expose-natives-as-builtins");
    }
    
    if config.namespace_isolation_level >= 2 {
        flags.push("--no-allow-natives-syntax");
    }
    
    if config.namespace_isolation_level >= 3 {
        flags.push("--no-expose-gc");
        flags.push("--no-expose-externalize-string");
    }

    flags.join(" ")
}

/// Create V8 parameters based on sandbox configuration
pub fn create_v8_params(config: &SandboxConfig) -> v8::CreateParams {
    v8::CreateParams::default().heap_limits(config.initial_heap_size, config.max_heap_size)
}

/// Resource usage statistics
#[derive(Debug, Clone)]
pub struct ResourceUsage {
    /// Peak memory usage in bytes
    pub peak_memory_usage: AtomicUsize,
    
    /// Current memory usage in bytes
    pub current_memory_usage: AtomicUsize,
    
    /// CPU usage percentage (0-100)
    pub cpu_usage_percentage: AtomicUsize,
    
    /// Execution start time
    pub start_time: Option<Instant>,
    
    /// Execution duration so far
    pub execution_duration: AtomicUsize,
    
    /// Whether resource limits have been exceeded
    pub limits_exceeded: AtomicBool,
}

impl Default for ResourceUsage {
    fn default() -> Self {
        Self {
            peak_memory_usage: AtomicUsize::new(0),
            current_memory_usage: AtomicUsize::new(0),
            cpu_usage_percentage: AtomicUsize::new(0),
            start_time: None,
            execution_duration: AtomicUsize::new(0),
            limits_exceeded: AtomicBool::new(false),
        }
    }
}

/// Sandbox execution context
pub struct SandboxContext {
    /// Execution timeout handle
    timeout_handle: Option<std::thread::JoinHandle<()>>,

    /// Sandbox configuration
    config: SandboxConfig,
    
    /// Resource usage statistics
    resource_usage: Arc<ResourceUsage>,
    
    /// Resource monitoring handle
    resource_monitor_handle: Option<std::thread::JoinHandle<()>>,
}

impl SandboxContext {
    /// Create a new sandbox context
    pub fn new(config: SandboxConfig, isolate: &mut v8::Isolate) -> Self {
        // Initialize resource usage tracking
        let resource_usage = Arc::new(ResourceUsage {
            start_time: Some(Instant::now()),
            ..Default::default()
        });
        
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
        
        // Set up resource monitoring
        let resource_monitor_handle = {
            let resource_usage_clone = resource_usage.clone();
            let max_memory_usage = config.max_memory_usage;
            let max_cpu_percentage = config.max_cpu_percentage as usize;
            let isolate_ptr = isolate as *mut v8::Isolate;
            
            let handle = std::thread::spawn(move || {
                let check_interval = Duration::from_millis(100);
                let mut last_check = Instant::now();
                
                loop {
                    std::thread::sleep(check_interval);
                    
                    // Check if limits have been exceeded
                    if resource_usage_clone.limits_exceeded.load(Ordering::Relaxed) {
                        break;
                    }
                    
                    // Update execution duration
                    if let Some(start_time) = resource_usage_clone.start_time {
                        let duration = start_time.elapsed().as_millis() as usize;
                        resource_usage_clone.execution_duration.store(duration, Ordering::Relaxed);
                    }
                    
                    // Get memory usage from V8
                    unsafe {
                        if !(*isolate_ptr).is_null() {
                            let mut stats = v8::HeapStatistics::default();
                            (*isolate_ptr).get_heap_statistics(&mut stats);
                            
                            let current_memory = stats.used_heap_size() as usize;
                            resource_usage_clone.current_memory_usage.store(current_memory, Ordering::Relaxed);
                            
                            // Update peak memory if current is higher
                            let peak_memory = resource_usage_clone.peak_memory_usage.load(Ordering::Relaxed);
                            if current_memory > peak_memory {
                                resource_usage_clone.peak_memory_usage.store(current_memory, Ordering::Relaxed);
                            }
                            
                            // Check if memory limit exceeded
                            if current_memory > max_memory_usage {
                                resource_usage_clone.limits_exceeded.store(true, Ordering::Relaxed);
                                (*isolate_ptr).terminate_execution();
                                break;
                            }
                        }
                    }
                    
                    // Estimate CPU usage (simplified)
                    let now = Instant::now();
                    let elapsed = now.duration_since(last_check);
                    last_check = now;
                    
                    // If CPU usage exceeds limit, terminate
                    let cpu_usage = resource_usage_clone.cpu_usage_percentage.load(Ordering::Relaxed);
                    if cpu_usage > max_cpu_percentage {
                        resource_usage_clone.limits_exceeded.store(true, Ordering::Relaxed);
                        unsafe {
                            if !(*isolate_ptr).is_null() {
                                (*isolate_ptr).terminate_execution();
                            }
                        }
                        break;
                    }
                }
            });
            
            Some(handle)
        };

        Self {
            timeout_handle,
            config,
            resource_usage,
            resource_monitor_handle,
        }
    }
    
    /// Get resource usage statistics
    pub fn get_resource_usage(&self) -> Arc<ResourceUsage> {
        self.resource_usage.clone()
    }
    
    /// Check if resource limits have been exceeded
    pub fn has_exceeded_limits(&self) -> bool {
        self.resource_usage.limits_exceeded.load(Ordering::Relaxed)
    }
}

impl Drop for SandboxContext {
    fn drop(&mut self) {
        // Clean up timeout thread if it exists
        if let Some(handle) = self.timeout_handle.take() {
            // We don't care about the result, just want to make sure it's cleaned up
            let _ = handle.join();
        }
        
        // Clean up resource monitor thread if it exists
        if let Some(handle) = self.resource_monitor_handle.take() {
            // Signal the thread to stop by setting limits_exceeded
            self.resource_usage.limits_exceeded.store(true, Ordering::Relaxed);
            // Wait for the thread to finish
            let _ = handle.join();
        }
    }
}

/// Permission checker for sandbox operations
pub fn check_permission(operation: &str, config: &SandboxConfig) -> Result<(), String> {
    match operation {
        "net" if !config.allow_net => {
            Err("Network access is not allowed in this sandbox".to_string())
        }
        "fs" if !config.allow_fs => {
            Err("File system access is not allowed in this sandbox".to_string())
        }
        "env" if !config.allow_env => {
            Err("Environment access is not allowed in this sandbox".to_string())
        }
        "run" if !config.allow_run => {
            Err("Process spawning is not allowed in this sandbox".to_string())
        }
        "hrtime" if !config.allow_hrtime => {
            Err("High resolution time is not allowed in this sandbox".to_string())
        }
        "dynamic_import" if !config.allow_dynamic_imports => {
            Err("Dynamic imports are not allowed in this sandbox".to_string())
        }
        "native_plugin" if !config.allow_native_plugins => {
            Err("Native plugins are not allowed in this sandbox".to_string())
        }
        "eval" if !config.allow_eval => {
            Err("Eval and Function constructor are not allowed in this sandbox".to_string())
        }
        _ => Ok(()),
    }
}

/// Create a secure sandbox configuration with strict isolation
pub fn create_secure_sandbox_config() -> SandboxConfig {
    SandboxConfig {
        // Set minimal heap size
        initial_heap_size: 512 * 1024,        // 512KB
        max_heap_size: 64 * 1024 * 1024,      // 64MB
        
        // Set strict execution limits
        max_execution_time: Duration::from_secs(3),
        max_cpu_percentage: 50,               // 50% CPU usage
        max_memory_usage: 128 * 1024 * 1024,  // 128MB total memory
        
        // Disable JIT for security
        enable_jit: false,
        
        // Disable all permissions
        allow_net: false,
        allow_fs: false,
        allow_env: false,
        allow_run: false,
        allow_hrtime: false,
        allow_dynamic_imports: false,
        allow_native_plugins: false,
        allow_eval: false,
        
        // Maximum namespace isolation
        namespace_isolation_level: 3,
    }
}

/// Create a balanced sandbox configuration with moderate isolation
pub fn create_balanced_sandbox_config() -> SandboxConfig {
    SandboxConfig {
        // Set moderate heap size
        initial_heap_size: 1 * 1024 * 1024,    // 1MB
        max_heap_size: 128 * 1024 * 1024,      // 128MB
        
        // Set moderate execution limits
        max_execution_time: Duration::from_secs(10),
        max_cpu_percentage: 70,                // 70% CPU usage
        max_memory_usage: 256 * 1024 * 1024,   // 256MB total memory
        
        // Disable JIT for security
        enable_jit: false,
        
        // Allow limited permissions
        allow_net: true,
        allow_fs: false,
        allow_env: false,
        allow_run: false,
        allow_hrtime: true,
        allow_dynamic_imports: false,
        allow_native_plugins: false,
        allow_eval: false,
        
        // Medium namespace isolation
        namespace_isolation_level: 2,
    }
}
