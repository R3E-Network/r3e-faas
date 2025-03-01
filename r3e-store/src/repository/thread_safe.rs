// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! Thread-safe repository implementations

use crate::error::{
    DeleteError, GetError, MultiDeleteError, MultiGetError, MultiPutError, PutError, ScanError,
};
use crate::repository::service::{BlockchainType, Service, ServiceRepository, ServiceType, CF_SERVICES};
use crate::repository::user::{User, UserRepository, CF_USERS};
use crate::storage::rocksdb::ThreadSafeRocksDBStore;
use crate::types::{PutInput, ScanInput, ScanOutput};
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Thread-safe service repository
pub struct ThreadSafeServiceRepository {
    store: Arc<ThreadSafeRocksDBStore>,
}

impl ThreadSafeServiceRepository {
    /// Create a new thread-safe service repository
    pub fn new(path: &str) -> Self {
        let store = Arc::new(ThreadSafeRocksDBStore::new(path));
        Self { store }
    }

    /// Find services by owner ID
    pub async fn find_by_owner(&self, owner_id: &str) -> Result<Vec<Service>, crate::error::GetError> {
        // Load all services
        let services = self.find_all().await?;
        
        // Filter by owner ID
        let filtered = services
            .into_iter()
            .filter(|service| service.owner_id == owner_id)
            .collect();
        
        Ok(filtered)
    }

    /// Find services by type
    pub async fn find_by_type(&self, service_type: &ServiceType) -> Result<Vec<Service>, crate::error::GetError> {
        // Load all services
        let services = self.find_all().await?;
        
        // Filter by service type
        let filtered = services
            .into_iter()
            .filter(|service| {
                match (&service.service_type, service_type) {
                    (ServiceType::Rest, ServiceType::Rest) => true,
                    (ServiceType::WebSocket, ServiceType::WebSocket) => true,
                    (ServiceType::Blockchain, ServiceType::Blockchain) => true,
                    (ServiceType::FullyHomomorphicEncryption, ServiceType::FullyHomomorphicEncryption) => true,
                    (ServiceType::ZeroKnowledge, ServiceType::ZeroKnowledge) => true,
                    (ServiceType::Other(a), ServiceType::Other(b)) if a == b => true,
                    _ => false,
                }
            })
            .collect();
        
        Ok(filtered)
    }

    /// Find enabled services
    pub async fn find_enabled(&self) -> Result<Vec<Service>, crate::error::GetError> {
        // Load all services
        let services = self.find_all().await?;
        
        // Filter by enabled status
        let filtered = services
            .into_iter()
            .filter(|service| service.enabled)
            .collect();
        
        Ok(filtered)
    }

    /// Find services by blockchain type
    pub async fn find_by_blockchain(
        &self,
        blockchain_type: &BlockchainType,
    ) -> Result<Vec<Service>, crate::error::GetError> {
        // Load all services
        let services = self.find_all().await?;
        
        // Filter by blockchain type
        let filtered = services
            .into_iter()
            .filter(|service| {
                if let Some(ref service_blockchain) = service.blockchain_type {
                    match (service_blockchain, blockchain_type) {
                        (BlockchainType::Ethereum, BlockchainType::Ethereum) => true,
                        (BlockchainType::Neo, BlockchainType::Neo) => true,
                        (BlockchainType::Solana, BlockchainType::Solana) => true,
                        (BlockchainType::Other(a), BlockchainType::Other(b)) if a == b => true,
                        _ => false,
                    }
                } else {
                    false
                }
            })
            .collect();
        
        Ok(filtered)
    }

    /// Find all services
    pub async fn find_all(&self) -> Result<Vec<Service>, crate::error::GetError> {
        let scan_input = ScanInput {
            start_key: None,
            end_key: None,
            limit: None,
            reverse: false,
        };
        
        let scan_output = self.store.scan(CF_SERVICES, scan_input)
            .map_err(|e| GetError::Storage(e.to_string()))?;
        
        let mut services = Vec::with_capacity(scan_output.items.len());
        
        for (_, value) in scan_output.items {
            let service: Service = bincode::deserialize(&value)
                .map_err(|e| GetError::Deserialization(e.to_string()))?;
            services.push(service);
        }
        
        Ok(services)
    }

