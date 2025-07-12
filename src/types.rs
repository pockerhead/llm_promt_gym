use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
    pub timestamp: f64,
}

#[derive(Serialize, Deserialize)]
pub struct ChatRequest {
    pub messages: Vec<ChatMessage>,
    pub model: String,
    pub temperature: f64,
    pub max_tokens: u32,
}

#[derive(Serialize, Deserialize)]
pub struct ChatResponse {
    pub message: String,
    pub error: Option<String>,
} 