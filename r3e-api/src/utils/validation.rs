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
            .map(|e| {
                e.message
                    .clone()
                    .unwrap_or_else(|| "Invalid value".into())
                    .to_string()
            })
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

    // Check for potentially dangerous patterns
    let dangerous_patterns = [
        // Process manipulation
        "process.exit",
        "process.kill",
        "process.abort",
        // Deno system access
        "Deno.exit",
        "Deno.permissions",
        "Deno.chmod",
        "Deno.chown",
        "Deno.remove",
        "Deno.symlink",
        "Deno.truncate",
        "Deno.writeFile",
        "Deno.writeTextFile",
        "Deno.writeFileSync",
        "Deno.writeTextFileSync",
        "Deno.run",
        "Deno.Command",
        // Eval and dynamic code execution
        "eval(",
        "new Function(",
        "setTimeout(",
        "setInterval(",
        "Function(",
        "constructor.constructor",
        // Network access bypassing
        "fetch(",
        "XMLHttpRequest",
        "WebSocket",
        // DOM access (should not be available but check anyway)
        "document.",
        "window.",
        "navigator.",
        "location.",
        // Storage access
        "localStorage",
        "sessionStorage",
        "indexedDB",
        // Worker threads
        "Worker(",
        "SharedWorker(",
        "ServiceWorker",
        // Crypto access that might be used for mining
        "crypto.subtle",
        "SubtleCrypto",
        // Prototype manipulation
        "__proto__",
        "Object.prototype",
        "Function.prototype",
        // Imports that bypass sandboxing
        "import(",
        "require(",
        "module.exports",
    ];

    for pattern in dangerous_patterns {
        if code.contains(pattern) {
            return Err(ValidationError::new(&format!(
                "code_contains_dangerous_pattern: {}",
                pattern
            )));
        }
    }

    // Check for export pattern to ensure the function is properly exported
    if !code.contains("export default")
        && !code.contains("export function")
        && !code.contains("export const")
    {
        return Err(ValidationError::new("code_missing_export"));
    }

    // Check for infinite loops patterns
    let loop_patterns = [
        "while(true)",
        "while (true)",
        "for(;;)",
        "for (;;)",
        "while(1)",
        "while (1)",
    ];

    for pattern in loop_patterns {
        if code.contains(pattern) {
            return Err(ValidationError::new(&format!(
                "code_contains_potential_infinite_loop: {}",
                pattern
            )));
        }
    }

    Ok(())
}

/// Validate JSON schema for function inputs
pub fn validate_json_schema(
    input: &serde_json::Value,
    schema: &serde_json::Value,
) -> Result<(), ValidationError> {
    use jsonschema::{Draft, JSONSchema};

    // Compile the schema
    let compiled_schema = JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(schema)
        .map_err(|e| ValidationError::new(&format!("invalid_schema: {}", e)))?;

    // Validate the input against the schema
    let result = compiled_schema.validate(input);

    if let Err(errors) = result {
        let error_messages: Vec<String> = errors.map(|error| format!("{}", error)).collect();

        return Err(ValidationError::new(&format!(
            "schema_validation_failed: {}",
            error_messages.join(", ")
        )));
    }

    Ok(())
}
