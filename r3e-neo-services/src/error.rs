// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Neo RPC error: {0}")]
    RpcError(String),

    #[error("Wallet error: {0}")]
    WalletError(String),

    #[error("Transaction error: {0}")]
    TransactionError(String),

    #[error("Gas bank error: {0}")]
    GasBankError(String),

    #[error("Meta transaction error: {0}")]
    MetaTxError(String),

    #[error("Abstract account error: {0}")]
    AbstractAccountError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Authentication error: {0}")]
    AuthError(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Insufficient funds: {0}")]
    InsufficientFunds(String),

    #[error("Invalid signature: {0}")]
    InvalidSignature(String),

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("Internal error: {0}")]
    InternalError(String),
}

impl From<neo3::prelude::Error> for Error {
    fn from(err: neo3::prelude::Error) -> Self {
        Error::RpcError(err.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::SerializationError(err.to_string())
    }
}
