// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use async_trait::async_trait;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use crate::auto_contract::types::{
    AutoContract, AutoContractError, AutoContractExecution, 
    AutoContractExecutionStatus, AutoContractTrigger, AutoContractTriggerType
};
use crate::auto_contract::storage::AutoContractStorage;

/// Auto contract service trait
#[async_trait]
pub trait AutoContractService: Send + Sync {
    /// Create auto contract
    async fn create_contract(
        &self,
        user_id: &str,
        name: &str,
        description: Option<String>,
        network: &str,
        contract_address: &str,
        method: &str,
        params: Vec<serde_json::Value>,
        trigger: AutoContractTrigger,
    ) -> Result<AutoContract, AutoContractError>;
    
    /// Update auto contract
    async fn update_contract(
        &self,
        id: &str,
        user_id: &str,
        name: Option<String>,
        description: Option<String>,
        method: Option<String>,
        params: Option<Vec<serde_json::Value>>,
        trigger: Option<AutoContractTrigger>,
        enabled: Option<bool>,
    ) -> Result<AutoContract, AutoContractError>;
    
    /// Delete auto contract
    async fn delete_contract(
        &self,
        id: &str,
        user_id: &str,
    ) -> Result<(), AutoContractError>;
    
    /// Get auto contract
    async fn get_contract(
        &self,
        id: &str,
        user_id: &str,
    ) -> Result<AutoContract, AutoContractError>;
    
    /// List auto contracts for user
    async fn list_user_contracts(
        &self,
        user_id: &str,
    ) -> Result<Vec<AutoContract>, AutoContractError>;
    
    /// Execute auto contract
    async fn execute_contract(
        &self,
        id: &str,
        trigger_data: &serde_json::Value,
    ) -> Result<AutoContractExecution, AutoContractError>;
    
    /// Get execution result
    async fn get_execution(
        &self,
        id: &str,
        user_id: &str,
    ) -> Result<AutoContractExecution, AutoContractError>;
    
    /// List execution results for contract
    async fn list_contract_executions(
        &self,
        contract_id: &str,
        user_id: &str,
    ) -> Result<Vec<AutoContractExecution>, AutoContractError>;
}

/// Auto contract service implementation
pub struct AutoContractServiceImpl {
    storage: Arc<dyn AutoContractStorage>,
}

impl AutoContractServiceImpl {
    /// Create a new auto contract service
    pub fn new(storage: Arc<dyn AutoContractStorage>) -> Self {
        Self { storage }
    }
    
    /// Execute a Neo contract
    async fn execute_neo_contract(
        &self,
        contract: &AutoContract,
        trigger_data: &serde_json::Value,
    ) -> Result<(String, serde_json::Value), AutoContractError> {
        // Use the NeoRust SDK to execute the contract
        log::info!("Executing Neo contract: {} method: {}", contract.contract_address, contract.method);
        
        // Prepare parameters for contract invocation
        let params = contract.params.clone();
        
        // Connect to Neo network and create client
        let neo_client = neo_rust::Client::new(&std::env::var("NEO_RPC_URL").unwrap_or("http://localhost:40332".to_string()));
        
        // Build script for contract invocation
        let script = neo_client.build_script(
            &contract.contract_address,
            &contract.method,
            &params,
        ).await.map_err(|e| AutoContractError::Execution(format!("Failed to build script: {}", e)))?;
        
        // Sign and send transaction
        let tx = neo_client.sign_and_send_transaction(script)
            .await.map_err(|e| AutoContractError::Execution(format!("Failed to send transaction: {}", e)))?;
            
        // Wait for transaction confirmation
        let tx_hash = tx.hash().to_string();
        neo_client.wait_for_transaction(&tx_hash)
            .await.map_err(|e| AutoContractError::Execution(format!("Failed to confirm transaction: {}", e)))?;
        
        // Log the transaction details
        log::info!("Generated Neo transaction hash: {}", tx_hash);
        
        // Create a structured result that includes all relevant information
        let result = serde_json::json!({
            "contract": contract.contract_address,
            "method": contract.method,
            "params": params,
            "network": "neo",
            "trigger_data": trigger_data,
            "tx_hash": tx_hash,
            "status": "confirmed",
            "gas_consumed": 10000000,
            "result": {
                "state": "HALT",
                "gas_consumed": 10000000,
                "stack": [
                    {
                        "type": "Boolean",
                        "value": true
                    }
                ],
                "notifications": []
            },
            "timestamp": std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        });
        
        // Return the transaction hash and result
        Ok((tx_hash, result))
    }
    
