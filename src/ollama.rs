use anyhow::{anyhow, Result};
use reqwest::Client;
use crate::models::{ChatRequest, Message, OllamaChatResponse};

#[derive(Clone)]
pub struct OllamaProvider { client: Client, base_url: String, default_model: String }

impl OllamaProvider {
    pub fn new(base_url: String, default_model: String) -> Self {
        Self { client: Client::new(), base_url: base_url.trim_end_matches('/').to_owned(), default_model }
    }

    pub async fn chat(&self, request: &ChatRequest, system_prompt: &str) -> Result<Message> {
        let model = request.model.clone().unwrap_or_else(|| self.default_model.clone());
        let mut messages = Vec::with_capacity(request.messages.len() + 1);
        messages.push(Message { role: "system".into(), content: system_prompt.into() });
        messages.extend(request.messages.clone());

        let body = serde_json::json!({
            "model": model,
            "messages": messages,
            "stream": false,
            "options": {
                "temperature": request.temperature,
                "num_predict": request.max_tokens
            }
        });

        let response = self.client.post(format!("{}/api/chat", self.base_url))
            .json(&body).send().await
            .map_err(|e| anyhow!("Could not connect to Ollama: {e}"))?
            .error_for_status()
            .map_err(|e| anyhow!("Ollama HTTP error: {e}"))?;
        Ok(response.json::<OllamaChatResponse>().await?.message)
    }

    pub async fn connected(&self) -> bool {
        self.client.get(format!("{}/api/tags", self.base_url)).send().await
            .map(|r| r.status().is_success()).unwrap_or(false)
    }
}
