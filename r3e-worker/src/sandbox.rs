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

/// Sandbox manager for JavaScript runtime
pub struct SandboxManager {
    /// Default sandbox configuration
    default_config: SandboxConfig,
}

impl SandboxManager {
    /// Create a new sandbox manager
    pub fn new(default_config: SandboxConfig) -> Self {
        Self { default_config }
    }

    /// Create a new sandbox manager with default configuration
    pub fn default() -> Self {
        Self {
            default_config: SandboxConfig::default(),
        }
    }

    /// Get the default sandbox configuration
    pub fn default_config(&self) -> &SandboxConfig {
        &self.default_config
    }

    /// Create a new sandbox configuration with custom settings
    pub fn create_config(
        &self,
        max_heap_size: Option<usize>,
        max_execution_time: Option<Duration>,
        enable_jit: Option<bool>,
        allow_net: Option<bool>,
        allow_fs: Option<bool>,
        allow_env: Option<bool>,
        allow_run: Option<bool>,
        allow_hrtime: Option<bool>,
    ) -> SandboxConfig {
        let mut config = self.default_config.clone();

        if let Some(max_heap_size) = max_heap_size {
            config.max_heap_size = max_heap_size;
        }

        if let Some(max_execution_time) = max_execution_time {
            config.max_execution_time = max_execution_time;
        }

        if let Some(enable_jit) = enable_jit {
            config.enable_jit = enable_jit;
        }

        if let Some(allow_net) = allow_net {
            config.allow_net = allow_net;
        }

        if let Some(allow_fs) = allow_fs {
            config.allow_fs = allow_fs;
        }

        if let Some(allow_env) = allow_env {
            config.allow_env = allow_env;
        }

        if let Some(allow_run) = allow_run {
            config.allow_run = allow_run;
        }

        if let Some(allow_hrtime) = allow_hrtime {
            config.allow_hrtime = allow_hrtime;
        }

        config
    }

    /// Create a sandbox configuration for a security level
    pub fn create_config_for_security_level(&self, security_level: &str) -> SandboxConfig {
        match security_level {
            "high" => {
                // High security: minimal permissions
                self.create_config(
                    Some(64 * 1024 * 1024), // 64MB
                    Some(Duration::from_secs(5)),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(false),
                )
            }
            "medium" => {
                // Medium security: some permissions
                self.create_config(
                    Some(128 * 1024 * 1024), // 128MB
                    Some(Duration::from_secs(10)),
                    Some(true),
                    Some(true),
                    Some(false),
                    Some(false),
                    Some(false),
                    Some(true),
                )
            }
            "low" => {
                // Low security: most permissions
                self.create_config(
                    Some(256 * 1024 * 1024), // 256MB
                    Some(Duration::from_secs(30)),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(true),
                    Some(false),
                    Some(true),
                )
            }
            _ => {
                // Default to high security
                self.create_config_for_security_level("high")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let manager = SandboxManager::default();
        let config = manager.default_config();

        assert_eq!(config.initial_heap_size, 1 * 1024 * 1024);
        assert_eq!(config.max_heap_size, 128 * 1024 * 1024);
        assert_eq!(config.max_execution_time, Duration::from_secs(10));
        assert_eq!(config.enable_jit, false);
        assert_eq!(config.allow_net, false);
        assert_eq!(config.allow_fs, false);
        assert_eq!(config.allow_env, false);
        assert_eq!(config.allow_run, false);
        assert_eq!(config.allow_hrtime, false);
    }

    #[test]
    fn test_create_config() {
        let manager = SandboxManager::default();
        let config = manager.create_config(
            Some(256 * 1024 * 1024),
            Some(Duration::from_secs(20)),
            Some(true),
            Some(true),
            Some(true),
            Some(true),
            Some(true),
            Some(true),
        );

        assert_eq!(config.initial_heap_size, 1 * 1024 * 1024);
        assert_eq!(config.max_heap_size, 256 * 1024 * 1024);
        assert_eq!(config.max_execution_time, Duration::from_secs(20));
        assert_eq!(config.enable_jit, true);
        assert_eq!(config.allow_net, true);
        assert_eq!(config.allow_fs, true);
        assert_eq!(config.allow_env, true);
        assert_eq!(config.allow_run, true);
        assert_eq!(config.allow_hrtime, true);
    }

    #[test]
    fn test_security_levels() {
        let manager = SandboxManager::default();

        // High security
        let high = manager.create_config_for_security_level("high");
        assert_eq!(high.max_heap_size, 64 * 1024 * 1024);
        assert_eq!(high.max_execution_time, Duration::from_secs(5));
        assert_eq!(high.enable_jit, false);
        assert_eq!(high.allow_net, false);

        // Medium security
        let medium = manager.create_config_for_security_level("medium");
        assert_eq!(medium.max_heap_size, 128 * 1024 * 1024);
        assert_eq!(medium.max_execution_time, Duration::from_secs(10));
        assert_eq!(medium.enable_jit, true);
        assert_eq!(medium.allow_net, true);

        // Low security
        let low = manager.create_config_for_security_level("low");
        assert_eq!(low.max_heap_size, 256 * 1024 * 1024);
        assert_eq!(low.max_execution_time, Duration::from_secs(30));
        assert_eq!(low.enable_jit, true);
        assert_eq!(low.allow_fs, true);
    }
}