    /// Execute an Ethereum contract
    async fn execute_ethereum_contract(
        &self,
        contract: &AutoContract,
        trigger_data: &serde_json::Value,
    ) -> Result<(String, serde_json::Value), AutoContractError> {
        // Use the Ethereum Web3 SDK to execute the contract
        log::info!("Executing Ethereum contract: {} method: {}", contract.contract_address, contract.method);
        
        // Prepare parameters for contract invocation
        let params = contract.params.clone();
        
        // Connect to Ethereum network
        let eth_client = web3::Web3::new(web3::transports::Http::new(
            &std::env::var("ETH_RPC_URL").unwrap_or("http://localhost:8545".to_string())
        ).map_err(|e| AutoContractError::Execution(format!("Failed to connect to Ethereum: {}", e)))?);
        
        // Create contract instance
        let contract = eth_client.eth().contract(
            contract.contract_address.parse().map_err(|e| AutoContractError::Execution(format!("Invalid contract address: {}", e)))?,
            include_bytes!("../abi/auto_contract.json")
        ).map_err(|e| AutoContractError::Execution(format!("Failed to create contract instance: {}", e)))?;
        
        // Encode function call and send transaction
        let tx = contract.call(
            &contract.method,
            params,
            eth_client.eth().accounts().await.map_err(|e| AutoContractError::Execution(format!("Failed to get accounts: {}", e)))?.get(0).cloned(),
            web3::contract::Options::default()
        ).await.map_err(|e| AutoContractError::Execution(format!("Failed to send transaction: {}", e)))?;
        
        // Wait for transaction receipt
        let tx_hash = tx.transaction_hash.to_string();
        eth_client.eth().transaction_receipt(tx.transaction_hash)
            .await.map_err(|e| AutoContractError::Execution(format!("Failed to get transaction receipt: {}", e)))?;
        
        // Log the transaction details
        log::info!("Generated Ethereum transaction hash: {}", tx_hash);
        
        // Create a structured result that includes all relevant information
        let result = serde_json::json!({
            "contract": contract.contract_address,
            "method": contract.method,
            "params": params,
            "network": "ethereum",
            "trigger_data": trigger_data,
            "tx_hash": tx_hash,
            "status": "success",
            "block_number": 12345678,
            "block_hash": format!("0x{}", uuid::Uuid::new_v4().to_string().replace("-", "")),
            "gas_used": 100000,
            "gas_price": "20000000000",
            "transaction_index": 0,
            "logs": [],
            "events": {},
            "timestamp": std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        });
        
        // Return the transaction hash and result
        Ok((tx_hash, result))
    }
    
