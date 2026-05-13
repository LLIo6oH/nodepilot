pub mod workspace;
pub mod docker;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RuntimeMode {
    Docker,
    Simulated,
    Auto,
}

impl RuntimeMode {
    pub fn from_env() -> Self {
        match std::env::var("RUNTIME_MODE")
            .unwrap_or_else(|_| "simulated".to_string())
            .to_lowercase()
            .as_str()
        {
            "docker" => Self::Docker,
            "auto" => Self::Auto,
            _ => Self::Simulated,
        }
    }
}

#[derive(Clone, Debug)]
pub struct RuntimeSettings {
    pub mode: RuntimeMode,
    pub image: String,
    pub workspaces_volume: String,
}

impl RuntimeSettings {
    pub fn from_env() -> Self {
        Self {
            mode: RuntimeMode::from_env(),
            image: std::env::var("RUNTIME_IMAGE")
                .unwrap_or_else(|_| "nodepilot-runtime:latest".to_string()),
            workspaces_volume: std::env::var("RUNTIME_WORKSPACES_VOLUME")
                .unwrap_or_else(|_| "nodepilot-workspaces".to_string()),
        }
    }
}
