// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Price index registry for mapping asset symbols to indices
pub struct PriceIndexRegistry {
    /// Symbol to index mapping
    symbol_to_index: Arc<RwLock<HashMap<String, u8>>>,
    
    /// Index to symbol mapping
    index_to_symbol: Arc<RwLock<HashMap<u8, String>>>,
}

impl PriceIndexRegistry {
    /// Create a new price index registry
    pub fn new() -> Self {
        let registry = Self {
            symbol_to_index: Arc::new(RwLock::new(HashMap::new())),
            index_to_symbol: Arc::new(RwLock::new(HashMap::new())),
        };
        
        // Initialize with default mappings is done separately to avoid async in constructor
        registry
    }
    
    /// Initialize with default mappings
    pub async fn initialize_defaults(&self) {
        let mut symbol_to_index = self.symbol_to_index.write().await;
        let mut index_to_symbol = self.index_to_symbol.write().await;
        
        // Default mappings
        symbol_to_index.insert("NEO/USD".to_string(), 0);
        symbol_to_index.insert("GAS/USD".to_string(), 1);
        symbol_to_index.insert("BTC/USD".to_string(), 2);
        symbol_to_index.insert("ETH/USD".to_string(), 3);
        
        index_to_symbol.insert(0, "NEO/USD".to_string());
        index_to_symbol.insert(1, "GAS/USD".to_string());
        index_to_symbol.insert(2, "BTC/USD".to_string());
        index_to_symbol.insert(3, "ETH/USD".to_string());
    }
    
    /// Get index for symbol
    pub async fn get_index(&self, symbol: &str) -> Option<u8> {
        let symbol_to_index = self.symbol_to_index.read().await;
        symbol_to_index.get(symbol).copied()
    }
    
    /// Get symbol for index
    pub async fn get_symbol(&self, index: u8) -> Option<String> {
        let index_to_symbol = self.index_to_symbol.read().await;
        index_to_symbol.get(&index).cloned()
    }
    
    /// Register a new symbol-index mapping
    pub async fn register(&self, symbol: &str, index: u8) -> Result<(), &'static str> {
        let mut symbol_to_index = self.symbol_to_index.write().await;
        let mut index_to_symbol = self.index_to_symbol.write().await;
        
        // Check if index is already used
        if index_to_symbol.contains_key(&index) {
            return Err("Index already in use");
        }
        
        // Check if symbol is already registered
        if symbol_to_index.contains_key(symbol) {
            return Err("Symbol already registered");
        }
        
        // Register mappings
        symbol_to_index.insert(symbol.to_string(), index);
        index_to_symbol.insert(index, symbol.to_string());
        
        Ok(())
    }
}
