// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! # R3E Config
//!
//! Configuration management for the R3E FaaS platform.

pub mod error;
pub mod loader;
pub mod provider;
pub mod types;

// Re-export important types
pub use error::{Error, Result};
pub use loader::ConfigLoader;
pub use provider::ConfigProvider;
pub use types::*;
