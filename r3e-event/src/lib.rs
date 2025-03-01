// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! # R3E Event
//!
//! Event handling system for the R3E FaaS platform.

pub mod config;
pub mod error;
pub mod registry;
pub mod source;
pub mod trigger;
pub mod types;

// Re-export important types
pub use error::{Error, Result};
pub use types::{Context, Event, EventData, Source, Trigger};
