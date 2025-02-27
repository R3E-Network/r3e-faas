// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::sync::Arc;
use tokio::test;
use r3e_neo_services::{
    gas_bank::{GasBankService, GasBankServiceTrait, GasBankAccount, GasBankDeposit, GasBankWithdrawal, GasBankTransaction, storage::GasBankStorage, storage::InMemoryGasBankStorage},
    types::FeeModel,
    Error,
};
use NeoRust::prelude::{Wallet, RpcClient};
use url::Url;

// Mock RPC client for testing
struct MockRpcClient;

impl RpcClient for MockRpcClient {
    // Implement required methods for the RpcClient trait
    // This is a simplified mock implementation for testing
}

// Helper function to create a test Gas Bank Service
async fn create_test_service() -> GasBankService<InMemoryGasBankStorage> {
    let storage = Arc::new(InMemoryGasBankStorage::new());
    let rpc_client = Arc::new(MockRpcClient);
    let wallet = Arc::new(Wallet::new("NeoWallet".to_string()));
    
    GasBankService::new(
        storage,
        rpc_client,
        wallet,
        "testnet".to_string(),
        FeeModel::Fixed(10),
        1000,
    )
}

#[tokio::test]
async fn test_create_account() {
    let service = create_test_service().await;
    
    // Test creating an account with fixed fee model
    let result = service.create_account(
        "neo1abc",
        FeeModel::Fixed(10),
        1000,
    ).await;
    
    assert!(result.is_ok());
    let account = result.unwrap();
    assert_eq!(account.address, "neo1abc");
    assert_eq!(account.balance, 0);
    assert!(matches!(account.fee_model, FeeModel::Fixed(10)));
    assert_eq!(account.credit_limit, 1000);
    assert_eq!(account.used_credit, 0);
    assert_eq!(account.status, "active");
    
    // Test creating an account with percentage fee model
    let result = service.create_account(
        "neo1def",
        FeeModel::Percentage(2.5),
        2000,
    ).await;
    
    assert!(result.is_ok());
    let account = result.unwrap();
    assert_eq!(account.address, "neo1def");
    assert!(matches!(account.fee_model, FeeModel::Percentage(p) if p == 2.5));
    
    // Test creating an account with dynamic fee model
    let result = service.create_account(
        "neo1ghi",
        FeeModel::Dynamic,
        3000,
    ).await;
    
    assert!(result.is_ok());
    let account = result.unwrap();
    assert_eq!(account.address, "neo1ghi");
    assert!(matches!(account.fee_model, FeeModel::Dynamic));
    
    // Test creating an account with free fee model
    let result = service.create_account(
        "neo1jkl",
        FeeModel::Free,
        4000,
    ).await;
    
    assert!(result.is_ok());
    let account = result.unwrap();
    assert_eq!(account.address, "neo1jkl");
    assert!(matches!(account.fee_model, FeeModel::Free));
    
    // Test creating a duplicate account (should fail)
    let result = service.create_account(
        "neo1abc",
        FeeModel::Fixed(10),
        1000,
    ).await;
    
    assert!(result.is_err());
    match result {
        Err(Error::InvalidParameter(_)) => (),
        _ => panic!("Expected InvalidParameter error"),
    }
}

#[tokio::test]
async fn test_get_account() {
    let service = create_test_service().await;
    
    // Create an account
    let _ = service.create_account(
        "neo1abc",
        FeeModel::Fixed(10),
        1000,
    ).await;
    
    // Test getting an existing account
    let result = service.get_account("neo1abc").await;
    assert!(result.is_ok());
    let account_opt = result.unwrap();
    assert!(account_opt.is_some());
    let account = account_opt.unwrap();
    assert_eq!(account.address, "neo1abc");
    
    // Test getting a non-existent account
    let result = service.get_account("neo1xyz").await;
    assert!(result.is_ok());
    let account_opt = result.unwrap();
    assert!(account_opt.is_none());
}

