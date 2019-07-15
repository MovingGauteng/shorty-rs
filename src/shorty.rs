#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Empty {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Counter {
    #[prost(string, tag="1")]
    pub id: std::string::String,
    #[prost(uint32, tag="2")]
    pub value: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShortenRequest {
    #[prost(string, tag="1")]
    pub url: std::string::String,
    #[prost(message, optional, tag="2")]
    pub campaign: ::std::option::Option<GoogleAnalyticsCampaign>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Shorty {
    #[prost(string, tag="1")]
    pub id: std::string::String,
    #[prost(string, tag="2")]
    pub url: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GoogleAnalyticsCampaign {
    #[prost(string, tag="1")]
    pub utm_source: std::string::String,
    #[prost(string, tag="2")]
    pub utm_medium: std::string::String,
    #[prost(string, tag="3")]
    pub utm_campaign: std::string::String,
    #[prost(string, tag="4")]
    pub utm_term: std::string::String,
    #[prost(string, tag="5")]
    pub utm_content: std::string::String,
}
pub mod server {
    use ::tower_grpc::codegen::server::*;
    use super::{ShortenRequest, Shorty, Counter, Empty};

    // Redefine the try_ready macro so that it doesn't need to be explicitly
    // imported by the user of this generated code.
    macro_rules! try_ready {
        ($e:expr) => (match $e {
            Ok(futures::Async::Ready(t)) => t,
            Ok(futures::Async::NotReady) => return Ok(futures::Async::NotReady),
            Err(e) => return Err(From::from(e)),
        })
    }

    pub trait ShortyService: Clone {
        type ShortenFuture: futures::Future<Item = grpc::Response<Shorty>, Error = grpc::Status>;
        type AddCounterFuture: futures::Future<Item = grpc::Response<Empty>, Error = grpc::Status>;
        type GetUrlFuture: futures::Future<Item = grpc::Response<Shorty>, Error = grpc::Status>;

        /// shorten an url
        fn shorten(&mut self, request: grpc::Request<ShortenRequest>) -> Self::ShortenFuture;

        /// increment number of times the url has been viewed
        fn add_counter(&mut self, request: grpc::Request<Counter>) -> Self::AddCounterFuture;

        /// get url, you only need to provide the url (shortened suffix)
        fn get_url(&mut self, request: grpc::Request<Shorty>) -> Self::GetUrlFuture;
    }

    #[derive(Debug, Clone)]
    pub struct ShortyServiceServer<T> {
        shorty_service: T,
    }

    impl<T> ShortyServiceServer<T>
    where T: ShortyService,
    {
        pub fn new(shorty_service: T) -> Self {
            Self { shorty_service }
        }
    }

    impl<T> tower::Service<http::Request<grpc::BoxBody>> for ShortyServiceServer<T>
    where T: ShortyService,
    {
        type Response = http::Response<shorty_service::ResponseBody<T>>;
        type Error = grpc::Never;
        type Future = shorty_service::ResponseFuture<T>;

        fn poll_ready(&mut self) -> futures::Poll<(), Self::Error> {
            Ok(().into())
        }

        fn call(&mut self, request: http::Request<grpc::BoxBody>) -> Self::Future {
            use self::shorty_service::Kind::*;

            match request.uri().path() {
                "/shorty.ShortyService/Shorten" => {
                    let service = shorty_service::methods::Shorten(self.shorty_service.clone());
                    let response = grpc::unary(service, request);
                    shorty_service::ResponseFuture { kind: Shorten(response) }
                }
                "/shorty.ShortyService/AddCounter" => {
                    let service = shorty_service::methods::AddCounter(self.shorty_service.clone());
                    let response = grpc::unary(service, request);
                    shorty_service::ResponseFuture { kind: AddCounter(response) }
                }
                "/shorty.ShortyService/GetUrl" => {
                    let service = shorty_service::methods::GetUrl(self.shorty_service.clone());
                    let response = grpc::unary(service, request);
                    shorty_service::ResponseFuture { kind: GetUrl(response) }
                }
                _ => {
                    shorty_service::ResponseFuture { kind: __Generated__Unimplemented(grpc::unimplemented(format!("unknown service: {:?}", request.uri().path()))) }
                }
            }
        }
    }

    impl<T> tower::Service<()> for ShortyServiceServer<T>
    where T: ShortyService,
    {
        type Response = Self;
        type Error = grpc::Never;
        type Future = futures::FutureResult<Self::Response, Self::Error>;

        fn poll_ready(&mut self) -> futures::Poll<(), Self::Error> {
            Ok(futures::Async::Ready(()))
        }

        fn call(&mut self, _target: ()) -> Self::Future {
            futures::ok(self.clone())
        }
    }

    impl<T> tower::Service<http::Request<tower_hyper::Body>> for ShortyServiceServer<T>
    where T: ShortyService,
    {
        type Response = <Self as tower::Service<http::Request<grpc::BoxBody>>>::Response;
        type Error = <Self as tower::Service<http::Request<grpc::BoxBody>>>::Error;
        type Future = <Self as tower::Service<http::Request<grpc::BoxBody>>>::Future;

        fn poll_ready(&mut self) -> futures::Poll<(), Self::Error> {
            tower::Service::<http::Request<grpc::BoxBody>>::poll_ready(self)
        }

        fn call(&mut self, request: http::Request<tower_hyper::Body>) -> Self::Future {
            let request = request.map(|b| grpc::BoxBody::map_from(b));
            tower::Service::<http::Request<grpc::BoxBody>>::call(self, request)
        }
    }

    pub mod shorty_service {
        use ::tower_grpc::codegen::server::*;
        use super::ShortyService;
        use super::super::{ShortenRequest, Counter, Shorty};

        pub struct ResponseFuture<T>
        where T: ShortyService,
        {
            pub(super) kind: Kind<
                // Shorten
                grpc::unary::ResponseFuture<methods::Shorten<T>, grpc::BoxBody, ShortenRequest>,
                // AddCounter
                grpc::unary::ResponseFuture<methods::AddCounter<T>, grpc::BoxBody, Counter>,
                // GetUrl
                grpc::unary::ResponseFuture<methods::GetUrl<T>, grpc::BoxBody, Shorty>,
                // A generated catch-all for unimplemented service calls
                grpc::unimplemented::ResponseFuture,
            >,
        }

        impl<T> futures::Future for ResponseFuture<T>
        where T: ShortyService,
        {
            type Item = http::Response<ResponseBody<T>>;
            type Error = grpc::Never;

            fn poll(&mut self) -> futures::Poll<Self::Item, Self::Error> {
                use self::Kind::*;

                match self.kind {
                    Shorten(ref mut fut) => {
                        let response = try_ready!(fut.poll());
                        let response = response.map(|body| {
                            ResponseBody { kind: Shorten(body) }
                        });
                        Ok(response.into())
                    }
                    AddCounter(ref mut fut) => {
                        let response = try_ready!(fut.poll());
                        let response = response.map(|body| {
                            ResponseBody { kind: AddCounter(body) }
                        });
                        Ok(response.into())
                    }
                    GetUrl(ref mut fut) => {
                        let response = try_ready!(fut.poll());
                        let response = response.map(|body| {
                            ResponseBody { kind: GetUrl(body) }
                        });
                        Ok(response.into())
                    }
                    __Generated__Unimplemented(ref mut fut) => {
                        let response = try_ready!(fut.poll());
                        let response = response.map(|body| {
                            ResponseBody { kind: __Generated__Unimplemented(body) }
                        });
                        Ok(response.into())
                    }
                }
            }
        }

        pub struct ResponseBody<T>
        where T: ShortyService,
        {
            pub(super) kind: Kind<
                // Shorten
                grpc::Encode<grpc::unary::Once<<methods::Shorten<T> as grpc::UnaryService<ShortenRequest>>::Response>>,
                // AddCounter
                grpc::Encode<grpc::unary::Once<<methods::AddCounter<T> as grpc::UnaryService<Counter>>::Response>>,
                // GetUrl
                grpc::Encode<grpc::unary::Once<<methods::GetUrl<T> as grpc::UnaryService<Shorty>>::Response>>,
                // A generated catch-all for unimplemented service calls
                (),
            >,
        }

        impl<T> tower::HttpBody for ResponseBody<T>
        where T: ShortyService,
        {
            type Data = <grpc::BoxBody as grpc::Body>::Data;
            type Error = grpc::Status;

            fn is_end_stream(&self) -> bool {
                use self::Kind::*;

                match self.kind {
                    Shorten(ref v) => v.is_end_stream(),
                    AddCounter(ref v) => v.is_end_stream(),
                    GetUrl(ref v) => v.is_end_stream(),
                    __Generated__Unimplemented(_) => true,
                }
            }

            fn poll_data(&mut self) -> futures::Poll<Option<Self::Data>, Self::Error> {
                use self::Kind::*;

                match self.kind {
                    Shorten(ref mut v) => v.poll_data(),
                    AddCounter(ref mut v) => v.poll_data(),
                    GetUrl(ref mut v) => v.poll_data(),
                    __Generated__Unimplemented(_) => Ok(None.into()),
                }
            }

            fn poll_trailers(&mut self) -> futures::Poll<Option<http::HeaderMap>, Self::Error> {
                use self::Kind::*;

                match self.kind {
                    Shorten(ref mut v) => v.poll_trailers(),
                    AddCounter(ref mut v) => v.poll_trailers(),
                    GetUrl(ref mut v) => v.poll_trailers(),
                    __Generated__Unimplemented(_) => Ok(None.into()),
                }
            }
        }

        #[allow(non_camel_case_types)]
        #[derive(Debug, Clone)]
        pub(super) enum Kind<Shorten, AddCounter, GetUrl, __Generated__Unimplemented> {
            Shorten(Shorten),
            AddCounter(AddCounter),
            GetUrl(GetUrl),
            __Generated__Unimplemented(__Generated__Unimplemented),
        }

        pub mod methods {
            use ::tower_grpc::codegen::server::*;
            use super::super::{ShortyService, ShortenRequest, Shorty, Counter, Empty};

            pub struct Shorten<T>(pub T);

            impl<T> tower::Service<grpc::Request<ShortenRequest>> for Shorten<T>
            where T: ShortyService,
            {
                type Response = grpc::Response<Shorty>;
                type Error = grpc::Status;
                type Future = T::ShortenFuture;

                fn poll_ready(&mut self) -> futures::Poll<(), Self::Error> {
                    Ok(futures::Async::Ready(()))
                }

                fn call(&mut self, request: grpc::Request<ShortenRequest>) -> Self::Future {
                    self.0.shorten(request)
                }
            }

            pub struct AddCounter<T>(pub T);

            impl<T> tower::Service<grpc::Request<Counter>> for AddCounter<T>
            where T: ShortyService,
            {
                type Response = grpc::Response<Empty>;
                type Error = grpc::Status;
                type Future = T::AddCounterFuture;

                fn poll_ready(&mut self) -> futures::Poll<(), Self::Error> {
                    Ok(futures::Async::Ready(()))
                }

                fn call(&mut self, request: grpc::Request<Counter>) -> Self::Future {
                    self.0.add_counter(request)
                }
            }

            pub struct GetUrl<T>(pub T);

            impl<T> tower::Service<grpc::Request<Shorty>> for GetUrl<T>
            where T: ShortyService,
            {
                type Response = grpc::Response<Shorty>;
                type Error = grpc::Status;
                type Future = T::GetUrlFuture;

                fn poll_ready(&mut self) -> futures::Poll<(), Self::Error> {
                    Ok(futures::Async::Ready(()))
                }

                fn call(&mut self, request: grpc::Request<Shorty>) -> Self::Future {
                    self.0.get_url(request)
                }
            }
        }
    }
}
