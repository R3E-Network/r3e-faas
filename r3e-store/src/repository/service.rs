// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! Service repository implementation

use serde::{Deserialize, Serialize};

use crate::rocksdb::{AsyncRocksDbClient, DbError, DbResult, repository_impl};

/// Column family name for services
pub const CF_SERVICES: &str = "services";

/// Column family name for service IDs
pub const CF_SERVICE_IDS: &str = "service_ids";

/// Column family name for service names
pub const CF_SERVICE_NAMES: &str = "service_names";

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

/// Service error
#[derive(Debug)]
pub enum ServiceError {
    /// Service not found
    NotFound(String),
    
    /// Service already exists
    AlreadyExists(String),
    
    /// Service name already exists
    NameAlreadyExists(String),
    
    /// DB error
    DbError(DbError),
    
    /// Other error
    Other(String),
}

impl std::fmt::Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceError::NotFound(msg) => write!(f, "Service not found: {}", msg),
            ServiceError::AlreadyExists(msg) => write!(f, "Service already exists: {}", msg),
            ServiceError::NameAlreadyExists(msg) => write!(f, "Service name already exists: {}", msg),
            ServiceError::DbError(e) => write!(f, "Database error: {}", e),
            ServiceError::Other(msg) => write!(f, "Service error: {}", msg),
        }
    }
}

impl std::error::Error for ServiceError {}

impl From<DbError> for ServiceError {
    fn from(error: DbError) -> Self {
        ServiceError::DbError(error)
    }
}

impl From<ServiceError> for DbError {
    fn from(error: ServiceError) -> Self {
        match error {
            ServiceError::AlreadyExists(msg) => DbError::Other(format!("Service already exists: {}", msg)),
            ServiceError::NotFound(msg) => DbError::Other(format!("Service not found: {}", msg)),
            ServiceError::NameAlreadyExists(msg) => DbError::Other(format!("Service name already exists: {}", msg)),
            ServiceError::DbError(err) => err,
            ServiceError::Other(msg) => DbError::Other(format!("Service error: {}", msg)),
        }
    }
}

/// Service repository implementation
pub struct ServiceRepository {
    db: AsyncRocksDbClient,
}

impl ServiceRepository {
    /// Create a new service repository
    pub fn new(db: AsyncRocksDbClient) -> Self {
        Self { db }
    }

    /// Get the service column family name
    fn cf_name() -> String {
        "service".to_string()
    }

    /// Create a new service
    pub async fn create(&self, service: Service) -> Result<(), ServiceError> {
        // Check if service already exists by ID
        if self.exists(&service.id).await? {
            return Err(ServiceError::AlreadyExists(service.id.clone()));
        }

        // Check if service name is already taken
        if self.exists_name(&service.name).await? {
            return Err(ServiceError::NameAlreadyExists(service.name.clone()));
        }

        // Clone the service ID for indexes
        let service_id = service.id.clone();
        let service_name = service.name.clone();

        // Save the service with ownership passed
        self.db.put_cf(Self::cf_name().as_str(), service_id.clone(), service)
            .await
            .map_err(|e| ServiceError::DbError(e))?;

        // Save the name index
        self.db.put_cf(CF_SERVICE_NAMES, format!("name:{}", service_name), service_id)
            .await?;

        Ok(())
    }

    async fn get_by_id(&self, id: &str) -> Result<Service, ServiceError> {
        let id_owned = id.to_string();
        match self.db.get_cf::<_, Service>(CF_SERVICES, id_owned).await {
            Ok(Some(service)) => Ok(service),
            Ok(None) => Err(ServiceError::NotFound(id.to_string())),
            Err(e) => Err(ServiceError::DbError(e))
        }
    }

    pub async fn find_by_id(&self, id: &str) -> DbResult<Service> {
        self.get_by_id(id).await.map_err(Into::into)
    }

    /// Find a service by name
    pub async fn find_by_name(&self, name: &str) -> Result<Option<Service>, ServiceError> {
        // Convert to owned string
        let name_owned = name.to_string();
        
        // Get the service id from the name index
        let service_id = self.db.get_cf::<_, String>(CF_SERVICE_NAMES, format!("name:{}", name_owned)).await
            .map_err(|e| ServiceError::DbError(e))?;
        
        // If found, get the service by ID
        match service_id {
            Some(id) => {
                let result = self.get_by_id(&id).await?;
                Ok(Some(result))
            },
            None => Ok(None),
        }
    }

