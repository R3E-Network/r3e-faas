// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use async_trait::async_trait;
use NeoRust::prelude::{Wallet, Transaction, TransactionBuilder, RpcClient, Signer};
use crate::Error;
use super::storage::AbstractAccountStorage;
use super::types::{
    AbstractAccount, AccountCreationRequest, AccountOperation, AccountOperationRequest,
    AccountOperationResponse, AccountOperationRecord, AccountController, AccountPolicy,
    AccountStatus, OperationStatus, AccountSignature
};
use std::sync::Arc;
use std::collections::HashMap;
use log::{debug, info, warn, error};
use uuid::Uuid;

/// Abstract account service trait
#[async_trait]
pub trait AbstractAccountServiceTrait: Send + Sync {
    /// Create abstract account
    async fn create_account(&self, request: AccountCreationRequest) -> Result<AbstractAccount, Error>;
    
    /// Get abstract account by address
    async fn get_account(&self, address: &str) -> Result<Option<AbstractAccount>, Error>;
    
    /// Get abstract accounts by owner
    async fn get_accounts_by_owner(&self, owner: &str) -> Result<Vec<AbstractAccount>, Error>;
    
    /// Execute account operation
    async fn execute_operation(&self, request: AccountOperationRequest) -> Result<AccountOperationResponse, Error>;
    
    /// Get operation status
    async fn get_operation_status(&self, request_id: &str) -> Result<OperationStatus, Error>;
    
    /// Get operation by ID
    async fn get_operation(&self, request_id: &str) -> Result<Option<AccountOperationRecord>, Error>;
    
    /// Get operations by account
    async fn get_operations_by_account(&self, address: &str) -> Result<Vec<AccountOperationRecord>, Error>;
    
    /// Get next nonce for account
    async fn get_next_nonce(&self, address: &str) -> Result<u64, Error>;
}

/// Abstract account service implementation
pub struct AbstractAccountService<S: AbstractAccountStorage> {
    /// Storage
    storage: Arc<S>,
    /// RPC client
    rpc_client: Arc<RpcClient>,
    /// Factory wallet
    factory_wallet: Arc<Wallet>,
    /// Network type
    network: String,
    /// Factory contract hash
    factory_contract_hash: String,
}

impl<S: AbstractAccountStorage> AbstractAccountService<S> {
    /// Create a new abstract account service
    pub fn new(
        storage: Arc<S>,
        rpc_client: Arc<RpcClient>,
        factory_wallet: Arc<Wallet>,
        network: String,
        factory_contract_hash: String,
    ) -> Self {
        Self {
            storage,
            rpc_client,
            factory_wallet,
            network,
            factory_contract_hash,
        }
    }
    
    /// Verify signature
    async fn verify_signature(&self, address: &str, data: &[u8], signature: &str) -> Result<bool, Error> {
        // Verify the signature using NeoRust SDK
        if signature.is_empty() {
            return Ok(false);
        }

        // Parse the signature
        let sig_bytes = hex::decode(signature.trim_start_matches("0x"))
            .map_err(|e| Error::InvalidSignature(format!("Invalid signature format: {}", e)))?;

        // Get the public key from the address
        let public_key = self.rpc_client.get_account_publickey(address)
            .await
            .map_err(|e| Error::InvalidSignature(format!("Failed to get public key: {}", e)))?;

        // Verify the signature using NeoRust
        NeoRust::crypto::verify_signature(data, &sig_bytes, &public_key)
            .map_err(|e| Error::InvalidSignature(format!("Signature verification failed: {}", e)))
    }
    
    /// Verify operation signatures
    async fn verify_operation_signatures(&self, account: &AbstractAccount, request: &AccountOperationRequest) -> Result<bool, Error> {
        // Check if the operation has expired
        let current_time = chrono::Utc::now().timestamp() as u64;
        if request.deadline < current_time {
            return Ok(false);
        }
        
        // Get required signatures count from policy
        let required_signatures = account.policy.required_signatures;
        
        // Count valid signatures
        let mut valid_signatures = 0;
        
        // Check each signature
        for signature in &request.signatures {
            // Find controller
            if let Some(controller) = account.controllers.iter().find(|c| c.address == signature.signer) {
                // Verify signature
                let data = serde_json::to_vec(&request.operation)?;
                if self.verify_signature(&signature.signer, &data, &signature.signature).await? {
                    // Add controller weight to valid signatures
                    valid_signatures += controller.weight;
                }
            }
        }
        
        // Check if we have enough valid signatures
        Ok(valid_signatures >= required_signatures)
    }
    
