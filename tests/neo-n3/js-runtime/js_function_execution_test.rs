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
    async fn test_js_function_execution_basic() -> Result<(), AnyError> {
        // Create a JavaScript runtime
        let mut runtime = create_js_runtime();
        
        // Test basic function execution
        let result = execute_js(&mut runtime, r#"
            // Define a simple function
            function add(a, b) {
                return a + b;
            }
            
            // Execute the function
            add(2, 3)
        "#).await?;
        
        // Verify the result
        assert_eq!(result, "5", "Basic function execution failed");
        
        // Test function with string parameters
        let result = execute_js(&mut runtime, r#"
            // Define a function that concatenates strings
            function concat(a, b) {
                return a + b;
            }
            
            // Execute the function
            concat("Hello, ", "World!")
        "#).await?;
        
        // Verify the result
        assert_eq!(result, "Hello, World!", "String function execution failed");
        
        // Test function with object parameters
        let result = execute_js(&mut runtime, r#"
            // Define a function that works with objects
            function getFullName(person) {
                return person.firstName + " " + person.lastName;
            }
            
            // Execute the function
            getFullName({ firstName: "John", lastName: "Doe" })
        "#).await?;
        
        // Verify the result
        assert_eq!(result, "John Doe", "Object function execution failed");
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_js_function_execution_async() -> Result<(), AnyError> {
        // Create a JavaScript runtime
        let mut runtime = create_js_runtime();
        
        // Test async function execution
        let result = execute_js(&mut runtime, r#"
            // Define an async function
            async function fetchData() {
                // Simulate an async operation
                return new Promise((resolve) => {
                    setTimeout(() => {
                        resolve("Data fetched successfully");
                    }, 100);
                });
            }
            
            // Execute the async function
            async function main() {
                const result = await fetchData();
                return result;
            }
            
            // Call the main function
            main()
        "#).await?;
        
        // Verify the result
        assert_eq!(result, "Data fetched successfully", "Async function execution failed");
        
        // Test async function with error handling
        let result = execute_js(&mut runtime, r#"
            // Define an async function that might throw an error
            async function fetchDataWithError(shouldFail) {
                // Simulate an async operation
                return new Promise((resolve, reject) => {
                    setTimeout(() => {
                        if (shouldFail) {
                            reject(new Error("Failed to fetch data"));
                        } else {
                            resolve("Data fetched successfully");
                        }
                    }, 100);
                });
            }
            
            // Execute the async function with error handling
            async function main() {
                try {
                    const result = await fetchDataWithError(false);
                    return result;
                } catch (error) {
                    return "Error: " + error.message;
                }
            }
            
            // Call the main function
            main()
        "#).await?;
        
        // Verify the result
        assert_eq!(result, "Data fetched successfully", "Async function with error handling failed");
        
        // Test async function with error
        let result = execute_js(&mut runtime, r#"
            // Define an async function that might throw an error
            async function fetchDataWithError(shouldFail) {
                // Simulate an async operation
                return new Promise((resolve, reject) => {
                    setTimeout(() => {
                        if (shouldFail) {
                            reject(new Error("Failed to fetch data"));
                        } else {
                            resolve("Data fetched successfully");
                        }
                    }, 100);
                });
            }
            
            // Execute the async function with error handling
            async function main() {
                try {
                    const result = await fetchDataWithError(true);
                    return result;
                } catch (error) {
                    return "Error: " + error.message;
                }
            }
            
            // Call the main function
            main()
        "#).await?;
        
        // Verify the result
        assert_eq!(result, "Error: Failed to fetch data", "Async function with error failed");
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_js_function_execution_with_context() -> Result<(), AnyError> {
        // Create a JavaScript runtime
        let mut runtime = create_js_runtime();
        
        // Set up a context object in the runtime
        execute_js(&mut runtime, r#"
            // Create a global context object
            globalThis.context = {
                userId: "user123",
                apiKey: "api456",
                environment: "test"
            };
        "#).await?;
        
        // Test function execution with context
        let result = execute_js(&mut runtime, r#"
            // Define a function that uses the context
            function getUserInfo() {
                return {
                    userId: globalThis.context.userId,
                    environment: globalThis.context.environment
                };
            }
            
            // Execute the function
            const userInfo = getUserInfo();
            JSON.stringify(userInfo)
        "#).await?;
        
        // Verify the result
        assert!(result.contains("user123"), "Context function execution failed");
        assert!(result.contains("test"), "Context function execution failed");
        
        // Test function that modifies the context
        let result = execute_js(&mut runtime, r#"
            // Define a function that modifies the context
            function updateUserEnvironment(newEnvironment) {
                globalThis.context.environment = newEnvironment;
                return globalThis.context.environment;
            }
            
            // Execute the function
            updateUserEnvironment("production")
        "#).await?;
        
        // Verify the result
        assert_eq!(result, "production", "Context modification failed");
        
        // Verify that the context was actually modified
        let result = execute_js(&mut runtime, r#"
            globalThis.context.environment
        "#).await?;
        
        // Verify the result
        assert_eq!(result, "production", "Context persistence failed");
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_js_function_execution_error_handling() -> Result<(), AnyError> {
        // Create a JavaScript runtime
        let mut runtime = create_js_runtime();
        
        // Test function with error handling
        let result = execute_js(&mut runtime, r#"
            // Define a function that might throw an error
            function divide(a, b) {
                if (b === 0) {
                    throw new Error("Division by zero");
                }
                return a / b;
            }
            
            // Execute the function with error handling
            try {
                divide(10, 2);
            } catch (error) {
                "Error: " + error.message;
            }
        "#).await?;
        
        // Verify the result
        assert_eq!(result, "5", "Error handling function execution failed");
        
        // Test function with error
        let result = execute_js(&mut runtime, r#"
            // Define a function that might throw an error
            function divide(a, b) {
                if (b === 0) {
                    throw new Error("Division by zero");
                }
                return a / b;
            }
            
            // Execute the function with error handling
            try {
                divide(10, 0);
            } catch (error) {
                "Error: " + error.message;
            }
        "#).await?;
        
        // Verify the result
        assert_eq!(result, "Error: Division by zero", "Error function execution failed");
        
        // Test function with nested error handling
        let result = execute_js(&mut runtime, r#"
            // Define a function with nested error handling
            function processData(data) {
                try {
                    // Try to parse the data
                    const parsed = JSON.parse(data);
                    return parsed.value;
                } catch (error) {
                    // Handle parsing error
                    return "Invalid data: " + error.message;
                }
            }
            
            // Execute the function with valid data
            processData('{"value": 42}')
        "#).await?;
        
        // Verify the result
        assert_eq!(result, "42", "Nested error handling with valid data failed");
        
        // Test function with nested error handling and invalid data
        let result = execute_js(&mut runtime, r#"
            // Define a function with nested error handling
            function processData(data) {
                try {
                    // Try to parse the data
                    const parsed = JSON.parse(data);
                    return parsed.value;
                } catch (error) {
                    // Handle parsing error
                    return "Invalid data: " + error.message;
                }
            }
            
            // Execute the function with invalid data
            processData('{"value":')
        "#).await?;
        
        // Verify the result
        assert!(result.contains("Invalid data:"), "Nested error handling with invalid data failed");
        
        Ok(())
    }
}
