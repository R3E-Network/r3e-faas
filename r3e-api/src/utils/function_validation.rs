// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use regex::Regex;
use serde_json::Value;
use validator::ValidationError;

/// Validate function input
pub fn validate_function_input(input: &Value) -> Result<(), String> {
    // Check that the input is a valid JSON object
    if !input.is_object() {
        return Err("Input must be a valid JSON object".to_string());
    }
    
    // Check input size
    let input_str = serde_json::to_string(input).unwrap_or_default();
    if input_str.len() > 1000000 {
        return Err("Input is too large (max 1MB)".to_string());
    }
    
    // Check for potentially dangerous patterns in string values
    check_dangerous_patterns_in_json(input)?;
    
    Ok(())
}

/// Check for dangerous patterns in JSON values
fn check_dangerous_patterns_in_json(value: &Value) -> Result<(), String> {
    match value {
        Value::String(s) => {
            check_dangerous_patterns_in_string(s)?;
        },
        Value::Array(arr) => {
            for item in arr {
                check_dangerous_patterns_in_json(item)?;
            }
        },
        Value::Object(obj) => {
            for (key, val) in obj {
                // Check keys for dangerous patterns
                check_dangerous_patterns_in_string(key)?;
                
                // Check values recursively
                check_dangerous_patterns_in_json(val)?;
            }
        },
        _ => {
            // Other value types (null, boolean, number) are safe
        }
    }
    
    Ok(())
}

/// Check for dangerous patterns in a string
fn check_dangerous_patterns_in_string(s: &str) -> Result<(), String> {
    // Check for potentially dangerous patterns
    let dangerous_patterns = [
        // Script injection
        "<script", "javascript:", "data:text/html",
        
        // Command injection
        "; rm -rf", "; cat /etc", "$(", "`", "&& ",
        
        // SQL injection
        "DROP TABLE", "DELETE FROM", "'; --", "1=1 --",
        
        // Path traversal
        "../", "..\\", "/etc/passwd", "C:\\Windows\\",
        
        // XML injection
        "<![CDATA[", "<!ENTITY", "<!DOCTYPE",
        
        // Template injection
        "{{", "}}", "${", "<%=", "<%",
        
        // Potential serialized objects
        "O:8:", "a:2:", "__PHP_Incomplete_Class",
    ];
    
    for pattern in dangerous_patterns {
        if s.contains(pattern) {
            return Err(format!(
                "Input contains potentially dangerous pattern: {}",
                pattern
            ));
        }
    }
    
    Ok(())
}

