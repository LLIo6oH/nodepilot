use chrono::Utc;
use uuid::Uuid;

use crate::{
    AppState,
    agents::{handlers::CreateAgentRequest, lifecycle},
    models::{Agent, AgentStatus},
    storage::repositories,
};

const DEFAULT_RUNTIME_KIND: &str = "local-simulated";

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

    tokio::spawn(async move {
        let _ = lifecycle::run_provisioning(state, agent_id).await;
    });

    Ok(())
}
