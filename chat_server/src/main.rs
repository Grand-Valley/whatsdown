use hyper::body;
use hyper::body::Body;
// use hyper::header::*;
use hyper::server::conn::AddrStream;
use hyper::server::Server;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Request, Response};
use std::collections::HashMap;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;

#[allow(unused_imports)]
mod models;
use crate::models::*;

type Cache = Arc<RwLock<HashMap<u64, Vec<(u64, String)>>>>;

#[tokio::main()]
async fn main() -> Result<(), Infallible> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3366));

    let cache: Cache = Arc::new(RwLock::new(HashMap::new()));

    let service = make_service_fn(move |_conn: &AddrStream| {
        let cache_clone = cache.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
                let cache_clone = cache_clone.clone();
                async move {
                    match req.uri().path() {
                        "/join" => join_chat_room(cache_clone, req).await,
                        "/create" => create_chat_room(cache_clone, req).await,
                        _ => Ok(Response::builder().status(404).body(Body::empty()).unwrap()),
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

async fn create_chat_room(cache: Cache, req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let body = body::to_bytes(req.into_body()).await.unwrap();
    let req: CreateChatRequest = serde_json::from_slice(&body).unwrap();
    let user_id = req.user_id;
    cache.write().await.entry(user_id).or_insert(Vec::new());
    Ok(Response::new(Body::from("Created")))
}

async fn join_chat_room(cache: Cache, req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let body = body::to_bytes(req.into_body()).await.unwrap();
    let req: JoinChatRequest = serde_json::from_slice(&body.slice(..)).unwrap();

    if cache.read().await.contains_key(&req.room_id) {
        return Ok::<_, Infallible>(Response::new(Body::from("Room does not exist")));
    }
    Ok::<_, Infallible>(Response::new(Body::from("Room exists")))
}
