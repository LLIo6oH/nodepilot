use std::path::Path;

use tokio::fs;
use uuid::Uuid;

use crate::runtime::workspace::{ensure_agent_workspace, sanitize_filename};
use crate::tools::router::ToolExecutionResult;

const CREATE_PREFIX: &str = "create a file named ";
const WRITE_PREFIX: &str = "write ";

pub async fn handle_write(
    workspace_root: &Path,
    agent_id: Uuid,
    message: &str,
) -> Result<ToolExecutionResult, String> {
    let workspace = ensure_agent_workspace(workspace_root, agent_id)
        .await
        .map_err(|err| format!("workspace init failed: {err}"))?;

    let (filename, content) = if message.to_lowercase().starts_with(CREATE_PREFIX) {
        parse_create_named(message)?
    } else {
        parse_write_with(message)?
    };

    let file_path = workspace.join(&filename);
    fs::write(&file_path, content.as_bytes())
        .await
        .map_err(|err| format!("file write failed: {err}"))?;

    Ok(ToolExecutionResult {
        tool_used: "file.write".to_string(),
        response: format!("Wrote {} in workspace.", filename),
        events: vec![
            "Tool selected: file.write".to_string(),
            format!("File created: {}", filename),
        ],
    })
}

pub async fn handle_read(
    workspace_root: &Path,
    agent_id: Uuid,
    message: &str,
) -> Result<ToolExecutionResult, String> {
    let workspace = ensure_agent_workspace(workspace_root, agent_id)
        .await
        .map_err(|err| format!("workspace init failed: {err}"))?;

    let name = if message.to_lowercase().starts_with("read ") {
        message[5..].trim()
    } else {
        message[5..].trim()
    };

    let filename = sanitize_filename(name).ok_or_else(|| {
        "invalid filename: only [a-zA-Z0-9._-] and no path separators".to_string()
    })?;
    let file_path = workspace.join(&filename);

    let content = fs::read_to_string(&file_path)
        .await
        .map_err(|err| format!("file read failed: {err}"))?;

    Ok(ToolExecutionResult {
        tool_used: "file.read".to_string(),
        response: content,
        events: vec![
            "Tool selected: file.read".to_string(),
            format!("File read: {}", filename),
        ],
    })
}

fn parse_create_named(message: &str) -> Result<(String, String), String> {
    let raw = message[CREATE_PREFIX.len()..].trim();
    if raw.is_empty() {
        return Err("missing filename for file.write".to_string());
    }

    let lower_raw = raw.to_lowercase();
    let (name_part, content_part) = if let Some(idx) = lower_raw.find(" with ") {
        (&raw[..idx], Some(raw[idx + 6..].trim()))
    } else {
        (raw, None)
    };

    let filename = sanitize_filename(name_part.trim()).ok_or_else(|| {
        "invalid filename: only [a-zA-Z0-9._-] and no path separators".to_string()
    })?;
    let content = content_part
        .filter(|value| !value.is_empty())
        .unwrap_or("Created by NodePilot.")
        .to_string();

    Ok((filename, content))
}

fn parse_write_with(message: &str) -> Result<(String, String), String> {
    let without_prefix = message[WRITE_PREFIX.len()..].trim();
    if without_prefix.is_empty() {
        return Err("missing filename for file.write".to_string());
    }

    let mut parts = without_prefix.splitn(2, " with ");
    let name = parts.next().unwrap_or("").trim();
    let content = parts
        .next()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "Created by NodePilot.".to_string());

    let filename = sanitize_filename(name).ok_or_else(|| {
        "invalid filename: only [a-zA-Z0-9._-] and no path separators".to_string()
    })?;

    Ok((filename, content))
}
