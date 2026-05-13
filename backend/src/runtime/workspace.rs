use std::path::{Path, PathBuf};

use tokio::fs;
use uuid::Uuid;

pub async fn ensure_agent_workspace(
    base_dir: &Path,
    agent_id: Uuid,
) -> Result<PathBuf, std::io::Error> {
    let dir = base_dir.join(agent_id.to_string());
    fs::create_dir_all(&dir).await?;
    Ok(dir)
}

pub fn sanitize_filename(raw: &str) -> Option<String> {
    let candidate = raw
        .trim()
        .trim_matches(|c: char| c == '"' || c == '\'' || c == '.' || c == ',')
        .to_string();

    if candidate.is_empty()
        || candidate.contains("..")
        || candidate.contains('/')
        || candidate.contains('\\')
    {
        return None;
    }

    if candidate
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == '.')
    {
        Some(candidate)
    } else {
        None
    }
}
