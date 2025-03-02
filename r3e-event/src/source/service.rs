#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Default, ::prost::Message)]
pub struct AcquireTaskInput {
    #[prost(uint64, tag = "1")]
    pub uid: u64,
    #[prost(uint64, tag = "2")]
    pub fid_hint: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Default, ::prost::Message)]
pub struct AcquireTaskOutput {
    #[prost(uint64, tag = "1")]
    pub uid: u64,
    #[prost(uint64, tag = "2")]
    pub fid: u64,
    #[prost(bytes, tag = "3")]
    pub event_data: Vec<u8>,
    /// This field is not serialized/deserialized by prost.
    #[serde(skip)]
    #[prost(skip)]
    pub event: ::core::option::Option<super::events::Event>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Default, ::prost::Message)]
pub struct AcquireFuncInput {
    #[prost(uint64, tag = "1")]
    pub uid: u64,
    #[prost(uint64, tag = "2")]
    pub fid: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Default, ::prost::Message)]
pub struct Func {
    #[prost(uint64, tag = "1")]
    pub version: u64,
    #[prost(string, tag = "2")]
    pub code: String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Default, ::prost::Message)]
pub struct AcquireFuncOutput {
    #[prost(message, optional, tag = "1")]
    pub func: ::core::option::Option<Func>,
}
/// Generated client implementations.
pub mod task_source_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct TaskSourceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TaskSourceClient<tonic::transport::Channel> {
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
    impl<T> TaskSourceClient<T>
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
        ) -> TaskSourceClient<InterceptedService<T, F>>
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
            TaskSourceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn acquire_task(
            &mut self,
            request: impl tonic::IntoRequest<super::AcquireTaskInput>,
        ) -> Result<tonic::Response<super::AcquireTaskOutput>, tonic::Status> {
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
                "/service.TaskSource/AcquireTask",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn acquire_func(
            &mut self,
            request: impl tonic::IntoRequest<super::AcquireFuncInput>,
        ) -> Result<tonic::Response<super::AcquireFuncOutput>, tonic::Status> {
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
                "/service.TaskSource/AcquireFunc",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod task_source_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Task
    #[derive(Debug, Clone)]
    pub struct Task {
        /// User ID
        pub uid: u64,
        /// Function ID
        pub fid: u64,
        /// Event
        pub event: super::events::Event,
    }
    /// Task error
    #[derive(Debug, Clone)]
    pub enum TaskError {
        /// No more task
        NoMoreTask(u64),
        /// Other error
        Other(String),
    }
    #[tonic::async_trait]
    pub trait TaskSource: Send + Sync + 'static {
        /// Acquire a task
        async fn acquire_task(&self, request: AcquireTaskInput) -> Result<Task, TaskError>;

        /// Acquire a function
        async fn acquire_fn(&self, request: AcquireFuncInput) -> Result<Func, TaskError>;
    }
    #[derive(Debug)]
    pub struct TaskSourceServer<T: TaskSource> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: TaskSource> TaskSourceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for TaskSourceServer<T>
    where
        T: TaskSource,
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
                "/service.TaskSource/AcquireTask" => {
                    #[allow(non_camel_case_types)]
                    struct AcquireTaskSvc<T: TaskSource>(pub Arc<T>);
                    impl<
                        T: TaskSource,
                    > tonic::server::UnaryService<super::AcquireTaskInput>
                    for AcquireTaskSvc<T> {
                        type Response = super::AcquireTaskOutput;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AcquireTaskInput>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                let req = request.into_inner();
                                match inner.acquire_task(req).await {
                                    Ok(task) => {
                                        // Serialize the event to bytes
                                        let event_data = serde_json::to_vec(&task.event).unwrap_or_default();
                                        
                                        // Create the response
                                        let output = AcquireTaskOutput {
                                            uid: task.uid,
                                            fid: task.fid,
                                            event_data,
                                            event: None,
                                        };
                                        
                                        Ok(tonic::Response::new(output))
                                    }
                                    Err(e) => {
                                        Err(tonic::Status::internal(format!("Task acquisition failed: {:?}", e)))
                                    }
                                }
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AcquireTaskSvc(inner);
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
                "/service.TaskSource/AcquireFunc" => {
                    #[allow(non_camel_case_types)]
                    struct AcquireFuncSvc<T: TaskSource>(pub Arc<T>);
                    impl<
                        T: TaskSource,
                    > tonic::server::UnaryService<super::AcquireFuncInput>
                    for AcquireFuncSvc<T> {
                        type Response = super::AcquireFuncOutput;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AcquireFuncInput>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).acquire_fn(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AcquireFuncSvc(inner);
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
    impl<T: TaskSource> Clone for TaskSourceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: TaskSource> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: TaskSource> tonic::server::NamedService for TaskSourceServer<T> {
        const NAME: &'static str = "service.TaskSource";
    }
}
