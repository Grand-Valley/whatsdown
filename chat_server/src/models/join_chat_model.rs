use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub trait DoesRoomExist {
    fn does_room_exist(self, cache: &HashMap<Uuid, ()>) -> bool;
}

#[derive(Serialize, Deserialize, Default)]
pub struct JoinChatRequest {
    pub room_id: Uuid,
    pub user_id: Uuid,
}

impl DoesRoomExist for JoinChatRequest {
    fn does_room_exist(self, cache: &HashMap<Uuid, ()>) -> bool {
        cache.contains_key(&self.room_id)
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct JoinChatResponse {
    pub response: JoinChatStatus,
}

#[derive(Serialize, Deserialize, Default)]
pub enum JoinChatStatus {
    Success,
    RoomFull,
    PermissionDenied,
    #[default]
    Failure,
}
