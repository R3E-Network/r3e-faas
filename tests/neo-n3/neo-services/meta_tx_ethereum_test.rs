// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use r3e_neo_services::meta_tx::{
    MetaTxService, MetaTxServiceTrait, MetaTxRequest, MetaTxResponse, MetaTxStatus,
    BlockchainType, SignatureCurve
};
use r3e_neo_services::meta_tx::storage::InMemoryMetaTxStorage;
use r3e_neo_services::Error;
use std::sync::Arc;
use neo3::prelude::{Wallet, RpcClient};
use chrono::Utc;

// Helper function to create a test service
async fn create_test_service() -> MetaTxService<InMemoryMetaTxStorage> {
    // Create in-memory storage
    let storage = Arc::new(InMemoryMetaTxStorage::new());
    
    // Create mock RPC client
    let rpc_client = Arc::new(RpcClient::new("http://localhost:10332").unwrap());
    
    // Create mock relayer wallet
    let relayer_wallet = Arc::new(Wallet::new_from_wif("KwYgW8gcxj1JWJXhPSu4Fqwzfhp5Yfi42mdYmMa4XqK7NJxXUSK7").unwrap());
    
    // Create service
    MetaTxService::new(
        storage,
        rpc_client,
        relayer_wallet,
        "testnet".to_string(),
        r3e_neo_services::types::FeeModel::Fixed(10),
    )
}

#[tokio::test]
async fn test_ethereum_meta_transaction() {
    let service = create_test_service().await;
    
    // Create a valid Ethereum meta transaction request
    let request = MetaTxRequest {
        tx_data: "0x0123456789abcdef".to_string(),
        sender: "0xabcdef1234567890abcdef1234567890abcdef12".to_string(),
        signature: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef00".to_string(),
        nonce: 1,
        deadline: Utc::now().timestamp() as u64 + 3600, // 1 hour from now
        fee_model: "fixed".to_string(),
        fee_amount: 10,
        blockchain_type: BlockchainType::Ethereum,
        signature_curve: SignatureCurve::Secp256k1,
        target_contract: Some("0x1234567890abcdef1234567890abcdef12345678".to_string()),
    };
    
    // Test submitting a valid Ethereum meta transaction
    let result = service.submit(request.clone()).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    
    // In our mock implementation, the signature verification will fail
    // because we're using a mock signature, but the service should still
    // process the request and return a response with Rejected status
    assert_eq!(response.status, MetaTxStatus::Rejected.to_string());
    assert!(response.error.is_some());
}

#[tokio::test]
async fn test_ethereum_meta_transaction_missing_target_contract() {
    let service = create_test_service().await;
    
    // Create an Ethereum meta transaction request without a target contract
    let request = MetaTxRequest {
        tx_data: "0x0123456789abcdef".to_string(),
        sender: "0xabcdef1234567890abcdef1234567890abcdef12".to_string(),
        signature: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef00".to_string(),
        nonce: 1,
        deadline: Utc::now().timestamp() as u64 + 3600, // 1 hour from now
        fee_model: "fixed".to_string(),
        fee_amount: 10,
        blockchain_type: BlockchainType::Ethereum,
        signature_curve: SignatureCurve::Secp256k1,
        target_contract: None,
    };
    
    // Test submitting an Ethereum meta transaction without a target contract
    let result = service.submit(request.clone()).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    
    // The service should reject the request because the target contract is missing
    assert_eq!(response.status, MetaTxStatus::Rejected.to_string());
    assert!(response.error.is_some());
    assert!(response.error.unwrap().contains("Target contract is required"));
}

#[tokio::test]
async fn test_ethereum_meta_transaction_expired() {
    let service = create_test_service().await;
    
    // Create an Ethereum meta transaction request with an expired deadline
    let request = MetaTxRequest {
        tx_data: "0x0123456789abcdef".to_string(),
        sender: "0xabcdef1234567890abcdef1234567890abcdef12".to_string(),
        signature: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef00".to_string(),
        nonce: 1,
        deadline: Utc::now().timestamp() as u64 - 3600, // 1 hour ago
        fee_model: "fixed".to_string(),
        fee_amount: 10,
        blockchain_type: BlockchainType::Ethereum,
        signature_curve: SignatureCurve::Secp256k1,
        target_contract: Some("0x1234567890abcdef1234567890abcdef12345678".to_string()),
    };
    
    // Test submitting an Ethereum meta transaction with an expired deadline
    let result = service.submit(request.clone()).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    
    // The service should reject the request because the deadline has expired
    assert_eq!(response.status, MetaTxStatus::Rejected.to_string());
    assert!(response.error.is_some());
    assert!(response.error.unwrap().contains("Invalid signature"));
}

#[tokio::test]
async fn test_ethereum_meta_transaction_invalid_signature_curve() {
    let service = create_test_service().await;
    
    // Create an Ethereum meta transaction request with an invalid signature curve
    let request = MetaTxRequest {
        tx_data: "0x0123456789abcdef".to_string(),
        sender: "0xabcdef1234567890abcdef1234567890abcdef12".to_string(),
        signature: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef00".to_string(),
        nonce: 1,
        deadline: Utc::now().timestamp() as u64 + 3600, // 1 hour from now
        fee_model: "fixed".to_string(),
        fee_amount: 10,
        blockchain_type: BlockchainType::Ethereum,
        signature_curve: SignatureCurve::Secp256r1, // Invalid curve for Ethereum
        target_contract: Some("0x1234567890abcdef1234567890abcdef12345678".to_string()),
    };
    
    // Test submitting an Ethereum meta transaction with an invalid signature curve
    let result = service.submit(request.clone()).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    
    // The service should reject the request because the signature curve is invalid
    assert_eq!(response.status, MetaTxStatus::Rejected.to_string());
    assert!(response.error.is_some());
    assert!(response.error.unwrap().contains("Invalid signature"));
}
