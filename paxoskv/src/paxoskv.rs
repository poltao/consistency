/// BallotNum is the ballot number in paxos. It consists of a monotonically
/// incremental number and a universally unique ProposerId.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BallotNum {
    #[prost(int64, tag = "1")]
    pub n: i64,
    #[prost(int64, tag = "2")]
    pub proposer_id: i64,
}
/// Value is the value part of a key-value record.
/// In this demo it is just a int64
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Value {
    #[prost(int64, tag = "1")]
    pub vi64: i64,
}
/// PaxosInstanceId specifies what paxos instance it runs on.
/// A paxos instance is used to determine a specific version of a record.
/// E.g.: for a key-value record foo₀=0, to set foo=2, a paxos instance is
/// created to choose the value for key "foo", ver "1", i.e., foo₁
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaxosInstanceId {
    /// the key of a record to operate on.
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    /// the version of the record to modify.
    #[prost(int64, tag = "2")]
    pub ver: i64,
}
/// Acceptor is the state of an Acceptor and also serves as the reply of the
/// Prepare/Accept.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Acceptor {
    /// the last ballot number the instance knows of.
    #[prost(message, optional, tag = "1")]
    pub last_bal: ::core::option::Option<BallotNum>,
    /// the voted value by this Acceptor
    #[prost(message, optional, tag = "2")]
    pub val: ::core::option::Option<Value>,
    /// at which ballot number the Acceptor voted it.
    #[prost(message, optional, tag = "3")]
    pub v_bal: ::core::option::Option<BallotNum>,
}
/// Proposer is the state of a Proposer and also serves as the request of
/// Prepare/Accept.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Proposer {
    /// what paxos instance it runs on
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<PaxosInstanceId>,
    /// Bal is the ballot number of a Proposer
    #[prost(message, optional, tag = "2")]
    pub bal: ::core::option::Option<BallotNum>,
    /// Val is the value a Proposer has chosen.
    #[prost(message, optional, tag = "3")]
    pub val: ::core::option::Option<Value>,
}
/// Generated client implementations.
pub mod paxos_kv_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// PaxosKV defines the paxos RPC.
    ///
    /// A Proposer sends all its fields in a Prepare request, with `Val` being left a nil.
    /// A Proposer sends all its fields in an Accept request, with `Val` being filled with
    /// the value it chose.
    ///
    /// An Acceptor responds all its fields in a Prepare reply.
    /// An Acceptor responds `LastBal` fields in a Accept reply.
    ///
    /// Thus we just use the struct of a Proposer as request struct.
    /// And the struct of an Acceptor as reply struct.
    #[derive(Debug, Clone)]
    pub struct PaxosKvClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PaxosKvClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> PaxosKvClient<T>
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
        ) -> PaxosKvClient<InterceptedService<T, F>>
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
            PaxosKvClient::new(InterceptedService::new(inner, interceptor))
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
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn prepare(
            &mut self,
            request: impl tonic::IntoRequest<super::Proposer>,
        ) -> std::result::Result<tonic::Response<super::Acceptor>, tonic::Status> {
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
            let path = http::uri::PathAndQuery::from_static("/paxoskv.PaxosKV/Prepare");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("paxoskv.PaxosKV", "Prepare"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn accept(
            &mut self,
            request: impl tonic::IntoRequest<super::Proposer>,
        ) -> std::result::Result<tonic::Response<super::Acceptor>, tonic::Status> {
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
            let path = http::uri::PathAndQuery::from_static("/paxoskv.PaxosKV/Accept");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("paxoskv.PaxosKV", "Accept"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod paxos_kv_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with PaxosKvServer.
    #[async_trait]
    pub trait PaxosKv: Send + Sync + 'static {
        async fn prepare(
            &self,
            request: tonic::Request<super::Proposer>,
        ) -> std::result::Result<tonic::Response<super::Acceptor>, tonic::Status>;
        async fn accept(
            &self,
            request: tonic::Request<super::Proposer>,
        ) -> std::result::Result<tonic::Response<super::Acceptor>, tonic::Status>;
    }
    /// PaxosKV defines the paxos RPC.
    ///
    /// A Proposer sends all its fields in a Prepare request, with `Val` being left a nil.
    /// A Proposer sends all its fields in an Accept request, with `Val` being filled with
    /// the value it chose.
    ///
    /// An Acceptor responds all its fields in a Prepare reply.
    /// An Acceptor responds `LastBal` fields in a Accept reply.
    ///
    /// Thus we just use the struct of a Proposer as request struct.
    /// And the struct of an Acceptor as reply struct.
    #[derive(Debug)]
    pub struct PaxosKvServer<T: PaxosKv> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: PaxosKv> PaxosKvServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
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
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for PaxosKvServer<T>
    where
        T: PaxosKv,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/paxoskv.PaxosKV/Prepare" => {
                    #[allow(non_camel_case_types)]
                    struct PrepareSvc<T: PaxosKv>(pub Arc<T>);
                    impl<T: PaxosKv> tonic::server::UnaryService<super::Proposer>
                    for PrepareSvc<T> {
                        type Response = super::Acceptor;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Proposer>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PaxosKv>::prepare(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PrepareSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/paxoskv.PaxosKV/Accept" => {
                    #[allow(non_camel_case_types)]
                    struct AcceptSvc<T: PaxosKv>(pub Arc<T>);
                    impl<T: PaxosKv> tonic::server::UnaryService<super::Proposer>
                    for AcceptSvc<T> {
                        type Response = super::Acceptor;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Proposer>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PaxosKv>::accept(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AcceptSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
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
    impl<T: PaxosKv> Clone for PaxosKvServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: PaxosKv> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: PaxosKv> tonic::server::NamedService for PaxosKvServer<T> {
        const NAME: &'static str = "paxoskv.PaxosKV";
    }
}
