pub mod join_chat_model;

pub use join_chat_model::*;

use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;
pub trait DoesRoomExist {
    fn does_room_exist(self, cache: Arc<HashMap<u64, Vec<(Uuid, String)>>>) -> bool;
}