    /// Update a service
    pub async fn update(&self, service: Service) -> DbResult<()> {
        // Check if the service exists
        let existing_result = self.get_by_id(&service.id).await;
        
        match existing_result {
            Ok(existing) => {
                // Remove old name index if it's changed
                if existing.name != service.name {
                    self.db
                        .delete_cf(CF_SERVICE_NAMES, format!("name:{}", existing.name))
                        .await?;
                }
            },
            Err(ServiceError::NotFound(_)) => {
                // Service doesn't exist, that's ok for update
            },
            Err(e) => return Err(e.into()),
        }
        
        // Update name index
        self.db
            .put_cf(CF_SERVICE_NAMES, format!("name:{}", service.name), service.id.clone())
            .await?;
        
        // Update the service
        self.db.put_cf(CF_SERVICES, service.id.clone(), service).await?;
        
        Ok(())
    }

    /// Delete a service
    pub async fn delete(&self, id: &str) -> DbResult<()> {
        // Get the service to remove indexes
        let service_result = self.get_by_id(id).await;
        
        // Only proceed with deletion if service exists
        match service_result {
            Ok(service) => {
                // Remove name index
                self.db
                    .delete_cf(CF_SERVICE_NAMES, format!("name:{}", service.name))
                    .await?;
                
                // Remove the service
                self.db.delete_cf(CF_SERVICES, id.to_string()).await?;
            },
            Err(ServiceError::NotFound(_)) => {
                // Service doesn't exist, nothing to delete
            },
            Err(e) => return Err(e.into()),
        }
        
        Ok(())
    }

    /// List all services
    pub async fn list(&self) -> DbResult<Vec<Service>> {
        // We use String as our Key type since all keys are strings
        let results: Vec<(String, Service)> = self.db.collect_cf(CF_SERVICES).await?;
        
        // Filter out non-service entries (like name: indexes)
        let services = results
            .into_iter()
            .filter_map(|(key, service)| {
                if !key.contains(':') {
                    Some(service)
                } else {
                    None
                }
            })
            .collect();
        
        Ok(services)
    }

    async fn get_all(&self) -> Result<Vec<Service>, ServiceError> {
        let results = self.db.collect_cf::<Service>(CF_SERVICES).await
            .map_err(|e| ServiceError::DbError(e))?;
        let services = results.into_iter().map(|(_, service)| service).collect();
        Ok(services)
    }

    pub async fn find_all(&self) -> DbResult<Vec<Service>> {
        self.get_all().await.map_err(Into::into)
    }

    async fn exists(&self, id: &str) -> Result<bool, ServiceError> {
        let id_owned = id.to_string();
        match self.db.get_cf::<_, Service>(CF_SERVICES, id_owned).await {
            Ok(Some(_)) => Ok(true),
            Ok(None) => Ok(false),
            Err(e) => Err(ServiceError::DbError(e)),
        }
    }

    async fn exists_name(&self, name: &str) -> Result<bool, ServiceError> {
        let name_owned = name.to_string();
        match self.db.get_cf::<_, String>(CF_SERVICE_NAMES, format!("name:{}", name_owned)).await {
            Ok(Some(_)) => Ok(true),
            Ok(None) => Ok(false),
            Err(e) => Err(ServiceError::DbError(e)),
        }
    }

    async fn get_by_owner(&self, owner_id: &str) -> Result<Vec<Service>, ServiceError> {
        // First, collect all services
        let services: Vec<(String, Service)> = self.db.collect_cf(CF_SERVICES).await
            .map_err(|e| ServiceError::DbError(e))?;
        
        // Then filter by owner_id
        let owner_services = services.into_iter()
            .map(|(_, service)| service)
            .filter(|service| service.owner_id == owner_id)
            .collect();
        
        Ok(owner_services)
    }

    pub async fn find_by_owner(&self, owner_id: &str) -> DbResult<Vec<Service>> {
        self.get_by_owner(owner_id).await.map_err(Into::into)
    }
}

// Implement the DbRepository trait using the macro
repository_impl!(
    ServiceRepository,
    AsyncRocksDbClient,
    Service,
    |service: &Service| service.id.to_string()
);
