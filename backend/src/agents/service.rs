use chrono::Utc;
use tracing::warn;
use uuid::Uuid;

use crate::{
    AppState,
    agents::{handlers::CreateAgentRequest, lifecycle},
    events::broadcaster::LifecycleEvent,
    models::{Agent, AgentEvent, AgentStatus, Message},
    runtime::docker,
    runtime::workspace::ensure_agent_workspace,
    storage::repositories,
    tools::router,
};

const DEFAULT_RUNTIME_KIND: &str = "local-simulated";

#[derive(Debug, Clone)]
pub struct ChatResult {
    pub agent_id: Uuid,
    pub user_message: String,
    pub agent_response: String,
    pub tool_used: String,
    pub timestamp: String,
}

#[derive(Debug, Clone)]
pub struct ChatError {
    pub status: axum::http::StatusCode,
    pub message: String,
}

impl ChatError {
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self {
            status: axum::http::StatusCode::BAD_REQUEST,
            message: message.into(),
        }
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self {
            status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            message: message.into(),
        }
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self {
            status: axum::http::StatusCode::NOT_FOUND,
            message: message.into(),
        }
    }
}

pub async fn create_agent(
    state: &AppState,
    request: CreateAgentRequest,
) -> Result<Agent, sqlx::Error> {
    let now = Utc::now().to_rfc3339();

    let agent = Agent {
        id: Uuid::new_v4(),
        user_id: Uuid::nil(),
        name: request.name,
        status: AgentStatus::Requested,
        runtime_kind: DEFAULT_RUNTIME_KIND.to_string(),
        created_at: now.clone(),
        updated_at: now,
    };

    repositories::create_agent(&state.db, &agent).await?;
    Ok(agent)
}

pub async fn list_agents(state: &AppState) -> Result<Vec<Agent>, sqlx::Error> {
    repositories::list_agents(&state.db).await
}

pub async fn agent_exists(state: &AppState, agent_id: Uuid) -> Result<bool, sqlx::Error> {
    Ok(repositories::get_agent(&state.db, agent_id)
        .await?
        .is_some())
}

pub async fn start_provisioning(
    state: AppState,
    agent_id: Uuid,
) -> Result<(), axum::http::StatusCode> {
    if !agent_exists(&state, agent_id)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?
    {
        return Err(axum::http::StatusCode::NOT_FOUND);
    }

    ensure_agent_workspace(&state.workspace_root, agent_id)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Err(err) = docker::preflight(&state.runtime).await {
        if matches!(state.runtime.mode, crate::runtime::RuntimeMode::Auto) {
            warn!(agent_id = %agent_id, error = %err, "docker preflight failed in auto mode, continuing with simulated runtime");
        } else {
            return Err(axum::http::StatusCode::SERVICE_UNAVAILABLE);
        }
    }

    tokio::spawn(async move {
        let _ = lifecycle::run_provisioning(state, agent_id).await;
    });

    Ok(())
}

pub async fn chat_with_agent(
    state: &AppState,
    agent_id: Uuid,
    user_message: String,
) -> Result<ChatResult, ChatError> {
    if !agent_exists(state, agent_id)
        .await
        .map_err(|_| ChatError::internal("failed to check agent existence"))?
    {
        return Err(ChatError::not_found("agent not found"));
    }

    let trimmed_message = user_message.trim();
    if trimmed_message.is_empty() {
        return Err(ChatError::bad_request("message is required"));
    }

    let now = Utc::now().to_rfc3339();
    let user_row = Message {
        id: Uuid::new_v4(),
        agent_id,
        role: "user".to_string(),
        content: trimmed_message.to_string(),
        created_at: now.clone(),
    };
    repositories::create_message(&state.db, &user_row)
        .await
        .map_err(|_| ChatError::internal("failed to persist user message"))?;

    let tool_result = router::route_and_execute(state, agent_id, trimmed_message)
        .await
        .map_err(ChatError::bad_request)?
        .ok_or_else(|| ChatError::bad_request("unsupported tool request"))?;

    let assistant_row = Message {
        id: Uuid::new_v4(),
        agent_id,
        role: "assistant".to_string(),
        content: tool_result.response.clone(),
        created_at: now.clone(),
    };
    repositories::create_message(&state.db, &assistant_row)
        .await
        .map_err(|_| ChatError::internal("failed to persist assistant response"))?;

    for event_text in &tool_result.events {
        emit_event(state, agent_id, "tool_event", event_text)
            .await
            .map_err(|_| ChatError::internal("failed to persist/broadcast tool event"))?;
    }

    Ok(ChatResult {
        agent_id,
        user_message: trimmed_message.to_string(),
        agent_response: tool_result.response,
        tool_used: tool_result.tool_used,
        timestamp: now,
    })
}

pub async fn emit_event(
    state: &AppState,
    agent_id: Uuid,
    event_type: &str,
    message: &str,
) -> Result<(), sqlx::Error> {
    let now = Utc::now().to_rfc3339();
    let event = AgentEvent {
        id: Uuid::new_v4(),
        agent_id,
        event_type: event_type.to_string(),
        message: message.to_string(),
        created_at: now,
    };

    repositories::create_agent_event(&state.db, &event).await?;

    state
        .events
        .publish(LifecycleEvent::new(agent_id, event_type, message))
        .await;

    Ok(())
}
