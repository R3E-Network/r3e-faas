//! Integration tests for Balance Management Service
//!
//! These tests verify the end-to-end flow of balance management in the system.

use std::sync::Arc;
use tokio::test;
use chrono::Utc;

use r3e_built_in_services::balance::{
    service::BalanceService,
    storage::InMemoryBalanceStorage,
    types::{
        BalanceAccount, BalanceTransaction, TransactionType,
        BalanceTransactionRequest, BalanceTransactionResponse,
    },
};

/// Test the end-to-end flow of account creation and deposit
#[tokio::test]
async fn test_account_creation_and_deposit() {
    // Set up test environment
    let balance_storage = Arc::new(InMemoryBalanceStorage::new());
    
    // Create the Balance service
    let balance_service = BalanceService::new(
        balance_storage.clone(),
    ).unwrap();
    
    // Create a test account
    let user_address = "neo1abc123def456";
    let result = balance_service.create_account(user_address, 0).await;
    
    // Verify the result
    assert!(result.is_ok());
    
    // Verify the account was created
    let account = balance_storage.get_account(user_address).await.unwrap();
    assert_eq!(account.address, user_address);
    assert_eq!(account.balance, 0);
    
    // Deposit funds to the account
    let deposit_request = BalanceTransactionRequest {
        address: user_address.to_string(),
        amount: 1000,
        transaction_type: TransactionType::Deposit,
        reference: Some("tx_hash_123".to_string()),
        description: Some("Initial deposit".to_string()),
    };
    
    let result = balance_service.process_transaction(deposit_request).await;
    
    // Verify the result
    assert!(result.is_ok());
    let response = result.unwrap();
    
    // Verify the response fields
    assert_eq!(response.address, user_address);
    assert_eq!(response.amount, 1000);
    assert_eq!(response.transaction_type, TransactionType::Deposit);
    assert_eq!(response.new_balance, 1000);
    
    // Verify the account balance was updated
    let account = balance_storage.get_account(user_address).await.unwrap();
    assert_eq!(account.balance, 1000);
    
    // Verify the transaction was stored
    let transactions = balance_storage.get_transactions(user_address).await.unwrap();
    assert_eq!(transactions.len(), 1);
    assert_eq!(transactions[0].amount, 1000);
    assert_eq!(transactions[0].transaction_type, TransactionType::Deposit);
}

/// Test the end-to-end flow of withdrawals
#[tokio::test]
async fn test_withdrawals() {
    // Set up test environment
    let balance_storage = Arc::new(InMemoryBalanceStorage::new());
    
    // Create the Balance service
    let balance_service = BalanceService::new(
        balance_storage.clone(),
    ).unwrap();
    
    // Create a test account with initial balance
    let user_address = "neo1abc123def456";
    balance_storage.create_account(user_address, 1000).await.unwrap();
    
    // Withdraw funds from the account
    let withdraw_request = BalanceTransactionRequest {
        address: user_address.to_string(),
        amount: 500,
        transaction_type: TransactionType::Withdrawal,
        reference: Some("tx_hash_456".to_string()),
        description: Some("Test withdrawal".to_string()),
    };
    
    let result = balance_service.process_transaction(withdraw_request).await;
    
    // Verify the result
    assert!(result.is_ok());
    let response = result.unwrap();
    
    // Verify the response fields
    assert_eq!(response.address, user_address);
    assert_eq!(response.amount, 500);
    assert_eq!(response.transaction_type, TransactionType::Withdrawal);
    assert_eq!(response.new_balance, 500);
    
    // Verify the account balance was updated
    let account = balance_storage.get_account(user_address).await.unwrap();
    assert_eq!(account.balance, 500);
    
    // Verify the transaction was stored
    let transactions = balance_storage.get_transactions(user_address).await.unwrap();
    assert_eq!(transactions.len(), 1);
    assert_eq!(transactions[0].amount, 500);
    assert_eq!(transactions[0].transaction_type, TransactionType::Withdrawal);
    
    // Attempt to withdraw more than the available balance
    let withdraw_request = BalanceTransactionRequest {
        address: user_address.to_string(),
        amount: 1000,
        transaction_type: TransactionType::Withdrawal,
        reference: Some("tx_hash_789".to_string()),
        description: Some("Excessive withdrawal".to_string()),
    };
    
    let result = balance_service.process_transaction(withdraw_request).await;
    
    // Verify the result is an error
    assert!(result.is_err());
    
    // Verify the account balance remains unchanged
    let account = balance_storage.get_account(user_address).await.unwrap();
    assert_eq!(account.balance, 500);
}

