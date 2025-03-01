// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::sync::Arc;
use std::collections::HashMap;
use tokio::test;
use r3e_neo_services::{
    abstract_account::{AbstractAccountService, AbstractAccountServiceTrait, AccountController, AccountPolicy, AccountOperation, AccountOperationRequest, AccountSignature, AccountStatus, OperationStatus, PolicyType, storage::InMemoryAbstractAccountStorage},
    Error,
};
use neo3::prelude::{Wallet, RpcClient};

// Mock RPC client for testing
struct MockRpcClient;
impl RpcClient for MockRpcClient {}

#[tokio::test]
async fn test_basic_account_operations() {
    // Create service
    let storage = Arc::new(InMemoryAbstractAccountStorage::new());
    let rpc_client = Arc::new(MockRpcClient);
    let wallet = Arc::new(Wallet::new("NeoWallet".to_string()));
    let service = AbstractAccountService::new(
        storage,
        rpc_client,
        wallet,
        "testnet".to_string(),
    );
    
    // Create policy
    let mut parameters = HashMap::new();
    parameters.insert("param1".to_string(), "value1".to_string());
    let policy = AccountPolicy {
        policy_type: PolicyType::MultiSig,
        parameters,
        required_signatures: 2,
        total_signatures: 3,
        time_lock: None,
        custom_script: None,
    };
    
    // Create controllers
    let controllers = vec![
        AccountController {
            address: "neo1abc".to_string(),
            weight: 1,
            controller_type: "standard".to_string(),
            added_at: chrono::Utc::now().timestamp() as u64,
            status: "active".to_string(),
        },
        AccountController {
            address: "neo1def".to_string(),
            weight: 1,
            controller_type: "standard".to_string(),
            added_at: chrono::Utc::now().timestamp() as u64,
            status: "active".to_string(),
        },
        AccountController {
            address: "neo1ghi".to_string(),
            weight: 1,
            controller_type: "standard".to_string(),
            added_at: chrono::Utc::now().timestamp() as u64,
            status: "active".to_string(),
        },
    ];
    
    // Create recovery addresses
    let recovery_addresses = vec!["neo1recovery".to_string()];
    
    // Create metadata
    let mut metadata = HashMap::new();
    metadata.insert("name".to_string(), "Test Account".to_string());
    
    // Test account creation
    let result = service.create_account(
        "neo1owner",
        controllers.clone(),
        recovery_addresses.clone(),
        policy.clone(),
        metadata.clone(),
        "0xsignature",
    ).await;
    
    assert!(result.is_ok());
    let account = result.unwrap();
    assert_eq!(account.owner, "neo1owner");
    assert_eq!(account.controllers.len(), 3);
    assert_eq!(account.recovery_addresses.len(), 1);
    assert_eq!(account.status, AccountStatus::Active.to_string());
}