    /// Deploy account contract
    async fn deploy_account_contract(&self, owner: &str, controllers: &[AccountController], policy: &AccountPolicy) -> Result<String, Error> {
        // Deploy the account contract using NeoRust SDK
        let factory_contract = self.rpc_client.get_contract(&self.factory_contract_hash)
            .await
            .map_err(|e| Error::ContractError(format!("Failed to get factory contract: {}", e)))?;

        // Build deployment transaction
        let tx = TransactionBuilder::new()
            .script_hash(factory_contract.hash)
            .operation("deployAccount")
            .args(&[
                owner.into(),
                serde_json::to_vec(&controllers)?.into(),
                serde_json::to_vec(&policy)?.into()
            ])
            .sign(&*self.factory_wallet)
            .build()
            .map_err(|e| Error::ContractError(format!("Failed to build deployment transaction: {}", e)))?;

        // Send transaction
        let tx_hash = self.rpc_client.send_raw_transaction(tx)
            .await
            .map_err(|e| Error::ContractError(format!("Failed to send deployment transaction: {}", e)))?;

        // Wait for transaction to be confirmed
        let tx_info = self.rpc_client.wait_for_transaction(&tx_hash, 60, 1)
            .await
            .map_err(|e| Error::ContractError(format!("Failed to confirm deployment: {}", e)))?;

        // Get contract hash from transaction
        let contract_hash = tx_info.contract_hash
            .ok_or_else(|| Error::ContractError("No contract hash in deployment response".to_string()))?;

        Ok(contract_hash)
    }
    
    /// Execute operation on account contract
    async fn execute_contract_operation(&self, account: &AbstractAccount, operation: &AccountOperation) -> Result<String, Error> {
        // Get the account contract
        let contract = self.rpc_client.get_contract(&account.contract_hash)
            .await
            .map_err(|e| Error::ContractError(format!("Failed to get account contract: {}", e)))?;

        // Build operation transaction based on operation type
        let tx = match operation {
            AccountOperation::AddController { address, weight } => {
                TransactionBuilder::new()
                    .script_hash(contract.hash)
                    .operation("addController")
                    .args(&[address.into(), (*weight as u64).into()])
                    .sign(&*self.factory_wallet)
                    .build()
            },
            AccountOperation::RemoveController { address } => {
                TransactionBuilder::new()
                    .script_hash(contract.hash)
                    .operation("removeController")
                    .args(&[address.into()])
                    .sign(&*self.factory_wallet)
                    .build()
            },
            AccountOperation::UpdatePolicy { policy } => {
                TransactionBuilder::new()
                    .script_hash(contract.hash)
                    .operation("updatePolicy")
                    .args(&[serde_json::to_vec(policy)?.into()])
                    .sign(&*self.factory_wallet)
                    .build()
            },
            AccountOperation::Recover { new_owner } => {
                TransactionBuilder::new()
                    .script_hash(contract.hash)
                    .operation("recover")
                    .args(&[new_owner.into()])
                    .sign(&*self.factory_wallet)
                    .build()
            },
            AccountOperation::Custom { method, args } => {
                TransactionBuilder::new()
                    .script_hash(contract.hash)
                    .operation(method)
                    .args(args)
                    .sign(&*self.factory_wallet)
                    .build()
            },
        }.map_err(|e| Error::ContractError(format!("Failed to build operation transaction: {}", e)))?;

        // Send transaction
        let tx_hash = self.rpc_client.send_raw_transaction(tx)
            .await
            .map_err(|e| Error::ContractError(format!("Failed to send operation transaction: {}", e)))?;

        // Wait for transaction to be confirmed
        self.rpc_client.wait_for_transaction(&tx_hash, 60, 1)
            .await
            .map_err(|e| Error::ContractError(format!("Failed to confirm operation: {}", e)))?;

        Ok(tx_hash)
    }
}

#[async_trait]
impl<S: AbstractAccountStorage> AbstractAccountServiceTrait for AbstractAccountService<S> {
    async fn create_account(&self, request: AccountCreationRequest) -> Result<AbstractAccount, Error> {
        // Verify signature
        let data = serde_json::to_vec(&request)?;
        if !self.verify_signature(&request.owner, &data, &request.signature).await? {
            return Err(Error::InvalidSignature("Invalid signature for account creation".to_string()));
        }
        
        // Deploy account contract
        let contract_hash = self.deploy_account_contract(&request.owner, &request.controllers, &request.policy).await?;
        
        // Generate account address
        let address = format!("neo-{}", Uuid::new_v4().to_string());
        
        // Create account
        let account = AbstractAccount {
            address: address.clone(),
            owner: request.owner.clone(),
            controllers: request.controllers.clone(),
            recovery_addresses: request.recovery_addresses.clone(),
            policy: request.policy.clone(),
            contract_hash,
            created_at: chrono::Utc::now().timestamp() as u64,
            status: AccountStatus::Active.to_string(),
            metadata: request.metadata.clone(),
        };
        
        // Store account
        self.storage.create_account(account.clone()).await?;
        
        Ok(account)
    }
    
    async fn get_account(&self, address: &str) -> Result<Option<AbstractAccount>, Error> {
        self.storage.get_account(address).await
    }
    
    async fn get_accounts_by_owner(&self, owner: &str) -> Result<Vec<AbstractAccount>, Error> {
        self.storage.get_accounts_by_owner(owner).await
    }
    