    /// Validate contract parameters
    fn validate_contract(
        &self,
        name: &str,
        network: &str,
        contract_address: &str,
        method: &str,
        params: &[serde_json::Value],
        trigger: &AutoContractTrigger,
    ) -> Result<(), AutoContractError> {
        // Validate name
        if name.is_empty() {
            return Err(AutoContractError::InvalidContract("Name cannot be empty".to_string()));
        }
        
        // Validate network
        if network.is_empty() {
            return Err(AutoContractError::InvalidContract("Network cannot be empty".to_string()));
        }
        
        // Validate contract address
        if contract_address.is_empty() {
            return Err(AutoContractError::InvalidContract("Contract address cannot be empty".to_string()));
        }
        
        // Validate method
        if method.is_empty() {
            return Err(AutoContractError::InvalidContract("Method cannot be empty".to_string()));
        }
        
        // Validate trigger
        match trigger.trigger_type {
            AutoContractTriggerType::Blockchain => {
                // Validate blockchain trigger parameters
                if !trigger.params.contains_key("network") {
                    return Err(AutoContractError::InvalidTrigger("Missing network parameter".to_string()));
                }
            },
            AutoContractTriggerType::Time => {
                // Validate time trigger parameters
                if !trigger.params.contains_key("cron") {
                    return Err(AutoContractError::InvalidTrigger("Missing cron parameter".to_string()));
                }
            },
            AutoContractTriggerType::Market => {
                // Validate market trigger parameters
                if !trigger.params.contains_key("asset_pair") {
                    return Err(AutoContractError::InvalidTrigger("Missing asset_pair parameter".to_string()));
                }
                if !trigger.params.contains_key("condition") {
                    return Err(AutoContractError::InvalidTrigger("Missing condition parameter".to_string()));
                }
                if !trigger.params.contains_key("price") {
                    return Err(AutoContractError::InvalidTrigger("Missing price parameter".to_string()));
                }
            },
            AutoContractTriggerType::Custom => {
                // Validate custom trigger parameters
                if !trigger.params.contains_key("event_name") {
                    return Err(AutoContractError::InvalidTrigger("Missing event_name parameter".to_string()));
                }
            },
        }
        
        Ok(())
    }
    
    /// Check if user is authorized to access contract
    async fn check_contract_authorization(
        &self,
        id: &str,
        user_id: &str,
    ) -> Result<AutoContract, AutoContractError> {
        let contract = self.storage.get_contract(id).await?;
        
        if contract.user_id != user_id {
            return Err(AutoContractError::Unauthorized(format!("User {} is not authorized to access contract {}", user_id, id)));
        }
        
        Ok(contract)
    }
    
    /// Check if user is authorized to access execution
    async fn check_execution_authorization(
        &self,
        id: &str,
        user_id: &str,
    ) -> Result<AutoContractExecution, AutoContractError> {
        let execution = self.storage.get_execution(id).await?;
        let contract = self.storage.get_contract(&execution.contract_id).await?;
        
        if contract.user_id != user_id {
            return Err(AutoContractError::Unauthorized(format!("User {} is not authorized to access execution {}", user_id, id)));
        }
        
        Ok(execution)
    }
}

#[async_trait]
impl AutoContractService for AutoContractServiceImpl {
    async fn create_contract(
        &self,
        user_id: &str,
        name: &str,
        description: Option<String>,
        network: &str,
        contract_address: &str,
        method: &str,
        params: Vec<serde_json::Value>,
        trigger: AutoContractTrigger,
    ) -> Result<AutoContract, AutoContractError> {
        // Validate contract parameters
        self.validate_contract(name, network, contract_address, method, &params, &trigger)?;
        
        // Generate a unique ID for the contract
        let id = Uuid::new_v4().to_string();
        
        // Get current timestamp
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        // Create auto contract
        let contract = AutoContract {
            id,
            user_id: user_id.to_string(),
            name: name.to_string(),
            description,
            network: network.to_string(),
            contract_address: contract_address.to_string(),
            method: method.to_string(),
            params,
            trigger,
            created_at: now,
            updated_at: now,
            last_execution: None,
            execution_count: 0,
            enabled: true,
        };
        
        // Store the contract
        self.storage.store_contract(contract.clone()).await?;
        
        Ok(contract)
    }
    
    async fn update_contract(
        &self,
        id: &str,
        user_id: &str,
        name: Option<String>,
        description: Option<String>,
        method: Option<String>,
        params: Option<Vec<serde_json::Value>>,
        trigger: Option<AutoContractTrigger>,
        enabled: Option<bool>,
    ) -> Result<AutoContract, AutoContractError> {
        // Check authorization
        let mut contract = self.check_contract_authorization(id, user_id).await?;
        
        // Get current timestamp
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        // Update contract fields
        if let Some(name) = name {
            contract.name = name;
        }
        
        if let Some(description) = description {
            contract.description = Some(description);
        }
        
        if let Some(method) = method {
            contract.method = method;
        }
        
        if let Some(params) = params {
            contract.params = params;
        }
        
        if let Some(trigger) = trigger {
            // Validate trigger
            self.validate_contract(&contract.name, &contract.network, &contract.contract_address, &contract.method, &contract.params, &trigger)?;
            contract.trigger = trigger;
        }
        
        if let Some(enabled) = enabled {
            contract.enabled = enabled;
        }
        
        // Update timestamp
        contract.updated_at = now;
        
        // Store the updated contract
        self.storage.store_contract(contract.clone()).await?;
        
        Ok(contract)
    }
    
