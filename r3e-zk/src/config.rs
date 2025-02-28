// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! Configuration for the Zero-Knowledge computing service.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Configuration for the Zero-Knowledge computing service.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkConfig {
    /// Storage configuration.
    pub storage: ZkStorageConfig,
    /// Provider configurations.
    pub providers: ZkProvidersConfig,
    /// Service configuration.
    pub service: ZkServiceConfig,
}

/// Storage configuration for the Zero-Knowledge computing service.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkStorageConfig {
    /// Storage type.
    pub storage_type: ZkStorageType,
    /// Path to the RocksDB database (if using RocksDB).
    pub rocksdb_path: Option<PathBuf>,
    /// Maximum cache size in MB.
    pub max_cache_size_mb: Option<usize>,
}

/// Storage type for the Zero-Knowledge computing service.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ZkStorageType {
    /// In-memory storage.
    Memory,
    /// RocksDB storage.
    RocksDb,
}

/// Provider configurations for the Zero-Knowledge computing service.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkProvidersConfig {
    /// Zokrates provider configuration.
    pub zokrates: Option<ZokratesConfig>,
    /// Bulletproofs provider configuration.
    pub bulletproofs: Option<BulletproofsConfig>,
    /// Circom provider configuration.
    pub circom: Option<CircomConfig>,
    /// Bellman provider configuration.
    pub bellman: Option<BellmanConfig>,
    /// Arkworks provider configuration.
    pub arkworks: Option<ArkworksConfig>,
}

/// Zokrates provider configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZokratesConfig {
    /// Whether to enable the Zokrates provider.
    pub enabled: bool,
    /// Path to the Zokrates binary (if not using the embedded library).
    pub binary_path: Option<PathBuf>,
    /// Default optimization level (0-3).
    pub default_optimization_level: u8,
}

/// Bulletproofs provider configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulletproofsConfig {
    /// Whether to enable the Bulletproofs provider.
    pub enabled: bool,
    /// Default number of generators to use.
    pub default_generators: usize,
}

/// Circom provider configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircomConfig {
    /// Whether to enable the Circom provider.
    pub enabled: bool,
    /// Path to the Circom binary (if not using the embedded library).
    pub binary_path: Option<PathBuf>,
    /// Default witness generation strategy.
    pub default_witness_strategy: String,
}

/// Bellman provider configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BellmanConfig {
    /// Whether to enable the Bellman provider.
    pub enabled: bool,
    /// Default curve type to use.
    pub default_curve: String,
}

/// Arkworks provider configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArkworksConfig {
    /// Whether to enable the Arkworks provider.
    pub enabled: bool,
    /// Default proving system to use.
    pub default_proving_system: String,
    /// Default curve type to use.
    pub default_curve: String,
}

/// Service configuration for the Zero-Knowledge computing service.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkServiceConfig {
    /// Default platform to use.
    pub default_platform: Option<String>,
    /// Maximum circuit size in bytes.
    pub max_circuit_size_bytes: usize,
    /// Maximum proof size in bytes.
    pub max_proof_size_bytes: usize,
    /// Maximum time for compilation in seconds.
    pub max_compilation_time_seconds: u64,
    /// Maximum time for proof generation in seconds.
    pub max_proof_generation_time_seconds: u64,
    /// Whether to enable verbose logging.
    pub verbose_logging: bool,
}

impl Default for ZkConfig {
    fn default() -> Self {
        Self {
            storage: ZkStorageConfig {
                storage_type: ZkStorageType::Memory,
                rocksdb_path: None,
                max_cache_size_mb: Some(1024),
            },
            providers: ZkProvidersConfig {
                zokrates: Some(ZokratesConfig {
                    enabled: true,
                    binary_path: None,
                    default_optimization_level: 2,
                }),
                bulletproofs: Some(BulletproofsConfig {
                    enabled: true,
                    default_generators: 256,
                }),
                circom: Some(CircomConfig {
                    enabled: true,
                    binary_path: None,
                    default_witness_strategy: "wasm".to_string(),
                }),
                bellman: Some(BellmanConfig {
                    enabled: true,
                    default_curve: "bls12_381".to_string(),
                }),
                arkworks: Some(ArkworksConfig {
                    enabled: true,
                    default_proving_system: "groth16".to_string(),
                    default_curve: "bls12_381".to_string(),
                }),
            },
            service: ZkServiceConfig {
                default_platform: Some("Zokrates".to_string()),
                max_circuit_size_bytes: 10 * 1024 * 1024, // 10 MB
                max_proof_size_bytes: 1 * 1024 * 1024,    // 1 MB
                max_compilation_time_seconds: 300,        // 5 minutes
                max_proof_generation_time_seconds: 600,   // 10 minutes
                verbose_logging: false,
            },
        }
    }
}
