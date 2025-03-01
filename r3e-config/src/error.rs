// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! Error types for the config crate.

use thiserror::Error;

/// Error type for the config crate
#[derive(Debug, Error)]
pub enum Error {
    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// YAML parsing error
    #[error("YAML parsing error: {0}")]
    YamlParsing(#[from] serde_yaml::Error),

    /// JSON parsing error
    #[error("JSON parsing error: {0}")]
    JsonParsing(#[from] serde_json::Error),

    /// Config error
    #[error("Config error: {0}")]
    Config(#[from] config::ConfigError),

    /// Environment variable error
    #[error("Environment variable error: {0}")]
    EnvVar(String),

    /// Missing configuration
    #[error("Missing configuration: {0}")]
    MissingConfig(String),

    /// Invalid configuration
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
}

/// Result type for the config crate
pub type Result<T> = std::result::Result<T, Error>;