/// Validate function metadata
pub fn validate_function_metadata(metadata: &Value) -> Result<(), String> {
    // Check that the metadata is a valid JSON object
    if !metadata.is_object() {
        return Err("Metadata must be a valid JSON object".to_string());
    }
    
    // Check required fields
    let required_fields = ["name", "description", "version"];
    for field in required_fields.iter() {
        if !metadata.get(*field).is_some() {
            return Err(format!("Missing required metadata field: {}", field));
        }
    }
    
    // Validate name
    if let Some(name) = metadata.get("name").and_then(|n| n.as_str()) {
        if name.is_empty() {
            return Err("Function name cannot be empty".to_string());
        }
        
        if name.len() > 50 {
            return Err("Function name is too long (max 50 characters)".to_string());
        }
        
        // Check name format
        let name_regex = Regex::new(r"^[a-zA-Z0-9_-]{3,50}$").unwrap();
        if !name_regex.is_match(name) {
            return Err("Function name must contain only alphanumeric characters, underscores, and hyphens".to_string());
        }
    }
    
    // Validate description
    if let Some(description) = metadata.get("description").and_then(|d| d.as_str()) {
        if description.is_empty() {
            return Err("Function description cannot be empty".to_string());
        }
        
        if description.len() > 1000 {
            return Err("Function description is too long (max 1000 characters)".to_string());
        }
    }
    
    // Validate version
    if let Some(version) = metadata.get("version").and_then(|v| v.as_str()) {
        if version.is_empty() {
            return Err("Function version cannot be empty".to_string());
        }
        
        // Check version format (semver)
        let version_regex = Regex::new(r"^\d+\.\d+\.\d+$").unwrap();
        if !version_regex.is_match(version) {
            return Err("Function version must be in semver format (e.g., 1.0.0)".to_string());
        }
    }
    
    // Validate author (optional)
    if let Some(author) = metadata.get("author").and_then(|a| a.as_str()) {
        if author.len() > 100 {
            return Err("Author name is too long (max 100 characters)".to_string());
        }
    }
    
    // Validate tags (optional)
    if let Some(tags) = metadata.get("tags") {
        if let Some(tags_array) = tags.as_array() {
            if tags_array.len() > 10 {
                return Err("Too many tags (max 10)".to_string());
            }
            
            for tag in tags_array {
                if let Some(tag_str) = tag.as_str() {
                    if tag_str.is_empty() {
                        return Err("Tag cannot be empty".to_string());
                    }
                    
                    if tag_str.len() > 20 {
                        return Err("Tag is too long (max 20 characters)".to_string());
                    }
                    
                    // Check tag format
                    let tag_regex = Regex::new(r"^[a-zA-Z0-9_-]{1,20}$").unwrap();
                    if !tag_regex.is_match(tag_str) {
                        return Err("Tag must contain only alphanumeric characters, underscores, and hyphens".to_string());
                    }
                } else {
                    return Err("Tags must be strings".to_string());
                }
            }
        } else {
            return Err("Tags must be an array".to_string());
        }
    }
    
    // Validate dependencies (optional)
    if let Some(dependencies) = metadata.get("dependencies") {
        if let Some(deps_obj) = dependencies.as_object() {
            if deps_obj.len() > 20 {
                return Err("Too many dependencies (max 20)".to_string());
            }
            
            for (dep_name, dep_version) in deps_obj {
                if dep_name.is_empty() {
                    return Err("Dependency name cannot be empty".to_string());
                }
                
                if dep_name.len() > 50 {
                    return Err("Dependency name is too long (max 50 characters)".to_string());
                }
                
                if let Some(version_str) = dep_version.as_str() {
                    if version_str.is_empty() {
                        return Err("Dependency version cannot be empty".to_string());
                    }
                    
                    if version_str.len() > 20 {
                        return Err("Dependency version is too long (max 20 characters)".to_string());
                    }
                } else {
                    return Err("Dependency version must be a string".to_string());
                }
            }
        } else {
            return Err("Dependencies must be an object".to_string());
        }
    }
    
    // Validate runtime (optional)
    if let Some(runtime) = metadata.get("runtime").and_then(|r| r.as_str()) {
        let allowed_runtimes = ["javascript", "typescript", "deno"];
        if !allowed_runtimes.contains(&runtime) {
            return Err(format!(
                "Invalid runtime: {}. Allowed runtimes: {}",
                runtime,
                allowed_runtimes.join(", ")
            ));
        }
    }
    
    // Validate permissions (optional)
    if let Some(permissions) = metadata.get("permissions") {
        if let Some(perms_obj) = permissions.as_object() {
            let allowed_permissions = ["net", "fs", "env", "run", "ffi", "hrtime"];
            
            for (perm_name, perm_value) in perms_obj {
                if !allowed_permissions.contains(&perm_name.as_str()) {
                    return Err(format!(
                        "Invalid permission: {}. Allowed permissions: {}",
                        perm_name,
                        allowed_permissions.join(", ")
                    ));
                }
                
                if !perm_value.is_boolean() {
                    return Err(format!("Permission value must be a boolean: {}", perm_name));
                }
            }
        } else {
            return Err("Permissions must be an object".to_string());
        }
    }
    
    Ok(())
}