#[tokio::test]
async fn test_deposit() {
    let service = create_test_service().await;
    
    // Create an account
    let _ = service.create_account(
        "neo1abc",
        FeeModel::Fixed(10),
        1000,
    ).await;
    
    // Test depositing to an existing account
    let result = service.deposit(
        "0x1234",
        "neo1abc",
        100,
    ).await;
    
    assert!(result.is_ok());
    let deposit = result.unwrap();
    assert_eq!(deposit.tx_hash, "0x1234");
    assert_eq!(deposit.address, "neo1abc");
    assert_eq!(deposit.amount, 100);
    assert_eq!(deposit.status, "confirmed");
    
    // Verify account balance was updated
    let account_result = service.get_account("neo1abc").await;
    assert!(account_result.is_ok());
    let account_opt = account_result.unwrap();
    assert!(account_opt.is_some());
    let account = account_opt.unwrap();
    assert_eq!(account.balance, 100);
    
    // Test depositing to a non-existent account (should create the account)
    let result = service.deposit(
        "0x5678",
        "neo1def",
        200,
    ).await;
    
    assert!(result.is_ok());
    let deposit = result.unwrap();
    assert_eq!(deposit.tx_hash, "0x5678");
    assert_eq!(deposit.address, "neo1def");
    assert_eq!(deposit.amount, 200);
    
    // Verify account was created with correct balance
    let account_result = service.get_account("neo1def").await;
    assert!(account_result.is_ok());
    let account_opt = account_result.unwrap();
    assert!(account_opt.is_some());
    let account = account_opt.unwrap();
    assert_eq!(account.balance, 200);
}

#[tokio::test]
async fn test_withdraw() {
    let service = create_test_service().await;
    
    // Create an account with initial deposit
    let _ = service.create_account(
        "neo1abc",
        FeeModel::Fixed(10),
        1000,
    ).await;
    
    let _ = service.deposit(
        "0x1234",
        "neo1abc",
        500,
    ).await;
    
    // Test withdrawing from an account
    let result = service.withdraw(
        "neo1abc",
        200,
    ).await;
    
    assert!(result.is_ok());
    let withdrawal = result.unwrap();
    assert_eq!(withdrawal.address, "neo1abc");
    assert_eq!(withdrawal.amount, 200);
    assert_eq!(withdrawal.fee, 10); // Fixed fee
    assert_eq!(withdrawal.status, "confirmed");
    
    // Verify account balance was updated
    let account_result = service.get_account("neo1abc").await;
    assert!(account_result.is_ok());
    let account_opt = account_result.unwrap();
    assert!(account_opt.is_some());
    let account = account_opt.unwrap();
    assert_eq!(account.balance, 290); // 500 - 200 - 10 (fee)
    
    // Test withdrawing more than available balance (should fail)
    let result = service.withdraw(
        "neo1abc",
        300,
    ).await;
    
    assert!(result.is_err());
    match result {
        Err(Error::InsufficientFunds(_)) => (),
        _ => panic!("Expected InsufficientFunds error"),
    }
    
    // Test withdrawing from a non-existent account (should fail)
    let result = service.withdraw(
        "neo1xyz",
        100,
    ).await;
    
    assert!(result.is_err());
    match result {
        Err(Error::NotFound(_)) => (),
        _ => panic!("Expected NotFound error"),
    }
}

#[tokio::test]
async fn test_pay_gas_for_transaction() {
    let service = create_test_service().await;
    
    // Create an account with initial deposit
    let _ = service.create_account(
        "neo1abc",
        FeeModel::Fixed(10),
        1000,
    ).await;
    
    let _ = service.deposit(
        "0x1234",
        "neo1abc",
        500,
    ).await;
    
    // Test paying gas for a transaction
    let result = service.pay_gas_for_transaction(
        "0x5678",
        "neo1abc",
        100,
    ).await;
    
    assert!(result.is_ok());
    let transaction = result.unwrap();
    assert_eq!(transaction.tx_hash, "0x5678");
    assert_eq!(transaction.address, "neo1abc");
    assert_eq!(transaction.amount, 100);
    assert_eq!(transaction.fee, 10); // Fixed fee
    assert_eq!(transaction.tx_type, "gas_payment");
    assert_eq!(transaction.status, "confirmed");
    
    // Verify account balance was updated
    let account_result = service.get_account("neo1abc").await;
    assert!(account_result.is_ok());
    let account_opt = account_result.unwrap();
    assert!(account_opt.is_some());
    let account = account_opt.unwrap();
    assert_eq!(account.balance, 390); // 500 - 100 - 10 (fee)
    
    // Test paying gas with insufficient balance but within credit limit
    let result = service.pay_gas_for_transaction(
        "0x9abc",
        "neo1abc",
        400,
    ).await;
    
    assert!(result.is_ok());
    let transaction = result.unwrap();
    assert_eq!(transaction.tx_hash, "0x9abc");
    assert_eq!(transaction.amount, 400);
    
    // Verify account balance and credit usage
    let account_result = service.get_account("neo1abc").await;
    assert!(account_result.is_ok());
    let account_opt = account_result.unwrap();
    assert!(account_opt.is_some());
    let account = account_opt.unwrap();
    assert_eq!(account.balance, 0);
    assert_eq!(account.used_credit, 20); // 400 + 10 (fee) - 390 (previous balance)
    
    // Test paying gas exceeding balance and credit limit (should fail)
    let result = service.pay_gas_for_transaction(
        "0xdef0",
        "neo1abc",
        1000,
    ).await;
    
    assert!(result.is_err());
    match result {
        Err(Error::InsufficientFunds(_)) => (),
        _ => panic!("Expected InsufficientFunds error"),
    }
    
    // Test paying gas for a non-existent account (should fail)
    let result = service.pay_gas_for_transaction(
        "0x1234",
        "neo1xyz",
        100,
    ).await;
    
    assert!(result.is_err());
    match result {
        Err(Error::NotFound(_)) => (),
        _ => panic!("Expected NotFound error"),
    }
}

