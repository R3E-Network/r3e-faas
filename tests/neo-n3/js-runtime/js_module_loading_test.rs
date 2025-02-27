// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

#[cfg(test)]
mod tests {
    use deno_core::error::AnyError;
    use deno_core::{JsRuntime, RuntimeOptions, ModuleLoader, ModuleSource, ModuleType, ModuleSpecifier};
    use r3e_deno::ext::init_ext;
    use std::rc::Rc;
    use std::sync::Arc;
    use std::pin::Pin;
    use std::future::Future;
    use std::path::Path;
    
    // Simple module loader for testing
    struct TestModuleLoader;
    
    impl ModuleLoader for TestModuleLoader {
        fn resolve(&self, specifier: &str, referrer: &str, _is_main: bool) -> Result<ModuleSpecifier, AnyError> {
            if specifier.starts_with("test:") {
                return Ok(ModuleSpecifier::parse(specifier)?);
            }
            
            // Handle relative imports
            if specifier.starts_with("./") || specifier.starts_with("../") {
                let referrer_url = ModuleSpecifier::parse(referrer)?;
                let referrer_path = referrer_url.path();
                let parent_path = Path::new(referrer_path).parent().unwrap_or_else(|| Path::new(""));
                let resolved_path = parent_path.join(specifier);
                let resolved_str = format!("test:{}", resolved_path.display());
                return Ok(ModuleSpecifier::parse(&resolved_str)?);
            }
            
            // Default to test: scheme
            Ok(ModuleSpecifier::parse(&format!("test:{}", specifier))?)
        }
        
        fn load(&self, module_specifier: &ModuleSpecifier, _maybe_referrer: Option<ModuleSpecifier>, _is_dynamic: bool) -> Pin<Box<deno_core::ModuleSourceFuture>> {
            let module_specifier = module_specifier.clone();
            
            Box::pin(async move {
                let path = module_specifier.path();
                
                // Mock module content based on the path
                let code = match path {
                    "/main.js" => r#"
                        import { add } from "./math.js";
                        import { greet } from "./utils.js";
                        
                        export function calculate(a, b) {
                            return add(a, b);
                        }
                        
                        export function sayHello(name) {
                            return greet(name);
                        }
                    "#,
                    "/math.js" => r#"
                        export function add(a, b) {
                            return a + b;
                        }
                        
                        export function subtract(a, b) {
                            return a - b;
                        }
                        
                        export function multiply(a, b) {
                            return a * b;
                        }
                    "#,
                    "/utils.js" => r#"
                        export function greet(name) {
                            return `Hello, ${name}!`;
                        }
                        
                        export function formatDate(date) {
                            return date.toISOString();
                        }
                    "#,
                    "/circular1.js" => r#"
                        import { value2 } from "./circular2.js";
                        
                        export const value1 = "Value 1";
                        export const combinedValue = `${value1} and ${value2}`;
                    "#,
                    "/circular2.js" => r#"
                        import { value1 } from "./circular1.js";
                        
                        export const value2 = "Value 2";
                        export const combinedValue = `${value2} and ${value1}`;
                    "#,
                    "/error.js" => r#"
                        // This module has a syntax error
                        export const value = "Error"
                        export function broken( {
                            return "Broken";
                        }
                    "#,
                    _ => return Err(AnyError::msg(format!("Module not found: {}", path))),
                };
                
                Ok(ModuleSource {
                    code: code.into(),
                    module_type: ModuleType::JavaScript,
                    module_url_specified: module_specifier.to_string(),
                    module_url_found: module_specifier.to_string(),
                })
            })
        }
    }
    
