// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

// Re-export all built-in services
pub mod oracle;
pub mod gas_bank;
pub mod tee;
pub mod balance;
pub mod indexing;
pub mod identity;
pub mod bridge;
pub mod pricing;

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
}
