use super::*;

#[derive(Deserialize)]
pub struct CreateChatRequest {
    pub user_id: u64,
    pub room_id: u64,
}

#[derive(Serialize, Deserialize, Default)]
pub struct CreateChatResponse {
    pub response: CreateChatStatus,
}

#[derive(Serialize, Deserialize, Default)]
pub enum CreateChatStatus {
    Success,
    #[default]
    Denied,
}
