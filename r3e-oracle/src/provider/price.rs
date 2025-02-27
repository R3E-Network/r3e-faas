// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::sync::RwLock;

use crate::{OracleError, OracleProvider, OracleRequest, OracleRequestType, OracleResponse};
use crate::types::{PriceData, PriceRequest, PriceResponse};

/// Price feed provider for cryptocurrency price data
pub struct PriceProvider {
    /// HTTP client for API requests
    client: Client,
    
    /// Cache for price data
    cache: Arc<RwLock<HashMap<String, PriceData>>>,
    
    /// Cache expiration time in seconds
    cache_expiration: u64,
}

impl PriceProvider {
    /// Create a new price provider
    pub fn new(cache_expiration: u64) -> Self {
        Self {
            client: Client::new(),
            cache: Arc::new(RwLock::new(HashMap::new())),
            cache_expiration,
        }
    }
    
    /// Get price data from CoinGecko API
    async fn get_price_from_coingecko(&self, symbol: &str) -> Result<PriceData, OracleError> {
        let url = format!(
            "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd",
            symbol.to_lowercase()
        );
        
        let response = self.client.get(&url)
            .send()
            .await
            .map_err(|e| OracleError::Provider(format!("CoinGecko API request failed: {}", e)))?;
        
        if !response.status().is_success() {
            return Err(OracleError::Provider(format!(
                "CoinGecko API returned error status: {}",
                response.status()
            )));
        }
        
        let data: serde_json::Value = response
            .json()
            .await
            .map_err(|e| OracleError::Provider(format!("Failed to parse CoinGecko response: {}", e)))?;
        
        let price = data
            .get(symbol.to_lowercase())
            .and_then(|v| v.get("usd"))
            .and_then(|v| v.as_f64())
            .ok_or_else(|| OracleError::Provider(format!("Price data not found for {}", symbol)))?;
        
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        Ok(PriceData {
            symbol: symbol.to_string(),
            price_usd: price,
            source: "coingecko".to_string(),
            timestamp: now,
        })
    }
    
    /// Get price data from Binance API
    async fn get_price_from_binance(&self, symbol: &str) -> Result<PriceData, OracleError> {
        // Convert symbol to Binance format (e.g., NEO -> NEOUSDT)
        let binance_symbol = format!("{}USDT", symbol.to_uppercase());
        
        let url = format!(
            "https://api.binance.com/api/v3/ticker/price?symbol={}",
            binance_symbol
        );
        
        let response = self.client.get(&url)
            .send()
            .await
            .map_err(|e| OracleError::Provider(format!("Binance API request failed: {}", e)))?;
        
        if !response.status().is_success() {
            return Err(OracleError::Provider(format!(
                "Binance API returned error status: {}",
                response.status()
            )));
        }
        
        #[derive(Deserialize)]
        struct BinancePrice {
            symbol: String,
            price: String,
        }
        
        let price_data: BinancePrice = response
            .json()
            .await
            .map_err(|e| OracleError::Provider(format!("Failed to parse Binance response: {}", e)))?;
        
        let price = price_data.price
            .parse::<f64>()
            .map_err(|e| OracleError::Provider(format!("Failed to parse price value: {}", e)))?;
        
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        Ok(PriceData {
            symbol: symbol.to_string(),
            price_usd: price,
            source: "binance".to_string(),
            timestamp: now,
        })
    }
    
    /// Get price data from cache or fetch from APIs
    async fn get_price(&self, symbol: &str, sources: &[String]) -> Result<Vec<PriceData>, OracleError> {
        let mut prices = Vec::new();
        
        // Check cache first
        let cache = self.cache.read().await;
        if let Some(cached_data) = cache.get(symbol) {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            
            // Use cached data if it's still valid
            if now - cached_data.timestamp < self.cache_expiration {
                prices.push(cached_data.clone());
                return Ok(prices);
            }
        }
        drop(cache);
        
        // Fetch from APIs
        let mut fetch_sources = Vec::new();
        
        if sources.is_empty() {
            // Default sources
            fetch_sources.push("coingecko".to_string());
            fetch_sources.push("binance".to_string());
        } else {
            fetch_sources = sources.to_vec();
        }
        
        for source in fetch_sources {
            match source.as_str() {
                "coingecko" => {
                    match self.get_price_from_coingecko(symbol).await {
                        Ok(price_data) => {
                            // Update cache
                            self.cache.write().await.insert(symbol.to_string(), price_data.clone());
                            prices.push(price_data);
                        }
                        Err(e) => {
                            log::warn!("Failed to get price from CoinGecko: {}", e);
                        }
                    }
                }
                "binance" => {
                    match self.get_price_from_binance(symbol).await {
                        Ok(price_data) => {
                            // Update cache
                            self.cache.write().await.insert(symbol.to_string(), price_data.clone());
                            prices.push(price_data);
                        }
                        Err(e) => {
                            log::warn!("Failed to get price from Binance: {}", e);
                        }
                    }
                }
                _ => {
                    log::warn!("Unsupported price source: {}", source);
                }
            }
        }
        
        if prices.is_empty() {
            return Err(OracleError::Provider(format!(
                "Failed to get price data for {} from any source",
                symbol
            )));
        }
        
        Ok(prices)
    }
}

#[async_trait]
impl OracleProvider for PriceProvider {
    fn name(&self) -> &str {
        "price"
    }
    
    fn description(&self) -> &str {
        "Provides cryptocurrency price data from various sources"
    }
    
    fn supported_types(&self) -> Vec<OracleRequestType> {
        vec![OracleRequestType::Price]
    }
    
    async fn process_request(&self, request: &OracleRequest) -> Result<OracleResponse, OracleError> {
        if request.request_type != OracleRequestType::Price {
            return Err(OracleError::Validation(format!(
                "Unsupported request type: {:?}",
                request.request_type
            )));
        }
        
        // Parse request data
        let price_request: PriceRequest = serde_json::from_str(&request.data)
            .map_err(|e| OracleError::Validation(format!("Invalid price request data: {}", e)))?;
        
        // Get price data
        let prices = self.get_price(&price_request.symbol, &price_request.sources).await?;
        
        // Calculate average price
        let total_price: f64 = prices.iter().map(|p| p.price_usd).sum();
        let avg_price = total_price / prices.len() as f64;
        
        // Create response
        let price_response = PriceResponse {
            symbol: price_request.symbol,
            currency: price_request.currency,
            price: avg_price,
            sources: prices.iter().map(|p| p.source.clone()).collect(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        };
        
        let response_data = serde_json::to_string(&price_response)
            .map_err(|e| OracleError::Internal(format!("Failed to serialize response: {}", e)))?;
        
        Ok(OracleResponse {
            request_id: request.id.clone(),
            data: response_data,
            status_code: 200,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            error: None,
        })
    }
}
