// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use uuid::Uuid;
use validator::ValidationError;

/// Validate UUID
pub fn validate_uuid(uuid: &Uuid) -> Result<(), ValidationError> {
    if uuid.is_nil() {
        return Err(ValidationError::new("uuid_is_nil"));
    }
    Ok(())
}

/// Validate timestamp
pub fn validate_timestamp(timestamp: &u64) -> Result<(), ValidationError> {
    if *timestamp == 0 {
        return Err(ValidationError::new("timestamp_is_zero"));
    }
    Ok(())
}

/// Validate function name
pub fn validate_function_name(name: &str) -> Result<(), ValidationError> {
    if name.is_empty() {
        return Err(ValidationError::new("function_name_empty"));
    }
    if name.len() > 256 {
        return Err(ValidationError::new("function_name_too_long"));
    }
    Ok(())
}
