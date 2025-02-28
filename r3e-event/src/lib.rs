// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! # R3E Event
//! 
//! Event handling system for the R3E FaaS platform.

pub mod types;
pub mod error;
pub mod config;
pub mod source;
pub mod registry;
pub mod trigger;

// Re-export important types
pub use types::{Trigger, Source, Context, EventData, Event};
pub use error::{Error, Result};
