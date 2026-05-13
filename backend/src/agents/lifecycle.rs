use std::time::Duration;

use chrono::Utc;
use tracing::error;
use uuid::Uuid;

use crate::{
    AppState,
    events::broadcaster::LifecycleEvent,
    models::{AgentEvent, AgentStatus},
    runtime::{RuntimeMode, docker, workspace::ensure_agent_workspace},
    storage::repositories,
};

const STAGE_DELAY_MS: u64 = 700;

async fn emit_stage(
    state: &AppState,
    agent_id: Uuid,
    status: AgentStatus,
    message: &str,
) -> Result<(), sqlx::Error> {
    repositories::update_agent_status(&state.db, agent_id, status.clone()).await?;
    let now = Utc::now().to_rfc3339();
    let event = AgentEvent {
        id: Uuid::new_v4(),
        agent_id,
        event_type: status.as_str().to_string(),
        message: message.to_string(),
        created_at: now,
    };
    repositories::create_agent_event(&state.db, &event).await?;
    state
        .events
        .publish(LifecycleEvent::new(agent_id, status.as_str(), message))
        .await;
    Ok(())
}

pub async fn run_provisioning(state: AppState, agent_id: Uuid) -> Result<(), sqlx::Error> {
    emit_stage(&state, agent_id, AgentStatus::Launching, "Preparing runtime session").await?;
    tokio::time::sleep(Duration::from_millis(STAGE_DELAY_MS)).await;

    if ensure_agent_workspace(&state.workspace_root, agent_id)
        .await
        .is_err()
    {
        emit_stage(
            &state,
            agent_id,
            AgentStatus::Configuring,
            "Provisioning failed: unable to prepare workspace",
        )
        .await?;
        return Ok(());
    }

    emit_stage(&state, agent_id, AgentStatus::Booting, "Creating workspace").await?;
    tokio::time::sleep(Duration::from_millis(STAGE_DELAY_MS)).await;

    let runtime_start = match state.runtime.mode {
        RuntimeMode::Docker => docker::start_agent_runtime(&state.runtime, agent_id).await,
        RuntimeMode::Auto => match docker::start_agent_runtime(&state.runtime, agent_id).await {
            Ok(()) => Ok(()),
            Err(err) => {
                error!(agent_id = %agent_id, error = %err, "docker runtime failed in auto mode; falling back to simulated runtime");
                Ok(())
            }
        },
        RuntimeMode::Simulated => Ok(()),
    };

    if let Err(err) = runtime_start {
        error!(agent_id = %agent_id, error = %err, "runtime startup failed");
        let detail = format!("Provisioning failed: {err}");
        emit_stage(&state, agent_id, AgentStatus::Configuring, &detail).await?;
        return Ok(());
    }

    emit_stage(
        &state,
        agent_id,
        AgentStatus::Configuring,
        "Starting Docker runtime",
    )
    .await?;
    tokio::time::sleep(Duration::from_millis(STAGE_DELAY_MS)).await;

    emit_stage(
        &state,
        agent_id,
        AgentStatus::Configuring,
        "Mounting workspace",
    )
    .await?;
    tokio::time::sleep(Duration::from_millis(STAGE_DELAY_MS)).await;

    emit_stage(
        &state,
        agent_id,
        AgentStatus::Configuring,
        "Registering tools",
    )
    .await?;
    tokio::time::sleep(Duration::from_millis(STAGE_DELAY_MS)).await;

    emit_stage(&state, agent_id, AgentStatus::Ready, "Atlas is online").await?;
    Ok(())
}
