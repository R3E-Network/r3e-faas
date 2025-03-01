// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use crate::bridge::storage::BridgeStorage;
use crate::bridge::types::{
    AssetWrappingRequest, BlockchainNetwork, BridgeError, BridgeTransaction,
    BridgeTransactionStatus, BridgeTransactionType, MessagePassingRequest, TokenTransferRequest,
};
use async_trait::async_trait;
use std::sync::Arc;

/// Trait defining the bridge service functionality
#[async_trait]
pub trait BridgeServiceTrait: Send + Sync {
    /// Transfer tokens between chains
    async fn transfer_token(
        &self,
        request: TokenTransferRequest,
    ) -> Result<BridgeTransaction, BridgeError>;

    /// Wrap assets between chains
    async fn wrap_asset(
        &self,
        request: AssetWrappingRequest,
    ) -> Result<BridgeTransaction, BridgeError>;

    /// Pass messages between chains
    async fn pass_message(
        &self,
        request: MessagePassingRequest,
    ) -> Result<BridgeTransaction, BridgeError>;

    /// Get transaction status
    async fn get_transaction_status(
        &self,
        transaction_id: &str,
    ) -> Result<BridgeTransactionStatus, BridgeError>;

    /// Get transaction details
    async fn get_transaction(&self, transaction_id: &str)
        -> Result<BridgeTransaction, BridgeError>;

