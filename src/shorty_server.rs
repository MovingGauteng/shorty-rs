use crate::shorty::*;
use crate::shorty_mongo;
use futures::future;
use tower_grpc::{Code, Request, Response, Status};

#[derive(Clone)]
pub struct ShortyImpl {
    pub client: mongodb::Client,
}

impl server::ShortyService for ShortyImpl {
    type ShortenFuture = future::FutureResult<Response<Shorty>, Status>;
    type AddCounterFuture = future::FutureResult<Response<Empty>, Status>;
    type GetUrlFuture = future::FutureResult<Response<Shorty>, Status>;

    fn shorten(&mut self, request: Request<ShortenRequest>) -> Self::ShortenFuture {
        let data = request.into_inner();
        debug!("Shorten request: {:#?}", &data);
        let shortened = shorty_mongo::shorten(&data, &self.client);
        debug!("Shorten response: {:#?}", &shortened);

        match shortened {
            Some(response) => future::ok(Response::new(response)),
            None => future::err(Status::new(
                Code::Internal,
                "Unable to shorten URL".to_owned(),
            )),
        }
    }

    fn add_counter(&mut self, request: Request<Counter>) -> Self::AddCounterFuture {
        let data = request.into_inner();
        shorty_mongo::ShortyDocument::add_counter(
            bson::oid::ObjectId::with_string(&data.id).unwrap(),
            1,
            &self.client,
        );

        future::ok(Response::new(Empty {}))
    }

    fn get_url(&mut self, request: Request<Shorty>) -> Self::GetUrlFuture {
        let data = request.into_inner();
        debug!("GetUrl request: {:#?}", &data);
        let url = shorty_mongo::ShortyDocument::find_original(&data.url, &self.client);
        debug!("GetUrl response: {:#?}", &url);
        match url {
            Some(document) => {
                let response = Shorty {
                    id: document.id.to_hex(),
                    url: document.constructed,
                };
                future::ok(Response::new(response))
            }
            None => future::err(Status::new(
                Code::NotFound,
                "Error getting shorty URL (1)".to_owned(),
            )),
        }
    }
}
