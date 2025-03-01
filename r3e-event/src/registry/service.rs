// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::sync::Arc;
use tonic::{Request, Response, Status};

use crate::registry::{
    function_registry_server::{
        FunctionRegistry as FunctionRegistryService, FunctionRegistryServer,
    },
    DeleteFunctionRequest, DeleteFunctionResponse, FunctionRegistry, GetFunctionRequest,
    GetFunctionResponse, ListFunctionsRequest, ListFunctionsResponse, RegisterFunctionRequest,
    RegisterFunctionResponse, RegistryError, UpdateFunctionRequest, UpdateFunctionResponse,
};

/// gRPC service implementation for the function registry
pub struct FunctionRegistryImpl {
    registry: Arc<FunctionRegistry>,
}

impl FunctionRegistryImpl {
    /// Create a new function registry service
    pub fn new(registry: Arc<FunctionRegistry>) -> Self {
        Self { registry }
    }

    /// Create a new gRPC server for the function registry
    pub fn server(self) -> FunctionRegistryServer<Self> {
        FunctionRegistryServer::new(self)
    }
}

#[tonic::async_trait]
impl FunctionRegistryService for FunctionRegistryImpl {
    async fn register_function(
        &self,
        request: Request<RegisterFunctionRequest>,
    ) -> Result<Response<RegisterFunctionResponse>, Status> {
        let request = request.into_inner();

        match self.registry.register_function(request).await {
            Ok(response) => Ok(Response::new(response)),
            Err(err) => Err(registry_error_to_status(err)),
        }
    }

    async fn update_function(
        &self,
        request: Request<UpdateFunctionRequest>,
    ) -> Result<Response<UpdateFunctionResponse>, Status> {
        let request = request.into_inner();

        match self.registry.update_function(request).await {
            Ok(response) => Ok(Response::new(response)),
            Err(err) => Err(registry_error_to_status(err)),
        }
    }

    async fn get_function(
        &self,
        request: Request<GetFunctionRequest>,
    ) -> Result<Response<GetFunctionResponse>, Status> {
        let request = request.into_inner();

        match self.registry.get_function(request).await {
            Ok(response) => Ok(Response::new(response)),
            Err(err) => Err(registry_error_to_status(err)),
        }
    }

    async fn list_functions(
        &self,
        request: Request<ListFunctionsRequest>,
    ) -> Result<Response<ListFunctionsResponse>, Status> {
        let request = request.into_inner();

        match self.registry.list_functions(request).await {
            Ok(response) => Ok(Response::new(response)),
            Err(err) => Err(registry_error_to_status(err)),
        }
    }

    async fn delete_function(
        &self,
        request: Request<DeleteFunctionRequest>,
    ) -> Result<Response<DeleteFunctionResponse>, Status> {
        let request = request.into_inner();

        match self.registry.delete_function(request).await {
            Ok(response) => Ok(Response::new(response)),
            Err(err) => Err(registry_error_to_status(err)),
        }
    }
}

/// Convert registry errors to gRPC status
fn registry_error_to_status(err: RegistryError) -> Status {
    match err {
        RegistryError::NotFound(msg) => Status::not_found(msg),
        RegistryError::Validation(msg) => Status::invalid_argument(msg),
        RegistryError::Storage(msg) => Status::internal(format!("Storage error: {}", msg)),
        RegistryError::Internal(msg) => Status::internal(format!("Internal error: {}", msg)),
    }
}
