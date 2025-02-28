// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::sync::Arc;
use async_trait::async_trait;
use crate::OracleError;
use crate::types::PriceData;
use crate::registry::PriceIndexRegistry;

/// Blockchain gateway service trait
#[async_trait]
pub trait BlockchainGatewayService: Send + Sync {
    /// Update price data on the blockchain
    async fn update_price_data(&self, price_data: &PriceData) -> Result<String, OracleError>;
    
    /// Get price data from the blockchain
    async fn get_price_data(&self, index: u8) -> Result<PriceData, OracleError>;
    
    /// Get price data by symbol from the blockchain
    async fn get_price_data_by_symbol(&self, symbol: &str) -> Result<PriceData, OracleError>;
}

/// Neo N3 blockchain gateway service implementation
pub struct NeoBlockchainGatewayService {
    /// RPC client
    rpc_client: Arc<reqwest::Client>,
    
    /// Oracle wallet
    wallet_address: String,
    
    /// Gateway contract hash
    gateway_contract_hash: String,
    
    /// Price index registry
    index_registry: Arc<PriceIndexRegistry>,
}

impl NeoBlockchainGatewayService {
    /// Create a new Neo blockchain gateway service
    pub fn new(
        rpc_client: Arc<reqwest::Client>,
        wallet_address: String,
        gateway_contract_hash: String,
        index_registry: Arc<PriceIndexRegistry>,
    ) -> Self {
        Self {
            rpc_client,
            wallet_address,
            gateway_contract_hash,
            index_registry,
        }
    }
}

#[async_trait]
impl BlockchainGatewayService for NeoBlockchainGatewayService {
    async fn update_price_data(&self, price_data: &PriceData) -> Result<String, OracleError> {
        // Check if price data has an index
        let index = match price_data.index {
            Some(idx) => idx,
            None => return Err(OracleError::Validation("Price data has no index".to_string())),
        };
        
        // Convert price to integer (multiply by 10^8 for precision)
        let price_int = (price_data.price_usd * 100_000_000.0) as u64;
        
        // Create transaction to call the gateway contract
        // In a real implementation, this would use the NeoRust SDK to create and send a transaction
        // For this example, we'll return a mock transaction hash
        let tx_hash = format!("0x{:016x}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs());
        
        // Log the update for debugging
        log::info!(
            "Updating price data on blockchain: index={}, symbol={}, price={}, tx_hash={}",
            index,
            price_data.symbol,
            price_data.price_usd,
            tx_hash
        );
        
        Ok(tx_hash)
    }
    
    async fn get_price_data(&self, index: u8) -> Result<PriceData, OracleError> {
        // In a real implementation, this would call the gateway contract to get price data
        // For this example, we'll return mock data
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
        
        // Get symbol from index registry
        let symbol = match self.index_registry.get_symbol(index).await {
            Some(symbol) => {
                // Extract the base symbol (remove /USD)
                symbol.split('/').next().unwrap_or("UNKNOWN").to_string()
            },
            None => {
                // Fallback mapping if not in registry
                match index {
                    0 => "NEO".to_string(),
                    1 => "GAS".to_string(),
                    2 => "BTC".to_string(),
                    3 => "ETH".to_string(),
                    _ => return Err(OracleError::Validation(format!("Invalid price index: {}", index))),
                }
            }
        };
        
        Ok(PriceData {
            symbol,
            price_usd: 50.0, // Mock price
            source: "blockchain".to_string(),
            timestamp: now,
            index: Some(index),
        })
    }
    
    async fn get_price_data_by_symbol(&self, symbol: &str) -> Result<PriceData, OracleError> {
        // Format symbol for lookup (add /USD if not present)
        let lookup_symbol = if symbol.contains('/') {
            symbol.to_string()
        } else {
            format!("{}/USD", symbol.to_uppercase())
        };
        
        // Get index from registry
        let index = match self.index_registry.get_index(&lookup_symbol).await {
            Some(idx) => idx,
            None => {
                // Fallback mapping if not in registry
                match symbol.to_uppercase().as_str() {
                    "NEO" => 0,
                    "GAS" => 1,
                    "BTC" => 2,
                    "ETH" => 3,
                    _ => return Err(OracleError::Validation(format!("Unsupported symbol: {}", symbol))),
                }
            }
        };
        
        self.get_price_data(index).await
    }
}

/// Ethereum blockchain gateway service implementation
pub struct EthereumBlockchainGatewayService {
    /// RPC client
    rpc_client: Arc<reqwest::Client>,
    
