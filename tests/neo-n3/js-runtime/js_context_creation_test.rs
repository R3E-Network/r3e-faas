// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

#[cfg(test)]
mod tests {
    use deno_core::error::AnyError;
    use deno_core::{JsRuntime, RuntimeOptions, OpState};
    use r3e_deno::ext::init_ext;
    use std::rc::Rc;
    use std::sync::Arc;
    use std::time::Duration;
    use std::cell::RefCell;
    
    // Helper function to create a JavaScript runtime
    fn create_js_runtime() -> JsRuntime {
        let mut runtime = JsRuntime::new(RuntimeOptions::default());
        
        // Initialize extensions
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
    async fn test_js_context_creation_basic() -> Result<(), AnyError> {
        // Create a JavaScript runtime
        let mut runtime = create_js_runtime();
        
        // Test basic context creation
        let result = execute_js(&mut runtime, r#"
            // Create a new context object
            const context = {
                name: "TestContext",
                version: "1.0.0",
                created: new Date().toISOString()
            };
            
            // Verify the context properties
            JSON.stringify(context);
        "#).await?;
        
        // Verify the result
        assert!(result.contains("TestContext"), "Context name not found");
        assert!(result.contains("1.0.0"), "Context version not found");
        
        // Test accessing context properties
        let result = execute_js(&mut runtime, r#"
            // Create a new context object
            const context = {
                name: "TestContext",
                version: "1.0.0",
                created: new Date().toISOString()
            };
            
            // Access context properties
            context.name + " v" + context.version;
        "#).await?;
        
        // Verify the result
        assert_eq!(result, "TestContext v1.0.0", "Context properties not accessed correctly");
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_js_context_creation_with_methods() -> Result<(), AnyError> {
        // Create a JavaScript runtime
        let mut runtime = create_js_runtime();
        
        // Test context creation with methods
        let result = execute_js(&mut runtime, r#"
            // Create a new context object with methods
            const context = {
                name: "TestContext",
                version: "1.0.0",
                created: new Date().toISOString(),
                getInfo: function() {
                    return this.name + " v" + this.version;
                },
                getCreationDate: function() {
                    return this.created;
                }
            };
            
            // Call context methods
            context.getInfo();
        "#).await?;
        
        // Verify the result
        assert_eq!(result, "TestContext v1.0.0", "Context method not called correctly");
        
        // Test calling another context method
        let result = execute_js(&mut runtime, r#"
            // Create a new context object with methods
            const context = {
                name: "TestContext",
                version: "1.0.0",
                created: new Date().toISOString(),
                getInfo: function() {
                    return this.name + " v" + this.version;
                },
                getCreationDate: function() {
                    return this.created;
                }
            };
            
            // Call context methods
            typeof context.getCreationDate();
        "#).await?;
        
        // Verify the result
        assert_eq!(result, "string", "Context method return type incorrect");
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_js_context_creation_with_nested_objects() -> Result<(), AnyError> {
        // Create a JavaScript runtime
        let mut runtime = create_js_runtime();
        
        // Test context creation with nested objects
        let result = execute_js(&mut runtime, r#"
            // Create a new context object with nested objects
            const context = {
                name: "TestContext",
                version: "1.0.0",
                config: {
                    debug: true,
                    timeout: 5000,
                    retries: 3
                },
                runtime: {
                    name: "Neo",
                    version: "3.0.0",
                    features: ["smart-contracts", "oracle", "tee"]
                }
            };
            
            // Access nested object properties
            JSON.stringify(context.config);
        "#).await?;
        
        // Verify the result
        assert!(result.contains("debug"), "Nested object property not found");
        assert!(result.contains("5000"), "Nested object property not found");
        assert!(result.contains("3"), "Nested object property not found");
        
        // Test accessing deeply nested properties
        let result = execute_js(&mut runtime, r#"
            // Create a new context object with nested objects
            const context = {
                name: "TestContext",
                version: "1.0.0",
                config: {
                    debug: true,
                    timeout: 5000,
                    retries: 3
                },
                runtime: {
                    name: "Neo",
                    version: "3.0.0",
                    features: ["smart-contracts", "oracle", "tee"]
                }
            };
            
            // Access deeply nested properties
            context.runtime.features.includes("oracle");
        "#).await?;
        
        // Verify the result
        assert_eq!(result, "true", "Deeply nested property not accessed correctly");
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_js_context_creation_with_prototype() -> Result<(), AnyError> {
        // Create a JavaScript runtime
        let mut runtime = create_js_runtime();
        
        // Test context creation with prototype
        let result = execute_js(&mut runtime, r#"
            // Create a prototype object
            const contextProto = {
                getInfo: function() {
                    return this.name + " v" + this.version;
                },
                getType: function() {
                    return "Context";
                }
            };
            
            // Create a new context object with the prototype
            const context = Object.create(contextProto);
            context.name = "TestContext";
            context.version = "1.0.0";
            
            // Call a method from the prototype
            context.getInfo();
        "#).await?;
        
        // Verify the result
        assert_eq!(result, "TestContext v1.0.0", "Prototype method not called correctly");
        
        // Test calling another prototype method
        let result = execute_js(&mut runtime, r#"
            // Create a prototype object
            const contextProto = {
                getInfo: function() {
                    return this.name + " v" + this.version;
                },
                getType: function() {
                    return "Context";
                }
            };
            
            // Create a new context object with the prototype
            const context = Object.create(contextProto);
            context.name = "TestContext";
            context.version = "1.0.0";
            
            // Call another method from the prototype
            context.getType();
        "#).await?;
        
        // Verify the result
        assert_eq!(result, "Context", "Prototype method not called correctly");
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_js_context_creation_with_global_context() -> Result<(), AnyError> {
        // Create a JavaScript runtime
        let mut runtime = create_js_runtime();
        
        // Test context creation with global context
        let result = execute_js(&mut runtime, r#"
            // Set up a global context
            globalThis.r3eContext = {
                name: "GlobalContext",
                version: "1.0.0",
                getInfo: function() {
                    return this.name + " v" + this.version;
                }
            };
            
            // Access the global context
            globalThis.r3eContext.name;
        "#).await?;
        
        // Verify the result
        assert_eq!(result, "GlobalContext", "Global context property not accessed correctly");
        
        // Test calling a method on the global context
        let result = execute_js(&mut runtime, r#"
            // Access the global context method
            globalThis.r3eContext.getInfo();
        "#).await?;
        
        // Verify the result
        assert_eq!(result, "GlobalContext v1.0.0", "Global context method not called correctly");
        
        // Test modifying the global context
        let result = execute_js(&mut runtime, r#"
            // Modify the global context
            globalThis.r3eContext.version = "2.0.0";
            
            // Access the modified property
            globalThis.r3eContext.version;
        "#).await?;
        
        // Verify the result
        assert_eq!(result, "2.0.0", "Global context property not modified correctly");
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_js_context_creation_with_function_context() -> Result<(), AnyError> {
        // Create a JavaScript runtime
        let mut runtime = create_js_runtime();
        
        // Test context creation with function context
        let result = execute_js(&mut runtime, r#"
            // Create a function that uses a context
            function createFunction(context) {
                return function() {
                    return context.name + " v" + context.version;
                };
            }
            
            // Create a context
            const context = {
                name: "FunctionContext",
                version: "1.0.0"
            };
            
            // Create a function with the context
            const func = createFunction(context);
            
            // Call the function
            func();
        "#).await?;
        
        // Verify the result
        assert_eq!(result, "FunctionContext v1.0.0", "Function context not used correctly");
        
        // Test modifying the context after function creation
        let result = execute_js(&mut runtime, r#"
            // Create a function that uses a context
            function createFunction(context) {
                return function() {
                    return context.name + " v" + context.version;
                };
            }
            
            // Create a context
            const context = {
                name: "FunctionContext",
                version: "1.0.0"
            };
            
            // Create a function with the context
            const func = createFunction(context);
            
            // Modify the context
            context.version = "2.0.0";
            
            // Call the function
            func();
        "#).await?;
        
        // Verify the result
        assert_eq!(result, "FunctionContext v2.0.0", "Modified function context not used correctly");
        
        Ok(())
    }
}
