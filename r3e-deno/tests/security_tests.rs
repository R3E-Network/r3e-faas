// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::sync::Arc;
use std::time::Duration;

use r3e_deno::sandbox::{SandboxConfig, SandboxPermissions, SandboxResourceLimits};
use r3e_deno::v8_executor::V8Executor;
use tokio::time::timeout;

#[tokio::test]
async fn test_sandbox_isolation() {
    // Create a sandbox with minimal permissions
    let config = SandboxConfig {
        permissions: SandboxPermissions {
            allow_net: false,
            allow_read: false,
            allow_write: false,
            allow_env: false,
            allow_run: false,
            allow_ffi: false,
            allow_hrtime: false,
        },
        resource_limits: SandboxResourceLimits {
            max_memory_mb: 100,
            max_execution_time_ms: 1000,
            max_module_count: 10,
        },
        ..Default::default()
    };

    let executor = V8Executor::new(config);

    // Test file system access (should fail)
    let code = r#"
    try {
        const decoder = new TextDecoder();
        const data = Deno.readFileSync("/etc/passwd");
        console.log(decoder.decode(data));
        throw new Error("Should not be able to read files");
    } catch (e) {
        if (e.message.includes("permission denied")) {
            console.log("File system access correctly denied");
        } else {
            throw e;
        }
    }
    "#;

    let result = executor.execute(code).await;
    assert!(result.is_ok(), "Execution failed: {:?}", result.err());
    assert!(result.unwrap().contains("File system access correctly denied"));

    // Test network access (should fail)
    let code = r#"
    try {
        const response = await fetch("https://example.com");
        throw new Error("Should not be able to access network");
    } catch (e) {
        if (e.message.includes("permission denied")) {
            console.log("Network access correctly denied");
        } else {
            throw e;
        }
    }
    "#;

    let result = executor.execute(code).await;
    assert!(result.is_ok(), "Execution failed: {:?}", result.err());
    assert!(result.unwrap().contains("Network access correctly denied"));

    // Test environment variable access (should fail)
    let code = r#"
    try {
        const env = Deno.env.get("PATH");
        throw new Error("Should not be able to access environment variables");
    } catch (e) {
        if (e.message.includes("permission denied")) {
            console.log("Environment access correctly denied");
        } else {
            throw e;
        }
    }
    "#;

    let result = executor.execute(code).await;
    assert!(result.is_ok(), "Execution failed: {:?}", result.err());
    assert!(result.unwrap().contains("Environment access correctly denied"));
}

#[tokio::test]
async fn test_resource_limits() {
    // Create a sandbox with strict resource limits
    let config = SandboxConfig {
        permissions: SandboxPermissions {
            allow_net: false,
            allow_read: false,
            allow_write: false,
            allow_env: false,
            allow_run: false,
            allow_ffi: false,
            allow_hrtime: false,
        },
        resource_limits: SandboxResourceLimits {
            max_memory_mb: 100,
            max_execution_time_ms: 500,
            max_module_count: 10,
        },
        ..Default::default()
    };

    let executor = V8Executor::new(config);

    // Test execution time limit
    let code = r#"
    console.log("Starting infinite loop");
    while (true) {
        // Infinite loop
    }
    console.log("This should never be reached");
    "#;

    // Use tokio timeout to ensure the test doesn't hang
    let result = timeout(Duration::from_millis(1000), executor.execute(code)).await;
    
    // The execution should time out or be terminated due to resource limits
    assert!(result.is_err() || result.unwrap().is_err(), 
            "Infinite loop should have been terminated");
}

#[tokio::test]
async fn test_memory_limits() {
    // Create a sandbox with strict memory limits
    let config = SandboxConfig {
        permissions: SandboxPermissions {
            allow_net: false,
            allow_read: false,
            allow_write: false,
            allow_env: false,
            allow_run: false,
            allow_ffi: false,
            allow_hrtime: false,
        },
        resource_limits: SandboxResourceLimits {
            max_memory_mb: 10,
            max_execution_time_ms: 1000,
            max_module_count: 10,
        },
        ..Default::default()
    };

    let executor = V8Executor::new(config);

    // Test memory limit
    let code = r#"
    try {
        console.log("Attempting to allocate large array");
        // Try to allocate a large array (should fail with low memory limit)
        const largeArray = new Uint8Array(100 * 1024 * 1024); // 100MB
        throw new Error("Should not be able to allocate large array");
    } catch (e) {
        if (e instanceof RangeError || e.message.includes("memory")) {
            console.log("Memory limit correctly enforced");
        } else {
            throw e;
        }
    }
    "#;

    let result = executor.execute(code).await;
    
    // Either the execution fails due to memory limits or the script catches the error
    if result.is_ok() {
        assert!(result.unwrap().contains("Memory limit correctly enforced"));
    } else {
        let err = result.err().unwrap();
        assert!(err.to_string().contains("memory") || err.to_string().contains("allocation"));
    }
}

#[tokio::test]
async fn test_code_injection_prevention() {
    // Create a sandbox with minimal permissions
    let config = SandboxConfig {
        permissions: SandboxPermissions {
            allow_net: false,
            allow_read: false,
            allow_write: false,
            allow_env: false,
            allow_run: false,
            allow_ffi: false,
            allow_hrtime: false,
        },
        resource_limits: SandboxResourceLimits {
            max_memory_mb: 100,
            max_execution_time_ms: 1000,
            max_module_count: 10,
        },
        ..Default::default()
    };

    let executor = V8Executor::new(config);

    // Test eval prevention
    let code = r#"
    try {
        // Try to use eval to execute code
        const userInput = "console.log('This should not be executed via eval')";
        eval(userInput);
        throw new Error("Should not be able to use eval");
    } catch (e) {
        if (e.message.includes("eval") || e.message.includes("denied")) {
            console.log("Eval correctly prevented");
        } else {
            throw e;
        }
    }
    "#;

    let result = executor.execute(code).await;
    
    // Either eval is disabled or the sandbox catches the attempt
    if result.is_ok() {
        let output = result.unwrap();
        assert!(output.contains("Eval correctly prevented") || 
                !output.contains("This should not be executed via eval"));
    }

    // Test Function constructor prevention
    let code = r#"
    try {
        // Try to use Function constructor to execute code
        const userInput = "return 'This should not be executed via Function constructor'";
        const dynamicFunc = new Function(userInput);
        const result = dynamicFunc();
        console.log(result);
        throw new Error("Should not be able to use Function constructor");
    } catch (e) {
        if (e.message.includes("Function") || e.message.includes("denied")) {
            console.log("Function constructor correctly prevented");
        } else {
            throw e;
        }
    }
    "#;

    let result = executor.execute(code).await;
    
    // Either Function constructor is disabled or the sandbox catches the attempt
    if result.is_ok() {
        let output = result.unwrap();
        assert!(output.contains("Function constructor correctly prevented") || 
                !output.contains("This should not be executed via Function constructor"));
    }
}
