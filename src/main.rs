#![recursion_limit = "128"]
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use(bson, doc)]
extern crate bson;
#[macro_use]
extern crate log;

use std::env;
use std::str::FromStr;

use dotenv::dotenv;
use futures::{Future, Stream};
use mongodb::{Client, ThreadedClient};
use tokio::net::TcpListener;
use tower_hyper::server::{Http, Server};

use crate::shorty_server::ShortyImpl;

fn main() {
    dotenv().ok();
    env_logger::init();

    let mongo_host = env::var("MONGO_HOST").expect("MONGO_HOST variable not found");
    let mongo_port = u16::from_str(env::var("MONGO_PORT").expect("MONGO_PORT variable not found").as_str()).unwrap();

    let client = Client::connect(&mongo_host, mongo_port).expect("Failed to connect to database");

    let handler = ShortyImpl { client };

    let new_service = shorty::server::ShortyServiceServer::new(handler);

    let mut server = Server::new(new_service);
    let http = Http::new().http2_only(true).clone();
    let addr = env::var("GRPC_SERVER_HOST").expect("GRPC_SERVER_HOST variable not found").as_str().parse().unwrap();
    let bind = TcpListener::bind(&addr).expect("Unable to bind");

    info!("shorty listening on {:?}", addr);

    let serve = bind
        .incoming()
        .for_each(move |sock| {
            if let Err(e) = sock.set_nodelay(true) {
                return Err(e);
            }

            let serve = server.serve_with(sock, http.clone());
            tokio::spawn(serve.map_err(|e| error!("h2 error: {:?}", e)));

            Ok(())
        })
        .map_err(|e| eprintln!("accept error: {}", e));

    tokio::run(serve);
}

mod shorty;
mod shorty_mongo;
mod shorty_server;
