mod api;
mod config;
mod memory;
mod models;
mod ollama;
mod tools;

use anyhow::Result;
use axum::{routing::{get, post}, Router};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::{
    api::{calculate, chat_completions, health, index, list_models, list_tools, read_file, shell, write_file},
    config::Config,
    memory::MemoryStore,
    ollama::OllamaProvider,
    tools::ToolRegistry,
};

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub memory: MemoryStore,
    pub ollama: OllamaProvider,
    pub tools: ToolRegistry,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("weebo_ai=info,tower_http=info")
        .init();

    let config = Config::from_env()?;
    let memory = MemoryStore::connect(&config.database_url).await?;
    memory.migrate().await?;
    tokio::fs::create_dir_all(&config.workspace).await?;

    let state = AppState {
        ollama: OllamaProvider::new(config.ollama_base_url.clone(), config.ollama_model.clone()),
        tools: ToolRegistry::new(config.clone()),
        config: config.clone(),
        memory,
    };

    let app = Router::new()
        .route("/", get(index))
        .route("/health", get(health))
        .route("/v1/models", get(list_models))
        .route("/v1/chat/completions", post(chat_completions))
        .route("/v1/tools", get(list_tools))
        .route("/v1/tools/calculate", post(calculate))
        .route("/v1/tools/read-file", post(read_file))
        .route("/v1/tools/write-file", post(write_file))
        .route("/v1/tools/shell", post(shell))
        .with_state(state)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    let address = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(&address).await?;
    tracing::info!("Weebo AI listening on http://{address}");
    axum::serve(listener, app).await?;
    Ok(())
}
