/// Function metadata schema for user-provided JavaScript functions
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FunctionMetadata {
    /// Unique identifier for the function
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// User-provided name for the function
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// User-provided description for the function
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Version information
    #[prost(uint64, tag = "4")]
    pub version: u64,
    /// Creation timestamp
    #[prost(uint64, tag = "5")]
    pub created_at: u64,
    /// Last updated timestamp
    #[prost(uint64, tag = "6")]
    pub updated_at: u64,
    /// Trigger configuration
    #[prost(message, optional, tag = "7")]
    pub trigger: ::core::option::Option<TriggerConfig>,
    /// Permission configuration
    #[prost(message, optional, tag = "8")]
    pub permissions: ::core::option::Option<PermissionConfig>,
    /// Resource limits
    #[prost(message, optional, tag = "9")]
    pub resources: ::core::option::Option<ResourceLimits>,
    /// Function code
    #[prost(string, tag = "10")]
    pub code: ::prost::alloc::string::String,
}
/// Trigger configuration for function execution
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TriggerConfig {
    /// Type of trigger
    #[prost(enumeration = "TriggerType", tag = "1")]
    pub r#type: i32,
    /// Trigger-specific configuration
    #[prost(oneof = "trigger_config::Config", tags = "2, 3, 4, 5")]
    pub config: ::core::option::Option<trigger_config::Config>,
}
/// Nested message and enum types in `TriggerConfig`.
pub mod trigger_config {
    /// Trigger-specific configuration
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Config {
        #[prost(message, tag = "2")]
        Blockchain(super::BlockchainTrigger),
        #[prost(message, tag = "3")]
        Schedule(super::ScheduleTrigger),
        #[prost(message, tag = "4")]
        Http(super::HttpTrigger),
        #[prost(message, tag = "5")]
        Oracle(super::OracleTrigger),
    }
}
/// Blockchain event trigger configuration
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockchainTrigger {
    /// Blockchain source
    #[prost(string, tag = "1")]
    pub source: ::prost::alloc::string::String,
    /// Event type
    #[prost(string, tag = "2")]
    pub event_type: ::prost::alloc::string::String,
    /// Optional filter for specific events
    #[prost(string, tag = "3")]
    pub filter: ::prost::alloc::string::String,
}
/// Schedule-based trigger configuration
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScheduleTrigger {
    /// Cron expression for scheduling
    #[prost(string, tag = "1")]
    pub cron: ::prost::alloc::string::String,
    /// Optional timezone
    #[prost(string, tag = "2")]
    pub timezone: ::prost::alloc::string::String,
}
/// HTTP endpoint trigger configuration
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpTrigger {
    /// HTTP path
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    /// HTTP methods
    #[prost(string, repeated, tag = "2")]
    pub methods: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Authentication required
    #[prost(bool, tag = "3")]
    pub auth_required: bool,
}
/// Oracle trigger configuration
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OracleTrigger {
    /// Oracle type
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    /// Oracle-specific configuration
    #[prost(string, tag = "2")]
    pub config: ::prost::alloc::string::String,
}
/// Permission configuration for function execution
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PermissionConfig {
    /// Network access permissions
    #[prost(message, optional, tag = "1")]
    pub network: ::core::option::Option<NetworkPermissions>,
    /// Storage access permissions
    #[prost(message, optional, tag = "2")]
    pub storage: ::core::option::Option<StoragePermissions>,
    /// Blockchain access permissions
    #[prost(message, optional, tag = "3")]
    pub blockchain: ::core::option::Option<BlockchainPermissions>,
}
/// Network access permissions
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkPermissions {
    /// Allow outbound network requests
    #[prost(bool, tag = "1")]
    pub allow_outbound: bool,
    /// Allowed domains for outbound requests
    #[prost(string, repeated, tag = "2")]
    pub allowed_domains: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Storage access permissions
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoragePermissions {
    /// Allow read access to storage
    #[prost(bool, tag = "1")]
    pub allow_read: bool,
    /// Allow write access to storage
    #[prost(bool, tag = "2")]
    pub allow_write: bool,
    /// Storage namespace for the function
    #[prost(string, tag = "3")]
    pub namespace: ::prost::alloc::string::String,
}
/// Blockchain access permissions
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockchainPermissions {
    /// Allow read access to blockchain
    #[prost(bool, tag = "1")]
    pub allow_read: bool,
    /// Allow write access to blockchain (transactions)
    #[prost(bool, tag = "2")]
    pub allow_write: bool,
    /// Allowed contract addresses
    #[prost(string, repeated, tag = "3")]
    pub allowed_contracts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Resource limits for function execution
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceLimits {
    /// Maximum memory usage in MB
    #[prost(uint32, tag = "1")]
    pub memory_mb: u32,
    /// Maximum CPU time in ms
    #[prost(uint32, tag = "2")]
    pub cpu_ms: u32,
    /// Maximum execution time in ms
    #[prost(uint32, tag = "3")]
    pub execution_time_ms: u32,
    /// Maximum storage usage in KB
    #[prost(uint32, tag = "4")]
    pub storage_kb: u32,
}
/// Function registration request
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterFunctionRequest {
    /// Function name
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Function description
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Trigger configuration
    #[prost(message, optional, tag = "3")]
    pub trigger: ::core::option::Option<TriggerConfig>,
    /// Permission configuration
    #[prost(message, optional, tag = "4")]
    pub permissions: ::core::option::Option<PermissionConfig>,
    /// Resource limits
    #[prost(message, optional, tag = "5")]
    pub resources: ::core::option::Option<ResourceLimits>,
    /// Function code
    #[prost(string, tag = "6")]
    pub code: ::prost::alloc::string::String,
}
/// Function registration response
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterFunctionResponse {
    /// Function metadata
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<FunctionMetadata>,
}
/// Function update request
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateFunctionRequest {
    /// Function ID
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Function name (optional)
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Function description (optional)
    #[prost(string, optional, tag = "3")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    /// Trigger configuration (optional)
    #[prost(message, optional, tag = "4")]
    pub trigger: ::core::option::Option<TriggerConfig>,
    /// Permission configuration (optional)
    #[prost(message, optional, tag = "5")]
    pub permissions: ::core::option::Option<PermissionConfig>,
    /// Resource limits (optional)
    #[prost(message, optional, tag = "6")]
    pub resources: ::core::option::Option<ResourceLimits>,
    /// Function code (optional)
    #[prost(string, optional, tag = "7")]
    pub code: ::core::option::Option<::prost::alloc::string::String>,
}
/// Function update response
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateFunctionResponse {
    /// Updated function metadata
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<FunctionMetadata>,
}
/// Function get request
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFunctionRequest {
    /// Function ID
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// Function get response
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFunctionResponse {
    /// Function metadata
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<FunctionMetadata>,
}
/// Function list request
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFunctionsRequest {
    /// Pagination token
    #[prost(string, tag = "1")]
    pub page_token: ::prost::alloc::string::String,
    /// Page size
    #[prost(uint32, tag = "2")]
    pub page_size: u32,
    /// Filter by trigger type
    #[prost(enumeration = "TriggerType", optional, tag = "3")]
    pub trigger_type: ::core::option::Option<i32>,
}
/// Function list response
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFunctionsResponse {
    /// Function metadata list
    #[prost(message, repeated, tag = "1")]
    pub functions: ::prost::alloc::vec::Vec<FunctionMetadata>,
    /// Next page token
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Function delete request
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteFunctionRequest {
    /// Function ID
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// Function delete response
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteFunctionResponse {
    /// Success flag
    #[prost(bool, tag = "1")]
    pub success: bool,
}
/// Types of triggers
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TriggerType {
    Unspecified = 0,
    Blockchain = 1,
    Schedule = 2,
    Http = 3,
    Oracle = 4,
}
impl TriggerType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TriggerType::Unspecified => "TRIGGER_TYPE_UNSPECIFIED",
            TriggerType::Blockchain => "TRIGGER_TYPE_BLOCKCHAIN",
            TriggerType::Schedule => "TRIGGER_TYPE_SCHEDULE",
            TriggerType::Http => "TRIGGER_TYPE_HTTP",
            TriggerType::Oracle => "TRIGGER_TYPE_ORACLE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TRIGGER_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "TRIGGER_TYPE_BLOCKCHAIN" => Some(Self::Blockchain),
            "TRIGGER_TYPE_SCHEDULE" => Some(Self::Schedule),
            "TRIGGER_TYPE_HTTP" => Some(Self::Http),
            "TRIGGER_TYPE_ORACLE" => Some(Self::Oracle),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod function_registry_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Function registry service
    #[derive(Debug, Clone)]
    pub struct FunctionRegistryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl FunctionRegistryClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> FunctionRegistryClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> FunctionRegistryClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            FunctionRegistryClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Register a new function
        pub async fn register_function(
            &mut self,
            request: impl tonic::IntoRequest<super::RegisterFunctionRequest>,
        ) -> Result<tonic::Response<super::RegisterFunctionResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/registry.FunctionRegistry/RegisterFunction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Update an existing function
        pub async fn update_function(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateFunctionRequest>,
        ) -> Result<tonic::Response<super::UpdateFunctionResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/registry.FunctionRegistry/UpdateFunction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Get a function by ID
        pub async fn get_function(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFunctionRequest>,
        ) -> Result<tonic::Response<super::GetFunctionResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/registry.FunctionRegistry/GetFunction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// List functions with optional filtering
        pub async fn list_functions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListFunctionsRequest>,
        ) -> Result<tonic::Response<super::ListFunctionsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/registry.FunctionRegistry/ListFunctions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Delete a function by ID
        pub async fn delete_function(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteFunctionRequest>,
        ) -> Result<tonic::Response<super::DeleteFunctionResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/registry.FunctionRegistry/DeleteFunction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod function_registry_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with FunctionRegistryServer.
    #[async_trait]
    pub trait FunctionRegistry: Send + Sync + 'static {
        /// Register a new function
        async fn register_function(
            &self,
            request: tonic::Request<super::RegisterFunctionRequest>,
        ) -> Result<tonic::Response<super::RegisterFunctionResponse>, tonic::Status>;
        /// Update an existing function
        async fn update_function(
            &self,
            request: tonic::Request<super::UpdateFunctionRequest>,
        ) -> Result<tonic::Response<super::UpdateFunctionResponse>, tonic::Status>;
        /// Get a function by ID
        async fn get_function(
            &self,
            request: tonic::Request<super::GetFunctionRequest>,
        ) -> Result<tonic::Response<super::GetFunctionResponse>, tonic::Status>;
        /// List functions with optional filtering
        async fn list_functions(
            &self,
            request: tonic::Request<super::ListFunctionsRequest>,
        ) -> Result<tonic::Response<super::ListFunctionsResponse>, tonic::Status>;
        /// Delete a function by ID
        async fn delete_function(
            &self,
            request: tonic::Request<super::DeleteFunctionRequest>,
        ) -> Result<tonic::Response<super::DeleteFunctionResponse>, tonic::Status>;
    }
    /// Function registry service
    #[derive(Debug)]
    pub struct FunctionRegistryServer<T: FunctionRegistry> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: FunctionRegistry> FunctionRegistryServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for FunctionRegistryServer<T>
    where
        T: FunctionRegistry,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/registry.FunctionRegistry/RegisterFunction" => {
                    #[allow(non_camel_case_types)]
                    struct RegisterFunctionSvc<T: FunctionRegistry>(pub Arc<T>);
                    impl<
                        T: FunctionRegistry,
                    > tonic::server::UnaryService<super::RegisterFunctionRequest>
                    for RegisterFunctionSvc<T> {
                        type Response = super::RegisterFunctionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RegisterFunctionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).register_function(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RegisterFunctionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/registry.FunctionRegistry/UpdateFunction" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateFunctionSvc<T: FunctionRegistry>(pub Arc<T>);
                    impl<
                        T: FunctionRegistry,
                    > tonic::server::UnaryService<super::UpdateFunctionRequest>
                    for UpdateFunctionSvc<T> {
                        type Response = super::UpdateFunctionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateFunctionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).update_function(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateFunctionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/registry.FunctionRegistry/GetFunction" => {
                    #[allow(non_camel_case_types)]
                    struct GetFunctionSvc<T: FunctionRegistry>(pub Arc<T>);
                    impl<
                        T: FunctionRegistry,
                    > tonic::server::UnaryService<super::GetFunctionRequest>
                    for GetFunctionSvc<T> {
                        type Response = super::GetFunctionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetFunctionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_function(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetFunctionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/registry.FunctionRegistry/ListFunctions" => {
                    #[allow(non_camel_case_types)]
                    struct ListFunctionsSvc<T: FunctionRegistry>(pub Arc<T>);
                    impl<
                        T: FunctionRegistry,
                    > tonic::server::UnaryService<super::ListFunctionsRequest>
                    for ListFunctionsSvc<T> {
                        type Response = super::ListFunctionsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListFunctionsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).list_functions(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListFunctionsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/registry.FunctionRegistry/DeleteFunction" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteFunctionSvc<T: FunctionRegistry>(pub Arc<T>);
                    impl<
                        T: FunctionRegistry,
                    > tonic::server::UnaryService<super::DeleteFunctionRequest>
                    for DeleteFunctionSvc<T> {
                        type Response = super::DeleteFunctionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteFunctionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_function(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteFunctionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: FunctionRegistry> Clone for FunctionRegistryServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: FunctionRegistry> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: FunctionRegistry> tonic::server::NamedService for FunctionRegistryServer<T> {
        const NAME: &'static str = "registry.FunctionRegistry";
    }
}
