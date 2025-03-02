// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use crate::error::Error;
use crate::gas_bank::service::{GasBankService, GasBankServiceTrait};
use crate::gas_bank::storage::GasBankStorage;
use crate::meta_tx::eip712::types::{EIP712Domain, MetaTxMessage};
use crate::meta_tx::eip712::utils::{get_typed_data, verify_eip712_signature};
use crate::meta_tx::storage::MetaTxStorage;
use crate::meta_tx::types::{BlockchainType, MetaTxRecord, MetaTxRequest, MetaTxResponse, MetaTxStatus};
use crate::types::FeeModel;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use ethers::types::transaction::eip712::Eip712;
use ethers::types::Signature;
use hex;
use log::{debug, error, info};
use neo3::neo_clients::APITrait;
use neo3::prelude::{HttpProvider, RpcClient, Wallet};
use std::sync::Arc;
use uuid::Uuid;

/// Meta transaction service trait
#[async_trait]
pub trait MetaTxServiceTrait: Send + Sync {
    /// Submit meta transaction
    async fn submit(&self, request: MetaTxRequest) -> Result<MetaTxResponse, Error>;

    /// Get meta transaction status
    async fn get_status(&self, request_id: &str) -> Result<String, Error>;

    /// Get meta transaction by ID
    async fn get_transaction(&self, request_id: &str) -> Result<Option<MetaTxRecord>, Error>;

    /// Get meta transactions by sender
    async fn get_transactions_by_sender(&self, sender: &str) -> Result<Vec<MetaTxRecord>, Error>;

    /// Get next nonce for sender
    async fn get_next_nonce(&self, sender: &str) -> Result<u64, Error>;

    /// Create gas bank service for contract
    async fn create_gas_bank_service_for_contract(&self, contract_hash: &str) -> Result<GasBankService, Error>;
}

/// Meta transaction service implementation
pub struct MetaTxService<S: MetaTxStorage> {
    /// Storage
    storage: Arc<S>,
    /// RPC client
    rpc_client: Arc<RpcClient<HttpProvider>>,
    /// Relayer wallet
    relayer_wallet: Arc<Wallet>,
    /// Network type
    network: String,
    /// Default fee model
    default_fee_model: FeeModel,
    /// Chain ID
    chain_id: u64,
    /// Gas bank storage
    gas_bank_storage: Arc<dyn GasBankStorage>,
}

impl<S: MetaTxStorage> MetaTxService<S> {
    /// Create a new meta transaction service
    pub fn new(
        storage: Arc<S>,
        rpc_client: Arc<RpcClient<HttpProvider>>,
        relayer_wallet: Arc<Wallet>,
        network: String,
        default_fee_model: FeeModel,
        chain_id: u64,
        gas_bank_storage: Arc<dyn GasBankStorage>,
    ) -> Self {
        Self {
            storage,
            rpc_client,
            relayer_wallet,
            network,
            default_fee_model,
            chain_id,
            gas_bank_storage,
        }
    }

    /// Calculate fee for meta transaction
    async fn calculate_fee(&self, tx_data: &str, fee_model: &FeeModel) -> Result<u64, Error> {
        match fee_model {
            FeeModel::Fixed(fixed_fee) => Ok(*fixed_fee),
            FeeModel::Percentage(percentage) => {
                // Calculate fee based on transaction size
                let tx_size = tx_data.len() as u64;
                let tx_base_fee = 100_000; // 0.0001 GAS
                
                // Calculate dynamic fee based on transaction size
                let size_fee = tx_size * tx_base_fee / 1000;
                
                // Add network fee
                let network_fee = self.get_estimated_fee(BlockchainType::NeoN3).await?;
                
                // Calculate percentage fee
                let fee = (size_fee + network_fee) as f64 * percentage / 100.0;
                
                Ok(fee as u64)
            }
            FeeModel::Dynamic => {
                // Calculate dynamic fee based on network congestion
                let network_fee = self.get_estimated_fee(BlockchainType::NeoN3).await?;
                let data_size_fee = tx_data.len() as u64 * network_fee / 1000;
                Ok(data_size_fee + network_fee)
            }
            FeeModel::Free => Ok(0),
        }
    }

    /// Get estimated fee
    async fn get_estimated_fee(&self, blockchain_type: BlockchainType) -> Result<u64, Error> {
        // For Neo N3, we can use the network usage as an estimate
        if blockchain_type == BlockchainType::NeoN3 {
            // Use a fixed value for now since get_network_usage is not available
            return Ok(1_000_000); // 0.001 GAS as network fee
        }
        
        // For Ethereum, we would need to get gas price and estimate gas
        // For now, return a default value
        Ok(100_000_000) // 0.1 GAS or 0.000001 ETH
    }