    // Helper function to create a JavaScript runtime with module support
    fn create_js_runtime() -> JsRuntime {
        let mut runtime = JsRuntime::new(RuntimeOptions {
            module_loader: Some(Rc::new(TestModuleLoader)),
            ..Default::default()
        });
        
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
    async fn test_js_module_loading_basic() -> Result<(), AnyError> {
        // Create a JavaScript runtime with module support
        let mut runtime = create_js_runtime();
        
        // Load and evaluate a module
        let module_id = runtime.load_main_module(&ModuleSpecifier::parse("test:/main.js")?, None).await?;
        let result = runtime.mod_evaluate(module_id);
        runtime.run_event_loop(false).await?;
        result.await?;
        
        // Test accessing module exports
        let result = execute_js(&mut runtime, r#"
            import { calculate, sayHello } from "test:/main.js";
            
            // Test the calculate function
            const sum = calculate(2, 3);
            
            // Test the sayHello function
            const greeting = sayHello("World");
            
            // Return the results
            JSON.stringify({ sum, greeting });
        "#).await?;
        
        // Verify the result
        assert!(result.contains("\"sum\":5"), "Module function calculate() not working correctly");
        assert!(result.contains("\"greeting\":\"Hello, World!\""), "Module function sayHello() not working correctly");
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_js_module_loading_direct_imports() -> Result<(), AnyError> {
        // Create a JavaScript runtime with module support
        let mut runtime = create_js_runtime();
        
        // Test direct imports from modules
        let result = execute_js(&mut runtime, r#"
            import { add, multiply } from "test:/math.js";
            import { greet } from "test:/utils.js";
            
            // Test the imported functions
            const sum = add(5, 7);
            const product = multiply(3, 4);
            const greeting = greet("Tester");
            
            // Return the results
            JSON.stringify({ sum, product, greeting });
        "#).await?;
        
        // Verify the result
        assert!(result.contains("\"sum\":12"), "Direct import of add() not working correctly");
        assert!(result.contains("\"product\":12"), "Direct import of multiply() not working correctly");
        assert!(result.contains("\"greeting\":\"Hello, Tester!\""), "Direct import of greet() not working correctly");
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_js_module_loading_dynamic_import() -> Result<(), AnyError> {
        // Create a JavaScript runtime with module support
        let mut runtime = create_js_runtime();
        
        // Test dynamic imports
        let result = execute_js(&mut runtime, r#"
            async function testDynamicImport() {
                // Dynamically import the math module
                const mathModule = await import("test:/math.js");
                
                // Test the imported functions
                const sum = mathModule.add(10, 20);
                const difference = mathModule.subtract(30, 15);
                
                return { sum, difference };
            }
            
            // Call the async function and return the result
            testDynamicImport().then(result => JSON.stringify(result));
        "#).await?;
        
        // Verify the result
        assert!(result.contains("\"sum\":30"), "Dynamic import of add() not working correctly");
        assert!(result.contains("\"difference\":15"), "Dynamic import of subtract() not working correctly");
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_js_module_loading_circular_dependencies() -> Result<(), AnyError> {
        // Create a JavaScript runtime with module support
        let mut runtime = create_js_runtime();
        
        // Test modules with circular dependencies
        let result = execute_js(&mut runtime, r#"
            import { value1, combinedValue as combined1 } from "test:/circular1.js";
            import { value2, combinedValue as combined2 } from "test:/circular2.js";
            
            // Return the results
            JSON.stringify({ value1, value2, combined1, combined2 });
        "#).await?;
        
        // Verify the result
        assert!(result.contains("\"value1\":\"Value 1\""), "Circular dependency value1 not working correctly");
        assert!(result.contains("\"value2\":\"Value 2\""), "Circular dependency value2 not working correctly");
        assert!(result.contains("combined1"), "Circular dependency combinedValue not working correctly");
        assert!(result.contains("combined2"), "Circular dependency combinedValue not working correctly");
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_js_module_loading_error_handling() -> Result<(), AnyError> {
        // Create a JavaScript runtime with module support
        let mut runtime = create_js_runtime();
        
        // Test loading a module with syntax errors
        let result = runtime.load_main_module(&ModuleSpecifier::parse("test:/error.js")?, None).await;
        
        // Verify that loading the module with syntax errors fails
        assert!(result.is_err(), "Loading module with syntax errors should fail");
        
        // Test handling import errors
        let result = execute_js(&mut runtime, r#"
            try {
                import("test:/nonexistent.js");
                "success"; // Should not reach here
            } catch (error) {
                "error: " + error.message;
            }
        "#).await?;
        
        // Verify the result
        assert!(result.contains("error:"), "Import error not caught correctly");
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_js_module_loading_reexports() -> Result<(), AnyError> {
        // Create a JavaScript runtime with module support
        let mut runtime = create_js_runtime();
        
        // Test re-exporting module functions
        let result = execute_js(&mut runtime, r#"
            // Create a module that re-exports functions from other modules
            const moduleCode = `
                import { add, multiply } from "test:/math.js";
                import { greet } from "test:/utils.js";
                
                // Re-export the functions
                export { add, multiply, greet };
                
                // Export a new function that uses the imported functions
                export function calculateAndGreet(a, b, name) {
                    return {
                        sum: add(a, b),
                        greeting: greet(name)
                    };
                }
            `;
            
            // Evaluate the module code
            const blob = new Blob([moduleCode], { type: "application/javascript" });
            const url = URL.createObjectURL(blob);
            
            // Import the module and test the re-exported functions
            async function testReexports() {
                const module = await import(url);
                
                const sum = module.add(15, 25);
                const product = module.multiply(5, 6);
                const greeting = module.greet("Tester");
                const combined = module.calculateAndGreet(10, 20, "World");
                
                return { sum, product, greeting, combined };
            }
            
            // Call the async function and return the result
            testReexports().then(result => JSON.stringify(result));
        "#).await?;
        
        // Verify the result
        assert!(result.contains("\"sum\":40"), "Re-exported add() not working correctly");
        assert!(result.contains("\"product\":30"), "Re-exported multiply() not working correctly");
        assert!(result.contains("\"greeting\":\"Hello, Tester!\""), "Re-exported greet() not working correctly");
        assert!(result.contains("\"combined\""), "Combined function not working correctly");
        
        Ok(())
    }
}