/// Test the end-to-end flow of service fee deductions
#[tokio::test]
async fn test_service_fee_deductions() {
    // Set up test environment
    let balance_storage = Arc::new(InMemoryBalanceStorage::new());
    
    // Create the Balance service
    let balance_service = BalanceService::new(
        balance_storage.clone(),
    ).unwrap();
    
    // Create a test account with initial balance
    let user_address = "neo1abc123def456";
    balance_storage.create_account(user_address, 1000).await.unwrap();
    
    // Deduct a service fee from the account
    let fee_request = BalanceTransactionRequest {
        address: user_address.to_string(),
        amount: 50,
        transaction_type: TransactionType::ServiceFee,
        reference: Some("service_123".to_string()),
        description: Some("Oracle service fee".to_string()),
    };
    
    let result = balance_service.process_transaction(fee_request).await;
    
    // Verify the result
    assert!(result.is_ok());
    let response = result.unwrap();
    
    // Verify the response fields
    assert_eq!(response.address, user_address);
    assert_eq!(response.amount, 50);
    assert_eq!(response.transaction_type, TransactionType::ServiceFee);
    assert_eq!(response.new_balance, 950);
    
    // Verify the account balance was updated
    let account = balance_storage.get_account(user_address).await.unwrap();
    assert_eq!(account.balance, 950);
    
    // Verify the transaction was stored
    let transactions = balance_storage.get_transactions(user_address).await.unwrap();
    assert_eq!(transactions.len(), 1);
    assert_eq!(transactions[0].amount, 50);
    assert_eq!(transactions[0].transaction_type, TransactionType::ServiceFee);
    
    // Deduct another service fee from the account
    let fee_request = BalanceTransactionRequest {
        address: user_address.to_string(),
        amount: 25,
        transaction_type: TransactionType::ServiceFee,
        reference: Some("service_456".to_string()),
        description: Some("Auto contract fee".to_string()),
    };
    
    let result = balance_service.process_transaction(fee_request).await;
    
    // Verify the result
    assert!(result.is_ok());
    let response = result.unwrap();
    
    // Verify the response fields
    assert_eq!(response.address, user_address);
    assert_eq!(response.amount, 25);
    assert_eq!(response.transaction_type, TransactionType::ServiceFee);
    assert_eq!(response.new_balance, 925);
    
    // Verify the account balance was updated
    let account = balance_storage.get_account(user_address).await.unwrap();
    assert_eq!(account.balance, 925);
    
    // Verify the transactions were stored
    let transactions = balance_storage.get_transactions(user_address).await.unwrap();
    assert_eq!(transactions.len(), 2);
}

/// Test the end-to-end flow of transaction history retrieval
#[tokio::test]
async fn test_transaction_history() {
    // Set up test environment
    let balance_storage = Arc::new(InMemoryBalanceStorage::new());
    
    // Create the Balance service
    let balance_service = BalanceService::new(
        balance_storage.clone(),
    ).unwrap();
    
    // Create a test account with initial balance
    let user_address = "neo1abc123def456";
    balance_storage.create_account(user_address, 0).await.unwrap();
    
    // Perform a series of transactions
    let transactions = vec![
        BalanceTransactionRequest {
            address: user_address.to_string(),
            amount: 1000,
            transaction_type: TransactionType::Deposit,
            reference: Some("tx_hash_123".to_string()),
            description: Some("Initial deposit".to_string()),
        },
        BalanceTransactionRequest {
            address: user_address.to_string(),
            amount: 50,
            transaction_type: TransactionType::ServiceFee,
            reference: Some("service_123".to_string()),
            description: Some("Oracle service fee".to_string()),
        },
        BalanceTransactionRequest {
            address: user_address.to_string(),
            amount: 200,
            transaction_type: TransactionType::Withdrawal,
            reference: Some("tx_hash_456".to_string()),
            description: Some("Test withdrawal".to_string()),
        },
        BalanceTransactionRequest {
            address: user_address.to_string(),
            amount: 25,
            transaction_type: TransactionType::ServiceFee,
            reference: Some("service_456".to_string()),
            description: Some("Auto contract fee".to_string()),
        },
    ];
    
    // Process all transactions
    for tx in transactions {
        let result = balance_service.process_transaction(tx).await;
        assert!(result.is_ok());
    }
    
    // Verify the account balance
    let account = balance_storage.get_account(user_address).await.unwrap();
    assert_eq!(account.balance, 725); // 1000 - 50 - 200 - 25
    
    // Get transaction history
    let result = balance_service.get_transaction_history(user_address).await;
    
    // Verify the result
    assert!(result.is_ok());
    let history = result.unwrap();
    
    // Verify the transaction history
    assert_eq!(history.len(), 4);
    
    // Verify the transactions are in the correct order (most recent first)
    assert_eq!(history[0].transaction_type, TransactionType::ServiceFee);
    assert_eq!(history[0].amount, 25);
    
    assert_eq!(history[1].transaction_type, TransactionType::Withdrawal);
    assert_eq!(history[1].amount, 200);
    
    assert_eq!(history[2].transaction_type, TransactionType::ServiceFee);
    assert_eq!(history[2].amount, 50);
    
    assert_eq!(history[3].transaction_type, TransactionType::Deposit);
    assert_eq!(history[3].amount, 1000);
}