    /// Verify signature
    async fn verify_signature(&self, request: &MetaTxRequest) -> Result<bool, Error> {
        debug!("Verifying signature: {:?}", request);

        // Create EIP-712 typed data for the meta transaction
        let domain = EIP712Domain {
            name: "R3E Meta Transaction".to_string(),
            version: "1".to_string(),
            chain_id: request.chain_id.unwrap_or_else(|| request.blockchain_type.to_chain_id()),
            verifying_contract: "0x0000000000000000000000000000000000000000".to_string(),
            salt: None,
        };

        // Create meta transaction message
        let message = MetaTxMessage::from_request(request.clone());
        
        // Create typed data
        let typed_data = get_typed_data(domain, message)?;
        
        // Verify the signature
        let is_valid = verify_eip712_signature(&typed_data, &request.signature, &request.sender)?;
        
        Ok(is_valid)
    }

    /// Validate meta transaction request
    async fn validate_request(&self, request: &MetaTxRequest) -> Result<(), Error> {
        debug!("Validating meta transaction request: {:?}", request);

        // Check if the transaction data is empty
        if request.tx_data.is_empty() {
            return Err(Error::InvalidParameter("Transaction data is empty".to_string()));
        }

        // Check if the sender address is empty
        if request.sender.is_empty() {
            return Err(Error::InvalidParameter("Sender address is empty".to_string()));
        }

        // Check if the target address is empty
        if request.target_address.is_empty() {
            return Err(Error::InvalidParameter("Target address is empty".to_string()));
        }

        // Check if the signature is empty
        if request.signature.is_empty() {
            return Err(Error::InvalidParameter("Signature is empty".to_string()));
        }

        // Check if the deadline has passed
        let now = chrono::Utc::now().timestamp() as u64;
        if request.deadline < now {
            return Err(Error::InvalidParameter(format!(
                "Transaction deadline has passed: {} < {}",
                request.deadline, now
            )));
        }

        // Validate based on blockchain type
        match request.blockchain_type {
            BlockchainType::NeoN3 => {
                // Validate Neo N3 transaction
                // Additional checks can be added here
            }
            BlockchainType::Ethereum => {
                // Validate Ethereum transaction
                // Additional checks can be added here
            }
        }

        Ok(())
    }

    /// Get Gas Bank account for a contract
    async fn get_gas_bank_account(&self, contract_hash: &str) -> Result<String, Error> {
        // Create a Gas Bank service
        let gas_bank_service = self.create_gas_bank_service_for_contract(contract_hash).await?;
        
        // Get the account for the contract
        match gas_bank_service.get_account_for_contract(contract_hash).await? {
            Some(account) => Ok(account.address),
            None => Err(Error::NotFound(format!("Gas Bank account not found for contract: {}", contract_hash))),
        }
    }

    /// Relay transaction for Neo N3 blockchain
    async fn relay_neo_transaction(&self, request: &MetaTxRequest) -> Result<String, Error> {
        // Parse the transaction data
        debug!("Relaying Neo N3 transaction: {:?}", request);

        // For Neo N3, we need to use the relayer wallet to pay for the transaction fees
        let rpc_client = self.rpc_client.clone();
        
        // Decode the hex transaction data
        let tx_data = match hex::decode(&request.tx_data) {
            Ok(data) => data,
            Err(e) => return Err(Error::InvalidParameter(format!("Invalid hex transaction data: {}", e)))
        };
        
        // Send the raw transaction using APITrait
        let result = match rpc_client.send_raw_transaction(hex::encode(&tx_data)).await {
            Ok(raw_tx) => {
                // Extract the transaction hash from the raw transaction response
                raw_tx.hash.to_string()
            },
            Err(e) => {
                return Err(Error::Network(format!("Failed to send transaction: {}", e)))
            }
        };

        info!("Relayed Neo N3 transaction: {}", result);
        Ok(result)
    }

    /// Relay a transaction
    async fn relay_transaction(&self, request: &MetaTxRequest) -> Result<String, Error> {
        // Check if the transaction is for Ethereum or Neo3
        match request.blockchain_type {
            BlockchainType::NeoN3 => self.relay_neo_transaction(request).await,
            BlockchainType::Ethereum => {
                // TODO: Implement Ethereum transaction relay
                Err(Error::InvalidParameter("Ethereum transactions not supported yet".to_string()))
            }
        }
    }

