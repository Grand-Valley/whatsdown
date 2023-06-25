use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Default)]
pub struct JoinChatRequest {
    pub room_id: Uuid,
    pub user_id: Uuid,
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