    /// Create a new service
    pub async fn create(&self, service: &Service) -> Result<(), crate::error::PutError> {
        let key = service.id.as_bytes();
        let value = bincode::serialize(service)
            .map_err(|e| PutError::Serialization(e.to_string()))?;
        
        self.store.put(CF_SERVICES, PutInput { key: key.to_vec(), value })
            .map_err(|e| PutError::Storage(e.to_string()))?;
        
        Ok(())
    }

    /// Get a service by ID
    pub async fn get(&self, id: &str) -> Result<Option<Service>, crate::error::GetError> {
        let key = id.as_bytes();
        
        let value = self.store.get(CF_SERVICES, key)
            .map_err(|e| GetError::Storage(e.to_string()))?;
        
        match value {
            Some(data) => {
                let service = bincode::deserialize(&data)
                    .map_err(|e| GetError::Deserialization(e.to_string()))?;
                Ok(Some(service))
            }
            None => Ok(None),
        }
    }

    /// Update a service
    pub async fn update(&self, service: &Service) -> Result<(), crate::error::PutError> {
        self.create(service).await
    }

    /// Delete a service by ID
    pub async fn delete(&self, id: &str) -> Result<Option<Service>, crate::error::DeleteError> {
        let key = id.as_bytes();
        
        let value = self.store.delete(CF_SERVICES, key)
            .map_err(|e| DeleteError::Storage(e.to_string()))?;
        
        match value {
            Some(data) => {
                let service = bincode::deserialize(&data)
                    .map_err(|e| DeleteError::Deserialization(e.to_string()))?;
                Ok(Some(service))
            }
            None => Ok(None),
        }
    }
}

/// Thread-safe user repository
pub struct ThreadSafeUserRepository {
    store: Arc<ThreadSafeRocksDBStore>,
}

impl ThreadSafeUserRepository {
    /// Create a new thread-safe user repository
    pub fn new(path: &str) -> Self {
        let store = Arc::new(ThreadSafeRocksDBStore::new(path));
        Self { store }
    }

    /// Find all users
    pub async fn find_all(&self) -> Result<Vec<User>, crate::error::GetError> {
        let scan_input = ScanInput {
            start_key: None,
            end_key: None,
            limit: None,
            reverse: false,
        };
        
        let scan_output = self.store.scan(CF_USERS, scan_input)
            .map_err(|e| GetError::Storage(e.to_string()))?;
        
        let mut users = Vec::with_capacity(scan_output.items.len());
        
        for (_, value) in scan_output.items {
            let user: User = bincode::deserialize(&value)
                .map_err(|e| GetError::Deserialization(e.to_string()))?;
            users.push(user);
        }
        
        Ok(users)
    }

    /// Create a new user
    pub async fn create(&self, user: &User) -> Result<(), crate::error::PutError> {
        let key = user.id.as_bytes();
        let value = bincode::serialize(user)
            .map_err(|e| PutError::Serialization(e.to_string()))?;
        
        self.store.put(CF_USERS, PutInput { key: key.to_vec(), value })
            .map_err(|e| PutError::Storage(e.to_string()))?;
        
        Ok(())
    }

    /// Get a user by ID
    pub async fn get(&self, id: &str) -> Result<Option<User>, crate::error::GetError> {
        let key = id.as_bytes();
        
        let value = self.store.get(CF_USERS, key)
            .map_err(|e| GetError::Storage(e.to_string()))?;
        
        match value {
            Some(data) => {
                let user = bincode::deserialize(&data)
                    .map_err(|e| GetError::Deserialization(e.to_string()))?;
                Ok(Some(user))
            }
            None => Ok(None),
        }
    }

    /// Update a user
    pub async fn update(&self, user: &User) -> Result<(), crate::error::PutError> {
        self.create(user).await
    }

    /// Delete a user by ID
    pub async fn delete(&self, id: &str) -> Result<Option<User>, crate::error::DeleteError> {
        let key = id.as_bytes();
        
        let value = self.store.delete(CF_USERS, key)
            .map_err(|e| DeleteError::Storage(e.to_string()))?;
        
        match value {
            Some(data) => {
                let user = bincode::deserialize(&data)
                    .map_err(|e| DeleteError::Deserialization(e.to_string()))?;
                Ok(Some(user))
            }
            None => Ok(None),
        }
    }
}
