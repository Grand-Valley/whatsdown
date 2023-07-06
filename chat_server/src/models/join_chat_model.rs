use super::*;

#[derive(Serialize, Deserialize, Default)]
pub struct JoinChatRequest {
    pub room_id: u64,
    pub user_id: u64,
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
