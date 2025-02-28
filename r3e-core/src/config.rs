// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! Configuration for the core crate.

use serde::{Deserialize, Serialize};

/// Configuration for the core crate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Log configuration file path
    pub log_config: Option<String>,
    
    /// V8 configuration
    pub v8: V8Config,
}

/// V8 configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct V8Config {
    /// Number of worker threads
    pub worker_threads: usize,
    
    /// Enable background compilation
    pub background_compilation: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            log_config: None,
            v8: V8Config::default(),
        }
    }
}

impl Default for V8Config {
    fn default() -> Self {
        Self {
            worker_threads: 0,
            background_compilation: false,
        }
    }
}
