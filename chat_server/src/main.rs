use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

mod models;
use models::*;

// type ReceivedMessage = (u64, String);
// type UserMessages = Vec<ReceivedMessage>;

// type WorldHistory = Vec<Cache>;

pub struct Cache {
    room_id: u64,
}

async fn create_room(
    cache: web::Data<Mutex<Cache>>,
    create_req: web::Json<CreateChatRequest>,
) -> impl Responder {
    let mut room = cache.lock().unwrap();
    room.room_id = create_req.room_id;
    HttpResponse::Ok().body("Created Room")
}

async fn list_rooms(cache: web::Data<Mutex<Cache>>) -> impl Responder {
    let room_id = cache.lock().unwrap();
    HttpResponse::Ok().body(room_id.room_id.to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let cache = web::Data::new(Mutex::new(Cache { room_id: 0 }));
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::clone(&cache))
            .route("/create", web::post().to(create_room))
            .route("/list", web::get().to(list_rooms))
    })
    .bind(("127.0.0.1", 3366))?
    .run()
    .await
}
