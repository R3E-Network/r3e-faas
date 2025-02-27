// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

#[cfg(test)]
mod tests {
    use deno_core::error::AnyError;
    use deno_core::{JsRuntime, RuntimeOptions};
    use r3e_deno::ext::neo::init_neo_ext;
    use r3e_deno::ext::init_ext;
    use std::rc::Rc;
    use std::sync::Arc;
    use std::time::Duration;
    
    // Helper function to create a JavaScript runtime with Neo extensions
    fn create_js_runtime() -> JsRuntime {
        let mut runtime = JsRuntime::new(RuntimeOptions::default());
        
        // Initialize the Neo extension
        init_neo_ext(&mut runtime);
        
        // Initialize other extensions
        init_ext(&mut runtime);
        
        runtime
    }
    
    // Helper function to execute JavaScript code in the runtime
    async fn execute_js(runtime: &mut JsRuntime, code: &str) -> Result<String, AnyError> {
        let result = runtime.execute_script("<test>", code.into())?;
        let result = runtime.resolve_value(result).await?;
        let scope = &mut runtime.handle_scope();
        let result = result.to_string(scope)?;
        Ok(result.to_rust_string_lossy(scope))
    }
    
    #[tokio::test]
    async fn test_neo_js_bindings() -> Result<(), AnyError> {
        // Create a JavaScript runtime with Neo extensions
        let mut runtime = create_js_runtime();
        
        // Test basic Neo object existence
        let result = execute_js(&mut runtime, "typeof r3e.neo").await?;
        assert_eq!(result, "object", "Neo object not found in r3e namespace");
        
        // Test Neo version
        let result = execute_js(&mut runtime, "r3e.neo.version").await?;
        assert!(!result.is_empty(), "Neo version is empty");
        
        // Test Neo client creation
        let result = execute_js(&mut runtime, "typeof r3e.neo.createClient").await?;
        assert_eq!(result, "function", "Neo createClient function not found");
        
        // Test Neo client with invalid URL (should return error object)
        let result = execute_js(&mut runtime, r#"
            try {
                const client = r3e.neo.createClient("invalid-url");
                "success"; // Should not reach here
            } catch (error) {
                "error: " + error.message;
            }
        "#).await?;
        assert!(result.contains("error:"), "Invalid URL should throw an error");
        
        // Test Neo utility functions
        let result = execute_js(&mut runtime, "typeof r3e.neo.utils").await?;
        assert_eq!(result, "object", "Neo utils object not found");
        
        // Test Neo address validation
        let result = execute_js(&mut runtime, r#"
            r3e.neo.utils.isValidAddress("NZNos2WqTbu5oCgyfss9kUJgBXJqhuYAaj")
        "#).await?;
        assert_eq!(result, "true", "Valid Neo address validation failed");
        
        // Test invalid Neo address validation
        let result = execute_js(&mut runtime, r#"
            r3e.neo.utils.isValidAddress("invalid-address")
        "#).await?;
        assert_eq!(result, "false", "Invalid Neo address validation failed");
        
        // Test Neo script hash conversion
        let result = execute_js(&mut runtime, r#"
            r3e.neo.utils.addressToScriptHash("NZNos2WqTbu5oCgyfss9kUJgBXJqhuYAaj")
        "#).await?;
        assert!(!result.is_empty(), "Address to script hash conversion failed");
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_neo_js_bindings_mock_blockchain() -> Result<(), AnyError> {
        // Create a JavaScript runtime with Neo extensions
        let mut runtime = create_js_runtime();
        
        // Test mock blockchain interaction
        let result = execute_js(&mut runtime, r#"
            // Create a mock Neo client
            const client = r3e.neo.createMockClient();
            
            // Test if the mock client was created successfully
            typeof client
        "#).await?;
        assert_eq!(result, "object", "Mock Neo client creation failed");
        
        // Test mock blockchain block height
        let result = execute_js(&mut runtime, r#"
            // Create a mock Neo client
            const client = r3e.neo.createMockClient();
            
            // Get the current block height
            client.getBlockHeight()
        "#).await?;
        assert!(result.parse::<u32>().is_ok(), "Mock block height should be a number");
        
        // Test mock blockchain transaction
        let result = execute_js(&mut runtime, r#"
            // Create a mock Neo client
            const client = r3e.neo.createMockClient();
            
            // Create a mock transaction
            const tx = client.createMockTransaction("mock_tx_hash");
            
            // Verify the transaction
            tx.hash === "mock_tx_hash"
        "#).await?;
        assert_eq!(result, "true", "Mock transaction creation failed");
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_neo_js_bindings_error_handling() -> Result<(), AnyError> {
        // Create a JavaScript runtime with Neo extensions
        let mut runtime = create_js_runtime();
        
        // Test error handling for invalid parameters
        let result = execute_js(&mut runtime, r#"
            try {
                // Try to create a client with null URL
                r3e.neo.createClient(null);
                "success"; // Should not reach here
            } catch (error) {
                "error: " + error.message;
            }
        "#).await?;
        assert!(result.contains("error:"), "Null URL should throw an error");
        
        // Test error handling for invalid method calls
        let result = execute_js(&mut runtime, r#"
            try {
                // Create a mock client
                const client = r3e.neo.createMockClient();
                
                // Try to call a non-existent method
                client.nonExistentMethod();
                "success"; // Should not reach here
            } catch (error) {
                "error: " + error.message;
            }
        "#).await?;
        assert!(result.contains("error:"), "Non-existent method should throw an error");
        
        // Test error handling for invalid script execution
        let result = execute_js(&mut runtime, r#"
            try {
                // Create a mock client
                const client = r3e.neo.createMockClient();
                
                // Try to execute an invalid script
                client.executeScript("invalid script");
                "success"; // Should not reach here
            } catch (error) {
                "error: " + error.message;
            }
        "#).await?;
        assert!(result.contains("error:"), "Invalid script execution should throw an error");
        
        Ok(())
    }
}
