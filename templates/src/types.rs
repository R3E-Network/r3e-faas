// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! Type definitions for the crate.

use serde::{Deserialize, Serialize};

/// Main type definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MainType {
    /// ID field
    pub id: String,
    
    /// Name field
    pub name: String,
    
    /// Description field
    pub description: Option<String>,
}
