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
use std::sync::Arc;
use uuid::Uuid;

mod models;
use crate::models::*;

#[tokio::main()]
async fn main() -> Result<(), Infallible> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3366));

    let cache: Arc<HashMap<u64, Vec<(Uuid, String)>>> = Arc::new(HashMap::new());

    let service = make_service_fn(move |_conn: &AddrStream| {
        let cache_clone = cache.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
                let cache_clone = cache_clone.clone();
                async move {
                    let body = body::to_bytes(req.into_body()).await.unwrap();
                    let req: JoinChatRequest = serde_json::from_slice(&body.slice(..)).unwrap();

                    if req.does_room_exist(cache_clone.clone()) {
                        Ok::<_, Infallible>(Response::new(Body::from("hello")))
                    } else {
                        Ok::<_, Infallible>(Response::new(Body::from("not hello")))
                    }
                }
            }))
        }
    });

    // TODO: Create Chat room
    let server = Server::bind(&addr).serve(service);
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }

    Ok(())
}

// async fn create_chat_room(req: Request<Body>) -> Result<Response<Body>, Infallible> {
//     let body = body::to_bytes(req.into()).await.unwrap();
//     // let req: CreateChatRequest = serde_json::from_slice(&body).unwrap();
//     // let room_id = req.room_id;
//     // Ok(Response::new())
// }

// async fn join_chat_room(req: Request<Body>) -> Result<Response<Body>, Infallible> {}
