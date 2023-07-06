use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::RwLock;

mod models;
use models::*;

// type ReceivedMessage = (u64, String);
// type UserMessages = Vec<ReceivedMessage>;

// type WorldHistory = Vec<Cache>;

// QUES: use?
pub struct Cache {
    rooms: Vec<u64>,
}

#[post("/create")]
async fn create_room(
    cache: web::Data<RwLock<Cache>>,
    create_req: web::Json<CreateChatRequest>,
) -> impl Responder {
    let mut wcache = cache.write().unwrap();
    wcache.rooms.push(create_req.room_id);
    HttpResponse::Ok().body("Created Room")
}

#[get("/list")]
async fn list_rooms(cache: web::Data<RwLock<Cache>>) -> impl Responder {
    let rcache = cache.read().unwrap();
    HttpResponse::Ok().body(
        rcache
            .rooms
            .iter()
            .map(|room_id| format!("{}", room_id))
            .collect::<Vec<String>>()
            .join(","),
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let cache = web::Data::new(RwLock::new(Cache { rooms: Vec::new() }));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::clone(&cache))
            .service(create_room)
            .service(list_rooms)
    })
    .bind(("127.0.0.1", 3366))?
    .run()
    .await
}
