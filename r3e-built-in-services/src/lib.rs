// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

// Re-export all built-in services
pub mod auto_contract;
pub mod balance;
pub mod bridge;
pub mod fhe;
pub mod gas_bank;
pub mod identity;
pub mod indexing;
pub mod oracle;
pub mod pricing;
pub mod tee;
pub mod zk;

// Error types
#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("Oracle error: {0}")]
    Oracle(#[from] oracle::OracleError),

    #[error("Gas bank error: {0}")]
    GasBank(#[from] gas_bank::Error),

    #[error("TEE error: {0}")]
    Tee(#[from] tee::TeeError),

    #[error("Balance error: {0}")]
    Balance(String),

    #[error("Indexing error: {0}")]
    Indexing(#[from] indexing::IndexingError),

    #[error("Identity error: {0}")]
    Identity(#[from] identity::IdentityError),

    #[error("Bridge error: {0}")]
    Bridge(#[from] bridge::BridgeError),

    #[error("Pricing error: {0}")]
    Pricing(#[from] pricing::PricingError),

    #[error("Auto contract error: {0}")]
    AutoContract(#[from] auto_contract::AutoContractError),

    #[error("Zero-Knowledge error: {0}")]
    Zk(#[from] zk::ZkError),

    #[error("Fully Homomorphic Encryption error: {0}")]
    Fhe(#[from] fhe::FheError),
}
