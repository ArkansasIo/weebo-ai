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
        provider.chat(request, &self.system_prompt).await
    }
}
