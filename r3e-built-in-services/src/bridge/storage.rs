// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use crate::bridge::types::{
    AssetWrapper, BlockchainNetwork, BridgeError, BridgeTransaction, BridgeTransactionStatus,
    MessageBridge, TokenBridge,
};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Trait defining the bridge storage functionality
#[async_trait]
pub trait BridgeStorage: Send + Sync {
    /// Create a new transaction
    async fn create_transaction(&self, transaction: BridgeTransaction) -> Result<(), BridgeError>;

    /// Get a transaction by ID
    async fn get_transaction(&self, transaction_id: &str)
        -> Result<BridgeTransaction, BridgeError>;

    /// Update a transaction
    async fn update_transaction(&self, transaction: BridgeTransaction) -> Result<(), BridgeError>;

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

    /// Get token bridges
    async fn get_token_bridges(&self) -> Result<Vec<TokenBridge>, BridgeError>;

    /// Get message bridges
    async fn get_message_bridges(&self) -> Result<Vec<MessageBridge>, BridgeError>;

    /// Get asset wrappers
    async fn get_asset_wrappers(&self) -> Result<Vec<AssetWrapper>, BridgeError>;

    /// Add a token bridge
    async fn add_token_bridge(&self, bridge: TokenBridge) -> Result<(), BridgeError>;

    /// Add a message bridge
    async fn add_message_bridge(&self, bridge: MessageBridge) -> Result<(), BridgeError>;

    /// Add an asset wrapper
    async fn add_asset_wrapper(&self, wrapper: AssetWrapper) -> Result<(), BridgeError>;

    /// Update a token bridge
    async fn update_token_bridge(&self, bridge: TokenBridge) -> Result<(), BridgeError>;

    /// Update a message bridge
    async fn update_message_bridge(&self, bridge: MessageBridge) -> Result<(), BridgeError>;

    /// Update an asset wrapper
    async fn update_asset_wrapper(&self, wrapper: AssetWrapper) -> Result<(), BridgeError>;
}

/// In-memory implementation of the bridge storage
pub struct MemoryBridgeStorage {
    /// Transactions by ID
    transactions: RwLock<HashMap<String, BridgeTransaction>>,

    /// Token bridges by ID
    token_bridges: RwLock<HashMap<String, TokenBridge>>,

    /// Message bridges by ID
    message_bridges: RwLock<HashMap<String, MessageBridge>>,

    /// Asset wrappers by ID
    asset_wrappers: RwLock<HashMap<String, AssetWrapper>>,
}

impl MemoryBridgeStorage {
    /// Create a new memory-based bridge storage
    pub fn new() -> Self {
        Self {
            transactions: RwLock::new(HashMap::new()),
            token_bridges: RwLock::new(HashMap::new()),
            message_bridges: RwLock::new(HashMap::new()),
            asset_wrappers: RwLock::new(HashMap::new()),
        }
    }

    /// Create a new memory-based bridge storage with default bridges and wrappers
    pub fn with_defaults() -> Self {
        let storage = Self::new();

        // Add default token bridges
        let token_bridges = vec![
            TokenBridge {
                id: "neo-eth-gas".to_string(),
                from_chain: BlockchainNetwork::NeoN3,
                to_chain: BlockchainNetwork::Ethereum,
                source_token: "0xd2a4cff31913016155e38e474a2c06d08be276cf".to_string(), // GAS on Neo N3
                destination_token: "0x8c6f28f2f1a3c87f0f938b96d27520d9751ec8d9".to_string(), // Wrapped GAS on Ethereum
                fee_percentage: 0.1,
                min_amount: 1,
                max_amount: Some(1000),
                enabled: true,
            },
            TokenBridge {
                id: "eth-neo-gas".to_string(),
                from_chain: BlockchainNetwork::Ethereum,
                to_chain: BlockchainNetwork::NeoN3,
                source_token: "0x8c6f28f2f1a3c87f0f938b96d27520d9751ec8d9".to_string(), // Wrapped GAS on Ethereum
                destination_token: "0xd2a4cff31913016155e38e474a2c06d08be276cf".to_string(), // GAS on Neo N3
                fee_percentage: 0.1,
                min_amount: 1,
                max_amount: Some(1000),
                enabled: true,
            },
        ];

        for bridge in token_bridges {
            let _ = storage
                .token_bridges
                .write()
                .unwrap()
                .insert(bridge.id.clone(), bridge);
        }

        // Add default message bridges
        let message_bridges = vec![
            MessageBridge {
                id: "neo-eth-message".to_string(),
                from_chain: BlockchainNetwork::NeoN3,
                to_chain: BlockchainNetwork::Ethereum,
                source_contract: "0x1234567890abcdef1234567890abcdef12345678".to_string(),
                destination_contract: "0xabcdef1234567890abcdef1234567890abcdef12".to_string(),
                fee: 1,
                max_message_size: 1024,
                enabled: true,
            },
            MessageBridge {
                id: "eth-neo-message".to_string(),
                from_chain: BlockchainNetwork::Ethereum,
                to_chain: BlockchainNetwork::NeoN3,
                source_contract: "0xabcdef1234567890abcdef1234567890abcdef12".to_string(),
                destination_contract: "0x1234567890abcdef1234567890abcdef12345678".to_string(),
                fee: 1,
                max_message_size: 1024,
                enabled: true,
            },
        ];

        for bridge in message_bridges {
            let _ = storage
                .message_bridges
                .write()
                .unwrap()
                .insert(bridge.id.clone(), bridge);
        }

        // Add default asset wrappers
        let asset_wrappers = vec![
            AssetWrapper {
                id: "neo-eth-nft".to_string(),
                from_chain: BlockchainNetwork::NeoN3,
                to_chain: BlockchainNetwork::Ethereum,
                source_asset: "0x2328c0254a4b2c094ec29bed73c33acf6b92a427".to_string(), // Neo NFT
                wrapped_asset: "0x7c40c393dc0f283f318791d746d894ddd3693572".to_string(), // Wrapped Neo NFT on Ethereum
                fee_percentage: 0.5,
                min_amount: 1,
                max_amount: Some(10),
                enabled: true,
            },
            AssetWrapper {
                id: "eth-neo-nft".to_string(),
                from_chain: BlockchainNetwork::Ethereum,
                to_chain: BlockchainNetwork::NeoN3,
                source_asset: "0x7c40c393dc0f283f318791d746d894ddd3693572".to_string(), // Wrapped Neo NFT on Ethereum
                wrapped_asset: "0x2328c0254a4b2c094ec29bed73c33acf6b92a427".to_string(), // Neo NFT
                fee_percentage: 0.5,
                min_amount: 1,
                max_amount: Some(10),
                enabled: true,
            },
        ];

        for wrapper in asset_wrappers {
            let _ = storage
                .asset_wrappers
                .write()
                .unwrap()
                .insert(wrapper.id.clone(), wrapper);
        }

        storage
    }
}

