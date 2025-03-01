// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::sync::Arc;
use tokio::test;
use r3e_neo_services::{
    meta_tx::{MetaTxService, MetaTxServiceTrait, MetaTxRequest, MetaTxResponse, MetaTxRecord, MetaTxStatus, storage::MetaTxStorage, storage::InMemoryMetaTxStorage},
    types::FeeModel,
    Error,
};
use neo3::prelude::{Wallet, RpcClient, Transaction, TransactionBuilder, Signer};
use url::Url;

// Mock RPC client for testing
struct MockRpcClient;

impl RpcClient for MockRpcClient {
    // Implement required methods for the RpcClient trait
    // This is a simplified mock implementation for testing
}

// Helper function to create a test Meta Transaction Service
async fn create_test_service() -> MetaTxService<InMemoryMetaTxStorage> {
    let storage = Arc::new(InMemoryMetaTxStorage::new());
    let rpc_client = Arc::new(MockRpcClient);
    let wallet = Arc::new(Wallet::new("NeoWallet".to_string()));
    
    MetaTxService::new(
        storage,
        rpc_client,
        wallet,
        "testnet".to_string(),
        FeeModel::Fixed(10),
    )
}

#[tokio::test]
async fn test_submit_meta_transaction() {
    let service = create_test_service().await;
    
    // Create a valid meta transaction request
    let request = MetaTxRequest {
        tx_data: "0x0123456789abcdef".to_string(),
        sender: "neo1abc".to_string(),
        signature: "0xsignature".to_string(),
        nonce: 1,
        deadline: chrono::Utc::now().timestamp() as u64 + 3600, // 1 hour from now
        fee_model: "fixed".to_string(),
        fee_amount: 10,
    };
    
    // Test submitting a valid meta transaction
    let result = service.submit(request.clone()).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.status, MetaTxStatus::Submitted.to_string());
    assert!(response.relayed_hash.is_some());
    assert!(response.error.is_none());
    
    // Create an invalid meta transaction request (expired deadline)
    let invalid_request = MetaTxRequest {
        tx_data: "0x0123456789abcdef".to_string(),
        sender: "neo1abc".to_string(),
        signature: "0xsignature".to_string(),
        nonce: 2,
        deadline: chrono::Utc::now().timestamp() as u64 - 3600, // 1 hour ago
        fee_model: "fixed".to_string(),
        fee_amount: 10,
    };
    
    // Test submitting an invalid meta transaction
    let result = service.submit(invalid_request).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.status, MetaTxStatus::Rejected.to_string());
    assert!(response.relayed_hash.is_none());
    assert!(response.error.is_some());
    
    // Create an invalid meta transaction request (empty signature)
    let invalid_request = MetaTxRequest {
        tx_data: "0x0123456789abcdef".to_string(),
        sender: "neo1abc".to_string(),
        signature: "".to_string(),
        nonce: 3,
        deadline: chrono::Utc::now().timestamp() as u64 + 3600, // 1 hour from now
        fee_model: "fixed".to_string(),
        fee_amount: 10,
    };
    
    // Test submitting an invalid meta transaction
    let result = service.submit(invalid_request).await;
    
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.status, MetaTxStatus::Rejected.to_string());
    assert!(response.relayed_hash.is_none());
    assert!(response.error.is_some());
}

#[tokio::test]
async fn test_get_status() {
    let service = create_test_service().await;
    
    // Create and submit a meta transaction
    let request = MetaTxRequest {
        tx_data: "0x0123456789abcdef".to_string(),
        sender: "neo1abc".to_string(),
        signature: "0xsignature".to_string(),
        nonce: 1,
        deadline: chrono::Utc::now().timestamp() as u64 + 3600, // 1 hour from now
        fee_model: "fixed".to_string(),
        fee_amount: 10,
    };
    
    let response = service.submit(request).await.unwrap();
    let request_id = response.request_id.clone();
    
    // Test getting the status of an existing meta transaction
    let result = service.get_status(&request_id).await;
    
    assert!(result.is_ok());
    let status = result.unwrap();
    assert_eq!(status, MetaTxStatus::Submitted);
    
    // Test getting the status of a non-existent meta transaction
    let result = service.get_status("non-existent-id").await;
    
    assert!(result.is_err());
    match result {
        Err(Error::NotFound(_)) => (),
        _ => panic!("Expected NotFound error"),
    }
}

#[tokio::test]
async fn test_get_transaction() {
    let service = create_test_service().await;
    
    // Create and submit a meta transaction
    let request = MetaTxRequest {
        tx_data: "0x0123456789abcdef".to_string(),
        sender: "neo1abc".to_string(),
        signature: "0xsignature".to_string(),
        nonce: 1,
        deadline: chrono::Utc::now().timestamp() as u64 + 3600, // 1 hour from now
        fee_model: "fixed".to_string(),
        fee_amount: 10,
    };
    
    let response = service.submit(request.clone()).await.unwrap();
    let request_id = response.request_id.clone();
    
    // Test getting an existing meta transaction
    let result = service.get_transaction(&request_id).await;
    
    assert!(result.is_ok());
    let record_opt = result.unwrap();
    assert!(record_opt.is_some());
    let record = record_opt.unwrap();
    assert_eq!(record.request_id, request_id);
    assert_eq!(record.request.tx_data, request.tx_data);
    assert_eq!(record.request.sender, request.sender);
    assert_eq!(record.status, MetaTxStatus::Submitted);
    
    // Test getting a non-existent meta transaction
    let result = service.get_transaction("non-existent-id").await;
    
    assert!(result.is_ok());
    let record_opt = result.unwrap();
    assert!(record_opt.is_none());
}

