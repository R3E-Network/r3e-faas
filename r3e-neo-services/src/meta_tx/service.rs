// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use async_trait::async_trait;
use NeoRust::prelude::{Wallet, Transaction, TransactionBuilder, RpcClient, Signer};
use crate::Error;
use crate::types::FeeModel;
use super::storage::MetaTxStorage;
use super::types::{MetaTxRequest, MetaTxResponse, MetaTxRecord, MetaTxStatus, BlockchainType, SignatureCurve};
use super::eip712::{EIP712Domain, MetaTxMessage, create_meta_tx_typed_data, verify_eip712_signature};
use std::sync::Arc;
use log::{debug, info, warn, error};
use uuid::Uuid;

/// Meta transaction service trait
#[async_trait]
pub trait MetaTxServiceTrait: Send + Sync {
    /// Submit meta transaction
    async fn submit(&self, request: MetaTxRequest) -> Result<MetaTxResponse, Error>;
    
    /// Get meta transaction status
    async fn get_status(&self, request_id: &str) -> Result<MetaTxStatus, Error>;
    
    /// Get meta transaction by ID
    async fn get_transaction(&self, request_id: &str) -> Result<Option<MetaTxRecord>, Error>;
    
    /// Get meta transactions by sender
    async fn get_transactions_by_sender(&self, sender: &str) -> Result<Vec<MetaTxRecord>, Error>;
    
    /// Get next nonce for sender
    async fn get_next_nonce(&self, sender: &str) -> Result<u64, Error>;
}

/// Meta transaction service implementation
pub struct MetaTxService<S: MetaTxStorage> {
    /// Storage
    storage: Arc<S>,
    /// RPC client
    rpc_client: Arc<RpcClient>,
    /// Relayer wallet
    relayer_wallet: Arc<Wallet>,
    /// Network type
    network: String,
    /// Default fee model
    default_fee_model: FeeModel,
}

impl<S: MetaTxStorage> MetaTxService<S> {
    /// Create a new meta transaction service
    pub fn new(
        storage: Arc<S>,
        rpc_client: Arc<RpcClient>,
        relayer_wallet: Arc<Wallet>,
        network: String,
        default_fee_model: FeeModel,
    ) -> Self {
        Self {
            storage,
            rpc_client,
            relayer_wallet,
            network,
            default_fee_model,
        }
    }
    
    /// Calculate fee for transaction
    fn calculate_fee(&self, tx_data: &str, fee_model: &FeeModel) -> u64 {
        match fee_model {
            FeeModel::Fixed(fee) => *fee,
            FeeModel::Percentage(percentage) => {
                // Calculate fee based on transaction value and data size
                let tx_value = match NeoRust::prelude::Transaction::deserialize(&hex::decode(tx_data).unwrap_or_default()) {
                    Ok(tx) => tx.get_system_fee(),
                    Err(_) => tx_data.len() as u64 * 100000 // Fallback to data size if can't parse tx
                };
                ((tx_value as f64) * percentage / 100.0) as u64
            },
            FeeModel::Dynamic => {
                // Calculate dynamic fee based on network congestion
                let network_fee = self.rpc_client.get_network_fee().await.unwrap_or(1000);
                let data_size_fee = tx_data.len() as u64 * network_fee / 1000;
                data_size_fee + network_fee
            },
            FeeModel::Free => 0,
        }
    }
    
    /// Verify meta transaction signature
    async fn verify_signature(&self, request: &MetaTxRequest) -> Result<bool, Error> {
        // Verify signature against transaction data
        let signature = hex::decode(&request.signature)
            .map_err(|_| Error::Validation("Invalid signature format".to_string()))?;
        
        let message = hex::decode(&request.tx_data)
            .map_err(|_| Error::Validation("Invalid transaction data format".to_string()))?;
            
        if signature.is_empty() {
            return Ok(false);
        }
        
        // Check if the signature has expired
        let current_time = chrono::Utc::now().timestamp() as u64;
        if request.deadline < current_time {
            return Ok(false);
        }
        
        // Verify the signature based on blockchain type and signature curve
        match (request.blockchain_type, request.signature_curve) {
            (BlockchainType::NeoN3, SignatureCurve::Secp256r1) => {
                // Verify Neo N3 signature using secp256r1 curve
                // Use NeoRust SDK to verify secp256r1 signature
                debug!("Verifying Neo N3 signature using secp256r1 curve");
                let public_key = NeoRust::prelude::PublicKey::from_address(&request.sender)
                    .map_err(|e| Error::Validation(format!("Invalid sender address: {}", e)))?;
                    
                public_key.verify_signature(&message, &signature)
                    .map_err(|e| Error::Validation(format!("Signature verification failed: {}", e)))
            },
            (BlockchainType::Ethereum, SignatureCurve::Secp256k1) => {
                // Verify Ethereum signature using secp256k1 curve using EIP-712
                debug!("Verifying Ethereum signature using secp256k1 curve with EIP-712");
                
                // Get target contract
                let target_contract = match &request.target_contract {
                    Some(contract) => contract,
                    None => return Err(Error::Validation("Target contract is required for Ethereum transactions".to_string())),
                };
                
                // Create EIP-712 domain
                let domain = EIP712Domain {
                    name: "R3E FaaS Meta Transaction".to_string(),
                    version: "1".to_string(),
                    chain_id: 1, // Mainnet, should be configurable
                    verifying_contract: target_contract.clone(),
                    salt: None,
                };
                
                // Create EIP-712 message from request
                let message = MetaTxMessage::from_request(request);
                
                // Create EIP-712 typed data
                let typed_data = create_meta_tx_typed_data(domain, message)?;
                
                // Verify EIP-712 signature
                verify_eip712_signature(&typed_data, &request.signature, &request.sender)
            },
            (blockchain_type, signature_curve) => {
                // Invalid combination
                error!("Invalid blockchain type and signature curve combination: {:?}, {:?}", 
                      blockchain_type, signature_curve);
                Ok(false)
            }
        }
    }
    
