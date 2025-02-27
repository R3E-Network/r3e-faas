// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use regex::Regex;
use std::collections::HashMap;
use validator::{Validate, ValidationError, ValidationErrors};

use crate::error::ApiError;

/// Validate a struct
pub fn validate<T: Validate>(value: &T) -> Result<(), ApiError> {
    value
        .validate()
        .map_err(|e| ApiError::Validation(format_validation_errors(&e)))
}

/// Format validation errors
pub fn format_validation_errors(errors: &ValidationErrors) -> String {
    let mut formatted_errors = HashMap::new();
    
    for (field, error) in errors.field_errors() {
        let messages: Vec<String> = error
            .iter()
            .map(|e| e.message.clone().unwrap_or_else(|| "Invalid value".into()).to_string())
            .collect();
        
        formatted_errors.insert(field.to_string(), messages);
    }
    
    serde_json::to_string(&formatted_errors).unwrap_or_else(|_| "Validation failed".to_string())
}

/// Validate an email address
pub fn validate_email(email: &str) -> Result<(), ValidationError> {
    let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
        .map_err(|_| ValidationError::new("invalid_email_regex"))?;
    
    if !email_regex.is_match(email) {
        return Err(ValidationError::new("invalid_email"));
    }
    
    Ok(())
}

/// Validate a username
pub fn validate_username(username: &str) -> Result<(), ValidationError> {
    let username_regex = Regex::new(r"^[a-zA-Z0-9_-]{3,50}$")
        .map_err(|_| ValidationError::new("invalid_username_regex"))?;
    
    if !username_regex.is_match(username) {
        return Err(ValidationError::new("invalid_username"));
    }
    
    Ok(())
}

/// Validate a password
pub fn validate_password(password: &str) -> Result<(), ValidationError> {
    if password.len() < 8 {
        return Err(ValidationError::new("password_too_short"));
    }
    
    if password.len() > 100 {
        return Err(ValidationError::new("password_too_long"));
    }
    
    // Check for at least one uppercase letter
    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    if !has_uppercase {
        return Err(ValidationError::new("password_no_uppercase"));
    }
    
    // Check for at least one lowercase letter
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    if !has_lowercase {
        return Err(ValidationError::new("password_no_lowercase"));
    }
    
    // Check for at least one digit
    let has_digit = password.chars().any(|c| c.is_digit(10));
    if !has_digit {
        return Err(ValidationError::new("password_no_digit"));
    }
    
    // Check for at least one special character
    let has_special = password.chars().any(|c| !c.is_alphanumeric());
    if !has_special {
        return Err(ValidationError::new("password_no_special"));
    }
    
    Ok(())
}

/// Validate a service name
pub fn validate_service_name(name: &str) -> Result<(), ValidationError> {
    let name_regex = Regex::new(r"^[a-zA-Z0-9_-]{3,50}$")
        .map_err(|_| ValidationError::new("invalid_service_name_regex"))?;
    
    if !name_regex.is_match(name) {
        return Err(ValidationError::new("invalid_service_name"));
    }
    
    Ok(())
}

/// Validate a function name
pub fn validate_function_name(name: &str) -> Result<(), ValidationError> {
    let name_regex = Regex::new(r"^[a-zA-Z0-9_-]{3,50}$")
        .map_err(|_| ValidationError::new("invalid_function_name_regex"))?;
    
    if !name_regex.is_match(name) {
        return Err(ValidationError::new("invalid_function_name"));
    }
    
    Ok(())
}

/// Validate JavaScript code
pub fn validate_javascript_code(code: &str) -> Result<(), ValidationError> {
    if code.is_empty() {
        return Err(ValidationError::new("code_empty"));
    }
    
    if code.len() > 1000000 {
        return Err(ValidationError::new("code_too_long"));
    }
    
    // TODO: Add more validation for JavaScript code
    
    Ok(())
}
