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
        
        // Create transaction to call the gateway contract using NeoRust SDK
        let rpc_client = self.rpc_client.clone();
        
        // Create a Neo RPC client for blockchain interaction
        let url = "http://seed1.neo.org:10332"; // Use appropriate RPC endpoint
        let neo_client = neo3::prelude::JsonRpcClient::new(url)?;
        
        // Load wallet from private key (in production, this would be securely stored)
        let wallet_account = neo3::prelude::Account::from_wif(&std::env::var("NEO_ORACLE_PRIVATE_KEY")
            .map_err(|_| OracleError::Configuration("NEO_ORACLE_PRIVATE_KEY environment variable not set".to_string()))?)?;
            
        // Create a GasToken instance for gas calculation
        let gas_token = neo3::prelude::NeoToken::default();
        
        // Build the script to invoke the gateway contract
        let script = neo3::prelude::ScriptBuilder::new()
            .contract_call(
                &self.gateway_contract_hash,
                "updatePriceData",  // Gateway contract method
                &[
                    neo3::prelude::ContractParameter::Integer(index as i64),
                    neo3::prelude::ContractParameter::String(price_data.symbol.clone()),
                    neo3::prelude::ContractParameter::Integer(price_int as i64),
                    neo3::prelude::ContractParameter::Integer(price_data.timestamp as i64),
                ],
            )
            .to_bytes();
            
        // Create and sign the transaction
        let transaction = neo3::prelude::TransactionBuilder::new()
            .script(script)
            .gas_limit(20_000_000)
            .valid_until_block(neo_client.get_block_count().await? + 5760)  // Valid for ~1 day
            .sign(&wallet_account)?;
            
        // Send the transaction
        let tx_hash = neo_client.send_raw_transaction(&transaction).await?;
        
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
        // Call the gateway contract to get price data using NeoRust SDK
        // Create a Neo RPC client for blockchain interaction
        let url = "http://seed1.neo.org:10332"; // Use appropriate RPC endpoint
        let neo_client = neo3::prelude::JsonRpcClient::new(url)?;
        
        // Build the script to invoke the gateway contract read method
        let script = neo3::prelude::ScriptBuilder::new()
            .contract_call(
                &self.gateway_contract_hash,
                "getPriceData",  // Gateway contract method
                &[neo3::prelude::ContractParameter::Integer(index as i64)],
            )
            .to_bytes();
            
        // Invoke the script in read-only mode
        let response = neo_client.invoke_script(&script).await?;
        
        // Parse the response
        if response.state != "HALT" {
            return Err(OracleError::Blockchain(format!("Invocation failed with state: {}", response.state)));
        }
        
        let stack_item = response.stack.first()
            .ok_or_else(|| OracleError::Parsing("Empty response stack".to_string()))?;
            
        // Parse the price data from the stack item
        // Assuming the contract returns an array with [symbol, price, timestamp]
        let array = match stack_item {
            neo3::prelude::StackItem::Array(arr) => arr,
            _ => return Err(OracleError::Parsing("Expected array response".to_string())),
        };
        
        if array.len() < 3 {
            return Err(OracleError::Parsing("Invalid price data response".to_string()));
        }
        
        // Extract symbol
        let symbol = match &array[0] {
            neo3::prelude::StackItem::ByteString(bytes) => String::from_utf8(bytes.clone())
                .map_err(|e| OracleError::Parsing(format!("Invalid symbol: {}", e)))?,
            _ => return Err(OracleError::Parsing("Expected string for symbol".to_string())),
        };
        
        // Extract price (stored as integer with 8 decimal places)
        let price_int = match &array[1] {
            neo3::prelude::StackItem::Integer(val) => val.as_i64()
                .map_err(|e| OracleError::Parsing(format!("Invalid price: {}", e)))?,
            _ => return Err(OracleError::Parsing("Expected integer for price".to_string())),
        };
        
        // Convert price back to float
        let price_usd = (price_int as f64) / 100_000_000.0;
        
        // Extract timestamp
        let timestamp = match &array[2] {
            neo3::prelude::StackItem::Integer(val) => val.as_i64()
                .map_err(|e| OracleError::Parsing(format!("Invalid timestamp: {}", e)))?,
            _ => return Err(OracleError::Parsing("Expected integer for timestamp".to_string())),
        };
        
        // Get symbol from index registry
        let symbol_from_registry = self.index_registry.get_symbol(index).await?;
        
        // Verify symbol matches the registry
        if symbol != symbol_from_registry {
            return Err(OracleError::Validation(format!(
                "Symbol mismatch: {} (contract) vs {} (registry)",
                symbol, symbol_from_registry
            )));
        }
        
        Ok(PriceData {
            symbol,
            price_usd,
            timestamp: timestamp as u64,
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
        
        // Import necessary ethers crates
        use ethers::prelude::*;
        use ethers::providers::{Http, Provider};
        use ethers::signers::{LocalWallet, Signer};
        use ethers::contract::abigen;
        
        // Generate contract bindings
        // Define the ABI for the gateway contract - this is simplified, adjust based on actual contract
        abigen!(
            GatewayContract,
            r#"[
                function updatePriceData(uint8 index, string symbol, uint256 price, uint256 timestamp) external returns (bool)
                function getPriceData(uint8 index) external view returns (string symbol, uint256 price, uint256 timestamp)
            ]"#
        );
        
        // Set up provider and wallet
        let provider_url = std::env::var("ETH_RPC_URL")
            .map_err(|_| OracleError::Configuration("ETH_RPC_URL environment variable not set".to_string()))?;
            
        let provider = Provider::<Http>::try_from(provider_url)
            .map_err(|e| OracleError::Blockchain(format!("Failed to create Ethereum provider: {}", e)))?;
            
        let chain_id = provider.get_chainid().await
            .map_err(|e| OracleError::Blockchain(format!("Failed to get chain ID: {}", e)))?
            .as_u64();
            
        let private_key = std::env::var("ETH_ORACLE_PRIVATE_KEY")
            .map_err(|_| OracleError::Configuration("ETH_ORACLE_PRIVATE_KEY environment variable not set".to_string()))?;
            
        let wallet = private_key.parse::<LocalWallet>()
            .map_err(|e| OracleError::Configuration(format!("Invalid private key: {}", e)))?
            .with_chain_id(chain_id);
            
        let client = SignerMiddleware::new(provider, wallet);
        
        // Create contract instance
        let contract_address: Address = self.gateway_contract_address.parse()
            .map_err(|e| OracleError::Configuration(format!("Invalid contract address: {}", e)))?;
            
        let contract = GatewayContract::new(contract_address, Arc::new(client));
        
        // Call the updatePriceData function
        let tx = contract.update_price_data(
            index, 
            price_data.symbol.clone(), 
            U256::from(price_int), 
            U256::from(price_data.timestamp)
        ).send().await
        .map_err(|e| OracleError::Blockchain(format!("Failed to send transaction: {}", e)))?;
        
        // Get transaction hash
        let tx_hash = format!("{:?}", tx.tx_hash());
        
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
        // Import necessary ethers crates
        use ethers::prelude::*;
        use ethers::providers::{Http, Provider};
        use ethers::contract::abigen;
        
        // Generate contract bindings
        abigen!(
            GatewayContract,
            r#"[
                function updatePriceData(uint8 index, string symbol, uint256 price, uint256 timestamp) external returns (bool)
                function getPriceData(uint8 index) external view returns (string symbol, uint256 price, uint256 timestamp)
            ]"#
        );
        
        // Set up provider
        let provider_url = std::env::var("ETH_RPC_URL")
            .map_err(|_| OracleError::Configuration("ETH_RPC_URL environment variable not set".to_string()))?;
            
        let provider = Provider::<Http>::try_from(provider_url)
            .map_err(|e| OracleError::Blockchain(format!("Failed to create Ethereum provider: {}", e)))?;
            
        // Create contract instance
        let contract_address: Address = self.gateway_contract_address.parse()
            .map_err(|e| OracleError::Configuration(format!("Invalid contract address: {}", e)))?;
            
        let contract = GatewayContract::new(contract_address, Arc::new(provider));
        
        // Call the getPriceData function
        let (symbol, price, timestamp) = contract.get_price_data(index).call().await
            .map_err(|e| OracleError::Blockchain(format!("Failed to call contract: {}", e)))?;
            
        // Convert price from uint256 to f64 (divide by 10^8 for precision)
        let price_usd = price.as_u128() as f64 / 100_000_000.0;
        
        // Get symbol from index registry
        let symbol_from_registry = self.index_registry.get_symbol(index).await?;
        
        // Verify symbol matches the registry
        if symbol != symbol_from_registry {
            return Err(OracleError::Validation(format!(
                "Symbol mismatch: {} (contract) vs {} (registry)",
                symbol, symbol_from_registry
            )));
        }
        
        Ok(PriceData {
            symbol,
            price_usd,
            timestamp: timestamp.as_u64(),
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
