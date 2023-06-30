pub mod create_chat_model;
pub mod join_chat_model;

pub use create_chat_model::*;
pub use join_chat_model::*;

// use std::collections::HashMap;
// use std::sync::Arc;
// pub trait DoesRoomExist {
// fn does_room_exist(self, cache: Arc<HashMap<u64, Vec<(Uuid, String)>>>) -> bool;
// }