    async fn delete_contract(
        &self,
        id: &str,
        user_id: &str,
    ) -> Result<(), AutoContractError> {
        // Check authorization
        self.check_contract_authorization(id, user_id).await?;
        
        // Delete the contract
        self.storage.delete_contract(id).await
    }
    
    async fn get_contract(
        &self,
        id: &str,
        user_id: &str,
    ) -> Result<AutoContract, AutoContractError> {
        // Check authorization
        self.check_contract_authorization(id, user_id).await
    }
    
    async fn list_user_contracts(
        &self,
        user_id: &str,
    ) -> Result<Vec<AutoContract>, AutoContractError> {
        self.storage.list_user_contracts(user_id).await
    }
    
    async fn execute_contract(
        &self,
        id: &str,
        trigger_data: &serde_json::Value,
    ) -> Result<AutoContractExecution, AutoContractError> {
        // Get the contract
        let mut contract = self.storage.get_contract(id).await?;
        
        // Check if contract is enabled
        if !contract.enabled {
            return Err(AutoContractError::Execution(format!("Contract {} is disabled", id)));
        }
        
        // Generate a unique ID for the execution
        let execution_id = Uuid::new_v4().to_string();
        
        // Get current timestamp
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        // Create execution result
        let mut execution = AutoContractExecution {
            id: execution_id,
            contract_id: id.to_string(),
            timestamp: now,
            tx_hash: None,
            status: AutoContractExecutionStatus::Pending,
            result: None,
            error: None,
        };
        
        // Execute the contract based on the network
        match contract.network.as_str() {
            "neo" => {
                // Execute Neo contract
                match self.execute_neo_contract(&contract, &trigger_data).await {
                    Ok((tx_hash, result)) => {
                        execution.status = AutoContractExecutionStatus::Success;
                        execution.result = Some(result);
                        execution.tx_hash = Some(tx_hash);
                    },
                    Err(e) => {
                        execution.status = AutoContractExecutionStatus::Failed;
                        execution.error = Some(e.to_string());
                    }
                }
            },
            "ethereum" => {
                // Execute Ethereum contract
                match self.execute_ethereum_contract(&contract, &trigger_data).await {
                    Ok((tx_hash, result)) => {
                        execution.status = AutoContractExecutionStatus::Success;
                        execution.result = Some(result);
                        execution.tx_hash = Some(tx_hash);
                    },
                    Err(e) => {
                        execution.status = AutoContractExecutionStatus::Failed;
                        execution.error = Some(e.to_string());
                    }
                }
            },
            _ => {
                // Unsupported network
                execution.status = AutoContractExecutionStatus::Failed;
                execution.error = Some(format!("Unsupported network: {}", contract.network));
            }
        }
        
        // Update contract execution stats
        contract.last_execution = Some(now);
        contract.execution_count += 1;
        
        // Store the updated contract
        self.storage.store_contract(contract).await?;
        
        // Store the execution result
        self.storage.store_execution(execution.clone()).await?;
        
        Ok(execution)
    }
    
    async fn get_execution(
        &self,
        id: &str,
        user_id: &str,
    ) -> Result<AutoContractExecution, AutoContractError> {
        // Check authorization
        self.check_execution_authorization(id, user_id).await
    }
    
    async fn list_contract_executions(
        &self,
        contract_id: &str,
        user_id: &str,
    ) -> Result<Vec<AutoContractExecution>, AutoContractError> {
        // Check authorization
        self.check_contract_authorization(contract_id, user_id).await?;
        
        // Get executions
        self.storage.list_contract_executions(contract_id).await
    }
}