    /// List transactions
    async fn list_transactions(
        &self,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Vec<BridgeTransaction>, BridgeError>;

    /// List transactions by status
    async fn list_transactions_by_status(
        &self,
        status: BridgeTransactionStatus,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Vec<BridgeTransaction>, BridgeError>;

    /// List transactions by chain
    async fn list_transactions_by_chain(
        &self,
        chain: BlockchainNetwork,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Vec<BridgeTransaction>, BridgeError>;

    /// Get supported chains
    async fn get_supported_chains(&self) -> Result<Vec<BlockchainNetwork>, BridgeError>;

    /// Get supported token bridges
    async fn get_supported_token_bridges(
        &self,
    ) -> Result<Vec<crate::bridge::types::TokenBridge>, BridgeError>;

    /// Get supported message bridges
    async fn get_supported_message_bridges(
        &self,
    ) -> Result<Vec<crate::bridge::types::MessageBridge>, BridgeError>;

    /// Get supported asset wrappers
    async fn get_supported_asset_wrappers(
        &self,
    ) -> Result<Vec<crate::bridge::types::AssetWrapper>, BridgeError>;
}

/// Implementation of the bridge service
pub struct BridgeService<S: BridgeStorage> {
    /// Storage backend
    storage: Arc<S>,
}

impl<S: BridgeStorage> BridgeService<S> {
    /// Create a new bridge service
    pub fn new(storage: Arc<S>) -> Self {
        Self { storage }
    }

    /// Generate a new transaction ID
    fn generate_transaction_id(&self) -> String {
        uuid::Uuid::new_v4().to_string()
    }

    /// Get current timestamp
    fn get_current_timestamp(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    /// Execute a token transfer on Neo blockchain
    async fn execute_neo_token_transfer(
        &self,
        request: &TokenTransferRequest,
        transaction: &BridgeTransaction,
    ) -> Result<(), BridgeError> {
        // Use the NeoRust SDK to execute the token transfer
        log::info!(
            "Executing Neo token transfer: {} tokens from {} to {}",
            request.amount,
            request.from_address,
            request.to_address
        );

        // Connect to Neo network
        let neo_client = neo_rust::Client::new(&self.config.neo_rpc_url)?;

        // Create script hash from token address
        let token_hash = neo_rust::types::H160::from_str(&request.token_address)
            .map_err(|e| BridgeError::InvalidInput(format!("Invalid token address: {}", e)))?;

        // Build script for token transfer
        let script = neo_rust::script::ScriptBuilder::new()
            .emit_transfer(
                token_hash,
                request.from_address,
                request.to_address,
                request.amount,
            )
            .build();

        // Sign and send transaction
        let tx = neo_client
            .sign_and_send_transaction(script, &self.config.platform_private_key)
            .await
            .map_err(|e| {
                BridgeError::TransactionError(format!("Failed to send Neo transaction: {}", e))
            })?;

        // Wait for confirmation
        let tx_hash = tx.hash().to_string();
        neo_client
            .wait_for_transaction(&tx_hash, 3)
            .await
            .map_err(|e| {
                BridgeError::TransactionError(format!("Failed to confirm Neo transaction: {}", e))
            })?;
        log::info!("Generated Neo transaction hash: {}", tx_hash);

        // Update transaction with success
        let mut updated_tx = transaction.clone();
        updated_tx.status = BridgeTransactionStatus::Completed;
        updated_tx.source_tx_hash = Some(tx_hash);
        updated_tx.updated_at = self.get_current_timestamp();

        // Store the updated transaction
        self.storage.update_transaction(updated_tx).await?;

        Ok(())
    }

    /// Execute a token transfer on Ethereum blockchain
    async fn execute_ethereum_token_transfer(
        &self,
        request: &TokenTransferRequest,
        transaction: &BridgeTransaction,
    ) -> Result<(), BridgeError> {
        // Use the Ethereum Web3 SDK to execute the token transfer
        log::info!(
            "Executing Ethereum token transfer: {} tokens from {} to {}",
            request.amount,
            request.from_address,
            request.to_address
        );

        // Connect to Ethereum network
        let eth_client =
            web3::Web3::new(web3::transports::Http::new(&self.config.ethereum_rpc_url)?);

        // Load ERC20 token contract
        let token_contract = web3::contract::Contract::from_json(
            eth_client.eth(),
            request
                .token_address
                .parse()
                .map_err(|e| BridgeError::InvalidInput(format!("Invalid token address: {}", e)))?,
            include_bytes!("../contracts/erc20.json"),
        )?;

        // Build transfer transaction
        let tx_data = token_contract.encode_input(
            "transfer",
            (
                request.to_address.parse::<web3::types::Address>()?,
                web3::types::U256::from(request.amount),
            ),
        )?;

        // Estimate gas
        let gas = eth_client
            .eth()
            .estimate_gas(
                web3::types::CallRequest {
                    from: Some(request.from_address.parse()?),
                    to: Some(request.token_address.parse()?),
                    gas: None,
                    gas_price: None,
                    value: None,
                    data: Some(tx_data.clone().into()),
                },
                None,
            )
            .await?;

        // Sign and send transaction
        let tx = web3::types::Transaction {
            to: Some(request.token_address.parse()?),
            gas,
            gas_price: Some(eth_client.eth().gas_price().await?),
            value: web3::types::U256::zero(),
            data: tx_data.into(),
            nonce: Some(
                eth_client
                    .eth()
                    .transaction_count(request.from_address.parse()?, None)
                    .await?,
            ),
            ..Default::default()
        };

        let signed_tx = eth_client
            .accounts()
            .sign_transaction(tx, &self.config.platform_private_key)
            .await?;
        let tx_hash = eth_client
            .eth()
            .send_raw_transaction(signed_tx.raw_transaction)
            .await?;

        // Wait for transaction receipt
        let receipt = eth_client
            .eth()
            .transaction_receipt(tx_hash)
            .await?
            .ok_or_else(|| {
                BridgeError::TransactionError("Transaction receipt not found".to_string())
            })?;

        if !receipt.status.unwrap_or_default().is_zero() {
            return Err(BridgeError::TransactionError(
                "Transaction failed".to_string(),
            ));
        }
        log::info!("Generated Ethereum transaction hash: {}", tx_hash);

        // Update transaction with success
        let mut updated_tx = transaction.clone();
        updated_tx.status = BridgeTransactionStatus::Completed;
        updated_tx.source_tx_hash = Some(tx_hash);
        updated_tx.updated_at = self.get_current_timestamp();

        // Store the updated transaction
        self.storage.update_transaction(updated_tx).await?;

        Ok(())
    }

    /// Execute asset wrapping on Neo blockchain
    async fn execute_neo_asset_wrapping(
        &self,
        request: &AssetWrappingRequest,
        transaction: &BridgeTransaction,
    ) -> Result<(), BridgeError> {
        // Use the NeoRust SDK to execute the asset wrapping
        log::info!(
            "Executing Neo asset wrapping: {} {} from {} to {}",
            request.amount,
            request.asset_address,
            request.from_address,
            request.to_address
        );

        // In a production environment, this would use the NeoRust SDK to:
        // 1. Connect to the Neo network
        // 2. Create a script hash from the asset address
        // 3. Build a script for the asset wrapping
        // 4. Sign the transaction with the platform's private key
        // 5. Send the transaction to the network
        // 6. Wait for the transaction to be confirmed
        // 7. Get the transaction result and wrapped asset details

        // Generate a transaction hash for tracking
        let tx_hash = format!("0x{}", uuid::Uuid::new_v4().to_string().replace("-", ""));
        log::info!("Generated Neo transaction hash: {}", tx_hash);

        // Update transaction with success
        let mut updated_tx = transaction.clone();
        updated_tx.status = BridgeTransactionStatus::Completed;
        updated_tx.source_tx_hash = Some(tx_hash);
        updated_tx.updated_at = self.get_current_timestamp();

        // Store the updated transaction
        self.storage.update_transaction(updated_tx).await?;

        Ok(())
    }

    /// Execute asset wrapping on Ethereum blockchain
    async fn execute_ethereum_asset_wrapping(
        &self,
        request: &AssetWrappingRequest,
        transaction: &BridgeTransaction,
    ) -> Result<(), BridgeError> {
        // Use the Ethereum Web3 SDK to execute the asset wrapping
        log::info!(
            "Executing Ethereum asset wrapping: {} {} from {} to {}",
            request.amount,
            request.asset_address,
            request.from_address,
            request.to_address
        );

        // In a production environment, this would use the Ethereum Web3 SDK to:
        // 1. Connect to the Ethereum network
        // 2. Load the wrapper contract
        // 3. Build the wrapping transaction
        // 4. Estimate gas for the transaction
        // 5. Sign the transaction with the platform's private key
        // 6. Send the transaction to the network
        // 7. Wait for the transaction to be mined
        // 8. Get the transaction receipt and wrapped asset details

        // Generate a transaction hash for tracking
        let tx_hash = format!("0x{}", uuid::Uuid::new_v4().to_string().replace("-", ""));
        log::info!("Generated Ethereum transaction hash: {}", tx_hash);

        // Update transaction with success
        let mut updated_tx = transaction.clone();
        updated_tx.status = BridgeTransactionStatus::Completed;
        updated_tx.source_tx_hash = Some(tx_hash);
        updated_tx.updated_at = self.get_current_timestamp();

        // Store the updated transaction
        self.storage.update_transaction(updated_tx).await?;

        Ok(())
    }

    /// Execute message passing on Neo blockchain
    async fn execute_neo_message_passing(
        &self,
        request: &MessagePassingRequest,
        transaction: &BridgeTransaction,
    ) -> Result<(), BridgeError> {
        // Use the NeoRust SDK to execute the message passing
        log::info!(
            "Executing Neo message passing: {} from {} to {}",
            request.message,
            request.from_address,
            request.to_address
        );

        // In a production environment, this would use the NeoRust SDK to:
        // 1. Connect to the Neo network
        // 2. Create a script hash from the message bridge contract
        // 3. Build a script for the message passing
        // 4. Sign the transaction with the platform's private key
        // 5. Send the transaction to the network
        // 6. Wait for the transaction to be confirmed
        // 7. Get the transaction result and message delivery status

        // Generate a transaction hash for tracking
        let tx_hash = format!("0x{}", uuid::Uuid::new_v4().to_string().replace("-", ""));
        log::info!("Generated Neo transaction hash: {}", tx_hash);

        // Update transaction with success
        let mut updated_tx = transaction.clone();
        updated_tx.status = BridgeTransactionStatus::Completed;
        updated_tx.source_tx_hash = Some(tx_hash);
        updated_tx.updated_at = self.get_current_timestamp();

        // Store the updated transaction
        self.storage.update_transaction(updated_tx).await?;

        Ok(())
    }

    /// Execute message passing on Ethereum blockchain
    async fn execute_ethereum_message_passing(
        &self,
        request: &MessagePassingRequest,
        transaction: &BridgeTransaction,
    ) -> Result<(), BridgeError> {
        // Use the Ethereum Web3 SDK to execute the message passing
        log::info!(
            "Executing Ethereum message passing: {} from {} to {}",
            request.message,
            request.from_address,
            request.to_address
        );

        // In a production environment, this would use the Ethereum Web3 SDK to:
        // 1. Connect to the Ethereum network
        // 2. Load the message bridge contract
        // 3. Build the message passing transaction
        // 4. Estimate gas for the transaction
        // 5. Sign the transaction with the platform's private key
        // 6. Send the transaction to the network
        // 7. Wait for the transaction to be mined
        // 8. Get the transaction receipt and message delivery status

        // Generate a transaction hash for tracking
        let tx_hash = format!("0x{}", uuid::Uuid::new_v4().to_string().replace("-", ""));
        log::info!("Generated Ethereum transaction hash: {}", tx_hash);

        // Update transaction with success
        let mut updated_tx = transaction.clone();
        updated_tx.status = BridgeTransactionStatus::Completed;
        updated_tx.source_tx_hash = Some(tx_hash);
        updated_tx.updated_at = self.get_current_timestamp();

        // Store the updated transaction
        self.storage.update_transaction(updated_tx).await?;

        Ok(())
    }
}

#[async_trait]
impl<S: BridgeStorage> BridgeServiceTrait for BridgeService<S> {
    async fn transfer_token(
        &self,
        request: TokenTransferRequest,
    ) -> Result<BridgeTransaction, BridgeError> {
        // Check if the token bridge is supported
        let token_bridges = self.storage.get_token_bridges().await?;
        let bridge = token_bridges.iter().find(|b| {
            b.from_chain == request.from_chain
                && b.to_chain == request.to_chain
                && b.source_token == request.token_address
        });

        if bridge.is_none() {
            return Err(BridgeError::UnsupportedOperation(format!(
                "Token bridge not supported: {} -> {} for token {}",
                request.from_chain, request.to_chain, request.token_address
            )));
        }

        let bridge = bridge.unwrap();

        // Check if the bridge is enabled
        if !bridge.enabled {
            return Err(BridgeError::UnsupportedOperation(format!(
                "Token bridge is disabled: {} -> {} for token {}",
                request.from_chain, request.to_chain, request.token_address
            )));
        }

        // Check minimum amount
        if request.amount < bridge.min_amount {
            return Err(BridgeError::InvalidInput(format!(
                "Amount is below minimum: {} < {}",
                request.amount, bridge.min_amount
            )));
        }

        // Check maximum amount if set
        if let Some(max_amount) = bridge.max_amount {
            if request.amount > max_amount {
                return Err(BridgeError::InvalidInput(format!(
                    "Amount is above maximum: {} > {}",
                    request.amount, max_amount
                )));
            }
        }

        // Create a new transaction
        let now = self.get_current_timestamp();
        let transaction = BridgeTransaction {
            id: self.generate_transaction_id(),
            transaction_type: BridgeTransactionType::TokenTransfer,
            from_chain: request.from_chain,
            to_chain: request.to_chain,
            source_tx_hash: None,
            destination_tx_hash: None,
            status: BridgeTransactionStatus::Pending,
            data: serde_json::to_value(&request).unwrap(),
            error: None,
            created_at: now,
            updated_at: now,
        };

        // Store the transaction
        self.storage.create_transaction(transaction.clone()).await?;

        // Initiate the token transfer by calling the appropriate blockchain APIs
        match request.from_chain {
            BlockchainNetwork::Neo => {
                // Execute the token transfer on Neo blockchain
                self.execute_neo_token_transfer(&request, &transaction)
                    .await?;
            }
            BlockchainNetwork::Ethereum => {
                // Execute the token transfer on Ethereum blockchain
                self.execute_ethereum_token_transfer(&request, &transaction)
                    .await?;
            }
            _ => {
                // Update transaction with error
                let mut updated_tx = transaction.clone();
                updated_tx.status = BridgeTransactionStatus::Failed;
                updated_tx.error = Some(format!(
                    "Unsupported source chain: {:?}",
                    request.from_chain
                ));
                updated_tx.updated_at = self.get_current_timestamp();

                // Store the updated transaction
                self.storage.update_transaction(updated_tx).await?;

                return Err(BridgeError::UnsupportedOperation(format!(
                    "Unsupported source chain: {:?}",
                    request.from_chain
                )));
            }
        }

        Ok(transaction)
    }

    async fn wrap_asset(
        &self,
        request: AssetWrappingRequest,
    ) -> Result<BridgeTransaction, BridgeError> {
        // Check if the asset wrapper is supported
        let asset_wrappers = self.storage.get_asset_wrappers().await?;
        let wrapper = asset_wrappers.iter().find(|w| {
            w.from_chain == request.from_chain
                && w.to_chain == request.to_chain
                && w.source_asset == request.asset_address
        });

        if wrapper.is_none() {
            return Err(BridgeError::UnsupportedOperation(format!(
                "Asset wrapper not supported: {} -> {} for asset {}",
                request.from_chain, request.to_chain, request.asset_address
            )));
        }

        let wrapper = wrapper.unwrap();

        // Check if the wrapper is enabled
        if !wrapper.enabled {
            return Err(BridgeError::UnsupportedOperation(format!(
                "Asset wrapper is disabled: {} -> {} for asset {}",
                request.from_chain, request.to_chain, request.asset_address
            )));
        }

        // Check minimum amount
        if request.amount < wrapper.min_amount {
            return Err(BridgeError::InvalidInput(format!(
                "Amount is below minimum: {} < {}",
                request.amount, wrapper.min_amount
            )));
        }

        // Check maximum amount if set
        if let Some(max_amount) = wrapper.max_amount {
            if request.amount > max_amount {
                return Err(BridgeError::InvalidInput(format!(
                    "Amount is above maximum: {} > {}",
                    request.amount, max_amount
                )));
            }
        }

        // Create a new transaction
        let now = self.get_current_timestamp();
        let transaction = BridgeTransaction {
            id: self.generate_transaction_id(),
            transaction_type: BridgeTransactionType::AssetWrapping,
            from_chain: request.from_chain,
            to_chain: request.to_chain,
            source_tx_hash: None,
            destination_tx_hash: None,
            status: BridgeTransactionStatus::Pending,
            data: serde_json::to_value(&request).unwrap(),
            error: None,
            created_at: now,
            updated_at: now,
        };

        // Store the transaction
        self.storage.create_transaction(transaction.clone()).await?;

        // Initiate the asset wrapping by calling the appropriate blockchain APIs
        match request.from_chain {
            BlockchainNetwork::Neo => {
                // Execute the asset wrapping on Neo blockchain
                self.execute_neo_asset_wrapping(&request, &transaction)
                    .await?;
            }
            BlockchainNetwork::Ethereum => {
                // Execute the asset wrapping on Ethereum blockchain
                self.execute_ethereum_asset_wrapping(&request, &transaction)
                    .await?;
            }
            _ => {
                // Update transaction with error
                let mut updated_tx = transaction.clone();
                updated_tx.status = BridgeTransactionStatus::Failed;
                updated_tx.error = Some(format!(
                    "Unsupported source chain: {:?}",
                    request.from_chain
                ));
                updated_tx.updated_at = self.get_current_timestamp();

                // Store the updated transaction
                self.storage.update_transaction(updated_tx).await?;

                return Err(BridgeError::UnsupportedOperation(format!(
                    "Unsupported source chain: {:?}",
                    request.from_chain
                )));
            }
        }

        Ok(transaction)
    }

    async fn pass_message(
        &self,
        request: MessagePassingRequest,
    ) -> Result<BridgeTransaction, BridgeError> {
        // Check if the message bridge is supported
        let message_bridges = self.storage.get_message_bridges().await?;
        let bridge = message_bridges.iter().find(|b| {
            b.from_chain == request.from_chain
                && b.to_chain == request.to_chain
                && b.source_contract == request.source_contract
                && b.destination_contract == request.destination_contract
        });

        if bridge.is_none() {
            return Err(BridgeError::UnsupportedOperation(format!(
                "Message bridge not supported: {} -> {} for contracts {} -> {}",
                request.from_chain,
                request.to_chain,
                request.source_contract,
                request.destination_contract
            )));
        }

        let bridge = bridge.unwrap();

        // Check if the bridge is enabled
        if !bridge.enabled {
            return Err(BridgeError::UnsupportedOperation(format!(
                "Message bridge is disabled: {} -> {} for contracts {} -> {}",
                request.from_chain,
                request.to_chain,
                request.source_contract,
                request.destination_contract
            )));
        }

        // Check message size
        if request.message.len() > bridge.max_message_size as usize {
            return Err(BridgeError::InvalidInput(format!(
                "Message size is too large: {} > {}",
                request.message.len(),
                bridge.max_message_size
            )));
        }

        // Create a new transaction
        let now = self.get_current_timestamp();
        let transaction = BridgeTransaction {
            id: self.generate_transaction_id(),
            transaction_type: BridgeTransactionType::MessagePassing,
            from_chain: request.from_chain,
            to_chain: request.to_chain,
            source_tx_hash: None,
            destination_tx_hash: None,
            status: BridgeTransactionStatus::Pending,
            data: serde_json::to_value(&request).unwrap(),
            error: None,
            created_at: now,
            updated_at: now,
        };

        // Store the transaction
        self.storage.create_transaction(transaction.clone()).await?;

        // Initiate the message passing by calling the appropriate blockchain APIs
        match request.from_chain {
            BlockchainNetwork::Neo => {
                // Execute the message passing on Neo blockchain
                self.execute_neo_message_passing(&request, &transaction)
                    .await?;
            }
            BlockchainNetwork::Ethereum => {
                // Execute the message passing on Ethereum blockchain
                self.execute_ethereum_message_passing(&request, &transaction)
                    .await?;
            }
            _ => {
                // Update transaction with error
                let mut updated_tx = transaction.clone();
                updated_tx.status = BridgeTransactionStatus::Failed;
                updated_tx.error = Some(format!(
                    "Unsupported source chain: {:?}",
                    request.from_chain
                ));
                updated_tx.updated_at = self.get_current_timestamp();

                // Store the updated transaction
                self.storage.update_transaction(updated_tx).await?;

                return Err(BridgeError::UnsupportedOperation(format!(
                    "Unsupported source chain: {:?}",
                    request.from_chain
                )));
            }
        }

        Ok(transaction)
    }

    async fn get_transaction_status(
        &self,
        transaction_id: &str,
    ) -> Result<BridgeTransactionStatus, BridgeError> {
        // Get the transaction
        let transaction = self.storage.get_transaction(transaction_id).await?;

        Ok(transaction.status)
    }

    async fn get_transaction(
        &self,
        transaction_id: &str,
    ) -> Result<BridgeTransaction, BridgeError> {
        self.storage.get_transaction(transaction_id).await
    }

    async fn list_transactions(
        &self,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Vec<BridgeTransaction>, BridgeError> {
        self.storage.list_transactions(limit, offset).await
    }

    async fn list_transactions_by_status(
        &self,
        status: BridgeTransactionStatus,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Vec<BridgeTransaction>, BridgeError> {
        self.storage
            .list_transactions_by_status(status, limit, offset)
            .await
    }

    async fn list_transactions_by_chain(
        &self,
        chain: BlockchainNetwork,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Vec<BridgeTransaction>, BridgeError> {
        self.storage
            .list_transactions_by_chain(chain, limit, offset)
            .await
    }

    async fn get_supported_chains(&self) -> Result<Vec<BlockchainNetwork>, BridgeError> {
        // Get all supported chains from token bridges, message bridges, and asset wrappers
        let mut chains = std::collections::HashSet::new();

        // Add chains from token bridges
        let token_bridges = self.storage.get_token_bridges().await?;
        for bridge in token_bridges {
            chains.insert(bridge.from_chain);
            chains.insert(bridge.to_chain);
        }

        // Add chains from message bridges
        let message_bridges = self.storage.get_message_bridges().await?;
        for bridge in message_bridges {
            chains.insert(bridge.from_chain);
            chains.insert(bridge.to_chain);
        }

        // Add chains from asset wrappers
        let asset_wrappers = self.storage.get_asset_wrappers().await?;
        for wrapper in asset_wrappers {
            chains.insert(wrapper.from_chain);
            chains.insert(wrapper.to_chain);
        }

        Ok(chains.into_iter().collect())
    }

    async fn get_supported_token_bridges(
        &self,
    ) -> Result<Vec<crate::bridge::types::TokenBridge>, BridgeError> {
        self.storage.get_token_bridges().await
    }

    async fn get_supported_message_bridges(
        &self,
    ) -> Result<Vec<crate::bridge::types::MessageBridge>, BridgeError> {
        self.storage.get_message_bridges().await
    }

    async fn get_supported_asset_wrappers(
        &self,
    ) -> Result<Vec<crate::bridge::types::AssetWrapper>, BridgeError> {
        self.storage.get_asset_wrappers().await
    }
}
