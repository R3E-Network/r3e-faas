// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! Configuration for the event crate.

use serde::{Deserialize, Serialize};

/// Configuration for the event crate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Storage type
    pub storage_type: StorageType,

    /// RocksDB path (if using RocksDB storage)
    pub rocksdb_path: Option<String>,

    /// Source configuration
    pub source: SourceConfig,

    /// Registry configuration
    pub registry: RegistryConfig,

    /// Trigger configuration
    pub trigger: TriggerConfig,
}

/// Storage type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorageType {
    /// Memory storage
    Memory,

    /// RocksDB storage
    RocksDB,
}

/// Source configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceConfig {
    /// Enabled sources
    pub enabled_sources: Vec<String>,

    /// Source-specific configurations
    pub sources: serde_json::Value,
}

/// Registry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryConfig {
    /// Maximum events to keep
    pub max_events: usize,

    /// Event TTL in seconds
    pub event_ttl: u64,
}

/// Trigger configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerConfig {
    /// Enabled triggers
    pub enabled_triggers: Vec<String>,

    /// Trigger-specific configurations
    pub triggers: serde_json::Value,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            storage_type: StorageType::Memory,
            rocksdb_path: None,
            source: SourceConfig::default(),
            registry: RegistryConfig::default(),
            trigger: TriggerConfig::default(),
        }
    }
}

impl Default for SourceConfig {
    fn default() -> Self {
        Self {
            enabled_sources: vec!["Neo".to_string()],
            sources: serde_json::json!({}),
        }
    }
}

impl Default for RegistryConfig {
    fn default() -> Self {
        Self {
            max_events: 1000,
            event_ttl: 86400, // 24 hours
        }
    }
}

impl Default for TriggerConfig {
    fn default() -> Self {
        Self {
            enabled_triggers: vec!["NeoNewBlock".to_string(), "NeoNewTx".to_string()],
            triggers: serde_json::json!({}),
        }
    }
}
