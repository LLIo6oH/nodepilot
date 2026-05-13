use axum::{Json, extract::State, http::StatusCode};
use chrono::Utc;
use serde::Serialize;

use crate::{AppState, storage::repositories};

#[derive(Serialize)]
pub struct HealthResponse {
    status: &'static str,
    database: &'static str,
    timestamp: String,
}

pub async fn health_check(
    State(state): State<AppState>,
) -> Result<Json<HealthResponse>, StatusCode> {
    repositories::ping(&state.db)
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;

    Ok(Json(HealthResponse {
        status: "ok",
        database: "ok",
        timestamp: Utc::now().to_rfc3339(),
    }))
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use sqlx::SqlitePool;

    use crate::{
        events::broadcaster::EventBroadcaster,
        runtime::{RuntimeMode, RuntimeSettings},
        storage,
    };

    #[tokio::test]
    async fn health_check_reports_database_ok() {
        let pool = SqlitePool::connect("sqlite::memory:")
            .await
            .expect("in-memory sqlite should connect");

        storage::initialize_schema(&pool)
            .await
            .expect("schema init should succeed");

        let state = AppState {
            service_name: "nodepilot-backend".to_string(),
            db: pool,
            events: EventBroadcaster::new(),
            workspace_root: PathBuf::from("/tmp/nodepilot-test-workspaces"),
            runtime: RuntimeSettings {
                mode: RuntimeMode::Simulated,
                image: "nodepilot-runtime:latest".to_string(),
                workspaces_volume: "nodepilot-workspaces".to_string(),
            },
        };

        let Json(body) = health_check(State(state))
            .await
            .expect("health check should pass");

        assert_eq!(body.status, "ok");
        assert_eq!(body.database, "ok");
        assert!(!body.timestamp.is_empty());
    }
}