#[tokio::test]
async fn test_get_gas_price() {
    let service = create_test_service().await;
    
    // Test getting gas price
    let result = service.get_gas_price().await;
    assert!(result.is_ok());
    let gas_price = result.unwrap();
    assert_eq!(gas_price, 1000); // Mock implementation returns 1000
}

#[tokio::test]
async fn test_estimate_gas() {
    let service = create_test_service().await;
    
    // Test estimating gas for a transaction
    let result = service.estimate_gas(&[1, 2, 3, 4, 5]).await;
    assert!(result.is_ok());
    let gas = result.unwrap();
    assert_eq!(gas, 21000 + (5 * 68)); // Mock implementation returns 21000 + (data_size * 68)
}

#[tokio::test]
async fn test_get_balance() {
    let service = create_test_service().await;
    
    // Create an account with initial deposit
    let _ = service.create_account(
        "neo1abc",
        FeeModel::Fixed(10),
        1000,
    ).await;
    
    let _ = service.deposit(
        "0x1234",
        "neo1abc",
        500,
    ).await;
    
    // Test getting balance for an existing account
    let result = service.get_balance("neo1abc").await;
    assert!(result.is_ok());
    let balance = result.unwrap();
    assert_eq!(balance, 500);
    
    // Test getting balance for a non-existent account (should fail)
    let result = service.get_balance("neo1xyz").await;
    assert!(result.is_err());
    match result {
        Err(Error::NotFound(_)) => (),
        _ => panic!("Expected NotFound error"),
    }
}

#[tokio::test]
async fn test_get_transactions() {
    let service = create_test_service().await;
    
    // Create an account with initial deposit
    let _ = service.create_account(
        "neo1abc",
        FeeModel::Fixed(10),
        1000,
    ).await;
    
    let _ = service.deposit(
        "0x1234",
        "neo1abc",
        500,
    ).await;
    
    // Perform some transactions
    let _ = service.withdraw(
        "neo1abc",
        100,
    ).await;
    
    let _ = service.pay_gas_for_transaction(
        "0x5678",
        "neo1abc",
        50,
    ).await;
    
    // Test getting transactions for an account
    let result = service.get_transactions("neo1abc").await;
    assert!(result.is_ok());
    let transactions = result.unwrap();
    assert_eq!(transactions.len(), 2); // 1 withdrawal + 1 gas payment
    
    // Verify transaction details
    let withdrawal = transactions.iter().find(|t| t.tx_type == "withdrawal").unwrap();
    assert_eq!(withdrawal.address, "neo1abc");
    assert_eq!(withdrawal.amount, 100);
    assert_eq!(withdrawal.fee, 10);
    
    let gas_payment = transactions.iter().find(|t| t.tx_type == "gas_payment").unwrap();
    assert_eq!(gas_payment.address, "neo1abc");
    assert_eq!(gas_payment.amount, 50);
    assert_eq!(gas_payment.fee, 10);
    
    // Test getting transactions for a non-existent account
    let result = service.get_transactions("neo1xyz").await;
    assert!(result.is_ok());
    let transactions = result.unwrap();
    assert_eq!(transactions.len(), 0);
}
