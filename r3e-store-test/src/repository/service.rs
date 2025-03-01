// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! Service repository implementation using RocksDB

use crate::rocksdb::{AsyncRocksDbClient, DbRepository, DbResult, RocksDbConfig};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Column family name for services
pub const CF_SERVICES: &str = "services";

/// Service entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    /// Service ID
    pub id: String,
    
    /// Service name
    pub name: String,
    
    /// Service description
    pub description: String,
    
    /// Service owner (user ID)
    pub owner_id: String,
    
    /// Service status (enabled/disabled)
    pub enabled: bool,
    
    /// Service type (REST, WS, etc.)
    pub service_type: ServiceType,
    
    /// Service endpoint URL
    pub endpoint: String,
    
    /// Service contract address (for blockchain services)
    pub contract_address: Option<String>,
    
    /// Service blockchain type (for blockchain services)
    pub blockchain_type: Option<BlockchainType>,
    
    /// Service metadata
    pub metadata: serde_json::Value,
    
    /// Created at timestamp (millis since epoch)
    pub created_at: u64,
    
    /// Updated at timestamp (millis since epoch)
    pub updated_at: u64,
}

/// Service type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceType {
    /// REST service
    Rest,
    
    /// WebSocket service
    WebSocket,
    
    /// Blockchain service
    Blockchain,
    
    /// FHE service
    FullyHomomorphicEncryption,
    
    /// ZK service
    ZeroKnowledge,
    
    /// Other service type
    Other(String),
}

/// Blockchain type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BlockchainType {
    /// Ethereum blockchain
    Ethereum,
    
    /// Neo blockchain
    Neo,
    
    /// Solana blockchain
    Solana,
    
    /// Other blockchain
    Other(String),
}

/// Service repository
pub struct ServiceRepository {
    db: Arc<AsyncRocksDbClient>,
}

impl ServiceRepository {
    /// Create a new service repository
    pub fn new(db: Arc<AsyncRocksDbClient>) -> Self {
        Self { db }
    }
    
    /// Initialize the repository
    pub fn from_config(config: RocksDbConfig) -> Self {
        // Make sure services column family is configured
        let mut config = config.clone();
        if !config.column_families.iter().any(|cf| cf.name == CF_SERVICES) {
            config.column_families.push(crate::rocksdb::ColumnFamilyConfig {
                name: CF_SERVICES.to_string(),
                prefix_extractor: None,
                block_size: 4096,
                block_cache_size: 8 * 1024 * 1024,
                bloom_filter_bits: 10,
                cache_index_and_filter_blocks: true,
            });
        }
        
        let db = Arc::new(AsyncRocksDbClient::new(config));
        Self::new(db)
    }
    
    /// Find services by owner ID
    pub async fn find_by_owner(&self, owner_id: &str) -> DbResult<Vec<Service>> {
        let inner = self.db.inner.clone();
        let cf_name = CF_SERVICES.to_string();
        let owner_id = owner_id.to_string();
        
        tokio::task::spawn_blocking(move || {
            let iter = inner.iter_cf::<Service>(&cf_name, rocksdb::IteratorMode::Start)?;
            let services: Vec<Service> = iter
                .filter_map(|(_, service)| {
                    if service.owner_id == owner_id {
                        Some(service)
                    } else {
                        None
                    }
                })
                .collect();
            
            Ok(services)
        })
        .await
        .map_err(|e| crate::rocksdb::DbError::TransactionFailed(e.to_string()))?
    }
    
    /// Find services by type
    pub async fn find_by_type(&self, service_type: &ServiceType) -> DbResult<Vec<Service>> {
        let inner = self.db.inner.clone();
        let cf_name = CF_SERVICES.to_string();
        // We need to clone the service_type for the move closure
        let service_type = service_type.clone();
        
        tokio::task::spawn_blocking(move || {
            let iter = inner.iter_cf::<Service>(&cf_name, rocksdb::IteratorMode::Start)?;
            let services: Vec<Service> = iter
                .filter_map(|(_, service)| {
                    match (&service.service_type, &service_type) {
                        (ServiceType::Rest, ServiceType::Rest) => Some(service),
                        (ServiceType::WebSocket, ServiceType::WebSocket) => Some(service),
                        (ServiceType::Blockchain, ServiceType::Blockchain) => Some(service),
                        (ServiceType::FullyHomomorphicEncryption, ServiceType::FullyHomomorphicEncryption) => Some(service),
                        (ServiceType::ZeroKnowledge, ServiceType::ZeroKnowledge) => Some(service),
                        (ServiceType::Other(a), ServiceType::Other(b)) if a == b => Some(service),
                        _ => None,
                    }
                })
                .collect();
            
            Ok(services)
        })
        .await
        .map_err(|e| crate::rocksdb::DbError::TransactionFailed(e.to_string()))?
    }
    
    /// Find enabled services
    pub async fn find_enabled(&self) -> DbResult<Vec<Service>> {
        let inner = self.db.inner.clone();
        let cf_name = CF_SERVICES.to_string();
        
        tokio::task::spawn_blocking(move || {
            let iter = inner.iter_cf::<Service>(&cf_name, rocksdb::IteratorMode::Start)?;
            let services: Vec<Service> = iter
                .filter_map(|(_, service)| {
                    if service.enabled {
                        Some(service)
                    } else {
                        None
                    }
                })
                .collect();
            
            Ok(services)
        })
        .await
        .map_err(|e| crate::rocksdb::DbError::TransactionFailed(e.to_string()))?
    }
    
    /// Find services by blockchain type
    pub async fn find_by_blockchain(&self, blockchain_type: &BlockchainType) -> DbResult<Vec<Service>> {
        let inner = self.db.inner.clone();
        let cf_name = CF_SERVICES.to_string();
        // We need to clone the blockchain_type for the move closure
        let blockchain_type = blockchain_type.clone();
        
        tokio::task::spawn_blocking(move || {
            let iter = inner.iter_cf::<Service>(&cf_name, rocksdb::IteratorMode::Start)?;
            let services: Vec<Service> = iter
                .filter_map(|(_, service)| {
                    if let Some(ref service_blockchain) = service.blockchain_type {
                        match (service_blockchain, &blockchain_type) {
                            (BlockchainType::Ethereum, BlockchainType::Ethereum) => Some(service),
                            (BlockchainType::Neo, BlockchainType::Neo) => Some(service),
                            (BlockchainType::Solana, BlockchainType::Solana) => Some(service),
                            (BlockchainType::Other(a), BlockchainType::Other(b)) if a == b => Some(service),
                            _ => None,
                        }
                    } else {
                        None
                    }
                })
                .collect();
            
            Ok(services)
        })
        .await
        .map_err(|e| crate::rocksdb::DbError::TransactionFailed(e.to_string()))?
    }
}

// Implement the DbRepository trait using the macro
crate::rocksdb::impl_db_repository!(ServiceRepository, Service, String, CF_SERVICES, |service: &Service| service.id.clone()); 