#[tokio::test]
async fn test_get_transactions_by_sender() {
    let service = create_test_service().await;
    
    // Create and submit multiple meta transactions from the same sender
    let sender = "neo1abc".to_string();
    
    for i in 1..=3 {
        let request = MetaTxRequest {
            tx_data: format!("0x{}", i),
            sender: sender.clone(),
            signature: "0xsignature".to_string(),
            nonce: i,
            deadline: chrono::Utc::now().timestamp() as u64 + 3600, // 1 hour from now
            fee_model: "fixed".to_string(),
            fee_amount: 10,
        };
        
        let _ = service.submit(request).await;
    }
    
    // Create and submit a meta transaction from a different sender
    let request = MetaTxRequest {
        tx_data: "0x4".to_string(),
        sender: "neo1def".to_string(),
        signature: "0xsignature".to_string(),
        nonce: 1,
        deadline: chrono::Utc::now().timestamp() as u64 + 3600, // 1 hour from now
        fee_model: "fixed".to_string(),
        fee_amount: 10,
    };
    
    let _ = service.submit(request).await;
    
    // Test getting transactions by sender
    let result = service.get_transactions_by_sender(&sender).await;
    
    assert!(result.is_ok());
    let records = result.unwrap();
    assert_eq!(records.len(), 3);
    
    // Verify all records have the correct sender
    for record in records {
        assert_eq!(record.request.sender, sender);
    }
    
    // Test getting transactions for a sender with no transactions
    let result = service.get_transactions_by_sender("neo1xyz").await;
    
    assert!(result.is_ok());
    let records = result.unwrap();
    assert_eq!(records.len(), 0);
}

#[tokio::test]
async fn test_get_next_nonce() {
    let service = create_test_service().await;
    
    // Create and submit multiple meta transactions from the same sender
    let sender = "neo1abc".to_string();
    
    for i in 1..=3 {
        let request = MetaTxRequest {
            tx_data: format!("0x{}", i),
            sender: sender.clone(),
            signature: "0xsignature".to_string(),
            nonce: i,
            deadline: chrono::Utc::now().timestamp() as u64 + 3600, // 1 hour from now
            fee_model: "fixed".to_string(),
            fee_amount: 10,
        };
        
        let _ = service.submit(request).await;
    }
    
    // Test getting the next nonce for a sender with transactions
    let result = service.get_next_nonce(&sender).await;
    
    assert!(result.is_ok());
    let nonce = result.unwrap();
    assert_eq!(nonce, 4); // 1, 2, 3 are used, so next is 4
    
    // Test getting the next nonce for a sender with no transactions
    let result = service.get_next_nonce("neo1xyz").await;
    
    assert!(result.is_ok());
    let nonce = result.unwrap();
    assert_eq!(nonce, 1); // No transactions, so start with 1
}

#[tokio::test]
async fn test_fee_models() {
    let service = create_test_service().await;
    
    // Test fixed fee model
    let request = MetaTxRequest {
        tx_data: "0x0123456789abcdef".to_string(),
        sender: "neo1abc".to_string(),
        signature: "0xsignature".to_string(),
        nonce: 1,
        deadline: chrono::Utc::now().timestamp() as u64 + 3600, // 1 hour from now
        fee_model: "fixed".to_string(),
        fee_amount: 20,
    };
    
    let result = service.submit(request).await;
    assert!(result.is_ok());
    
    // Test percentage fee model
    let request = MetaTxRequest {
        tx_data: "0x0123456789abcdef".to_string(),
        sender: "neo1def".to_string(),
        signature: "0xsignature".to_string(),
        nonce: 1,
        deadline: chrono::Utc::now().timestamp() as u64 + 3600, // 1 hour from now
        fee_model: "percentage".to_string(),
        fee_amount: 5,
    };
    
    let result = service.submit(request).await;
    assert!(result.is_ok());
    
    // Test dynamic fee model
    let request = MetaTxRequest {
        tx_data: "0x0123456789abcdef".to_string(),
        sender: "neo1ghi".to_string(),
        signature: "0xsignature".to_string(),
        nonce: 1,
        deadline: chrono::Utc::now().timestamp() as u64 + 3600, // 1 hour from now
        fee_model: "dynamic".to_string(),
        fee_amount: 0,
    };
    
    let result = service.submit(request).await;
    assert!(result.is_ok());
    
    // Test free fee model
    let request = MetaTxRequest {
        tx_data: "0x0123456789abcdef".to_string(),
        sender: "neo1jkl".to_string(),
        signature: "0xsignature".to_string(),
        nonce: 1,
        deadline: chrono::Utc::now().timestamp() as u64 + 3600, // 1 hour from now
        fee_model: "free".to_string(),
        fee_amount: 0,
    };
    
    let result = service.submit(request).await;
    assert!(result.is_ok());
}
