use anyhow::Result;
use crate::{models::{Message, ChatRequest}, ollama::OllamaProvider};

#[derive(Clone)]
pub struct Agent {
    pub name: String,
    pub system_prompt: String,
}

impl Agent {
    pub fn weebo() -> Self {
        Self {
            name: "Weebo".into(),
            system_prompt: "You are Weebo AI, a local computer assistant. Be precise, practical, and transparent about tool actions. Never claim an action succeeded unless the tool confirms it.".into(),
        }
    }

    pub async fn run(&self, provider: &OllamaProvider, request: &ChatRequest) -> Result<Message> {
        let mut messages = Vec::with_capacity(request.messages.len() + 1);
        messages.push(Message { role: "system".into(), content: self.system_prompt.clone() });
        messages.extend(request.messages.clone());
        provider.chat(&messages, request.model.clone(), request.temperature, request.max_tokens).await
    }
}
