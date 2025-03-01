// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! Fully Homomorphic Encryption service for the R3E FaaS platform.
//!
//! This crate provides a service for fully homomorphic encryption operations,
//! supporting multiple FHE schemes like TFHE and OpenFHE.

mod config;
mod error;
pub mod scheme;
mod service;
pub mod storage;
mod types;

pub use config::*;
pub use error::*;
pub use service::*;
pub use types::*;
