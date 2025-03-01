// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

pub mod rocksdb;
pub mod service;
pub mod storage;

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};

use uuid::Uuid;

pub use crate::registry::proto::*;
use crate::registry::storage::FunctionStorage;

// Re-export generated proto types
pub mod proto {
    pub use crate::registry::*;
}

/// Function registry for managing user-provided JavaScript functions
pub struct FunctionRegistry {
    storage: Arc<RwLock<Box<dyn FunctionStorage>>>,
}

impl FunctionRegistry {
    /// Create a new function registry with the given storage backend
    pub fn new(storage: Box<dyn FunctionStorage>) -> Self {
        Self {
            storage: Arc::new(RwLock::new(storage)),
        }
    }

    /// Register a new function
    pub async fn register_function(
        &self,
        request: RegisterFunctionRequest,
    ) -> Result<RegisterFunctionResponse, RegistryError> {
        // Generate a unique ID for the function
        let id = Uuid::new_v4().to_string();

        // Get current timestamp
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        // Create function metadata
        let metadata = FunctionMetadata {
            id,
            name: request.name,
            description: request.description,
            version: 1, // Initial version
            created_at: now,
            updated_at: now,
            trigger: request.trigger,
            permissions: request.permissions,
            resources: request.resources,
            code: request.code,
        };

        // Store the function metadata
        self.storage.write().unwrap().store_function(&metadata)?;

        Ok(RegisterFunctionResponse {
            metadata: Some(metadata),
        })
    }

    /// Update an existing function
    pub async fn update_function(
        &self,
        request: UpdateFunctionRequest,
    ) -> Result<UpdateFunctionResponse, RegistryError> {
        // Get the existing function
        let mut metadata = self.storage.read().unwrap().get_function(&request.id)?;

        // Get current timestamp
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        // Update function metadata
        if let Some(name) = request.name {
            metadata.name = name;
        }

        if let Some(description) = request.description {
            metadata.description = description;
        }

        if let Some(trigger) = request.trigger {
            metadata.trigger = Some(trigger);
        }

        if let Some(permissions) = request.permissions {
            metadata.permissions = Some(permissions);
        }

        if let Some(resources) = request.resources {
            metadata.resources = Some(resources);
        }

        if let Some(code) = request.code {
            metadata.code = code;
        }

        // Increment version
        metadata.version += 1;
        metadata.updated_at = now;

        // Store the updated function metadata
        self.storage.write().unwrap().store_function(&metadata)?;

        Ok(UpdateFunctionResponse {
            metadata: Some(metadata),
        })
    }

    /// Get a function by ID
    pub async fn get_function(
        &self,
        request: GetFunctionRequest,
    ) -> Result<GetFunctionResponse, RegistryError> {
        let metadata = self.storage.read().unwrap().get_function(&request.id)?;
        Ok(GetFunctionResponse {
            metadata: Some(metadata),
        })
    }

    /// List functions with optional filtering
    pub async fn list_functions(
        &self,
        request: ListFunctionsRequest,
    ) -> Result<ListFunctionsResponse, RegistryError> {
        let functions = self.storage.read().unwrap().list_functions(
            request.page_token,
            request.page_size,
            request.trigger_type,
        )?;

        // For simplicity, we're not implementing pagination in this example
        Ok(ListFunctionsResponse {
            functions,
            next_page_token: "".to_string(),
        })
    }

    /// Delete a function by ID
    pub async fn delete_function(
        &self,
        request: DeleteFunctionRequest,
    ) -> Result<DeleteFunctionResponse, RegistryError> {
        let success = self.storage.write().unwrap().delete_function(&request.id)?;
        Ok(DeleteFunctionResponse { success })
    }
}

/// Error types for function registry operations
#[derive(Debug, thiserror::Error)]
pub enum RegistryError {
    #[error("function not found: {0}")]
    NotFound(String),

    #[error("storage error: {0}")]
    Storage(String),