    async fn submit(&self, request: MetaTxRequest) -> Result<MetaTxResponse, Error> {
        debug!("Submitting meta transaction: {:?}", request);

        // Validate the request
        self.validate_request(&request).await?;

        // Verify the signature
        let is_valid = self.verify_signature(&request).await?;
        if !is_valid {
            error!("Invalid signature");
            return Err(Error::InvalidParameter("Invalid signature".to_string()));
        }

        // Create a domain for the EIP-712 message
        let domain = EIP712Domain {
            name: "R3E Meta Transaction".to_string(),
            version: "1".to_string(),
            chain_id: request.chain_id.unwrap_or_else(|| request.blockchain_type.to_chain_id()),
            verifying_contract: "0x0000000000000000000000000000000000000000".to_string(),
            salt: None,
        };

        // Get the meta transaction message
        let message = MetaTxMessage::from_request(request.clone());

        // Create the typed data
        let typed_data = get_typed_data(domain, message)?;

        // Verify the EIP-712 signature
        let signature_is_valid = verify_eip712_signature(&typed_data, &request.signature, &request.sender)?;
        if !signature_is_valid {
            error!("Invalid EIP-712 signature");
            return Err(Error::InvalidParameter("Invalid EIP-712 signature".to_string()));
        }

        // Relay the transaction
        let tx_hash = self.relay_transaction(&request).await?;

        // Create the response
        let request_id = Uuid::new_v4().to_string();
        let timestamp = chrono::Utc::now().timestamp() as u64;

        // Create a new record
        let record = MetaTxRecord {
            request_id: request_id.clone(),
            request: request.clone(),
            response: None,
            status: MetaTxStatus::Pending,
            created_at: timestamp,
            updated_at: timestamp,
        };

        // Store the record
        self.storage.create_record(record).await?;

        // Create the response
        let response = MetaTxResponse {
            request_id: request_id.clone(),
            original_hash: tx_hash,
            relayed_hash: None,
            status: "pending".to_string(),
            error: None,
            timestamp,
        };

        // Update the record with the response
        let updated_record = MetaTxRecord {
            request_id: request_id.clone(),
            request: request.clone(),
            response: Some(response.clone()),
            status: MetaTxStatus::Submitted,
            created_at: timestamp,
            updated_at: timestamp,
        };

        // Store the updated record
        self.storage.update_record(updated_record).await?;

        // Return the response
        Ok(response)
    }
}

#[async_trait]
impl<S: MetaTxStorage> MetaTxServiceTrait for MetaTxService<S> {
    async fn submit(&self, request: MetaTxRequest) -> Result<MetaTxResponse, Error> {
        self.submit(request).await
    }

    async fn get_status(&self, request_id: &str) -> Result<String, Error> {
        // Get record
        let record = match self.storage.get_record(request_id).await? {
            Some(record) => record,
            None => {
                return Err(Error::NotFound(format!(
                    "Meta transaction with id {} not found",
                    request_id
                )))
            }
        };

        Ok(record.status.to_string())
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

    async fn create_gas_bank_service_for_contract(&self, contract_hash: &str) -> Result<GasBankService, Error> {
        // Create a gas bank service with default settings
        let storage = self.gas_bank_storage.clone();
        let rpc_client = self.rpc_client.clone();
        let wallet = self.relayer_wallet.clone();
        let network = self.network.clone();
        
        // Default settings
        let fee_model = FeeModel::Percentage(1.0); // 1% fee
        let credit_limit = 1_000_000_000; // 1 GAS

        let gas_bank_service = GasBankService::new(
            storage,
            rpc_client,
            wallet,
            network,
            fee_model.clone(),
            credit_limit,
        );
        
        // Check if there's an account for this contract
        match gas_bank_service
            .get_account_for_contract(contract_hash)
            .await
        {
            Ok(Some(_)) => {
                // Account exists, return the service
                Ok(gas_bank_service)
            }
            Ok(None) => {
                // Create a new account for this contract
                let contract_account = format!("gas_bank_{}", contract_hash);
                gas_bank_service
                    .create_account(&contract_account, fee_model, credit_limit)
                    .await?;
                
                // Associate the account with the contract
                gas_bank_service
                    .set_account_for_contract(contract_hash, &contract_account)
                    .await?;
                
                Ok(gas_bank_service)
            }
            Err(e) => Err(e),
        }
    }
}