#[async_trait]
impl BridgeStorage for MemoryBridgeStorage {
    async fn create_transaction(&self, transaction: BridgeTransaction) -> Result<(), BridgeError> {
        let mut transactions = self
            .transactions
            .write()
            .map_err(|e| BridgeError::Storage(format!("Failed to acquire write lock: {}", e)))?;

        // Check if the transaction already exists
        if transactions.contains_key(&transaction.id) {
            return Err(BridgeError::InvalidInput(format!(
                "Transaction already exists: {}",
                transaction.id
            )));
        }

        // Store the transaction
        transactions.insert(transaction.id.clone(), transaction);

        Ok(())
    }

    async fn get_transaction(
        &self,
        transaction_id: &str,
    ) -> Result<BridgeTransaction, BridgeError> {
        let transactions = self
            .transactions
            .read()
            .map_err(|e| BridgeError::Storage(format!("Failed to acquire read lock: {}", e)))?;

        // Get the transaction
        transactions.get(transaction_id).cloned().ok_or_else(|| {
            BridgeError::NotFound(format!("Transaction not found: {}", transaction_id))
        })
    }

    async fn update_transaction(&self, transaction: BridgeTransaction) -> Result<(), BridgeError> {
        let mut transactions = self
            .transactions
            .write()
            .map_err(|e| BridgeError::Storage(format!("Failed to acquire write lock: {}", e)))?;

        // Check if the transaction exists
        if !transactions.contains_key(&transaction.id) {
            return Err(BridgeError::NotFound(format!(
                "Transaction not found: {}",
                transaction.id
            )));
        }

        // Update the transaction
        transactions.insert(transaction.id.clone(), transaction);

        Ok(())
    }