    async fn execute_operation(&self, request: AccountOperationRequest) -> Result<AccountOperationResponse, Error> {
        // Generate request ID
        let request_id = Uuid::new_v4().to_string();
        
        // Create initial response
        let mut response = AccountOperationResponse {
            request_id: request_id.clone(),
            account_address: request.account_address.clone(),
            operation: request.operation.clone(),
            tx_hash: None,
            status: OperationStatus::Pending.to_string(),
            error: None,
            timestamp: chrono::Utc::now().timestamp() as u64,
        };
        
        // Create record
        let mut record = AccountOperationRecord {
            request_id: request_id.clone(),
            account_address: request.account_address.clone(),
            request: request.clone(),
            response: Some(response.clone()),
            status: OperationStatus::Pending,
            created_at: chrono::Utc::now().timestamp() as u64,
            updated_at: chrono::Utc::now().timestamp() as u64,
        };
        
        // Store record
        self.storage.create_operation_record(record.clone()).await?;
        
        // Get account
        let account = match self.storage.get_account(&request.account_address).await? {
            Some(account) => account,
            None => {
                response.status = OperationStatus::Rejected.to_string();
                response.error = Some(format!("Account not found: {}", request.account_address));
                
                record.status = OperationStatus::Rejected;
                record.response = Some(response.clone());
                record.updated_at = chrono::Utc::now().timestamp() as u64;
                
                self.storage.update_operation_record(record).await?;
                
                return Ok(response);
            }
        };
        
        // Check account status
        if account.status != AccountStatus::Active.to_string() {
            response.status = OperationStatus::Rejected.to_string();
            response.error = Some(format!("Account is not active: {}", account.status));
            
            record.status = OperationStatus::Rejected;
            record.response = Some(response.clone());
            record.updated_at = chrono::Utc::now().timestamp() as u64;
            
            self.storage.update_operation_record(record).await?;
            
            return Ok(response);
        }
        
        // Verify signatures
        if !self.verify_operation_signatures(&account, &request).await? {
            response.status = OperationStatus::Rejected.to_string();
            response.error = Some("Invalid signatures".to_string());
            
            record.status = OperationStatus::Rejected;
            record.response = Some(response.clone());
            record.updated_at = chrono::Utc::now().timestamp() as u64;
            
            self.storage.update_operation_record(record).await?;
            
            return Ok(response);
        }
        
        // Execute operation
        match self.execute_contract_operation(&account, &request.operation).await {
            Ok(tx_hash) => {
                response.tx_hash = Some(tx_hash);
                response.status = OperationStatus::Submitted.to_string();
                
                record.status = OperationStatus::Submitted;
                record.response = Some(response.clone());
                record.updated_at = chrono::Utc::now().timestamp() as u64;
                
                self.storage.update_operation_record(record).await?;
                
                // Update account if needed
                match &request.operation {
                    AccountOperation::AddController { address, weight } => {
                        let mut updated_account = account.clone();
                        updated_account.controllers.push(AccountController {
                            address: address.clone(),
                            weight: *weight,
                            controller_type: "standard".to_string(),
                            added_at: chrono::Utc::now().timestamp() as u64,
                            status: "active".to_string(),
                        });
                        self.storage.update_account(updated_account).await?;
                    },
                    AccountOperation::RemoveController { address } => {
                        let mut updated_account = account.clone();
                        updated_account.controllers.retain(|c| c.address != *address);
                        self.storage.update_account(updated_account).await?;
                    },
                    AccountOperation::UpdatePolicy { policy } => {
                        let mut updated_account = account.clone();
                        updated_account.policy = policy.clone();
                        self.storage.update_account(updated_account).await?;
                    },
                    AccountOperation::Recover { new_owner } => {
                        let mut updated_account = account.clone();
                        updated_account.owner = new_owner.clone();
                        updated_account.status = AccountStatus::Recovered.to_string();
                        self.storage.update_account(updated_account).await?;
                    },
                    _ => {}
                }
                
                Ok(response)
            },
            Err(err) => {
                response.status = OperationStatus::Failed.to_string();
                response.error = Some(err.to_string());
                
                record.status = OperationStatus::Failed;
                record.response = Some(response.clone());
                record.updated_at = chrono::Utc::now().timestamp() as u64;
                
                self.storage.update_operation_record(record).await?;
                
                Ok(response)
            }
        }
    }
    
    async fn get_operation_status(&self, request_id: &str) -> Result<OperationStatus, Error> {
        // Get record
        let record = match self.storage.get_operation_record(request_id).await? {
            Some(record) => record,
            None => return Err(Error::NotFound(format!("Operation not found with ID: {}", request_id))),
        };
        
        Ok(record.status)
    }
    
    async fn get_operation(&self, request_id: &str) -> Result<Option<AccountOperationRecord>, Error> {
        self.storage.get_operation_record(request_id).await
    }
    
    async fn get_operations_by_account(&self, address: &str) -> Result<Vec<AccountOperationRecord>, Error> {
        self.storage.get_operation_records_by_account(address).await
    }
    
    async fn get_next_nonce(&self, address: &str) -> Result<u64, Error> {
        self.storage.get_next_nonce(address).await
    }
}
