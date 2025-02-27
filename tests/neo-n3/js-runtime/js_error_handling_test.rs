// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

#[cfg(test)]
mod tests {
    use deno_core::error::AnyError;
    use deno_core::{JsRuntime, RuntimeOptions};
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
    async fn test_js_syntax_error_handling() -> Result<(), AnyError> {
        // Create a JavaScript runtime
        let mut runtime = create_js_runtime();
        
        // Test syntax error handling
        let result = runtime.execute_script("<test>", r#"
            // This code has a syntax error (missing closing parenthesis
            console.log("Hello, World!"
        "#.into());
        
        // Verify that the execution failed with a syntax error
        assert!(result.is_err(), "Syntax error not detected");
        
        // Get the error message
        let error = result.unwrap_err();
        let error_message = error.to_string();
        
        // Verify that the error message contains "SyntaxError"
        assert!(error_message.contains("SyntaxError"), "Error message does not contain 'SyntaxError'");
        
        // Test recovery after syntax error
        let result = execute_js(&mut runtime, r#"
            // This code is valid
            "Recovered from syntax error"
        "#).await?;
        
        // Verify that the runtime recovered from the syntax error
        assert_eq!(result, "Recovered from syntax error", "Failed to recover from syntax error");
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_js_reference_error_handling() -> Result<(), AnyError> {
        // Create a JavaScript runtime
        let mut runtime = create_js_runtime();
        
        // Test reference error handling with try-catch
        let result = execute_js(&mut runtime, r#"
            try {
                // This code has a reference error (undefined variable)
                nonExistentVariable;
            } catch (error) {
                if (error instanceof ReferenceError) {
                    "Caught ReferenceError: " + error.message;
                } else {
                    "Caught unexpected error: " + error.message;
                }
            }
        "#).await?;
        
        // Verify that the reference error was caught
        assert!(result.contains("Caught ReferenceError"), "Reference error not caught");
        
        // Test reference error handling without try-catch
        let result = runtime.execute_script("<test>", r#"
            // This code has a reference error (undefined variable)
            nonExistentVariable;
        "#.into());
        
        // Verify that the execution failed with a reference error
        assert!(result.is_err(), "Reference error not detected");
        
        // Get the error message
        let error = result.unwrap_err();
        let error_message = error.to_string();
        
        // Verify that the error message contains "ReferenceError"
        assert!(error_message.contains("ReferenceError"), "Error message does not contain 'ReferenceError'");
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_js_type_error_handling() -> Result<(), AnyError> {
        // Create a JavaScript runtime
        let mut runtime = create_js_runtime();
        
        // Test type error handling with try-catch
        let result = execute_js(&mut runtime, r#"
            try {
                // This code has a type error (calling a non-function)
                const obj = {};
                obj();
            } catch (error) {
                if (error instanceof TypeError) {
                    "Caught TypeError: " + error.message;
                } else {
                    "Caught unexpected error: " + error.message;
                }
            }
        "#).await?;
        
        // Verify that the type error was caught
        assert!(result.contains("Caught TypeError"), "Type error not caught");
        
        // Test type error handling without try-catch
        let result = runtime.execute_script("<test>", r#"
            // This code has a type error (calling a non-function)
            const obj = {};
            obj();
        "#.into());
        
        // Verify that the execution failed with a type error
        assert!(result.is_err(), "Type error not detected");
        
        // Get the error message
        let error = result.unwrap_err();
        let error_message = error.to_string();
        
        // Verify that the error message contains "TypeError"
        assert!(error_message.contains("TypeError"), "Error message does not contain 'TypeError'");
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_js_error_propagation() -> Result<(), AnyError> {
        // Create a JavaScript runtime
        let mut runtime = create_js_runtime();
        
        // Test error propagation through function calls
        let result = execute_js(&mut runtime, r#"
            function innerFunction() {
                throw new Error("Inner error");
            }
            
            function middleFunction() {
                innerFunction();
            }
            
            function outerFunction() {
                try {
                    middleFunction();
                } catch (error) {
                    return "Caught error in outerFunction: " + error.message;
                }
            }
            
            outerFunction();
        "#).await?;
        
        // Verify that the error was propagated and caught
        assert!(result.contains("Caught error in outerFunction: Inner error"), "Error not propagated correctly");
        
        // Test error propagation with stack trace
        let result = execute_js(&mut runtime, r#"
            function innerFunction() {
                throw new Error("Inner error with stack trace");
            }
            
            function middleFunction() {
                innerFunction();
            }
            
            function outerFunction() {
                try {
                    middleFunction();
                } catch (error) {
                    // Check if the stack trace contains the function names
                    const stackHasInner = error.stack.includes("innerFunction");
                    const stackHasMiddle = error.stack.includes("middleFunction");
                    const stackHasOuter = error.stack.includes("outerFunction");
                    
                    return `Stack trace contains: innerFunction=${stackHasInner}, middleFunction=${stackHasMiddle}, outerFunction=${stackHasOuter}`;
                }
            }
            
            outerFunction();
        "#).await?;
        
        // Verify that the stack trace contains the function names
        assert!(result.contains("innerFunction=true"), "Stack trace does not contain innerFunction");
        assert!(result.contains("middleFunction=true"), "Stack trace does not contain middleFunction");
        assert!(result.contains("outerFunction=true"), "Stack trace does not contain outerFunction");
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_js_custom_error_handling() -> Result<(), AnyError> {
        // Create a JavaScript runtime
        let mut runtime = create_js_runtime();
        
        // Test custom error handling
        let result = execute_js(&mut runtime, r#"
            // Define a custom error class
            class CustomError extends Error {
                constructor(message) {
                    super(message);
                    this.name = "CustomError";
                    this.code = "CUSTOM_ERROR";
                }
            }
            
            try {
                // Throw a custom error
                throw new CustomError("Custom error message");
            } catch (error) {
                if (error.name === "CustomError") {
                    "Caught CustomError: " + error.message + ", Code: " + error.code;
                } else {
                    "Caught unexpected error: " + error.message;
                }
            }
        "#).await?;
        
        // Verify that the custom error was caught
        assert!(result.contains("Caught CustomError"), "Custom error not caught");
        assert!(result.contains("Custom error message"), "Custom error message not included");
        assert!(result.contains("Code: CUSTOM_ERROR"), "Custom error code not included");
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_js_async_error_handling() -> Result<(), AnyError> {
        // Create a JavaScript runtime
        let mut runtime = create_js_runtime();
        
        // Test async error handling with try-catch
        let result = execute_js(&mut runtime, r#"
            async function asyncFunction() {
                throw new Error("Async error");
            }
            
            async function main() {
                try {
                    await asyncFunction();
                } catch (error) {
                    return "Caught async error: " + error.message;
                }
            }
            
            main();
        "#).await?;
        
        // Verify that the async error was caught
        assert!(result.contains("Caught async error"), "Async error not caught");
        
        // Test async error handling with Promise.catch
        let result = execute_js(&mut runtime, r#"
            async function asyncFunction() {
                throw new Error("Async error for Promise.catch");
            }
            
            async function main() {
                return asyncFunction()
                    .catch(error => "Caught with Promise.catch: " + error.message);
            }
            
            main();
        "#).await?;
        
        // Verify that the async error was caught with Promise.catch
        assert!(result.contains("Caught with Promise.catch"), "Async error not caught with Promise.catch");
        
        Ok(())
    }
}