    async fn list_transactions(
        &self,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Vec<BridgeTransaction>, BridgeError> {
        let transactions = self
            .transactions
            .read()
            .map_err(|e| BridgeError::Storage(format!("Failed to acquire read lock: {}", e)))?;

        // Get all transactions
        let mut all_transactions: Vec<BridgeTransaction> = transactions.values().cloned().collect();

        // Sort by creation time (newest first)
        all_transactions.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        // Apply pagination
        let offset = offset.unwrap_or(0) as usize;
        let limit = limit.unwrap_or(100) as usize;

        let paginated_transactions = all_transactions
            .into_iter()
            .skip(offset)
            .take(limit)
            .collect();

        Ok(paginated_transactions)
    }

    async fn list_transactions_by_status(
        &self,
        status: BridgeTransactionStatus,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Vec<BridgeTransaction>, BridgeError> {
        let transactions = self
            .transactions
            .read()
            .map_err(|e| BridgeError::Storage(format!("Failed to acquire read lock: {}", e)))?;

        // Filter transactions by status
        let mut filtered_transactions: Vec<BridgeTransaction> = transactions
            .values()
            .filter(|tx| tx.status == status)
            .cloned()
            .collect();

        // Sort by creation time (newest first)
        filtered_transactions.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        // Apply pagination
        let offset = offset.unwrap_or(0) as usize;
        let limit = limit.unwrap_or(100) as usize;

        let paginated_transactions = filtered_transactions
            .into_iter()
            .skip(offset)
            .take(limit)
            .collect();

        Ok(paginated_transactions)
    }

    async fn list_transactions_by_chain(
        &self,
        chain: BlockchainNetwork,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Vec<BridgeTransaction>, BridgeError> {
        let transactions = self
            .transactions
            .read()
            .map_err(|e| BridgeError::Storage(format!("Failed to acquire read lock: {}", e)))?;

        // Filter transactions by chain (either source or destination)
        let mut filtered_transactions: Vec<BridgeTransaction> = transactions
            .values()
            .filter(|tx| tx.from_chain == chain || tx.to_chain == chain)
            .cloned()
            .collect();

        // Sort by creation time (newest first)
        filtered_transactions.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        // Apply pagination
        let offset = offset.unwrap_or(0) as usize;
        let limit = limit.unwrap_or(100) as usize;

        let paginated_transactions = filtered_transactions
            .into_iter()
            .skip(offset)
            .take(limit)
            .collect();

        Ok(paginated_transactions)
    }

    async fn get_token_bridges(&self) -> Result<Vec<TokenBridge>, BridgeError> {
        let token_bridges = self
            .token_bridges
            .read()
            .map_err(|e| BridgeError::Storage(format!("Failed to acquire read lock: {}", e)))?;

        // Get all token bridges
        let bridges: Vec<TokenBridge> = token_bridges.values().cloned().collect();

        Ok(bridges)
    }

    async fn get_message_bridges(&self) -> Result<Vec<MessageBridge>, BridgeError> {
        let message_bridges = self
            .message_bridges
            .read()
            .map_err(|e| BridgeError::Storage(format!("Failed to acquire read lock: {}", e)))?;

        // Get all message bridges
        let bridges: Vec<MessageBridge> = message_bridges.values().cloned().collect();

        Ok(bridges)
    }

    async fn get_asset_wrappers(&self) -> Result<Vec<AssetWrapper>, BridgeError> {
        let asset_wrappers = self
            .asset_wrappers
            .read()
            .map_err(|e| BridgeError::Storage(format!("Failed to acquire read lock: {}", e)))?;

        // Get all asset wrappers
        let wrappers: Vec<AssetWrapper> = asset_wrappers.values().cloned().collect();

        Ok(wrappers)
    }

    async fn add_token_bridge(&self, bridge: TokenBridge) -> Result<(), BridgeError> {
        let mut token_bridges = self
            .token_bridges
            .write()
            .map_err(|e| BridgeError::Storage(format!("Failed to acquire write lock: {}", e)))?;

        // Store the token bridge
        token_bridges.insert(bridge.id.clone(), bridge);

        Ok(())
    }

    async fn add_message_bridge(&self, bridge: MessageBridge) -> Result<(), BridgeError> {
        let mut message_bridges = self
            .message_bridges
            .write()
            .map_err(|e| BridgeError::Storage(format!("Failed to acquire write lock: {}", e)))?;

        // Store the message bridge
        message_bridges.insert(bridge.id.clone(), bridge);

        Ok(())
    }

    async fn add_asset_wrapper(&self, wrapper: AssetWrapper) -> Result<(), BridgeError> {
        let mut asset_wrappers = self
            .asset_wrappers
            .write()
            .map_err(|e| BridgeError::Storage(format!("Failed to acquire write lock: {}", e)))?;

        // Store the asset wrapper
        asset_wrappers.insert(wrapper.id.clone(), wrapper);

        Ok(())
    }

    async fn update_token_bridge(&self, bridge: TokenBridge) -> Result<(), BridgeError> {
        let mut token_bridges = self
            .token_bridges
            .write()
            .map_err(|e| BridgeError::Storage(format!("Failed to acquire write lock: {}", e)))?;

        // Check if the bridge exists
        if !token_bridges.contains_key(&bridge.id) {
            return Err(BridgeError::NotFound(format!(
                "Token bridge not found: {}",
                bridge.id
            )));
        }

        // Update the bridge
        token_bridges.insert(bridge.id.clone(), bridge);

        Ok(())
    }

    async fn update_message_bridge(&self, bridge: MessageBridge) -> Result<(), BridgeError> {
        let mut message_bridges = self
            .message_bridges
            .write()
            .map_err(|e| BridgeError::Storage(format!("Failed to acquire write lock: {}", e)))?;

        // Check if the bridge exists
        if !message_bridges.contains_key(&bridge.id) {
            return Err(BridgeError::NotFound(format!(
                "Message bridge not found: {}",
                bridge.id
            )));
        }

        // Update the bridge
        message_bridges.insert(bridge.id.clone(), bridge);

        Ok(())
    }

    async fn update_asset_wrapper(&self, wrapper: AssetWrapper) -> Result<(), BridgeError> {
        let mut asset_wrappers = self
            .asset_wrappers
            .write()
            .map_err(|e| BridgeError::Storage(format!("Failed to acquire write lock: {}", e)))?;

        // Check if the wrapper exists
        if !asset_wrappers.contains_key(&wrapper.id) {
            return Err(BridgeError::NotFound(format!(
                "Asset wrapper not found: {}",
                wrapper.id
            )));
        }

        // Update the wrapper
        asset_wrappers.insert(wrapper.id.clone(), wrapper);

        Ok(())
    }
}
