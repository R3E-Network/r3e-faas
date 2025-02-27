// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use async_trait::async_trait;
use std::sync::Arc;
use crate::bridge::storage::BridgeStorage;
use crate::bridge::types::{
    AssetWrappingRequest, BlockchainNetwork, BridgeError, BridgeTransaction, 
    BridgeTransactionStatus, BridgeTransactionType, MessagePassingRequest, 
    TokenTransferRequest
};

/// Trait defining the bridge service functionality
#[async_trait]
pub trait BridgeServiceTrait: Send + Sync {
    /// Transfer tokens between chains
    async fn transfer_token(&self, request: TokenTransferRequest) -> Result<BridgeTransaction, BridgeError>;
    
    /// Wrap assets between chains
    async fn wrap_asset(&self, request: AssetWrappingRequest) -> Result<BridgeTransaction, BridgeError>;
    
    /// Pass messages between chains
    async fn pass_message(&self, request: MessagePassingRequest) -> Result<BridgeTransaction, BridgeError>;
    
    /// Get transaction status
    async fn get_transaction_status(&self, transaction_id: &str) -> Result<BridgeTransactionStatus, BridgeError>;
    
    /// Get transaction details
    async fn get_transaction(&self, transaction_id: &str) -> Result<BridgeTransaction, BridgeError>;
    
    /// List transactions
    async fn list_transactions(&self, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<BridgeTransaction>, BridgeError>;
    
    /// List transactions by status
    async fn list_transactions_by_status(&self, status: BridgeTransactionStatus, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<BridgeTransaction>, BridgeError>;
    
    /// List transactions by chain
    async fn list_transactions_by_chain(&self, chain: BlockchainNetwork, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<BridgeTransaction>, BridgeError>;
    
    /// Get supported chains
    async fn get_supported_chains(&self) -> Result<Vec<BlockchainNetwork>, BridgeError>;
    
    /// Get supported token bridges
    async fn get_supported_token_bridges(&self) -> Result<Vec<crate::bridge::types::TokenBridge>, BridgeError>;
    
    /// Get supported message bridges
    async fn get_supported_message_bridges(&self) -> Result<Vec<crate::bridge::types::MessageBridge>, BridgeError>;
    
    /// Get supported asset wrappers
    async fn get_supported_asset_wrappers(&self) -> Result<Vec<crate::bridge::types::AssetWrapper>, BridgeError>;
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
}

#[async_trait]
impl<S: BridgeStorage> BridgeServiceTrait for BridgeService<S> {
    async fn transfer_token(&self, request: TokenTransferRequest) -> Result<BridgeTransaction, BridgeError> {
        // Check if the token bridge is supported
        let token_bridges = self.storage.get_token_bridges().await?;
        let bridge = token_bridges.iter().find(|b| {
            b.from_chain == request.from_chain && 
            b.to_chain == request.to_chain && 
            b.source_token == request.token_address
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
        
        // In a real implementation, this would initiate the token transfer
        // by calling the appropriate blockchain APIs
        
        Ok(transaction)
    }
    
    async fn wrap_asset(&self, request: AssetWrappingRequest) -> Result<BridgeTransaction, BridgeError> {
        // Check if the asset wrapper is supported
        let asset_wrappers = self.storage.get_asset_wrappers().await?;
        let wrapper = asset_wrappers.iter().find(|w| {
            w.from_chain == request.from_chain && 
            w.to_chain == request.to_chain && 
            w.source_asset == request.asset_address
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
        
        // In a real implementation, this would initiate the asset wrapping
        // by calling the appropriate blockchain APIs
        
        Ok(transaction)
    }
    
    async fn pass_message(&self, request: MessagePassingRequest) -> Result<BridgeTransaction, BridgeError> {
        // Check if the message bridge is supported
        let message_bridges = self.storage.get_message_bridges().await?;
        let bridge = message_bridges.iter().find(|b| {
            b.from_chain == request.from_chain && 
            b.to_chain == request.to_chain && 
            b.source_contract == request.source_contract &&
            b.destination_contract == request.destination_contract
        });
        
        if bridge.is_none() {
            return Err(BridgeError::UnsupportedOperation(format!(
                "Message bridge not supported: {} -> {} for contracts {} -> {}",
                request.from_chain, request.to_chain, request.source_contract, request.destination_contract
            )));
        }
        
        let bridge = bridge.unwrap();
        
        // Check if the bridge is enabled
        if !bridge.enabled {
            return Err(BridgeError::UnsupportedOperation(format!(
                "Message bridge is disabled: {} -> {} for contracts {} -> {}",
                request.from_chain, request.to_chain, request.source_contract, request.destination_contract
            )));
        }
        
        // Check message size
        if request.message.len() > bridge.max_message_size as usize {
            return Err(BridgeError::InvalidInput(format!(
                "Message size is too large: {} > {}",
                request.message.len(), bridge.max_message_size
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
        
        // In a real implementation, this would initiate the message passing
        // by calling the appropriate blockchain APIs
        
        Ok(transaction)
    }
    
    async fn get_transaction_status(&self, transaction_id: &str) -> Result<BridgeTransactionStatus, BridgeError> {
        // Get the transaction
        let transaction = self.storage.get_transaction(transaction_id).await?;
        
        Ok(transaction.status)
    }
    
    async fn get_transaction(&self, transaction_id: &str) -> Result<BridgeTransaction, BridgeError> {
        self.storage.get_transaction(transaction_id).await
    }
    
    async fn list_transactions(&self, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<BridgeTransaction>, BridgeError> {
        self.storage.list_transactions(limit, offset).await
    }
    
    async fn list_transactions_by_status(&self, status: BridgeTransactionStatus, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<BridgeTransaction>, BridgeError> {
        self.storage.list_transactions_by_status(status, limit, offset).await
    }
    
    async fn list_transactions_by_chain(&self, chain: BlockchainNetwork, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<BridgeTransaction>, BridgeError> {
        self.storage.list_transactions_by_chain(chain, limit, offset).await
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
    
    async fn get_supported_token_bridges(&self) -> Result<Vec<crate::bridge::types::TokenBridge>, BridgeError> {
        self.storage.get_token_bridges().await
    }
    
    async fn get_supported_message_bridges(&self) -> Result<Vec<crate::bridge::types::MessageBridge>, BridgeError> {
        self.storage.get_message_bridges().await
    }
    
    async fn get_supported_asset_wrappers(&self) -> Result<Vec<crate::bridge::types::AssetWrapper>, BridgeError> {
        self.storage.get_asset_wrappers().await
    }
}
