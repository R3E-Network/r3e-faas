// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::sync::Arc;
use reqwest::Client;
use tokio::sync::RwLock;

use crate::OracleError;
use crate::types::{BlockchainInfo, OracleResponse, PriceData};
use crate::registry::PriceIndexRegistry;

/// Neo blockchain gateway service
pub struct NeoBlockchainGatewayService {
    /// HTTP client
    client: Arc<Client>,
    
    /// Wallet name
    wallet_name: String,
    
    /// Contract hash
    contract_hash: String,
    
    /// Price index registry
    price_index_registry: Arc<PriceIndexRegistry>,
}

impl NeoBlockchainGatewayService {
    /// Create a new Neo blockchain gateway service
    pub fn new(
        client: Arc<Client>,
        wallet_name: String,
        contract_hash: String,
        price_index_registry: Arc<PriceIndexRegistry>,
    ) -> Self {
        Self {
            client,
            wallet_name,
            contract_hash,
            price_index_registry,
        }
    }
    
    /// Send Oracle response to Neo blockchain
    pub async fn send_oracle_response(
        &self,
        blockchain_info: &BlockchainInfo,
        response: &OracleResponse,
    ) -> Result<(), OracleError> {
        // Get the RPC URL from blockchain info
        let rpc_url = match &blockchain_info.rpc_url {
            Some(url) => url,
            None => return Err(OracleError::Validation("RPC URL is required for Neo blockchain".to_string())),
        };
        
        // Create the request body
        let request_body = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "invokefunction",
            "params": [
                self.contract_hash,
                "updateOracleResponse",
                [
                    {
                        "type": "String",
                        "value": response.request_id
                    },
                    {
                        "type": "String",
                        "value": response.data
                    },
                    {
                        "type": "Integer",
                        "value": response.status_code
                    },
                    {
                        "type": "Integer",
                        "value": response.timestamp
                    }
                ],
                [
                    {
                        "account": self.wallet_name,
                        "scopes": "CalledByEntry"
                    }
                ]
            ]
        });
        
        // Send the request
        let response = self.client.post(rpc_url)
            .json(&request_body)
            .send()
            .await
            .map_err(|e| OracleError::Network(format!("Failed to send request to Neo RPC: {}", e)))?;
        
        // Check the response
        let response_json = response.json::<serde_json::Value>().await
            .map_err(|e| OracleError::Network(format!("Failed to parse Neo RPC response: {}", e)))?;
        
        // Check for errors
        if let Some(error) = response_json.get("error") {
            return Err(OracleError::Network(format!("Neo RPC error: {}", error)));
        }
        
        Ok(())
    }
    
    /// Update price data on Neo blockchain
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
            "method": "invokefunction",
            "params": [
                self.contract_hash,
                "updatePriceData",
                [
                    {
                        "type": "Integer",
                        "value": index
                    },
                    {
                        "type": "Integer",
                        "value": price
                    },
                    {
                        "type": "Integer",
                        "value": timestamp
                    }
                ],
                [
                    {
                        "account": self.wallet_name,
                        "scopes": "CalledByEntry"
                    }
                ]
            ]
        });
        
        // Send the request
        let response = self.client.post("https://rpc.neo.org")
            .json(&request_body)
            .send()
            .await
            .map_err(|e| OracleError::Network(format!("Failed to send request to Neo RPC: {}", e)))?;
        
        // Check the response
        let response_json = response.json::<serde_json::Value>().await
            .map_err(|e| OracleError::Network(format!("Failed to parse Neo RPC response: {}", e)))?;
        
        // Check for errors
        if let Some(error) = response_json.get("error") {
            return Err(OracleError::Network(format!("Neo RPC error: {}", error)));
        }
        
        // Get the transaction hash
        let result = response_json.get("result").ok_or_else(|| {
            OracleError::Network("No result in Neo RPC response".to_string())
        })?;
        
        let tx_hash = result.get("hash").ok_or_else(|| {
            OracleError::Network("No transaction hash in Neo RPC response".to_string())
        })?;
        
        Ok(tx_hash.as_str().unwrap_or("").to_string())
    }
}
