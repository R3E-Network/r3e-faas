// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! Price index registry for Oracle service

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::OracleError;

/// Price index registry
pub struct PriceIndexRegistry {
    /// Symbol to index mapping
    symbol_to_index: Arc<RwLock<HashMap<String, u8>>>,
    
    /// Index to symbol mapping
    index_to_symbol: Arc<RwLock<HashMap<u8, String>>>,
}

impl PriceIndexRegistry {
    /// Create a new price index registry
    pub fn new() -> Self {
        Self {
            symbol_to_index: Arc::new(RwLock::new(HashMap::new())),
            index_to_symbol: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Initialize with default mappings
    pub async fn initialize_defaults(&self) {
        let mut symbol_to_index = self.symbol_to_index.write().await;
        let mut index_to_symbol = self.index_to_symbol.write().await;
        
        // Add default mappings
        symbol_to_index.insert("NEO/USD".to_string(), 0);
        index_to_symbol.insert(0, "NEO/USD".to_string());
        
        symbol_to_index.insert("GAS/USD".to_string(), 1);
        index_to_symbol.insert(1, "GAS/USD".to_string());
        
        symbol_to_index.insert("BTC/USD".to_string(), 2);
        index_to_symbol.insert(2, "BTC/USD".to_string());
        
        symbol_to_index.insert("ETH/USD".to_string(), 3);
        index_to_symbol.insert(3, "ETH/USD".to_string());
    }
    
    /// Register a new price index
    pub async fn register(&self, symbol: &str, index: u8) -> Result<(), OracleError> {
        let mut symbol_to_index = self.symbol_to_index.write().await;
        let mut index_to_symbol = self.index_to_symbol.write().await;
        
        // Check if symbol already exists
        if symbol_to_index.contains_key(symbol) {
            return Err(OracleError::Validation(format!("Symbol already registered: {}", symbol)));
        }
        
        // Check if index already exists
        if index_to_symbol.contains_key(&index) {
            return Err(OracleError::Validation(format!("Index already registered: {}", index)));
        }
        
        // Register symbol and index
        symbol_to_index.insert(symbol.to_string(), index);
        index_to_symbol.insert(index, symbol.to_string());
        
        Ok(())
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
    
    /// Get all registered symbols
    pub async fn get_all_symbols(&self) -> Vec<String> {
        let symbol_to_index = self.symbol_to_index.read().await;
        symbol_to_index.keys().cloned().collect()
    }
    
    /// Get all registered indices
    pub async fn get_all_indices(&self) -> Vec<u8> {
        let index_to_symbol = self.index_to_symbol.read().await;
        index_to_symbol.keys().copied().collect()
    }
    
    /// Get all mappings
    pub async fn get_all_mappings(&self) -> HashMap<String, u8> {
        let symbol_to_index = self.symbol_to_index.read().await;
        symbol_to_index.clone()
    }
    
    /// Clear all mappings
    pub async fn clear(&self) {
        let mut symbol_to_index = self.symbol_to_index.write().await;
        let mut index_to_symbol = self.index_to_symbol.write().await;
        
        symbol_to_index.clear();
        index_to_symbol.clear();
    }
}
