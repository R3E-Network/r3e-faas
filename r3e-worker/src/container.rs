// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::process::{Command, Stdio};
use std::time::Duration;
use thiserror::Error;
use tracing::{debug, error, info, warn};

/// Container isolation error
#[derive(Debug, Error)]
pub enum ContainerError {
    #[error("Failed to create container: {0}")]
    Creation(String),

    #[error("Failed to start container: {0}")]
    Start(String),

    #[error("Failed to stop container: {0}")]
    Stop(String),

    #[error("Failed to execute command in container: {0}")]
    Execution(String),

    #[error("Container timeout")]
    Timeout,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Container isolation configuration
#[derive(Debug, Clone)]
pub struct ContainerConfig {
    /// Base image to use
    pub base_image: String,

    /// Memory limit in bytes
    pub memory_limit: u64,

    /// CPU limit (number of cores)
    pub cpu_limit: f64,

    /// Network access (none, host, bridge)
    pub network_mode: NetworkMode,

    /// Maximum execution time
    pub max_execution_time: Duration,

    /// Allow file system access
    pub allow_fs: bool,

    /// Environment variables
    pub env_vars: Vec<(String, String)>,
}

impl Default for ContainerConfig {
    fn default() -> Self {
        Self {
            base_image: "node:18-alpine".to_string(),
            memory_limit: 256 * 1024 * 1024, // 256MB
            cpu_limit: 0.5, // Half a core
            network_mode: NetworkMode::None,
            max_execution_time: Duration::from_secs(10),
            allow_fs: false,
            env_vars: Vec::new(),
        }
    }
}

/// Network mode for container
#[derive(Debug, Clone, PartialEq)]
pub enum NetworkMode {
    /// No network access
    None,

    /// Host network (unrestricted)
    Host,

    /// Bridge network (restricted)
    Bridge,
}

impl NetworkMode {
    /// Convert to Docker network mode string
    fn to_docker_mode(&self) -> &'static str {
        match self {
            NetworkMode::None => "none",
            NetworkMode::Host => "host",
            NetworkMode::Bridge => "bridge",
        }
    }
}

/// Container manager for function isolation
pub struct ContainerManager {
    /// Container configuration
    config: ContainerConfig,
}

impl ContainerManager {
    /// Create a new container manager
    pub fn new(config: ContainerConfig) -> Self {
        Self { config }
    }

    /// Run a function in a container
    pub fn run_function(&self, function_id: &str, code: &str) -> Result<String, ContainerError> {
        // Create a temporary directory for the function
        let temp_dir = std::env::temp_dir().join(format!("r3e-function-{}", function_id));
        std::fs::create_dir_all(&temp_dir)?;

        // Write the function code to a file
        let function_file = temp_dir.join("index.js");
        std::fs::write(&function_file, code)?;

        // Create a unique container name
        let container_name = format!("r3e-function-{}", function_id);

        // Build the Docker run command
        let mut cmd = Command::new("docker");
        cmd.arg("run")
            .arg("--rm") // Remove container after execution
            .arg("--name").arg(&container_name)
            .arg("--network").arg(self.config.network_mode.to_docker_mode())
            .arg("--memory").arg(format!("{}b", self.config.memory_limit))
            .arg("--cpus").arg(self.config.cpu_limit.to_string())
            .arg("--read-only") // Read-only file system
            .arg("-v").arg(format!("{}:/app", temp_dir.to_string_lossy()));

        // Add environment variables
        for (key, value) in &self.config.env_vars {
            cmd.arg("-e").arg(format!("{}={}", key, value));
        }

        // Set timeout
        cmd.arg("--stop-timeout").arg(self.config.max_execution_time.as_secs().to_string());

        // Add security options
        cmd.arg("--security-opt").arg("no-new-privileges:true");
        
        if !self.config.allow_fs {
            // Mount empty volumes over sensitive directories
            cmd.arg("-v").arg("/dev/null:/proc/acpi");
            cmd.arg("-v").arg("/dev/null:/proc/keys");
            cmd.arg("-v").arg("/dev/null:/proc/latency_stats");
            cmd.arg("-v").arg("/dev/null:/proc/timer_list");
            cmd.arg("-v").arg("/dev/null:/proc/timer_stats");
            cmd.arg("-v").arg("/dev/null:/proc/sched_debug");
            cmd.arg("-v").arg("/dev/null:/sys/firmware");
        }

        // Add the image and command
        cmd.arg(&self.config.base_image)
            .arg("node")
            .arg("/app/index.js");

        // Execute the command
        debug!("Running container command: {:?}", cmd);
        
        let output = cmd.stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()?;

        // Clean up the temporary directory
        std::fs::remove_dir_all(temp_dir)?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(ContainerError::Execution(String::from_utf8_lossy(&output.stderr).to_string()))
        }
    }

    /// Stop a running function container
    pub fn stop_function(&self, function_id: &str) -> Result<(), ContainerError> {
        let container_name = format!("r3e-function-{}", function_id);
        
        let output = Command::new("docker")
            .arg("stop")
            .arg("--time").arg("1") // Give it 1 second to stop gracefully
            .arg(&container_name)
            .output()?;

        if output.status.success() {
            Ok(())
        } else {
            Err(ContainerError::Stop(String::from_utf8_lossy(&output.stderr).to_string()))
        }
    }
}