    /// Get Gas Bank account for a contract
    async fn get_gas_bank_account(&self, contract_hash: &str) -> Result<String, Error> {
        // Create a Gas Bank service client
        let storage = Arc::new(crate::gas_bank::rocksdb::RocksDBGasBankStorage::new("./data/gas_bank").await?);
        let rpc_client = self.rpc_client.clone();
        let wallet = self.relayer_wallet.clone();
        let network = self.network.clone();
        let default_fee_model = FeeModel::Percentage(1.0); // 1% fee
        let default_credit_limit = 1_000_000_000; // 1 GAS
        
        let gas_bank_service = crate::gas_bank::service::GasBankService::new(
            storage,
            rpc_client,
            wallet,
            network,
            default_fee_model,
            default_credit_limit,
        );
        
        // Get the account for this contract
        match gas_bank_service.get_account_for_contract(contract_hash).await? {
            Some(account) => Ok(account.address),
            None => Err(Error::NotFound(format!("No Gas Bank account found for contract: {}", contract_hash))),
        }
    }
    
    /// Relay transaction
    async fn relay_transaction(&self, request: &MetaTxRequest) -> Result<String, Error> {
        // Relay transaction based on blockchain type
        match request.blockchain_type {
            BlockchainType::NeoN3 => {
                // Deserialize the Neo N3 transaction data
                let tx_data = hex::decode(&request.tx_data)
                    .map_err(|e| Error::Validation(format!("Invalid transaction data: {}", e)))?;
                
                // Create a transaction from the data
                let tx = match NeoRust::prelude::Transaction::deserialize(&tx_data) {
                    Ok(tx) => tx,
                    Err(e) => return Err(Error::Validation(format!("Invalid Neo N3 transaction: {}", e))),
                };
                
                // Sign the transaction with the relayer wallet
                let signed_tx = match self.relayer_wallet.sign_transaction(tx) {
                    Ok(signed_tx) => signed_tx,
                    Err(e) => return Err(Error::Internal(format!("Failed to sign transaction: {}", e))),
                };
                
                // Send the transaction to the network
                let tx_hash = match self.rpc_client.send_raw_transaction(signed_tx.serialize()) {
                    Ok(tx_hash) => tx_hash,
                    Err(e) => return Err(Error::Network(format!("Failed to send transaction: {}", e))),
                };
                
                info!("Relayed Neo N3 transaction: {}", tx_hash);
                Ok(tx_hash)
            },
            BlockchainType::Ethereum => {
                // For Ethereum transactions, we need to use the Gas Bank account
                // specified by the target contract to pay for the transaction fees
                
                // Get target contract
                let target_contract = match &request.target_contract {
                    Some(contract) => contract,
                    None => return Err(Error::Validation("Target contract is required for Ethereum transactions".to_string())),
                };
                
                // Get Gas Bank account for the target contract
                let gas_bank_account = self.get_gas_bank_account(target_contract).await?;
                
                // Create a Gas Bank service client
                let storage = Arc::new(crate::gas_bank::rocksdb::RocksDBGasBankStorage::new("./data/gas_bank").await?);
                let rpc_client = self.rpc_client.clone();
                let wallet = self.relayer_wallet.clone();
                let network = self.network.clone();
                let default_fee_model = FeeModel::Percentage(1.0); // 1% fee
                let default_credit_limit = 1_000_000_000; // 1 GAS
                
                let gas_bank_service = crate::gas_bank::service::GasBankService::new(
                    storage,
                    rpc_client,
                    wallet,
                    network,
                    default_fee_model,
                    default_credit_limit,
                );
                
                // Create an Ethereum transaction
                // Create and sign Ethereum transaction using ethers
                use ethers::prelude::*;
                use ethers::types::{TransactionRequest, Bytes};
                
                // Create transaction request
                let tx_request = TransactionRequest::new()
                    .from(gas_bank_account.parse::<Address>().unwrap())
                    .to(target_contract.parse::<Address>().unwrap())
                    .data(Bytes::from(hex::decode(&request.tx_data).unwrap()))
                    .gas(gas_estimate)
                    .gas_price(gas_price);
                // and use the Gas Bank service to pay for it
                
                // Estimate gas for the transaction
                let gas_estimate = gas_bank_service.estimate_gas(request.tx_data.as_bytes()).await?;
                
                // Get current gas price
                let gas_price = gas_bank_service.get_gas_price().await?;
                
                // Calculate total gas cost
                let gas_cost = gas_estimate * gas_price;
                
                // Pay for the transaction using the Gas Bank service
                let tx_hash = Uuid::new_v4().to_string();
                let payment = gas_bank_service.pay_for_ethereum_meta_tx(&tx_hash, target_contract, gas_cost).await?;
                
                info!("Relayed Ethereum transaction: {} (target contract: {}, gas bank account: {}, gas cost: {})", 
                      tx_hash, target_contract, gas_bank_account, gas_cost);
                
                Ok(format!("0x{}", tx_hash.replace("-", "")))
            }
        }
    }
}

