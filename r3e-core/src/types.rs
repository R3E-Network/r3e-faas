// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! Type definitions for the core crate.

use serde::{Deserialize, Serialize};

/// Platform type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Platform {
    /// Linux platform
    Linux,
    
    /// macOS platform
    Darwin,
    
    /// Windows platform
    Windows,
    
    /// WebAssembly platform
    Wasi,
    
    /// Unknown platform
    Unknown,
}

impl Platform {
    /// Get the current platform
    pub fn current() -> Self {
        if cfg!(target_os = "linux") {
            Platform::Linux
        } else if cfg!(target_os = "macos") {
            Platform::Darwin
        } else if cfg!(target_os = "windows") {
            Platform::Windows
        } else if cfg!(target_os = "wasi") {
            Platform::Wasi
        } else {
            Platform::Unknown
        }
    }
    
    /// Convert platform to string
    pub fn to_str(&self) -> &'static str {
        match self {
            Platform::Linux => "linux",
            Platform::Darwin => "darwin",
            Platform::Windows => "windows",
            Platform::Wasi => "wasi",
            Platform::Unknown => "unknown",
        }
    }
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}
