use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub status: AgentStatus,
    pub runtime_kind: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Message {
    pub id: Uuid,
    pub agent_id: Uuid,
    pub role: String,
    pub content: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentEvent {
    pub id: Uuid,
    pub agent_id: Uuid,
    pub event_type: String,
    pub message: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentStatus {
    Requested,
    Launching,
    Booting,
    Configuring,
    Ready,
}

impl AgentStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Requested => "requested",
            Self::Launching => "launching",
            Self::Booting => "booting",
            Self::Configuring => "configuring",
            Self::Ready => "ready",
        }
    }

    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            "requested" => Some(Self::Requested),
            "launching" => Some(Self::Launching),
            "booting" => Some(Self::Booting),
            "configuring" => Some(Self::Configuring),
            "ready" => Some(Self::Ready),
            _ => None,
        }
    }
}
