// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::sync::Arc;
use tokio::test;

use r3e_neo_services::gas_bank::{GasBankService, GasBankStorage, GasBankAccount};
use r3e_neo_services::types::FeeModel;
use r3e_neo_services::Error;

// Mock implementation of GasBankStorage for testing
struct MockGasBankStorage {
    accounts: std::collections::HashMap<String, GasBankAccount>,
    contract_mappings: std::collections::HashMap<String, String>,
}

#[async_trait::async_trait]
impl GasBankStorage for MockGasBankStorage {
    async fn get_account(&self, address: &str) -> Result<Option<GasBankAccount>, Error> {
        Ok(self.accounts.get(address).cloned())
    }
    
    async fn create_account(&self, account: GasBankAccount) -> Result<(), Error> {
        if self.accounts.contains_key(&account.address) {
            return Err(Error::InvalidParameter(format!("Account already exists for address: {}", account.address)));
        }
        
        self.accounts.insert(account.address.clone(), account);
        Ok(())
    }
    
    async fn update_account(&self, account: GasBankAccount) -> Result<(), Error> {
        if !self.accounts.contains_key(&account.address) {
            return Err(Error::NotFound(format!("Account not found for address: {}", account.address)));
        }
        
        self.accounts.insert(account.address.clone(), account);
        Ok(())
    }
    
    async fn get_deposits(&self, _address: &str) -> Result<Vec<r3e_neo_services::gas_bank::GasBankDeposit>, Error> {
        Ok(Vec::new())
    }
    
    async fn add_deposit(&self, _deposit: r3e_neo_services::gas_bank::GasBankDeposit) -> Result<(), Error> {
        Ok(())
    }
    
    async fn get_withdrawals(&self, _address: &str) -> Result<Vec<r3e_neo_services::gas_bank::GasBankWithdrawal>, Error> {
        Ok(Vec::new())
    }
    
    async fn add_withdrawal(&self, _withdrawal: r3e_neo_services::gas_bank::GasBankWithdrawal) -> Result<(), Error> {
        Ok(())
    }
    
    async fn get_transactions(&self, _address: &str) -> Result<Vec<r3e_neo_services::gas_bank::GasBankTransaction>, Error> {
        Ok(Vec::new())
    }
    
    async fn add_transaction(&self, _transaction: r3e_neo_services::gas_bank::GasBankTransaction) -> Result<(), Error> {
        Ok(())
    }
    
    async fn get_contract_account_mapping(&self, contract_hash: &str) -> Result<Option<String>, Error> {
        Ok(self.contract_mappings.get(contract_hash).cloned())
    }
    
    async fn set_contract_account_mapping(&self, contract_hash: &str, address: &str) -> Result<(), Error> {
        self.contract_mappings.insert(contract_hash.to_string(), address.to_string());
        Ok(())
    }
}

impl MockGasBankStorage {
    fn new() -> Self {
        Self {
            accounts: std::collections::HashMap::new(),
            contract_mappings: std::collections::HashMap::new(),
        }
    }
}

