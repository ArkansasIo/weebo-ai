use anyhow::Result;
use std::{env, path::PathBuf};

#[derive(Clone, Debug)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub ollama_base_url: String,
    pub ollama_model: String,
    pub system_prompt: String,
    pub workspace: PathBuf,
    pub allow_shell: bool,
    pub allow_file_write: bool,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();
        Ok(Self {
            host: env::var("WEEBO_HOST").unwrap_or_else(|_| "127.0.0.1".into()),
            port: env::var("WEEBO_PORT").unwrap_or_else(|_| "8080".into()).parse()?,
            database_url: env::var("WEEBO_DATABASE_URL").unwrap_or_else(|_| "sqlite://weebo.db".into()),
            ollama_base_url: env::var("OLLAMA_BASE_URL").unwrap_or_else(|_| "http://127.0.0.1:11434".into()),
            ollama_model: env::var("OLLAMA_MODEL").unwrap_or_else(|_| "llama3.2".into()),
            system_prompt: env::var("WEEBO_SYSTEM_PROMPT").unwrap_or_else(|_| "You are Weebo AI, a helpful local computer AI assistant.".into()),
            workspace: PathBuf::from(env::var("WEEBO_WORKSPACE").unwrap_or_else(|_| "./workspace".into())),
            allow_shell: env::var("WEEBO_ALLOW_SHELL").unwrap_or_else(|_| "false".into()).parse()?,
            allow_file_write: env::var("WEEBO_ALLOW_FILE_WRITE").unwrap_or_else(|_| "false".into()).parse()?,
        })
    }
}