/// Validate function code
pub fn validate_function_code(code: &str) -> Result<(), String> {
    if code.is_empty() {
        return Err("Function code cannot be empty".to_string());
    }
    
    if code.len() > 1000000 {
        return Err("Function code is too large (max 1MB)".to_string());
    }
    
    // Check for potentially dangerous patterns
    let dangerous_patterns = [
        // Process manipulation
        "process.exit", "process.kill", "process.abort",
        
        // Deno system access
        "Deno.exit", "Deno.permissions", "Deno.chmod", "Deno.chown",
        "Deno.remove", "Deno.symlink", "Deno.truncate",
        "Deno.writeFile", "Deno.writeTextFile", "Deno.writeFileSync",
        "Deno.writeTextFileSync", "Deno.run", "Deno.Command",
        
        // Eval and dynamic code execution
        "eval(", "new Function(", "setTimeout(", "setInterval(",
        "Function(", "constructor.constructor",
        
        // Network access bypassing
        "fetch(", "XMLHttpRequest", "WebSocket",
        
        // DOM access (should not be available but check anyway)
        "document.", "window.", "navigator.", "location.",
        
        // Storage access
        "localStorage", "sessionStorage", "indexedDB",
        
        // Worker threads
        "Worker(", "SharedWorker(", "ServiceWorker",
        
        // Crypto access that might be used for mining
        "crypto.subtle", "SubtleCrypto",
        
        // Prototype manipulation
        "__proto__", "Object.prototype", "Function.prototype",
        
        // Imports that bypass sandboxing
        "import(", "require(", "module.exports",
    ];
    
    for pattern in dangerous_patterns {
        if code.contains(pattern) {
            return Err(format!(
                "Code contains potentially dangerous pattern: {}",
                pattern
            ));
        }
    }
    
    // Check for export pattern to ensure the function is properly exported
    if !code.contains("export default") && !code.contains("export function") && !code.contains("export const") {
        return Err("Function must have a default export".to_string());
    }
    
    // Check for infinite loops patterns
    let loop_patterns = [
        "while(true)", "while (true)", "for(;;)", "for (;;)",
        "while(1)", "while (1)",
    ];
    
    for pattern in loop_patterns {
        if code.contains(pattern) {
            return Err(format!(
                "Code contains potential infinite loop: {}",
                pattern
            ));
        }
    }
    
    // Check for resource exhaustion patterns
    let resource_patterns = [
        "new Array(1000000000)", "new Uint8Array(1000000000)",
        "'.'.repeat(1000000000)", "Buffer.alloc(1000000000)",
    ];
    
    for pattern in resource_patterns {
        if code.contains(pattern) {
            return Err(format!(
                "Code contains potential resource exhaustion: {}",
                pattern
            ));
        }
    }
    
    // Check for recursive function calls without base case
    // This is a simple heuristic and may have false positives
    let recursive_patterns = [
        "function x() { x(); }", "const x = () => x()",
        "function x() { return x(); }", "const x = () => { return x(); }",
    ];
    
    for pattern in recursive_patterns {
        if code.contains(pattern) {
            return Err(format!(
                "Code contains potential infinite recursion: {}",
                pattern
            ));
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validate_function_input_valid() {
        let input = serde_json::json!({
            "name": "test",
            "value": 123,
            "nested": {
                "key": "value"
            }
        });
        
        let result = validate_function_input(&input);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_validate_function_input_invalid() {
        // Test non-object input
        let input = serde_json::json!("not an object");
        let result = validate_function_input(&input);
        assert!(result.is_err());
        
        // Test dangerous pattern in string
        let input = serde_json::json!({
            "name": "<script>alert('xss')</script>"
        });
        let result = validate_function_input(&input);
        assert!(result.is_err());
        
        // Test dangerous pattern in nested object
        let input = serde_json::json!({
            "name": "test",
            "nested": {
                "key": "'; DROP TABLE users; --"
            }
        });
        let result = validate_function_input(&input);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_validate_function_metadata_valid() {
        let metadata = serde_json::json!({
            "name": "test-function",
            "description": "A test function",
            "version": "1.0.0",
            "author": "Test Author",
            "tags": ["test", "example"],
            "dependencies": {
                "lodash": "4.17.21"
            },
            "runtime": "javascript",
            "permissions": {
                "net": true,
                "fs": false
            }
        });
        
        let result = validate_function_metadata(&metadata);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_validate_function_metadata_invalid() {
        // Test missing required fields
        let metadata = serde_json::json!({
            "name": "test-function",
            "description": "A test function"
        });
        let result = validate_function_metadata(&metadata);
        assert!(result.is_err());
        
        // Test invalid name
        let metadata = serde_json::json!({
            "name": "test function with spaces",
            "description": "A test function",
            "version": "1.0.0"
        });
        let result = validate_function_metadata(&metadata);
        assert!(result.is_err());
        
        // Test invalid version
        let metadata = serde_json::json!({
            "name": "test-function",
            "description": "A test function",
            "version": "invalid"
        });
        let result = validate_function_metadata(&metadata);
        assert!(result.is_err());
        
        // Test invalid runtime
        let metadata = serde_json::json!({
            "name": "test-function",
            "description": "A test function",
            "version": "1.0.0",
            "runtime": "invalid"
        });
        let result = validate_function_metadata(&metadata);
        assert!(result.is_err());
        
        // Test invalid permission
        let metadata = serde_json::json!({
            "name": "test-function",
            "description": "A test function",
            "version": "1.0.0",
            "permissions": {
                "invalid": true
            }
        });
        let result = validate_function_metadata(&metadata);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_validate_function_code_valid() {
        let code = r#"
        // A simple function that adds two numbers
        export default function(input) {
            return input.a + input.b;
        }
        "#;
        
        let result = validate_function_code(code);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_validate_function_code_invalid() {
        // Test missing export
        let code = r#"
        // A simple function that adds two numbers
        function add(input) {
            return input.a + input.b;
        }
        "#;
        let result = validate_function_code(code);
        assert!(result.is_err());
        
        // Test dangerous pattern
        let code = r#"
        // A function that uses eval
        export default function(input) {
            return eval(input.code);
        }
        "#;
        let result = validate_function_code(code);
        assert!(result.is_err());
        
        // Test infinite loop
        let code = r#"
        // A function with an infinite loop
        export default function(input) {
            while(true) {
                console.log("Infinite loop");
            }
            return input.a + input.b;
        }
        "#;
        let result = validate_function_code(code);
        assert!(result.is_err());
        
        // Test resource exhaustion
        let code = r#"
        // A function that allocates a large array
        export default function(input) {
            const largeArray = new Array(1000000000);
            return input.a + input.b;
        }
        "#;
        let result = validate_function_code(code);
        assert!(result.is_err());
    }
}
