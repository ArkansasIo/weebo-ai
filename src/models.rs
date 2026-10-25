use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message { pub role: String, pub content: String }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRequest {
    pub model: Option<String>,
    pub messages: Vec<Message>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
    pub conversation_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ChatResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub conversation_id: Uuid,
}

#[derive(Debug, Clone, Serialize)]
pub struct Choice { pub index: u32, pub message: Message, pub finish_reason: String }

#[derive(Debug, Clone, Serialize)]
pub struct ModelInfo { pub id: String, pub object: String, pub owned_by: String }

#[derive(Debug, Clone, Serialize)]
pub struct ModelsResponse { pub object: String, pub data: Vec<ModelInfo> }

#[derive(Debug, Clone, Deserialize)]
pub struct ToolInput { pub input: String }

#[derive(Debug, Clone, Serialize)]
pub struct ToolInfo { pub name: String, pub description: String, pub enabled: bool }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaChatResponse { pub model: String, pub message: Message, pub done: bool }
