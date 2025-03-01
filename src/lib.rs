// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! # R3E Store Test
//!
//! Storage abstractions for the R3E FaaS platform - test implementation.

pub mod config;
pub mod error;
pub mod repository;
pub mod rocksdb;
pub mod storage;
pub mod types;

// Re-export repository types
pub use repository::user::{User, UserRepository, CF_USERS};
pub use repository::service::{
    Service, ServiceRepository, CF_SERVICES, ServiceType, BlockchainType,
}; 