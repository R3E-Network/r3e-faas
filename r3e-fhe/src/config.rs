// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! Configuration for the Fully Homomorphic Encryption service.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Configuration for the Fully Homomorphic Encryption service.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FheConfig {
    /// Storage configuration.
    pub storage: FheStorageConfig,
    /// Scheme configurations.
    pub schemes: FheSchemesConfig,
    /// Service configuration.
    pub service: FheServiceConfig,
}

/// Storage configuration for the Fully Homomorphic Encryption service.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FheStorageConfig {
    /// Storage type.
    pub storage_type: FheStorageType,
    /// Path to the RocksDB database (if using RocksDB).
    pub rocksdb_path: Option<PathBuf>,
    /// Maximum cache size in MB.
    pub max_cache_size_mb: Option<usize>,
}

/// Storage type for the Fully Homomorphic Encryption service.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FheStorageType {
    /// In-memory storage.
    Memory,
    /// RocksDB storage.
    RocksDb,
}

/// Scheme configurations for the Fully Homomorphic Encryption service.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FheSchemesConfig {
    /// TFHE scheme configuration.
    pub tfhe: Option<TfheConfig>,
    /// OpenFHE scheme configuration.
    pub openfhe: Option<OpenFheConfig>,
}

/// TFHE scheme configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TfheConfig {
    /// Whether to enable the TFHE scheme.
    pub enabled: bool,
    /// Default security level in bits.
    pub default_security_level: u32,
    /// Default polynomial modulus degree.
    pub default_polynomial_modulus_degree: u32,
    /// Default plaintext modulus.
    pub default_plaintext_modulus: u32,
}

/// OpenFHE scheme configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenFheConfig {
    /// Whether to enable the OpenFHE scheme.
    pub enabled: bool,
    /// Path to the OpenFHE library (if not using the embedded library).
    pub library_path: Option<PathBuf>,
    /// Default security level in bits.
    pub default_security_level: u32,
    /// Default polynomial modulus degree.
    pub default_polynomial_modulus_degree: u32,
    /// Default plaintext modulus.
    pub default_plaintext_modulus: u32,
}

/// Service configuration for the Fully Homomorphic Encryption service.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FheServiceConfig {
    /// Default scheme to use.
    pub default_scheme: Option<String>,
    /// Maximum ciphertext size in bytes.
    pub max_ciphertext_size_bytes: usize,
    /// Maximum plaintext size in bytes.
    pub max_plaintext_size_bytes: usize,
    /// Maximum time for key generation in seconds.
    pub max_key_generation_time_seconds: u64,
    /// Maximum time for encryption in seconds.
    pub max_encryption_time_seconds: u64,
    /// Maximum time for decryption in seconds.
    pub max_decryption_time_seconds: u64,
    /// Maximum time for homomorphic operations in seconds.
    pub max_homomorphic_operation_time_seconds: u64,
    /// Whether to enable verbose logging.
    pub verbose_logging: bool,
}

impl Default for FheConfig {
    fn default() -> Self {
        Self {
            storage: FheStorageConfig {
                storage_type: FheStorageType::Memory,
                rocksdb_path: None,
                max_cache_size_mb: Some(1024),
            },
            schemes: FheSchemesConfig {
                tfhe: Some(TfheConfig {
                    enabled: true,
                    default_security_level: 128,
                    default_polynomial_modulus_degree: 4096,
                    default_plaintext_modulus: 1024,
                }),
                openfhe: Some(OpenFheConfig {
                    enabled: false,
                    library_path: None,
                    default_security_level: 128,
                    default_polynomial_modulus_degree: 4096,
                    default_plaintext_modulus: 1024,
                }),
            },
            service: FheServiceConfig {
                default_scheme: Some("TFHE".to_string()),
                max_ciphertext_size_bytes: 10 * 1024 * 1024, // 10 MB
                max_plaintext_size_bytes: 1 * 1024 * 1024,   // 1 MB
                max_key_generation_time_seconds: 300,        // 5 minutes
                max_encryption_time_seconds: 60,             // 1 minute
                max_decryption_time_seconds: 60,             // 1 minute
                max_homomorphic_operation_time_seconds: 300, // 5 minutes
                verbose_logging: false,
            },
        }
    }
}