    /// Oracle wallet
    wallet_address: String,
    
    /// Gateway contract address
    gateway_contract_address: String,
    
    /// Price index registry
    index_registry: Arc<PriceIndexRegistry>,
}

impl EthereumBlockchainGatewayService {
    /// Create a new Ethereum blockchain gateway service
    pub fn new(
        rpc_client: Arc<reqwest::Client>,
        wallet_address: String,
        gateway_contract_address: String,
        index_registry: Arc<PriceIndexRegistry>,
    ) -> Self {
        Self {
            rpc_client,
            wallet_address,
            gateway_contract_address,
            index_registry,
        }
    }
}

#[async_trait]
impl BlockchainGatewayService for EthereumBlockchainGatewayService {
    async fn update_price_data(&self, price_data: &PriceData) -> Result<String, OracleError> {
        // Check if price data has an index
        let index = match price_data.index {
            Some(idx) => idx,
            None => return Err(OracleError::Validation("Price data has no index".to_string())),
        };
        
        // Convert price to integer (multiply by 10^8 for precision)
        let price_int = (price_data.price_usd * 100_000_000.0) as u64;
        
        // Create transaction to call the gateway contract
        // In a real implementation, this would use an Ethereum SDK to create and send a transaction
        // For this example, we'll return a mock transaction hash
        let tx_hash = format!("0x{:064x}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs());
        
        // Log the update for debugging
        log::info!(
            "Updating price data on Ethereum blockchain: index={}, symbol={}, price={}, tx_hash={}",
            index,
            price_data.symbol,
            price_data.price_usd,
            tx_hash
        );
        
        Ok(tx_hash)
    }
    
    async fn get_price_data(&self, index: u8) -> Result<PriceData, OracleError> {
        // In a real implementation, this would call the gateway contract to get price data
        // For this example, we'll return mock data
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
        
        // Get symbol from index registry
        let symbol = match self.index_registry.get_symbol(index).await {
            Some(symbol) => {
                // Extract the base symbol (remove /USD)
                symbol.split('/').next().unwrap_or("UNKNOWN").to_string()
            },
            None => {
                // Fallback mapping if not in registry
                match index {
                    0 => "NEO".to_string(),
                    1 => "GAS".to_string(),
                    2 => "BTC".to_string(),
                    3 => "ETH".to_string(),
                    _ => return Err(OracleError::Validation(format!("Invalid price index: {}", index))),
                }
            }
        };
        
        Ok(PriceData {
            symbol,
            price_usd: 50.0, // Mock price
            source: "ethereum_blockchain".to_string(),
            timestamp: now,
            index: Some(index),
        })
    }
    
    async fn get_price_data_by_symbol(&self, symbol: &str) -> Result<PriceData, OracleError> {
        // Format symbol for lookup (add /USD if not present)
        let lookup_symbol = if symbol.contains('/') {
            symbol.to_string()
        } else {
            format!("{}/USD", symbol.to_uppercase())
        };
        
        // Get index from registry
        let index = match self.index_registry.get_index(&lookup_symbol).await {
            Some(idx) => idx,
            None => {
                // Fallback mapping if not in registry
                match symbol.to_uppercase().as_str() {
                    "NEO" => 0,
                    "GAS" => 1,
                    "BTC" => 2,
                    "ETH" => 3,
                    _ => return Err(OracleError::Validation(format!("Unsupported symbol: {}", symbol))),
                }
            }
        };
        
        self.get_price_data(index).await
    }
}
