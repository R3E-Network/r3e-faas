// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

// Re-export all built-in services
pub mod oracle;
pub mod gas_bank;
pub mod tee;
pub mod balance;

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
}
