// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

//! Test utilities for Neo N3 FaaS platform tests

use std::sync::Arc;

/// Create a mock Neo RPC client for testing
pub fn create_mock_neo_rpc_client() -> Arc<dyn NeoRpcClient> {
    // TODO: Implement mock Neo RPC client
    unimplemented!()
}

/// Create a mock JavaScript runtime for testing
pub fn create_mock_js_runtime() -> Arc<dyn JsRuntime> {
    // TODO: Implement mock JavaScript runtime
    unimplemented!()
}

/// Create a mock Oracle provider for testing
pub fn create_mock_oracle_provider() -> Arc<dyn OracleProvider> {
    // TODO: Implement mock Oracle provider
    unimplemented!()
}

/// Create a mock TEE provider for testing
pub fn create_mock_tee_provider() -> Arc<dyn TeeProvider> {
    // TODO: Implement mock TEE provider
    unimplemented!()
}

/// Create a mock API client for testing
pub fn create_mock_api_client() -> Arc<dyn ApiClient> {
    // TODO: Implement mock API client
    unimplemented!()
}

/// Trait definitions for mocking
pub trait NeoRpcClient: Send + Sync {}
pub trait JsRuntime: Send + Sync {}
pub trait OracleProvider: Send + Sync {}
pub trait TeeProvider: Send + Sync {}
pub trait ApiClient: Send + Sync {}