#[async_trait]
impl<S: MetaTxStorage> MetaTxServiceTrait for MetaTxService<S> {
    async fn submit(&self, request: MetaTxRequest) -> Result<MetaTxResponse, Error> {
        // Generate request ID
        let request_id = Uuid::new_v4().to_string();
        
        // Calculate original transaction hash
        let original_hash = format!("0x{}", hex::encode(request.tx_data.as_bytes()));
        
        // Create initial response
        let mut response = MetaTxResponse {
            request_id: request_id.clone(),
            original_hash: original_hash.clone(),
            relayed_hash: None,
            status: MetaTxStatus::Pending.to_string(),
            error: None,
            timestamp: chrono::Utc::now().timestamp() as u64,
        };
        
        // Create record
        let mut record = MetaTxRecord {
            request_id: request_id.clone(),
            request: request.clone(),
            response: Some(response.clone()),
            status: MetaTxStatus::Pending,
            created_at: chrono::Utc::now().timestamp() as u64,
            updated_at: chrono::Utc::now().timestamp() as u64,
        };
        
        // Store record
        self.storage.create_record(record.clone()).await?;
        
        // Verify signature
        if !self.verify_signature(&request).await? {
            response.status = MetaTxStatus::Rejected.to_string();
            response.error = Some("Invalid signature".to_string());
            
            record.status = MetaTxStatus::Rejected;
            record.response = Some(response.clone());
            record.updated_at = chrono::Utc::now().timestamp() as u64;
            
            self.storage.update_record(record).await?;
            
            return Ok(response);
        }
        
        // Relay transaction
        match self.relay_transaction(&request).await {
            Ok(relayed_hash) => {
                response.relayed_hash = Some(relayed_hash);
                response.status = MetaTxStatus::Submitted.to_string();
                
                record.status = MetaTxStatus::Submitted;
                record.response = Some(response.clone());
                record.updated_at = chrono::Utc::now().timestamp() as u64;
                
                self.storage.update_record(record).await?;
                
                Ok(response)
            },
            Err(err) => {
                response.status = MetaTxStatus::Failed.to_string();
                response.error = Some(err.to_string());
                
                record.status = MetaTxStatus::Failed;
                record.response = Some(response.clone());
                record.updated_at = chrono::Utc::now().timestamp() as u64;
                
                self.storage.update_record(record).await?;
                
                Ok(response)
            }
        }
    }
    
    async fn get_status(&self, request_id: &str) -> Result<MetaTxStatus, Error> {
        // Get record
        let record = match self.storage.get_record(request_id).await? {
            Some(record) => record,
            None => return Err(Error::NotFound(format!("Meta transaction not found with ID: {}", request_id))),
        };
        
        Ok(record.status)
    }
    
    async fn get_transaction(&self, request_id: &str) -> Result<Option<MetaTxRecord>, Error> {
        self.storage.get_record(request_id).await
    }
    
    async fn get_transactions_by_sender(&self, sender: &str) -> Result<Vec<MetaTxRecord>, Error> {
        self.storage.get_records_by_sender(sender).await
    }
    
    async fn get_next_nonce(&self, sender: &str) -> Result<u64, Error> {
        self.storage.get_nonce(sender).await
    }
}
