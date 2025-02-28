// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! Zero-Knowledge computing service for the R3E FaaS platform.
//!
//! This crate provides a service for zero-knowledge proof generation and verification,
//! supporting multiple ZK platforms like Zokrates and Bulletproofs.

mod config;
mod error;
mod service;
mod types;
pub mod provider;
pub mod storage;

pub use config::*;
pub use error::*;
pub use service::*;
pub use types::*;
