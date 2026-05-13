use axum::{
    Json,
    extract::{Path, State, rejection::JsonRejection},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use tracing::{error, info};
use uuid::Uuid;

use crate::{AppState, agents::service, events::sse};

#[derive(Debug, Deserialize)]
pub struct CreateAgentRequest {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    pub message: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ChatResponse {
    pub agent_id: Uuid,
    pub user_message: String,
    pub agent_response: String,
    pub tool_used: String,
    pub timestamp: String,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

type ApiError = (StatusCode, Json<ErrorResponse>);

pub async fn create_agent(
    State(state): State<AppState>,
    Json(request): Json<CreateAgentRequest>,
) -> Result<Json<crate::models::Agent>, StatusCode> {
    let agent = service::create_agent(&state, request)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(agent))
}

pub async fn list_agents(
    State(state): State<AppState>,
) -> Result<Json<Vec<crate::models::Agent>>, StatusCode> {
    let agents = service::list_agents(&state)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(agents))
}

pub async fn provision_agent(
    State(state): State<AppState>,
    Path(agent_id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    service::start_provisioning(state, agent_id).await?;
    Ok(StatusCode::ACCEPTED)
}

pub async fn chat_with_agent(
    State(state): State<AppState>,
    Path(agent_id): Path<Uuid>,
    payload: Result<Json<ChatRequest>, JsonRejection>,
) -> Result<Json<ChatResponse>, ApiError> {
    let request = payload.map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "invalid JSON body, expected {\"message\":\"...\"}".to_string(),
            }),
        )
    })?;

    let message = request.0.message.ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "message is required".to_string(),
            }),
        )
    })?;

    info!(agent_id = %agent_id, message = %message, "incoming chat request");

    let result = service::chat_with_agent(&state, agent_id, message)
        .await
        .map_err(|err| {
            error!(agent_id = %agent_id, status = %err.status, error = %err.message, "chat request failed");
            (err.status, Json(ErrorResponse { error: err.message }))
        })?;

    Ok(Json(ChatResponse {
        agent_id: result.agent_id,
        user_message: result.user_message,
        agent_response: result.agent_response,
        tool_used: result.tool_used,
        timestamp: result.timestamp,
    }))
}

pub async fn stream_agent_events(
    State(state): State<AppState>,
    Path(agent_id): Path<Uuid>,
) -> Result<
    axum::response::sse::Sse<
        impl tokio_stream::Stream<Item = Result<axum::response::sse::Event, std::convert::Infallible>>,
    >,
    StatusCode,
> {
    if service::agent_exists(&state, agent_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        let receiver = state.events.subscribe(agent_id).await;
        Ok(sse::stream(receiver))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::time::Duration;

    use axum::Json;
    use axum::extract::{Path, State};
    use axum::http::StatusCode;
    use uuid::Uuid;

    use crate::AppState;
    use crate::events::broadcaster::EventBroadcaster;
    use crate::runtime::{RuntimeMode, RuntimeSettings};
    use crate::storage;

    use super::*;

    async fn test_state() -> AppState {
        let db = sqlx::SqlitePool::connect("sqlite::memory:")
            .await
            .expect("in-memory sqlite should connect");
        storage::initialize_schema(&db)
            .await
            .expect("schema should initialize");

        AppState {
            service_name: "test-backend".to_string(),
            db,
            events: EventBroadcaster::new(),
            workspace_root: PathBuf::from(format!(
                "/tmp/nodepilot-test-workspaces-{}",
                Uuid::new_v4()
            )),
            runtime: RuntimeSettings {
                mode: RuntimeMode::Simulated,
                image: "nodepilot-runtime:latest".to_string(),
                workspaces_volume: "nodepilot-workspaces".to_string(),
            },
        }
    }

    #[tokio::test]
    async fn create_and_list_agents_work() {
        let state = test_state().await;
        let create = CreateAgentRequest {
            name: "Atlas".to_string(),
        };

        let Json(created) = create_agent(State(state.clone()), Json(create))
            .await
            .expect("create should succeed");

        let Json(agents) = list_agents(State(state))
            .await
            .expect("list should succeed");

        assert_eq!(agents.len(), 1);
        assert_eq!(agents[0].id, created.id);
        assert_eq!(agents[0].name, "Atlas");
    }

    #[tokio::test]
    async fn provision_publishes_events() {
        let state = test_state().await;
        let create = CreateAgentRequest {
            name: "Atlas".to_string(),
        };

        let Json(created) = create_agent(State(state.clone()), Json(create))
            .await
            .expect("create should succeed");

        let _ = stream_agent_events(State(state.clone()), Path(created.id))
            .await
            .expect("sse stream should open");
        let mut receiver = state.events.subscribe(created.id).await;

        let response = provision_agent(State(state), Path(created.id))
            .await
            .expect("provision should be accepted");
        assert_eq!(response, StatusCode::ACCEPTED);

        let first = tokio::time::timeout(Duration::from_secs(2), receiver.recv())
            .await
            .expect("event should arrive")
            .expect("event payload should be valid");

        assert_eq!(first.agent_id, created.id);
    }

    #[tokio::test]
    async fn chat_tool_flows_work() {
        let state = test_state().await;
        let Json(created) = create_agent(
            State(state.clone()),
            Json(CreateAgentRequest {
                name: "Atlas".to_string(),
            }),
        )
        .await
        .expect("create should succeed");

        let Json(write_response) = chat_with_agent(
            State(state.clone()),
            Path(created.id),
            Ok(Json(ChatRequest {
                message: Some("create a file named roadmap.txt".to_string()),
            })),
        )
        .await
        .expect("file write chat should succeed");
        assert_eq!(write_response.tool_used, "file.write");

        let Json(read_response) = chat_with_agent(
            State(state.clone()),
            Path(created.id),
            Ok(Json(ChatRequest {
                message: Some("read roadmap.txt".to_string()),
            })),
        )
        .await
        .expect("file read chat should succeed");
        assert_eq!(read_response.tool_used, "file.read");

        let Json(email_response) = chat_with_agent(
            State(state.clone()),
            Path(created.id),
            Ok(Json(ChatRequest {
                message: Some("write an interview follow-up email".to_string()),
            })),
        )
        .await
        .expect("email compose chat should succeed");
        assert_eq!(email_response.tool_used, "email.compose");

        let Json(shell_response) = chat_with_agent(
            State(state),
            Path(created.id),
            Ok(Json(ChatRequest {
                message: Some("pwd".to_string()),
            })),
        )
        .await
        .expect("shell chat should succeed");
        assert_eq!(shell_response.tool_used, "shell.run_limited");
    }
}
