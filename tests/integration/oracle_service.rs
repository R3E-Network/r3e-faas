//! Integration tests for Oracle Service
//!
//! These tests verify the end-to-end flow of oracle services in the system.

use std::sync::Arc;
use tokio::test;
use chrono::Utc;

use r3e_oracle::{
    service::OracleService,
    provider::{
        price::PriceProvider,
        random::RandomProvider,
    },
    types::{
        OracleRequest, OracleResponse, OracleType,
        PriceData, PriceIndex, RandomData,
    },
    storage::InMemoryOracleStorage,
};

/// Test the end-to-end flow of a price oracle request
#[tokio::test]
async fn test_price_oracle_flow() {
    // Set up test environment
    let oracle_storage = Arc::new(InMemoryOracleStorage::new());
    
    // Create a mock price provider for testing
    let price_provider = Arc::new(MockPriceProvider::new());
    
    // Create the Oracle service
    let oracle_service = OracleService::new(
        oracle_storage.clone(),
        Some(price_provider.clone()),
        None, // No random provider for this test
        "testnet".to_string(),
    ).unwrap();
    
    // Create a test request for NEO/USD price
    let request = OracleRequest {
        oracle_type: OracleType::Price,
        data: serde_json::to_string(&PriceData {
            asset_pair: "NEO/USD".to_string(),
            timestamp: Utc::now().timestamp() as u64,
        }).unwrap(),
        callback_url: Some("https://example.com/callback".to_string()),
        callback_auth: Some("Bearer token123".to_string()),
    };
    
    // Submit the request
    let result = oracle_service.submit_request(request.clone()).await;
    
    // Verify the result
    assert!(result.is_ok());
    let response = result.unwrap();
    
    // Verify the response fields
    assert_eq!(response.oracle_type, OracleType::Price);
    assert!(response.data.contains("price"));
    
    // Parse the response data
    let price_response: PriceData = serde_json::from_str(&response.data).unwrap();
    
    // Verify the price data
    assert_eq!(price_response.asset_pair, "NEO/USD");
    assert!(price_response.price.is_some());
    assert!(price_response.price.unwrap() > 0.0);
    
    // Verify the request was stored
    let stored_request = oracle_storage.get_request(&response.request_id).await.unwrap();
    assert_eq!(stored_request.oracle_type, OracleType::Price);
}

/// Test the end-to-end flow of a random number oracle request
#[tokio::test]
async fn test_random_oracle_flow() {
    // Set up test environment
    let oracle_storage = Arc::new(InMemoryOracleStorage::new());
    
    // Create a mock random provider for testing
    let random_provider = Arc::new(MockRandomProvider::new());
    
    // Create the Oracle service
    let oracle_service = OracleService::new(
        oracle_storage.clone(),
        None, // No price provider for this test
        Some(random_provider.clone()),
        "testnet".to_string(),
    ).unwrap();
    
    // Create a test request for a random number
    let request = OracleRequest {
        oracle_type: OracleType::Random,
        data: serde_json::to_string(&RandomData {
            min: 1,
            max: 100,
            count: 1,
        }).unwrap(),
        callback_url: Some("https://example.com/callback".to_string()),
        callback_auth: Some("Bearer token123".to_string()),
    };
    
    // Submit the request
    let result = oracle_service.submit_request(request.clone()).await;
    
    // Verify the result
    assert!(result.is_ok());
    let response = result.unwrap();
    
    // Verify the response fields
    assert_eq!(response.oracle_type, OracleType::Random);
    assert!(response.data.contains("numbers"));
    
    // Parse the response data
    let random_response: RandomData = serde_json::from_str(&response.data).unwrap();
    
    // Verify the random data
    assert!(random_response.numbers.is_some());
    assert_eq!(random_response.numbers.as_ref().unwrap().len(), 1);
    assert!(random_response.numbers.as_ref().unwrap()[0] >= 1);
    assert!(random_response.numbers.as_ref().unwrap()[0] <= 100);
    
    // Verify the request was stored
    let stored_request = oracle_storage.get_request(&response.request_id).await.unwrap();
    assert_eq!(stored_request.oracle_type, OracleType::Random);
}

