use crate::action::ActionEngine;
use crate::memory::MemorySystem;
use crate::state::StateManager;
use crate::task::TaskPlanner;
use crate::vision::VisionSystem;
use anyhow::Result;
use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
};
use serde_json::Value;
use std::sync::Arc;
use tracing::info;

#[derive(Clone)]
pub struct AppState {
    pub action_engine: Arc<ActionEngine>,
    pub vision: Arc<VisionSystem>,
    pub state_manager: Arc<StateManager>,
    pub task_planner: Arc<TaskPlanner>,
    pub memory: Arc<MemorySystem>,
}

async fn handle_status() -> Json<Value> {
    Json(serde_json::json!({
        "status": "running",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

async fn handle_capabilities(State(state): State<AppState>) -> Json<Value> {
    Json(serde_json::json!({
        "actions": state.action_engine.get_available_actions(),
        "vision": state.vision.get_capabilities()
    }))
}

async fn handle_execute_action(
    State(state): State<AppState>,
    Json(action): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    // TODO: Parse and execute action
    Ok(Json(serde_json::json!({"success": true})))
}

async fn handle_screenshot(State(state): State<AppState>) -> Result<Json<Value>, StatusCode> {
    match state.vision.capture_screen(true).await {
        Ok(path) => Ok(Json(serde_json::json!({"screenshot": path}))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

