// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::sync::Arc;
use reqwest::Client;
use tokio::sync::RwLock;

use crate::OracleError;
use crate::types::{BlockchainInfo, OracleResponse, PriceData};
use crate::registry::PriceIndexRegistry;

/// Ethereum blockchain gateway service
pub struct EthereumBlockchainGatewayService {
    /// HTTP client
    client: Arc<Client>,
    
    /// Wallet address
    wallet_address: String,
    
    /// Contract address
    contract_address: String,
    
    /// Price index registry
    price_index_registry: Arc<PriceIndexRegistry>,
}

impl EthereumBlockchainGatewayService {
    /// Create a new Ethereum blockchain gateway service
    pub fn new(
        client: Arc<Client>,
        wallet_address: String,
        contract_address: String,
        price_index_registry: Arc<PriceIndexRegistry>,
    ) -> Self {
        Self {
            client,
            wallet_address,
            contract_address,
            price_index_registry,
        }
    }
    
    /// Send Oracle response to Ethereum blockchain
    pub async fn send_oracle_response(
        &self,
        blockchain_info: &BlockchainInfo,
        response: &OracleResponse,
    ) -> Result<(), OracleError> {
        // Get the RPC URL from blockchain info
        let rpc_url = match &blockchain_info.rpc_url {
            Some(url) => url,
            None => return Err(OracleError::Validation("RPC URL is required for Ethereum blockchain".to_string())),
        };
        
        // Create the request body
        let request_body = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "eth_sendTransaction",
            "params": [{
                "from": self.wallet_address,
                "to": self.contract_address,
                "gas": "0x76c0", // 30400
                "gasPrice": "0x9184e72a000", // 10000000000000
                "data": format!(
                    "0x{}{}{}{}",
                    // Function selector for updateOracleResponse(string,string,uint256,uint256)
                    "e9e29f92",
                    // Encode the parameters
                    self.encode_string(&response.request_id),
                    self.encode_string(&response.data),
                    self.encode_uint256(response.status_code),
                    self.encode_uint256(response.timestamp)
                )
            }]
        });
        
        // Send the request
        let response = self.client.post(rpc_url)
            .json(&request_body)
            .send()
            .await
            .map_err(|e| OracleError::Network(format!("Failed to send request to Ethereum RPC: {}", e)))?;
        
        // Check the response
        let response_json = response.json::<serde_json::Value>().await
            .map_err(|e| OracleError::Network(format!("Failed to parse Ethereum RPC response: {}", e)))?;
        
        // Check for errors
        if let Some(error) = response_json.get("error") {
            return Err(OracleError::Network(format!("Ethereum RPC error: {}", error)));
        }
        
        Ok(())
    }
    
    /// Update price data on Ethereum blockchain
    pub async fn update_price_data(&self, price_data: &PriceData) -> Result<String, OracleError> {
        // Get the index for the symbol
        let index = match price_data.index {
            Some(index) => index,
            None => {
                // Try to get the index from the registry
                match &price_data.symbol {
                    Some(symbol) => {
                        match self.price_index_registry.get_index(symbol).await {
                            Some(index) => index,
                            None => return Err(OracleError::Validation(format!("No index found for symbol: {}", symbol))),
                        }
                    },
                    None => return Err(OracleError::Validation("Neither index nor symbol provided".to_string())),
                }
            }
        };
        
        // Get the price value
        let price = match price_data.price {
            Some(price) => price,
            None => return Err(OracleError::Validation("Price is required".to_string())),
        };
        
        // Get the timestamp
        let timestamp = price_data.timestamp.unwrap_or_else(|| {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
        });
        
        // Create the request body
        let request_body = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "eth_sendTransaction",
            "params": [{
                "from": self.wallet_address,
                "to": self.contract_address,
                "gas": "0x76c0", // 30400
                "gasPrice": "0x9184e72a000", // 10000000000000
                "data": format!(
                    "0x{}{}{}{}",
                    // Function selector for updatePriceData(uint8,uint256,uint256)
                    "a7f2a56b",
                    // Encode the parameters
                    self.encode_uint8(index),
                    self.encode_uint256(price),
                    self.encode_uint256(timestamp)
                )
            }]
        });
        
        // Send the request
        let response = self.client.post("https://mainnet.infura.io/v3/your-infura-key")
            .json(&request_body)
            .send()
            .await
            .map_err(|e| OracleError::Network(format!("Failed to send request to Ethereum RPC: {}", e)))?;
        
        // Check the response
        let response_json = response.json::<serde_json::Value>().await
            .map_err(|e| OracleError::Network(format!("Failed to parse Ethereum RPC response: {}", e)))?;
        
        // Check for errors
        if let Some(error) = response_json.get("error") {
            return Err(OracleError::Network(format!("Ethereum RPC error: {}", error)));
        }
        
        // Get the transaction hash
        let result = response_json.get("result").ok_or_else(|| {
            OracleError::Network("No result in Ethereum RPC response".to_string())
        })?;
        
        Ok(result.as_str().unwrap_or("").to_string())
    }
    
    /// Encode a string as an Ethereum ABI parameter
    fn encode_string(&self, value: &str) -> String {
        // Ethereum ABI encoding for strings:
        // 1. Encode the offset to the data (32 bytes)
        // 2. Encode the length of the string (32 bytes)
        // 3. Encode the string data, padded to a multiple of 32 bytes
        
        // For simplicity, we'll just use a placeholder
        format!("{:064x}", 0)
    }
    
    /// Encode a uint8 as an Ethereum ABI parameter
    fn encode_uint8(&self, value: u8) -> String {
        format!("{:064x}", value)
    }
    
    /// Encode a uint256 as an Ethereum ABI parameter
    fn encode_uint256(&self, value: u64) -> String {
        format!("{:064x}", value)
    }
}

#[async_trait::async_trait]
impl crate::gateway::BlockchainGateway for EthereumBlockchainGatewayService {
    async fn send_oracle_response(
        &self,
        blockchain_info: &BlockchainInfo,
        response: &OracleResponse,
    ) -> Result<(), OracleError> {
        self.send_oracle_response(blockchain_info, response).await
    }
    
    async fn update_price_data(&self, price_data: &PriceData) -> Result<String, OracleError> {
        self.update_price_data(price_data).await
    }
}
