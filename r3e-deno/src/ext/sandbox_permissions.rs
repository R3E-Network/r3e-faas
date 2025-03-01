// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use deno_core::error::AnyError;
use deno_core::op2;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::Mutex;

use crate::sandbox::{check_permission, SandboxConfig};

/// Sandbox permission request
#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionRequest {
    pub operation: String,
    pub resource: Option<String>,
}

/// Sandbox permission response
#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionResponse {
    pub granted: bool,
    pub message: Option<String>,
}

#[op2]
#[serde]
pub fn op_request_permission(
    #[serde] request: PermissionRequest,
    #[state] sandbox_config: &Arc<Mutex<SandboxConfig>>,
) -> Result<PermissionResponse, AnyError> {
    let config = sandbox_config.lock().unwrap();

    match check_permission(&request.operation, &config) {
        Ok(_) => Ok(PermissionResponse {
            granted: true,
            message: None,
        }),
        Err(message) => Ok(PermissionResponse {
            granted: false,
            message: Some(message),
        }),
    }
}
