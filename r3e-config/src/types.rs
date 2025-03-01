// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! Type definitions for the config crate.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Main configuration for the R3E FaaS platform
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaasConfig {
    /// General configuration
    #[serde(default)]
    pub general: GeneralConfig,

    /// Storage configuration
    #[serde(default)]
    pub storage: StorageConfig,

    /// Runtime configuration
    #[serde(default)]
    pub runtime: RuntimeConfig,

    /// Service configuration
    #[serde(default)]
    pub services: ServicesConfig,

    /// API configuration
    #[serde(default)]
    pub api: ApiConfig,

    /// Logging configuration
    #[serde(default)]
    pub logging: LoggingConfig,
}

/// General configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    /// Environment (development, staging, production)
    pub environment: String,

    /// Instance ID
    pub instance_id: String,

    /// Data directory
    pub data_dir: String,
}

/// Storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    /// Storage type (memory, rocksdb)
    pub storage_type: String,

    /// RocksDB path
    pub rocksdb_path: Option<String>,

    /// Memory store capacity
    pub memory_capacity: Option<usize>,
}

/// Runtime configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeConfig {
    /// JavaScript runtime configuration
    pub js: JsRuntimeConfig,

    /// Sandbox configuration
    pub sandbox: SandboxConfig,
}

/// JavaScript runtime configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsRuntimeConfig {
    /// Maximum memory (in MB)
    pub max_memory_mb: usize,

    /// Maximum execution time (in ms)
    pub max_execution_time_ms: u64,

    /// Enable JIT compilation
    pub enable_jit: bool,
}

/// Sandbox configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxConfig {
    /// Enable network access
    pub enable_network: bool,

    /// Enable filesystem access
    pub enable_filesystem: bool,

    /// Enable environment access
    pub enable_environment: bool,

    /// Allowed domains for network access
    pub allowed_domains: Vec<String>,
}

/// Services configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServicesConfig {
    /// Oracle service configuration
    pub oracle: OracleConfig,

    /// Gas bank service configuration
    pub gas_bank: GasBankConfig,

    /// TEE service configuration
    pub tee: TeeConfig,

    /// Balance service configuration
    pub balance: BalanceConfig,

    /// Indexing service configuration
    pub indexing: IndexingConfig,

    /// Identity service configuration
    pub identity: IdentityConfig,

    /// Bridge service configuration
    pub bridge: BridgeConfig,

    /// Auto contract service configuration
    pub auto_contract: AutoContractConfig,
}

/// Oracle service configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OracleConfig {
    /// Enable oracle service
    pub enabled: bool,

    /// Default timeout (in ms)
    pub default_timeout_ms: u64,

    /// Rate limit (requests per minute)
    pub rate_limit: usize,
}

/// Gas bank service configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GasBankConfig {
    /// Enable gas bank service
    pub enabled: bool,

    /// Neo RPC URL
    pub neo_rpc_url: String,

    /// Gas bank wallet address
    pub wallet_address: String,
}

/// TEE service configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeeConfig {
    /// Enable TEE service
    pub enabled: bool,

    /// TEE platform (sgx, sev, nitro, etc.)
    pub platform: String,

    /// Platform-specific configuration
    pub platform_config: HashMap<String, String>,
}

/// Balance service configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceConfig {
    /// Enable balance service
    pub enabled: bool,

    /// Neo RPC URL
    pub neo_rpc_url: String,

    /// Platform wallet address
    pub platform_wallet_address: String,
}

/// Indexing service configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexingConfig {
    /// Enable indexing service
    pub enabled: bool,

    /// Maximum indexed documents
    pub max_documents: usize,
}

/// Identity service configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityConfig {
    /// Enable identity service
    pub enabled: bool,

    /// Supported DID methods
    pub supported_did_methods: Vec<String>,
}

/// Bridge service configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeConfig {
    /// Enable bridge service
    pub enabled: bool,

    /// Supported chains
    pub supported_chains: Vec<String>,
}

/// Auto contract service configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoContractConfig {
    /// Enable auto contract service
    pub enabled: bool,

    /// Maximum contracts per user
    pub max_contracts_per_user: usize,
}

/// API configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    /// Host
    pub host: String,

    /// Port
    pub port: u16,

    /// Enable CORS
    pub enable_cors: bool,

    /// CORS allowed origins
    pub cors_allowed_origins: Vec<String>,

    /// Enable authentication
    pub enable_auth: bool,

    /// JWT secret
    pub jwt_secret: Option<String>,
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level
    pub level: String,

    /// Log format
    pub format: String,

    /// Log file
    pub file: Option<String>,
}

impl Default for FaasConfig {
    fn default() -> Self {
        Self {
            general: GeneralConfig::default(),
            storage: StorageConfig::default(),
            runtime: RuntimeConfig::default(),
            services: ServicesConfig::default(),
            api: ApiConfig::default(),
            logging: LoggingConfig::default(),
        }
    }
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            environment: "development".to_string(),
            instance_id: uuid::Uuid::new_v4().to_string(),
            data_dir: "./data".to_string(),
        }
    }
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            storage_type: "memory".to_string(),
            rocksdb_path: None,
            memory_capacity: None,
        }
    }
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            js: JsRuntimeConfig::default(),
            sandbox: SandboxConfig::default(),
        }
    }
}

impl Default for JsRuntimeConfig {
    fn default() -> Self {
        Self {
            max_memory_mb: 128,
            max_execution_time_ms: 5000,
            enable_jit: false,
        }
    }
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self {
            enable_network: false,
            enable_filesystem: false,
            enable_environment: false,
            allowed_domains: vec![],
        }
    }
}

impl Default for ServicesConfig {
    fn default() -> Self {
        Self {
            oracle: OracleConfig::default(),
            gas_bank: GasBankConfig::default(),
            tee: TeeConfig::default(),
            balance: BalanceConfig::default(),
            indexing: IndexingConfig::default(),
            identity: IdentityConfig::default(),
            bridge: BridgeConfig::default(),
            auto_contract: AutoContractConfig::default(),
        }
    }
}

impl Default for OracleConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            default_timeout_ms: 5000,
            rate_limit: 100,
        }
    }
}

impl Default for GasBankConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            neo_rpc_url: "http://localhost:10332".to_string(),
            wallet_address: "".to_string(),
        }
    }
}

impl Default for TeeConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            platform: "none".to_string(),
            platform_config: HashMap::new(),
        }
    }
}

impl Default for BalanceConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            neo_rpc_url: "http://localhost:10332".to_string(),
            platform_wallet_address: "".to_string(),
        }
    }
}

impl Default for IndexingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_documents: 10000,
        }
    }
}

impl Default for IdentityConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            supported_did_methods: vec!["neo".to_string()],
        }
    }
}

impl Default for BridgeConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            supported_chains: vec!["neo".to_string()],
        }
    }
}

impl Default for AutoContractConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_contracts_per_user: 10,
        }
    }
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            enable_cors: true,
            cors_allowed_origins: vec!["*".to_string()],
            enable_auth: false,
            jwt_secret: None,
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            format: "json".to_string(),
            file: None,
        }
    }
}
