use std::time::Duration;

use chrono::Utc;
use uuid::Uuid;

use crate::{
    AppState,
    events::broadcaster::LifecycleEvent,
    models::{AgentEvent, AgentStatus},
    storage::repositories,
};

const STAGE_DELAY_MS: u64 = 700;

pub async fn run_provisioning(state: AppState, agent_id: Uuid) -> Result<(), sqlx::Error> {
    let stages = [
        (AgentStatus::Launching, "Allocating runtime..."),
        (AgentStatus::Booting, "Preparing workspace..."),
        (AgentStatus::Configuring, "Installing runtime..."),
        (AgentStatus::Configuring, "Connecting gateway..."),
        (AgentStatus::Ready, "Agent ready."),
    ];

    for (status, message) in stages {
        repositories::update_agent_status(&state.db, agent_id, status.clone()).await?;

        let now = Utc::now().to_rfc3339();
        let event = AgentEvent {
            id: Uuid::new_v4(),
            agent_id,
            event_type: status.as_str().to_string(),
            message: message.to_string(),
            created_at: now.clone(),
        };

        repositories::create_agent_event(&state.db, &event).await?;

        state
            .events
            .publish(LifecycleEvent::new(agent_id, status.as_str(), message))
            .await;

        tokio::time::sleep(Duration::from_millis(STAGE_DELAY_MS)).await;
    }

    Ok(())
}