/// Test the price index registry functionality
#[tokio::test]
async fn test_price_index_registry() {
    // Set up test environment
    let oracle_storage = Arc::new(InMemoryOracleStorage::new());
    
    // Create a mock price provider for testing
    let price_provider = Arc::new(MockPriceProvider::new());
    
    // Create the Oracle service
    let oracle_service = OracleService::new(
        oracle_storage.clone(),
        Some(price_provider.clone()),
        None, // No random provider for this test
        "testnet".to_string(),
    ).unwrap();
    
    // Register price indices
    let indices = vec![
        PriceIndex {
            id: 0,
            asset_pair: "NEO/USD".to_string(),
        },
        PriceIndex {
            id: 1,
            asset_pair: "GAS/USD".to_string(),
        },
        PriceIndex {
            id: 2,
            asset_pair: "BTC/USD".to_string(),
        },
    ];
    
    for index in indices.iter() {
        let result = oracle_service.register_price_index(index.clone()).await;
        assert!(result.is_ok());
    }
    
    // Get price by index
    let price_result = oracle_service.get_price_by_index(0).await;
    assert!(price_result.is_ok());
    let price = price_result.unwrap();
    assert!(price > 0.0);
    
    // Get price by asset pair
    let price_result = oracle_service.get_price_by_asset_pair("GAS/USD").await;
    assert!(price_result.is_ok());
    let price = price_result.unwrap();
    assert!(price > 0.0);
    
    // Get all indices
    let indices_result = oracle_service.get_all_price_indices().await;
    assert!(indices_result.is_ok());
    let retrieved_indices = indices_result.unwrap();
    assert_eq!(retrieved_indices.len(), 3);
}

/// Mock price provider for testing
struct MockPriceProvider {
    // Add fields as needed
}

impl MockPriceProvider {
    fn new() -> Self {
        Self {}
    }
}

impl PriceProvider for MockPriceProvider {
    async fn get_price(&self, asset_pair: &str) -> Result<f64, String> {
        // Return a mock price based on the asset pair
        match asset_pair {
            "NEO/USD" => Ok(42.5),
            "GAS/USD" => Ok(15.2),
            "BTC/USD" => Ok(50000.0),
            _ => Err(format!("Price not available for {}", asset_pair)),
        }
    }
    
    async fn get_historical_price(&self, asset_pair: &str, timestamp: u64) -> Result<f64, String> {
        // Return a mock historical price
        match asset_pair {
            "NEO/USD" => Ok(40.0),
            "GAS/USD" => Ok(14.0),
            "BTC/USD" => Ok(48000.0),
            _ => Err(format!("Historical price not available for {}", asset_pair)),
        }
    }
}

/// Mock random provider for testing
struct MockRandomProvider {
    // Add fields as needed
}

impl MockRandomProvider {
    fn new() -> Self {
        Self {}
    }
}

impl RandomProvider for MockRandomProvider {
    async fn generate_random_numbers(&self, min: u64, max: u64, count: u32) -> Result<Vec<u64>, String> {
        // Return mock random numbers
        let mut numbers = Vec::with_capacity(count as usize);
        for i in 0..count {
            // Generate a deterministic but seemingly random number for testing
            let number = min + (i as u64 % (max - min + 1));
            numbers.push(number);
        }
        Ok(numbers)
    }
    
    async fn generate_random_bytes(&self, length: usize) -> Result<Vec<u8>, String> {
        // Return mock random bytes
        let mut bytes = Vec::with_capacity(length);
        for i in 0..length {
            // Generate a deterministic but seemingly random byte for testing
            bytes.push((i % 256) as u8);
        }
        Ok(bytes)
    }
}
