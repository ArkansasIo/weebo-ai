use axum::{extract::State, http::StatusCode, response::Html, Json};
use chrono::Utc;
use uuid::Uuid;
use crate::{models::{ChatRequest, ChatResponse, Choice, ModelInfo, ModelsResponse, ToolInput}, AppState};

pub async fn index() -> Html<&'static str> { Html(include_str!("../web/index.html")) }

pub async fn health(State(state): State<AppState>) -> Json<serde_json::Value> {
    let connected = state.ollama.connected().await;
    Json(serde_json::json!({
        "status":"ok", "name":"Weebo AI", "version":env!("CARGO_PKG_VERSION"),
        "ollama":{"connected":connected,"base_url":state.config.ollama_base_url,"model":state.config.ollama_model}
    }))
}

pub async fn list_models(State(state): State<AppState>) -> Json<ModelsResponse> {
    Json(ModelsResponse { object:"list".into(), data:vec![ModelInfo {
        id:state.config.ollama_model.clone(), object:"model".into(), owned_by:"ollama".into()
    }]})
}

pub async fn chat_completions(
    State(state): State<AppState>, Json(mut request): Json<ChatRequest>,
) -> Result<Json<ChatResponse>, (StatusCode, Json<serde_json::Value>)> {
    if request.messages.is_empty() {
        return Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({
            "error":{"type":"invalid_request_error","message":"messages must not be empty"}
        }))));
    }
    if request.messages.iter().any(|m| !["system","user","assistant","tool"].contains(&m.role.as_str())) {
        return Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({"error":"unsupported message role"}))));
    }
    let conversation_id = request.conversation_id.unwrap_or_else(Uuid::new_v4);
    request.conversation_id = Some(conversation_id);
    let requested_model = request.model.clone().unwrap_or_else(|| state.config.ollama_model.clone());

    for message in request.messages.iter().filter(|m| m.role=="user" || m.role=="assistant") {
        let _ = state.memory.store_message(conversation_id, message).await;
    }
    let answer = state.ollama.chat(&request, &state.config.system_prompt).await.map_err(|e| {
        (StatusCode::BAD_GATEWAY, Json(serde_json::json!({"error":{"type":"ollama_error","message":e.to_string()}})))
    })?;
    let _ = state.memory.store_message(conversation_id, &answer).await;

    Ok(Json(ChatResponse {
        id:format!("weebo-{}",Uuid::new_v4()), object:"chat.completion".into(),
        created:Utc::now().timestamp(), model:requested_model,
        choices:vec![Choice{index:0,message:answer,finish_reason:"stop".into()}],
        conversation_id
    }))
}

pub async fn list_tools(State(state): State<AppState>) -> Json<serde_json::Value> {
    Json(serde_json::json!({"tools":state.tools.list()}))
}

pub async fn calculate(State(state): State<AppState>, Json(input): Json<ToolInput>)
    -> Result<Json<serde_json::Value>, (StatusCode,Json<serde_json::Value>)> {
    let result=state.tools.calculate(&input.input).map_err(bad)?;
    Ok(Json(serde_json::json!({"tool":"calculator","result":result})))
}

pub async fn read_file(State(state): State<AppState>, Json(input): Json<ToolInput>)
    -> Result<Json<serde_json::Value>, (StatusCode,Json<serde_json::Value>)> {
    let content=state.tools.read_file(&input.input).await.map_err(bad)?;
    Ok(Json(serde_json::json!({"tool":"read_file","content":content})))
}

pub async fn write_file(State(state): State<AppState>, Json(input): Json<serde_json::Value>)
    -> Result<Json<serde_json::Value>, (StatusCode,Json<serde_json::Value>)> {
    let path=input["path"].as_str().ok_or_else(|| bad("missing path"))?;
    let content=input["content"].as_str().ok_or_else(|| bad("missing content"))?;
    state.tools.write_file(path,content).await.map_err(bad)?;
    Ok(Json(serde_json::json!({"tool":"write_file","success":true})))
}

pub async fn shell(State(state): State<AppState>, Json(input): Json<ToolInput>)
    -> Result<Json<serde_json::Value>, (StatusCode,Json<serde_json::Value>)> {
    let output=state.tools.run_shell(&input.input).await.map_err(|e| {
        if e.to_string().contains("disabled") {
            (StatusCode::FORBIDDEN,Json(serde_json::json!({"error":e.to_string()})))
        } else { bad(e) }
    })?;
    Ok(Json(serde_json::json!({"tool":"shell","output":output})))
}

fn bad<E: std::fmt::Display>(error:E)->(StatusCode,Json<serde_json::Value>) {
    (StatusCode::BAD_REQUEST,Json(serde_json::json!({"error":error.to_string()})))
}
