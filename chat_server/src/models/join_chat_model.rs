use super::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct JoinChatRequest {
    pub room_id: u64,
    pub user_id: u64,
}

impl DoesRoomExist for JoinChatRequest {
    fn does_room_exist(self, cache: Arc<HashMap<u64, Vec<(Uuid, String)>>>) -> bool {
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
