use hyper::body;
use hyper::body::{Body, Bytes};
use hyper::header::*;
use hyper::server::conn::AddrStream;
use hyper::server::Server;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Request, Response};
use std::collections::HashMap;
use std::convert::Infallible;
use std::net::SocketAddr;

mod models;
use models::*;

use tokio::*;

#[tokio::main()]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3366));

    let service = make_service_fn(|conn: &AddrStream| async {
        Ok::<_, Infallible>(service_fn(join_chat_room))
    });

    // TODO: Create Chat room
    let server = Server::bind(&addr).serve(service);
}

// async fn create_chat_room(req: Request<Body>) -> Result<Response<Body>, Infallible> {
//     let body = body::to_bytes(req.into()).await.unwrap();
//     // let req: CreateChatRequest = serde_json::from_slice(&body).unwrap();
//     // let room_id = req.room_id;
//     // Ok(Response::new())
// }

async fn join_chat_room(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let body = body::to_bytes(req.into_body()).await.unwrap();
    let req: JoinChatRequest = serde_json::from_slice(&body.slice(..)).unwrap();
    let room_id = req.room_id;
    let user_id = req.user_id;
    Ok(Response::new(Body::empty()))
}
