use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{AppState, agents::service, events::sse};

#[derive(Debug, Deserialize)]
pub struct CreateAgentRequest {
    pub name: String,
}

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
    service::start_provisioning(state, agent_id)
        .await
        .map_err(|status| status)?;

    Ok(StatusCode::ACCEPTED)
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
    use std::time::Duration;

    use axum::Json;
    use axum::extract::{Path, State};
    use axum::http::StatusCode;

    use crate::AppState;
    use crate::events::broadcaster::EventBroadcaster;
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
}
