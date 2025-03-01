// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use r3e_worker::{ContainerConfig, ContainerError, ContainerManager, NetworkMode};
use std::time::Duration;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Requires Docker to be installed and running
    fn test_container_creation() {
        let config = ContainerConfig::default();
        let manager = ContainerManager::new(config);

        // Simple function that returns a value
        let code = r#"
            console.log(JSON.stringify({ result: "Hello from container" }));
        "#;

        let result = manager.run_function("test-function", code);
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(output.contains("Hello from container"));
    }

    #[test]
    #[ignore] // Requires Docker to be installed and running
    fn test_memory_limit() {
        // Create a config with very low memory limit
        let config = ContainerConfig {
            memory_limit: 10 * 1024 * 1024, // 10MB
            ..Default::default()
        };

        let manager = ContainerManager::new(config);

        // Function that tries to allocate a large array
        let code = r#"
            try {
                const arr = new Array(1024 * 1024 * 100).fill(0); // Try to allocate ~800MB
                console.log(JSON.stringify({ result: "Memory allocation succeeded" }));
            } catch (e) {
                console.log(JSON.stringify({ error: e.message }));
            }
        "#;

        let result = manager.run_function("memory-test", code);

        // Should fail due to memory limit
        assert!(result.is_err() || result.unwrap().contains("error"));
    }

    #[test]
    #[ignore] // Requires Docker to be installed and running
    fn test_network_isolation() {
        // Create a config with no network access
        let config = ContainerConfig {
            network_mode: NetworkMode::None,
            ..Default::default()
        };

        let manager = ContainerManager::new(config);

        // Function that tries to access the network
        let code = r#"
            try {
                const http = require('http');
                const req = http.get('http://example.com', (res) => {
                    console.log(JSON.stringify({ result: "Network access succeeded" }));
                });
                req.on('error', (e) => {
                    console.log(JSON.stringify({ error: e.message }));
                });
                
                // Wait a bit for the request to complete or fail
                setTimeout(() => {
                    console.log(JSON.stringify({ result: "Timeout reached" }));
                }, 2000);
            } catch (e) {
                console.log(JSON.stringify({ error: e.message }));
            }
        "#;

        let result = manager.run_function("network-test", code);

        // Should contain an error about network access
        assert!(result.is_ok() && result.unwrap().contains("error"));
    }

    #[test]
    #[ignore] // Requires Docker to be installed and running
    fn test_filesystem_restrictions() {
        // Create a config with no filesystem access
        let config = ContainerConfig {
            allow_fs: false,
            ..Default::default()
        };

        let manager = ContainerManager::new(config);

        // Function that tries to access the filesystem
        let code = r#"
            try {
                const fs = require('fs');
                fs.readFileSync('/etc/passwd');
                console.log(JSON.stringify({ result: "Filesystem access succeeded" }));
            } catch (e) {
                console.log(JSON.stringify({ error: e.message }));
            }
        "#;

        let result = manager.run_function("fs-test", code);

        // Should contain an error about filesystem access
        assert!(result.is_ok() && result.unwrap().contains("error"));
    }

    #[test]
    #[ignore] // Requires Docker to be installed and running
    fn test_execution_timeout() {
        // Create a config with a short timeout
        let config = ContainerConfig {
            max_execution_time: Duration::from_secs(2),
            ..Default::default()
        };

        let manager = ContainerManager::new(config);

        // Function that runs an infinite loop
        let code = r#"
            try {
                console.log(JSON.stringify({ result: "Starting infinite loop" }));
                while(true) {
                    // Infinite loop
                }
            } catch (e) {
                console.log(JSON.stringify({ error: e.message }));
            }
        "#;

        let result = manager.run_function("timeout-test", code);

        // Should fail due to timeout
        assert!(result.is_err());
    }

    #[test]
    #[ignore] // Requires Docker to be installed and running
    fn test_security_options() {
        // Create a config with default security options
        let config = ContainerConfig::default();
        let manager = ContainerManager::new(config);

        // Function that tries to access system information
        let code = r#"
            try {
                const { execSync } = require('child_process');
                const result = execSync('id').toString();
                console.log(JSON.stringify({ result }));
            } catch (e) {
                console.log(JSON.stringify({ error: e.message }));
            }
        "#;

        let result = manager.run_function("security-test", code);

        // Should contain an error about executing commands
        assert!(result.is_ok() && result.unwrap().contains("error"));
    }
}