#[test]
async fn test_contract_account_mapping() {
    // Create a mock storage
    let storage = Arc::new(MockGasBankStorage::new());
    
    // Create a Gas Bank service
    let rpc_client = Arc::new(MockRpcClient::new());
    let wallet = Arc::new(MockWallet::new());
    let network = "testnet".to_string();
    let fee_model = FeeModel::Percentage(1.0); // 1% fee
    let credit_limit = 1_000_000_000; // 1 GAS
    
    let service = GasBankService::new(
        storage.clone(),
        rpc_client,
        wallet,
        network,
        fee_model,
        credit_limit,
    );
    
    // Test contract hash
    let contract_hash = "0x1234567890abcdef1234567890abcdef12345678";
    
    // Test that no account is mapped initially
    let account = service.get_account_for_contract(contract_hash).await.unwrap();
    assert!(account.is_none());
    
    // Create an account
    let address = "NeoAddress123";
    let account = GasBankAccount {
        address: address.to_string(),
        balance: 1_000_000_000, // 1 GAS
        credit_limit: 500_000_000, // 0.5 GAS
        fee_model: FeeModel::Percentage(1.0), // 1% fee
        created_at: 0,
        updated_at: 0,
    };
    
    service.create_account(account.clone()).await.unwrap();
    
    // Map the contract to the account
    service.set_contract_account_mapping(contract_hash, address).await.unwrap();
    
    // Test that the account is now mapped
    let mapped_account = service.get_account_for_contract(contract_hash).await.unwrap();
    assert!(mapped_account.is_some());
    assert_eq!(mapped_account.unwrap().address, address);
    
    // Test updating the mapping
    let new_address = "NewNeoAddress456";
    let new_account = GasBankAccount {
        address: new_address.to_string(),
        balance: 2_000_000_000, // 2 GAS
        credit_limit: 1_000_000_000, // 1 GAS
        fee_model: FeeModel::Percentage(2.0), // 2% fee
        created_at: 0,
        updated_at: 0,
    };
    
    service.create_account(new_account.clone()).await.unwrap();
    service.set_contract_account_mapping(contract_hash, new_address).await.unwrap();
    
    // Test that the account mapping is updated
    let mapped_account = service.get_account_for_contract(contract_hash).await.unwrap();
    assert!(mapped_account.is_some());
    assert_eq!(mapped_account.unwrap().address, new_address);
}

#[test]
async fn test_fee_calculation() {
    // Create a mock storage
    let storage = Arc::new(MockGasBankStorage::new());
    
    // Create a Gas Bank service
    let rpc_client = Arc::new(MockRpcClient::new());
    let wallet = Arc::new(MockWallet::new());
    let network = "testnet".to_string();
    let fee_model = FeeModel::Percentage(1.0); // 1% fee
    let credit_limit = 1_000_000_000; // 1 GAS
    
    let service = GasBankService::new(
        storage.clone(),
        rpc_client,
        wallet,
        network,
        fee_model.clone(),
        credit_limit,
    );
    
    // Test percentage fee model
    let gas_cost = 1_000_000_000; // 1 GAS
    let fee = service.calculate_fee(gas_cost, &fee_model).await.unwrap();
    assert_eq!(fee, 10_000_000); // 1% of 1 GAS = 0.01 GAS
    
    // Test fixed fee model
    let fixed_fee_model = FeeModel::Fixed(5_000_000); // 0.005 GAS
    let fee = service.calculate_fee(gas_cost, &fixed_fee_model).await.unwrap();
    assert_eq!(fee, 5_000_000); // 0.005 GAS
    
    // Test tiered fee model
    let tiered_fee_model = FeeModel::Tiered(vec![
        (500_000_000, 5_000_000),   // Up to 0.5 GAS: 0.005 GAS fee
        (1_000_000_000, 8_000_000), // Up to 1 GAS: 0.008 GAS fee
        (2_000_000_000, 15_000_000), // Up to 2 GAS: 0.015 GAS fee
    ]);
    
    // Test tier 1
    let gas_cost = 400_000_000; // 0.4 GAS
    let fee = service.calculate_fee(gas_cost, &tiered_fee_model).await.unwrap();
    assert_eq!(fee, 5_000_000); // 0.005 GAS
    
    // Test tier 2
    let gas_cost = 800_000_000; // 0.8 GAS
    let fee = service.calculate_fee(gas_cost, &tiered_fee_model).await.unwrap();
    assert_eq!(fee, 8_000_000); // 0.008 GAS
    
    // Test tier 3
    let gas_cost = 1_500_000_000; // 1.5 GAS
    let fee = service.calculate_fee(gas_cost, &tiered_fee_model).await.unwrap();
    assert_eq!(fee, 15_000_000); // 0.015 GAS
    
    // Test above highest tier
    let gas_cost = 3_000_000_000; // 3 GAS
    let fee = service.calculate_fee(gas_cost, &tiered_fee_model).await.unwrap();
    assert_eq!(fee, 15_000_000); // 0.015 GAS (highest tier)
}

// Mock RPC client for testing
struct MockRpcClient;

impl MockRpcClient {
    fn new() -> Self {
        Self
    }
}

// Mock wallet for testing
struct MockWallet;

impl MockWallet {
    fn new() -> Self {
        Self
    }
}
