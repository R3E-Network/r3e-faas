// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! Type definitions for the event crate.

use serde::{Deserialize, Serialize};

/// Trigger type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Trigger {
    /// Generic triggers
    NewBlock,
    NewTx,
    
    /// Neo-specific triggers
    NeoNewBlock,
    NeoNewTx,
    NeoContractNotification,
    NeoApplicationLog,
}

/// Source type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Source {
    /// Bitcoin blockchain
    Bitcoin,
    
    /// Neo blockchain
    Neo,
}

/// Event context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Context {
    /// Event trigger type
    pub trigger: Trigger,
    
    /// Event triggered time
    pub triggered_time: u64,
    
    /// Event source
    pub source: Source,
}

/// Event data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventData {
    /// Event ID
    pub id: String,
    
    /// Event payload
    pub payload: serde_json::Value,
}

/// Event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    /// Event context
    pub context: Context,
    
    /// Event data
    pub data: EventData,
}
