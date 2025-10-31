use crate::agent::PurchaseOrderAgent;
use a2a::{A2AProtocol, Message, Part};
use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing::{info, error};

/// HTTP request structure for sending tasks
#[derive(Debug, Deserialize)]
pub struct SendTaskRequest {
    pub message: Message,
}

/// HTTP response structure for task operations
#[derive(Debug, Serialize)]
pub struct TaskResponse {
    pub task_id: String,
    pub status: String,
    pub csv_output: Option<String>,
    pub detailed_result: Option<serde_json::Value>,
    pub error: Option<String>,
}

/// HTTP response for agent information
#[derive(Debug, Serialize)]
pub struct AgentInfoResponse {
    pub name: String,
    pub description: Option<String>,
    pub url: String,
    pub version: String,
    pub endpoints: Vec<EndpointInfo>,
}

#[derive(Debug, Serialize)]
pub struct EndpointInfo {
    pub path: String,
    pub method: String,
    pub description: String,
}

/// Shared application state
pub struct AppState {
    pub agent: Arc<PurchaseOrderAgent>,
}

/// Create the web server router
pub fn create_router(agent: Arc<PurchaseOrderAgent>) -> Router {
    let state = Arc::new(AppState { agent });

    Router::new()
        .route("/", get(get_agent_info))
        .route("/agent/info", get(get_agent_info))
        .route("/agent/task", post(send_task))
        .route("/agent/task/:task_id", get(get_task))
        .route("/agent/task/:task_id/cancel", post(cancel_task))
        .route("/health", get(health_check))
        .layer(CorsLayer::permissive())
        .with_state(state)
}

/// Get agent information
async fn get_agent_info(State(state): State<Arc<AppState>>) -> Json<AgentInfoResponse> {
    let card = state.agent.get_agent_card();
    
    let endpoints = vec![
        EndpointInfo {
            path: "/".to_string(),
            method: "GET".to_string(),
            description: "Get agent information and available endpoints".to_string(),
        },
        EndpointInfo {
            path: "/agent/info".to_string(),
            method: "GET".to_string(),
            description: "Get detailed agent information".to_string(),
        },
        EndpointInfo {
            path: "/agent/task".to_string(),
            method: "POST".to_string(),
            description: "Send a purchase order for processing".to_string(),
        },
        EndpointInfo {
            path: "/agent/task/{task_id}".to_string(),
            method: "GET".to_string(),
            description: "Get the status and result of a specific task".to_string(),
        },
        EndpointInfo {
            path: "/agent/task/{task_id}/cancel".to_string(),
            method: "POST".to_string(),
            description: "Cancel a specific task".to_string(),
        },
        EndpointInfo {
            path: "/health".to_string(),
            method: "GET".to_string(),
            description: "Health check endpoint".to_string(),
        },
    ];

    Json(AgentInfoResponse {
        name: card.name.clone(),
        description: card.description.clone(),
        url: card.url.clone(),
        version: card.version.clone(),
        endpoints,
    })
}

/// Send a task to the agent
async fn send_task(
    State(state): State<Arc<AppState>>,
    Json(request): Json<SendTaskRequest>,
) -> Result<Json<TaskResponse>, StatusCode> {
    info!("Received task request from role: {}", request.message.role);

    match state.agent.send_task(request.message).await {
        Ok(task) => {
            let mut csv_output = None;
            let mut detailed_result = None;

            // Extract CSV output and detailed result from the task response
            if let Some(ref message) = task.status.message {
                for part in &message.parts {
                    match part {
                        Part::Text { text } => {
                            csv_output = Some(text.clone());
                        }
                        Part::Data { data } => {
                            detailed_result = Some(data.clone());
                        }
                        _ => {}
                    }
                }
            }

            let status_str = match task.status.state {
                a2a::TaskState::Completed => "completed",
                a2a::TaskState::Failed => "failed",
                a2a::TaskState::Submitted => "submitted",
                a2a::TaskState::Working => "working",
                a2a::TaskState::InputRequired => "input_required",
                a2a::TaskState::Canceled => "cancelled",
            };

            Ok(Json(TaskResponse {
                task_id: task.id,
                status: status_str.to_string(),
                csv_output,
                detailed_result,
                error: None,
            }))
        }
        Err(e) => {
            error!("Failed to process task: {}", e);
            Ok(Json(TaskResponse {
                task_id: "".to_string(),
                status: "error".to_string(),
                csv_output: None,
                detailed_result: None,
                error: Some(e.to_string()),
            }))
        }
    }
}

/// Get a task by ID
async fn get_task(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(task_id): axum::extract::Path<String>,
) -> Result<Json<TaskResponse>, StatusCode> {
    info!("Looking up task: {}", task_id);

    match state.agent.get_task(&task_id).await {
        Ok(task) => {
            let mut csv_output = None;
            let mut detailed_result = None;

            if let Some(ref message) = task.status.message {
                for part in &message.parts {
                    match part {
                        Part::Text { text } => {
                            csv_output = Some(text.clone());
                        }
                        Part::Data { data } => {
                            detailed_result = Some(data.clone());
                        }
                        _ => {}
                    }
                }
            }

            let status_str = match task.status.state {
                a2a::TaskState::Completed => "completed",
                a2a::TaskState::Failed => "failed",
                a2a::TaskState::Submitted => "submitted",
                a2a::TaskState::Working => "working",
                a2a::TaskState::InputRequired => "input_required",
                a2a::TaskState::Canceled => "cancelled",
            };

            Ok(Json(TaskResponse {
                task_id: task.id,
                status: status_str.to_string(),
                csv_output,
                detailed_result,
                error: None,
            }))
        }
        Err(e) => {
            error!("Failed to get task {}: {}", task_id, e);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

/// Cancel a task by ID
async fn cancel_task(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(task_id): axum::extract::Path<String>,
) -> Result<Json<TaskResponse>, StatusCode> {
    info!("Cancelling task: {}", task_id);

    match state.agent.cancel_task(&task_id).await {
        Ok(task) => {
            let status_str = match task.status.state {
                a2a::TaskState::Completed => "completed",
                a2a::TaskState::Failed => "cancelled",
                a2a::TaskState::Submitted => "submitted",
                a2a::TaskState::Working => "working",
                a2a::TaskState::InputRequired => "input_required",
                a2a::TaskState::Canceled => "cancelled",
            };

            Ok(Json(TaskResponse {
                task_id: task.id,
                status: status_str.to_string(),
                csv_output: None,
                detailed_result: None,
                error: None,
            }))
        }
        Err(e) => {
            error!("Failed to cancel task {}: {}", task_id, e);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

/// Health check endpoint
async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "Purchase Order Processing Agent",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}