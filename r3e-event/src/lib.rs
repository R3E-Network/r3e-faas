// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use serde::{Deserialize, Serialize};

pub mod source;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Trigger {
    // Generic triggers
    NewBlock,
    NewTx,
    
    // Neo-specific triggers
    NeoNewBlock,
    NeoNewTx,
    NeoContractNotification,
    NeoApplicationLog,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Source {
    Bitcoin,
    // Ethereum,
    Neo,
}

// A function input is a pair of Context and Event
//
// Context is used to describe the context of the event,
// such as the trigger type, the triggered time, the source of the event
//
// Event is used to describe the params of the event,
// such as the data of the event
pub struct Context {
    // event trigger type
    pub tigger: Trigger,

    // event triggered time
    pub triggered_time: u64,

    // event source
    pub source: Source,
}
