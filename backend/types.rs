use candid::CandidType;
use serde::Deserialize;

#[derive(CandidType, Clone, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}