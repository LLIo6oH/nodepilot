use std::{collections::HashMap, sync::Arc};

use chrono::Utc;
use serde::Serialize;
use tokio::sync::{RwLock, broadcast};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize)]
pub struct LifecycleEvent {
    pub agent_id: Uuid,
    pub status: String,
    pub message: String,
    pub timestamp: String,
}

impl LifecycleEvent {
    pub fn new(agent_id: Uuid, status: &str, message: &str) -> Self {
        Self {
            agent_id,
            status: status.to_string(),
            message: message.to_string(),
            timestamp: Utc::now().to_rfc3339(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct EventBroadcaster {
    channels: Arc<RwLock<HashMap<Uuid, broadcast::Sender<LifecycleEvent>>>>,
}

impl EventBroadcaster {
    pub fn new() -> Self {
        Self {
            channels: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn subscribe(&self, agent_id: Uuid) -> broadcast::Receiver<LifecycleEvent> {
        let mut channels = self.channels.write().await;
        let sender = channels
            .entry(agent_id)
            .or_insert_with(|| broadcast::channel(128).0);
        sender.subscribe()
    }

    pub async fn publish(&self, event: LifecycleEvent) {
        let mut channels = self.channels.write().await;
        let sender = channels
            .entry(event.agent_id)
            .or_insert_with(|| broadcast::channel(128).0);
        let _ = sender.send(event);
    }
}