    #[error("validation error: {0}")]
    Validation(String),

    #[error("internal error: {0}")]
    Internal(String),
}

impl From<std::io::Error> for RegistryError {
    fn from(err: std::io::Error) -> Self {
        RegistryError::Storage(err.to_string())
    }
}

use crate::db::DatabaseClient;
use crate::models::{Service, ServiceSignature};

use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Service registry for managing and invoking services
#[derive(Clone)]
pub struct ServiceRegistry {
    db_client: Arc<DatabaseClient>,
    service_cache: Arc<RwLock<HashMap<Uuid, Service>>>,
    cache_ttl: std::time::Duration,
    last_cache_refresh: Arc<RwLock<std::time::Instant>>,
}

impl ServiceRegistry {
    /// Create a new service registry
    pub fn new(db_client: Arc<DatabaseClient>) -> Self {
        Self {
            db_client,
            service_cache: Arc::new(RwLock::new(HashMap::new())),
            cache_ttl: std::time::Duration::from_secs(60), // 1 minute cache TTL
            last_cache_refresh: Arc::new(RwLock::new(std::time::Instant::now())),
        }
    }

    /// Get a service by ID
    pub async fn get_service(&self, service_id: &Uuid) -> Result<Option<Service>, String> {
        // Check if we need to refresh the cache
        self.maybe_refresh_cache().await?;
        
        // Try to get from cache first
        {
            let cache = self.service_cache.read().await;
            if let Some(service) = cache.get(service_id) {
                return Ok(Some(service.clone()));
            }
        }
        
        // If not in cache, get from database
        match self.db_client.get_service(service_id).await {
            Ok(Some(service)) => {
                // Add to cache
                let mut cache = self.service_cache.write().await;
                cache.insert(*service_id, service.clone());
                Ok(Some(service))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(format!("Failed to get service from database: {}", e)),
        }
    }

    /// List all services
    pub async fn list_services(&self) -> Result<Vec<Service>, String> {
        // Check if we need to refresh the cache
        self.maybe_refresh_cache().await?;
        
        // Get from cache
        let cache = self.service_cache.read().await;
        let services: Vec<Service> = cache.values().cloned().collect();
        
        // If cache is empty, get from database
        if services.is_empty() {
            match self.db_client.list_services().await {
                Ok(services) => {
                    // Add to cache
                    let mut cache = self.service_cache.write().await;
                    for service in &services {
                        cache.insert(service.id, service.clone());
                    }
                    Ok(services)
                }
                Err(e) => Err(format!("Failed to list services from database: {}", e)),
            }
        } else {
            Ok(services)
        }
    }

    /// Register a new service
    pub async fn register_service(&self, service: Service) -> Result<Uuid, String> {
        // Save to database
        match self.db_client.create_service(&service).await {
            Ok(service_id) => {
                // Add to cache
                let mut cache = self.service_cache.write().await;
                cache.insert(service_id, service);
                Ok(service_id)
            }
            Err(e) => Err(format!("Failed to register service in database: {}", e)),
        }
    }

    /// Update an existing service
    pub async fn update_service(&self, service_id: &Uuid, service: Service) -> Result<(), String> {
        // Update in database
        match self.db_client.update_service(service_id, &service).await {
            Ok(_) => {
                // Update in cache
                let mut cache = self.service_cache.write().await;
                cache.insert(*service_id, service);
                Ok(())
            }
            Err(e) => Err(format!("Failed to update service in database: {}", e)),
        }
    }

    /// Delete a service
    pub async fn delete_service(&self, service_id: &Uuid) -> Result<(), String> {
        // Delete from database
        match self.db_client.delete_service(service_id).await {
            Ok(_) => {
                // Remove from cache
                let mut cache = self.service_cache.write().await;
                cache.remove(service_id);
                Ok(())
            }
            Err(e) => Err(format!("Failed to delete service from database: {}", e)),
        }
    }

    /// Invoke a service function
    pub async fn invoke_service(
        &self,
        service_id: &Uuid,
        function_name: &str,
        parameters: &Value,
        auth_token: Option<&str>,
        signature: Option<&ServiceSignature>,
    ) -> Result<Value, String> {
        // Get the service
        let service = match self.get_service(service_id).await? {
            Some(s) => s,
            None => return Err(format!("Service not found: {}", service_id)),
        };
        
        // Check if service is enabled
        if !service.is_enabled {
            return Err(format!("Service is disabled: {}", service_id));
        }
        
        // Find the function
        let function = service.functions.iter().find(|f| f.name == function_name);
        let function = match function {
            Some(f) => f,
            None => {
                return Err(format!(
                    "Function not found: {}.{}",
                    service_id, function_name
                ))
            }
        };
        
        // Check function auth requirements
        if function.requires_auth && auth_token.is_none() {
            return Err(format!("Auth token required for function: {}.{}", service_id, function_name));
        }
        
        // Check signature requirements
        if function.requires_signature && signature.is_none() {
            return Err(format!("Signature required for function: {}.{}", service_id, function_name));
        }
        
        // Validate parameters
        for param_def in &function.parameters {
            if param_def.required {
                if let Value::Object(params) = parameters {
                    if !params.contains_key(&param_def.name) {
                        return Err(format!(
                            "Required parameter missing: {}.{}.{}",
                            service_id, function_name, param_def.name
                        ));
                    }
                } else {
                    return Err(format!(
                        "Invalid parameters format for function: {}.{}",
                        service_id, function_name
                    ));
                }
            }
        }
        
        // Execute the function based on the adapter type
        match service.adapter_type.as_str() {
            "http" => {
                self.execute_http_function(&service, function_name, parameters, auth_token, signature).await
            },
            "grpc" => {
                self.execute_grpc_function(&service, function_name, parameters, auth_token, signature).await
            },
            "blockchain" => {
                self.execute_blockchain_function(&service, function_name, parameters, auth_token, signature).await
            },
            "local" => {
                self.execute_local_function(&service, function_name, parameters, auth_token, signature).await
            },
            _ => Err(format!("Unsupported adapter type: {}", service.adapter_type)),
        }
    }
    
    /// Refresh the service cache if needed
    async fn maybe_refresh_cache(&self) -> Result<(), String> {
        let now = std::time::Instant::now();
        let should_refresh = {
            let last_refresh = self.last_cache_refresh.read().await;
            now.duration_since(*last_refresh) > self.cache_ttl
        };
        
        if should_refresh {
            // Update the last refresh time
            *self.last_cache_refresh.write().await = now;
            
            // Refresh the cache
            match self.db_client.list_services().await {
                Ok(services) => {
                    let mut cache = self.service_cache.write().await;
                    cache.clear();
                    for service in services {
                        cache.insert(service.id, service);
                    }
                    Ok(())
                }
                Err(e) => Err(format!("Failed to refresh service cache: {}", e)),
            }
        } else {
            Ok(())
        }
    }
    
    /// Execute an HTTP function
    async fn execute_http_function(
        &self,
        service: &Service,
        function_name: &str,
        parameters: &Value,
        auth_token: Option<&str>,
        signature: Option<&ServiceSignature>,
    ) -> Result<Value, String> {
        // Get the endpoint URL from the service adapter configuration
        let config = match &service.adapter_config {
            Value::Object(config) => config,
            _ => return Err("Invalid adapter configuration".to_string()),
        };
        
        let base_url = match config.get("base_url") {
            Some(Value::String(url)) => url,
            _ => return Err("Missing or invalid base_url in adapter configuration".to_string()),
        };
        
        // Find the function endpoint
        let function = service.functions.iter().find(|f| f.name == function_name).unwrap();
        let endpoint = match &function.adapter_config {
            Value::Object(config) => {
                match config.get("endpoint") {
                    Some(Value::String(endpoint)) => endpoint,
                    _ => return Err("Missing or invalid endpoint in function configuration".to_string()),
                }
            },
            _ => return Err("Invalid function adapter configuration".to_string()),
        };
        
        // Build the full URL
        let url = format!("{}{}", base_url, endpoint);
        
        // Determine the HTTP method
        let method = match &function.adapter_config {
            Value::Object(config) => {
                match config.get("method") {
                    Some(Value::String(method)) => method.to_uppercase(),
                    _ => "POST".to_string(), // Default to POST
                }
            },
            _ => "POST".to_string(),
        };
        
        // Build the request
        let client = reqwest::Client::new();
        let mut request_builder = match method.as_str() {
            "GET" => client.get(&url),
            "POST" => client.post(&url),
            "PUT" => client.put(&url),
            "DELETE" => client.delete(&url),
            "PATCH" => client.patch(&url),
            _ => return Err(format!("Unsupported HTTP method: {}", method)),
        };
        
        // Add headers
        if let Some(Value::Object(headers)) = config.get("headers") {
            for (key, value) in headers {
                if let Value::String(value) = value {
                    request_builder = request_builder.header(key, value);
                }
            }
        }
        
        // Add authentication if provided
        if let Some(token) = auth_token {
            let auth_type = match config.get("auth_type") {
                Some(Value::String(auth_type)) => auth_type,
                _ => "Bearer", // Default to Bearer
            };
            
            request_builder = request_builder.header("Authorization", format!("{} {}", auth_type, token));
        }
        
        // Add signature if provided
        if let Some(sig) = signature {
            // Add custom headers for signature verification
            request_builder = request_builder.header("X-Signature", &sig.signature);
            request_builder = request_builder.header("X-Address", &sig.address);
            request_builder = request_builder.header("X-Blockchain-Type", &sig.blockchain_type);
            if let Some(curve) = &sig.signature_curve {
                request_builder = request_builder.header("X-Signature-Curve", curve);
            }
        }
        
        // Add parameters
        let request = if method == "GET" {
            if let Value::Object(params) = parameters {
                let mut query_params = Vec::new();
                for (key, value) in params {
                    if let Value::String(value) = value {
                        query_params.push((key, value));
                    } else {
                        query_params.push((key, &value.to_string()));
                    }
                }
                request_builder.query(&query_params)
            } else {
                request_builder
            }
        } else {
            request_builder.json(parameters)
        };
        
        // Send the request
        match request.send().await {
            Ok(response) => {
                // Check for successful status code
                if !response.status().is_success() {
                    return Err(format!(
                        "HTTP request failed with status: {}",
                        response.status()
                    ));
                }
                
                // Parse the response body as JSON
                match response.json::<Value>().await {
                    Ok(result) => Ok(result),
                    Err(e) => Err(format!("Failed to parse HTTP response: {}", e)),
                }
            }
            Err(e) => Err(format!("HTTP request failed: {}", e)),
        }
    }
    
    /// Execute a gRPC function
    async fn execute_grpc_function(
        &self,
        service: &Service,
        function_name: &str,
        parameters: &Value,
        _auth_token: Option<&str>,
        _signature: Option<&ServiceSignature>,
    ) -> Result<Value, String> {
        // Get the endpoint URL from the service adapter configuration
        let config = match &service.adapter_config {
            Value::Object(config) => config,
            _ => return Err("Invalid adapter configuration".to_string()),
        };
        
        let endpoint = match config.get("endpoint") {
            Some(Value::String(url)) => url,
            _ => return Err("Missing or invalid endpoint in adapter configuration".to_string()),
        };
        
        // Find the service and method names
        let function = service.functions.iter().find(|f| f.name == function_name).unwrap();
        let grpc_service = match &function.adapter_config {
            Value::Object(config) => {
                match config.get("service") {
                    Some(Value::String(service)) => service,
                    _ => return Err("Missing or invalid gRPC service name in function configuration".to_string()),
                }
            },
            _ => return Err("Invalid function adapter configuration".to_string()),
        };
        
        let grpc_method = match &function.adapter_config {
            Value::Object(config) => {
                match config.get("method") {
                    Some(Value::String(method)) => method,
                    _ => return Err("Missing or invalid gRPC method name in function configuration".to_string()),
                }
            },
            _ => return Err("Invalid function adapter configuration".to_string()),
        };
        
        // Use tonic to create a gRPC client and make the call
        // For a real implementation, we would need to use reflection or generated code
        // This is a simplified version that uses the gRPC reflection service
        
        // Convert parameters to bytes
        let param_bytes = match serde_json::to_vec(parameters) {
            Ok(bytes) => bytes,
            Err(e) => return Err(format!("Failed to serialize parameters: {}", e)),
        };
        
        // Use the Reflection API to make a dynamic gRPC call
        // Note: In a real implementation, we would use generated code for type safety
        
        // For this simplified example, we'll use the grpcurl command-line tool
        // In a real implementation, we would use a proper gRPC client library
        use std::process::Command;
        
        let output = Command::new("grpcurl")
            .arg("-d")
            .arg(format!("'{}'", serde_json::to_string(parameters).unwrap()))
            .arg("-plaintext")
            .arg(endpoint)
            .arg(format!("{}/{}", grpc_service, grpc_method))
            .output()
            .map_err(|e| format!("Failed to execute gRPC call: {}", e))?;
        
        if !output.status.success() {
            return Err(format!(
                "gRPC call failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
        
        // Parse the response JSON
        match serde_json::from_slice::<Value>(&output.stdout) {
            Ok(result) => Ok(result),
            Err(e) => Err(format!("Failed to parse gRPC response: {}", e)),
        }
    }
    
    /// Execute a blockchain function
    async fn execute_blockchain_function(
        &self,
        service: &Service,
        function_name: &str,
        parameters: &Value,
        _auth_token: Option<&str>,
        signature: Option<&ServiceSignature>,
    ) -> Result<Value, String> {
        // Get the blockchain configuration from the service adapter configuration
        let config = match &service.adapter_config {
            Value::Object(config) => config,
            _ => return Err("Invalid adapter configuration".to_string()),
        };
        
        let blockchain_type = match config.get("blockchain_type") {
            Some(Value::String(blockchain_type)) => blockchain_type,
            _ => return Err("Missing or invalid blockchain_type in adapter configuration".to_string()),
        };
        
        let network = match config.get("network") {
            Some(Value::String(network)) => network,
            _ => return Err("Missing or invalid network in adapter configuration".to_string()),
        };
        
        // Find the contract address and method
        let function = service.functions.iter().find(|f| f.name == function_name).unwrap();
        let contract_address = match &function.adapter_config {
            Value::Object(config) => {
                match config.get("contract_address") {
                    Some(Value::String(address)) => address,
                    _ => return Err("Missing or invalid contract_address in function configuration".to_string()),
                }
            },
            _ => return Err("Invalid function adapter configuration".to_string()),
        };
        
        let contract_method = match &function.adapter_config {
            Value::Object(config) => {
                match config.get("method") {
                    Some(Value::String(method)) => method,
                    _ => return Err("Missing or invalid contract method in function configuration".to_string()),
                }
            },
            _ => return Err("Invalid function adapter configuration".to_string()),
        };
        
        // Check if this is a read-only operation
        let is_readonly = match &function.adapter_config {
            Value::Object(config) => {
                match config.get("readonly") {
                    Some(Value::Bool(readonly)) => *readonly,
                    _ => true, // Default to read-only
                }
            },
            _ => true,
        };
        
        // Execute the blockchain function based on the blockchain type
        match blockchain_type.as_str() {
            "ethereum" => {
                self.execute_ethereum_function(
                    contract_address,
                    contract_method,
                    parameters,
                    network,
                    is_readonly,
                    signature,
                ).await
            },
            "neo_n3" => {
                self.execute_neo_function(
                    contract_address,
                    contract_method,
                    parameters,
                    network,
                    is_readonly,
                    signature,
                ).await
            },
            "solana" => {
                self.execute_solana_function(
                    contract_address,
                    contract_method,
                    parameters,
                    network,
                    is_readonly,
                    signature,
                ).await
            },
            _ => Err(format!("Unsupported blockchain type: {}", blockchain_type)),
        }
    }
    
    /// Execute an Ethereum blockchain function
    async fn execute_ethereum_function(
        &self,
        contract_address: &str,
        contract_method: &str,
        parameters: &Value,
        network: &str,
        is_readonly: bool,
        signature: Option<&ServiceSignature>,
    ) -> Result<Value, String> {
        use ethers::{
            contract::{abigen, Contract},
            core::types::{Address, U256},
            providers::{Http, Provider},
            signers::{LocalWallet, Signer},
        };
        
        // Parse the contract address
        let address = contract_address.parse::<Address>()
            .map_err(|e| format!("Invalid Ethereum address: {}", e))?;
        
        // Get the RPC URL based on the network
        let rpc_url = match network {
            "mainnet" => "https://mainnet.infura.io/v3/your-project-id",
            "sepolia" => "https://sepolia.infura.io/v3/your-project-id",
            "goerli" => "https://goerli.infura.io/v3/your-project-id",
            _ => return Err(format!("Unsupported Ethereum network: {}", network)),
        };
        
        // Create a provider
        let provider = Provider::<Http>::try_from(rpc_url)
            .map_err(|e| format!("Failed to create Ethereum provider: {}", e))?;
        
        // Create a contract instance
        // For simplicity, we'll assume an ABI for common ERC20 functions
        // In a real implementation, we would use a dynamic ABI based on the contract
        abigen!(
            ERC20,
            r#"[
                function balanceOf(address owner) view returns (uint256)
                function transfer(address to, uint256 amount) returns (bool)
                function approve(address spender, uint256 amount) returns (bool)
                function allowance(address owner, address spender) view returns (uint256)
                function transferFrom(address from, address to, uint256 amount) returns (bool)
            ]"#
        );
        
        // Create a contract instance
        let contract = ERC20::new(address, Arc::new(provider.clone()));
        
        // Execute the contract method based on the name
        match contract_method {
            "balanceOf" => {
                // Get the owner address from parameters
                let owner = match parameters.get("owner") {
                    Some(Value::String(owner)) => {
                        owner.parse::<Address>()
                            .map_err(|e| format!("Invalid owner address: {}", e))?
                    },
                    _ => return Err("Missing or invalid owner parameter".to_string()),
                };
                
                // Call the balanceOf method
                match contract.balance_of(owner).call().await {
                    Ok(balance) => {
                        Ok(serde_json::json!({
                            "balance": balance.to_string()
                        }))
                    },
                    Err(e) => Err(format!("Failed to call balanceOf: {}", e)),
                }
            },
            "transfer" => {
                if is_readonly {
                    return Err("Cannot call transfer method in read-only mode".to_string());
                }
                
                // We need a signature for a write operation
                if signature.is_none() {
                    return Err("Signature required for transfer method".to_string());
                }
                
                // Get the parameters
                let to = match parameters.get("to") {
                    Some(Value::String(to)) => {
                        to.parse::<Address>()
                            .map_err(|e| format!("Invalid to address: {}", e))?
                    },
                    _ => return Err("Missing or invalid to parameter".to_string()),
                };
                
                let amount = match parameters.get("amount") {
                    Some(Value::String(amount)) => {
                        amount.parse::<U256>()
                            .map_err(|e| format!("Invalid amount: {}", e))?
                    },
                    _ => return Err("Missing or invalid amount parameter".to_string()),
                };
                
                // Get the wallet from the signature
                // This is a simplified example - in reality, we'd recover the wallet from the signature
                let wallet = LocalWallet::new(&mut rand::thread_rng());
                
                // Create a new client with the wallet
                let client = ethers::providers::SignerMiddleware::new(provider, wallet);
                let contract = ERC20::new(address, Arc::new(client));
                
                // Call the transfer method
                match contract.transfer(to, amount).send().await {
                    Ok(tx) => {
                        let tx_hash = tx.tx_hash();
                        Ok(serde_json::json!({
                            "tx_hash": format!("{:?}", tx_hash)
                        }))
                    },
                    Err(e) => Err(format!("Failed to call transfer: {}", e)),
                }
            },
            _ => Err(format!("Unsupported Ethereum contract method: {}", contract_method)),
        }
    }
    
    /// Execute a Neo N3 blockchain function
    async fn execute_neo_function(
        &self,
        contract_address: &str,
        contract_method: &str,
        parameters: &Value,
        network: &str,
        is_readonly: bool,
        signature: Option<&ServiceSignature>,
    ) -> Result<Value, String> {
        use neo::prelude::*;
        
        // Get the RPC URL based on the network
        let rpc_url = match network {
            "mainnet" => "http://seed1.neo.org:10332",
            "testnet" => "http://seed1t5.neo.org:20332",
            _ => return Err(format!("Unsupported Neo network: {}", network)),
        };
        
        // Create a provider
        let provider = HttpProvider::new(rpc_url);
        let client = ProviderClient::new(provider);
        
        // Parse the contract hash
        let contract_hash = contract_address.parse::<ScriptHash>()
            .map_err(|e| format!("Invalid Neo contract hash: {}", e))?;
        
        // Execute the contract method based on whether it's read-only or not
        if is_readonly {
            // Create a contract call for reading data
            let call_builder = client.invoke_function(contract_hash, contract_method);
            
            // Add parameters
            let call_builder = add_neo_parameters(call_builder, parameters)?;
            
            // Call the contract
            match call_builder.test_invoke().await {
                Ok(result) => {
                    // Parse the result
                    parse_neo_result(&result)
                },
                Err(e) => Err(format!("Failed to call Neo contract: {}", e)),
            }
        } else {
            // We need a signature for a write operation
            if signature.is_none() {
                return Err("Signature required for write operations".to_string());
            }
            
            // Get wallet from the signature
            // This is a simplified example - in reality, we'd recover the wallet from the signature
            let wallet = Wallet::new_from_wif("your_wif_private_key")
                .map_err(|e| format!("Failed to create Neo wallet: {}", e))?;
            
            // Create a contract call for writing data
            let call_builder = client.invoke_function(contract_hash, contract_method);
            
            // Add parameters
            let call_builder = add_neo_parameters(call_builder, parameters)?;
            
            // Send the transaction
            match call_builder.sign_and_invoke(&wallet).await {
                Ok(result) => {
                    Ok(serde_json::json!({
                        "tx_hash": result.tx_id.to_string()
                    }))
                },
                Err(e) => Err(format!("Failed to call Neo contract: {}", e)),
            }
        }
    }
    
    /// Add parameters to a Neo function call
    fn add_neo_parameters(
        mut builder: InvokeMethodBuilder,
        parameters: &Value,
    ) -> Result<InvokeMethodBuilder, String> {
        if let Value::Object(params) = parameters {
            for (key, value) in params {
                match value {
                    Value::String(s) => {
                        builder = builder.with_parameter_string(s);
                    },
                    Value::Number(n) => {
                        if n.is_i64() {
                            builder = builder.with_parameter_integer(n.as_i64().unwrap());
                        } else {
                            return Err("Only integer numbers are supported for Neo parameters".to_string());
                        }
                    },
                    Value::Bool(b) => {
                        builder = builder.with_parameter_bool(*b);
                    },
                    Value::Array(_) => {
                        return Err("Array parameters are not supported".to_string());
                    },
                    Value::Object(_) => {
                        return Err("Nested object parameters are not supported".to_string());
                    },
                    Value::Null => {
                        builder = builder.with_parameter_null();
                    },
                }
            }
        }
        
        Ok(builder)
    }
    
    /// Parse a Neo contract result
    fn parse_neo_result(result: &InvokeResult) -> Result<Value, String> {
        if !result.has_state_fault() {
            // Parse the stack items
            if let Some(stack) = result.stack() {
                if let Some(item) = stack.first() {
                    match item {
                        StackItem::Integer(n) => {
                            Ok(serde_json::json!({ "result": n.to_string() }))
                        },
                        StackItem::String(s) => {
                            Ok(serde_json::json!({ "result": s }))
                        },
                        StackItem::Bool(b) => {
                            Ok(serde_json::json!({ "result": b }))
                        },
                        _ => Err("Unsupported Neo result type".to_string()),
                    }
                } else {
                    Ok(serde_json::json!({ "result": null }))
                }
            } else {
                Ok(serde_json::json!({ "result": null }))
            }
        } else {
            Err(format!("Neo contract execution failed: {:?}", result.exception()))
        }
    }
    
    /// Execute a Solana blockchain function
    async fn execute_solana_function(
        &self,
        contract_address: &str,
        contract_method: &str,
        parameters: &Value,
        network: &str,
        is_readonly: bool,
        signature: Option<&ServiceSignature>,
    ) -> Result<Value, String> {
        // This is a simplified implementation - in reality, we'd use the Solana SDK
        
        // Get the RPC URL based on the network
        let rpc_url = match network {
            "mainnet" => "https://api.mainnet-beta.solana.com",
            "testnet" => "https://api.testnet.solana.com",
            "devnet" => "https://api.devnet.solana.com",
            _ => return Err(format!("Unsupported Solana network: {}", network)),
        };
        
        // For now, we'll just return a mock result
        Ok(serde_json::json!({
            "status": "success",
            "contract": contract_address,
            "method": contract_method,
            "is_readonly": is_readonly,
            "network": network,
            "parameters": parameters,
            "result": "Mock Solana result"
        }))
    }
    
    /// Execute a local function
    async fn execute_local_function(
        &self,
        service: &Service,
        function_name: &str,
        parameters: &Value,
        _auth_token: Option<&str>,
        _signature: Option<&ServiceSignature>,
    ) -> Result<Value, String> {
        // Get the function path from the service adapter configuration
        let config = match &service.adapter_config {
            Value::Object(config) => config,
            _ => return Err("Invalid adapter configuration".to_string()),
        };
        
        let function_path = match config.get("function_path") {
            Some(Value::String(path)) => path,
            _ => return Err("Missing or invalid function_path in adapter configuration".to_string()),
        };
        
        // Find the function configuration
        let function = service.functions.iter().find(|f| f.name == function_name).unwrap();
        let function_config = match &function.adapter_config {
            Value::Object(config) => config.clone(),
            _ => serde_json::Map::new(),
        };
        
        // For security, we only allow a set of predefined functions
        // In a real implementation, this would be more sophisticated
        match (function_path.as_str(), function_name) {
            ("examples/price_oracle", "get_price") => {
                // Call the price oracle function
                match crate::registry::examples::get_price(parameters) {
                    Ok(result) => Ok(result),
                    Err(e) => Err(e),
                }
            },
            ("examples/random_generator", "generate_random") => {
                // Call the random generator function
                match crate::registry::examples::generate_random(parameters) {
                    Ok(result) => Ok(result),
                    Err(e) => Err(e),
                }
            },
            _ => Err(format!("Unsupported local function: {}.{}", function_path, function_name)),
        }
    }
}
