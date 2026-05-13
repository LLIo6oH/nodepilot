use tracing::{error, info};
use uuid::Uuid;

use crate::AppState;
use crate::tools::{email_tool, file_tool, shell_tool};

#[derive(Debug, Clone)]
pub struct ToolExecutionResult {
    pub tool_used: String,
    pub response: String,
    pub events: Vec<String>,
}

pub async fn route_and_execute(
    state: &AppState,
    agent_id: Uuid,
    message: &str,
) -> Result<Option<ToolExecutionResult>, String> {
    let trimmed = message.trim();
    let lower = trimmed.to_lowercase();

    if lower.starts_with("create a file named ")
        || (lower.starts_with("write ") && lower.contains(" with "))
    {
        info!(agent_id = %agent_id, tool = "file.write", "selected tool");
        info!(agent_id = %agent_id, tool = "file.write", "tool execution start");
        let result = file_tool::handle_write(&state.workspace_root, agent_id, trimmed).await;
        match result {
            Ok(value) => {
                info!(agent_id = %agent_id, tool = "file.write", "tool execution success");
                return Ok(Some(value));
            }
            Err(err) => {
                error!(agent_id = %agent_id, tool = "file.write", error = %err, "tool execution failure");
                return Err(err);
            }
        }
    }

    if lower.starts_with("read ") || lower.starts_with("show ") {
        info!(agent_id = %agent_id, tool = "file.read", "selected tool");
        info!(agent_id = %agent_id, tool = "file.read", "tool execution start");
        let result = file_tool::handle_read(&state.workspace_root, agent_id, trimmed).await;
        match result {
            Ok(value) => {
                info!(agent_id = %agent_id, tool = "file.read", "tool execution success");
                return Ok(Some(value));
            }
            Err(err) => {
                error!(agent_id = %agent_id, tool = "file.read", error = %err, "tool execution failure");
                return Err(err);
            }
        }
    }

    if lower.contains("email") {
        info!(agent_id = %agent_id, tool = "email.compose", "selected tool");
        info!(agent_id = %agent_id, tool = "email.compose", "tool execution start");
        let result = email_tool::handle_compose(&state.workspace_root, agent_id, trimmed).await;
        match result {
            Ok(value) => {
                info!(agent_id = %agent_id, tool = "email.compose", "tool execution success");
                return Ok(Some(value));
            }
            Err(err) => {
                error!(agent_id = %agent_id, tool = "email.compose", error = %err, "tool execution failure");
                return Err(err);
            }
        }
    }

    if let Some(command) = shell_tool::extract_shell_command(trimmed) {
        info!(agent_id = %agent_id, tool = "shell.run_limited", command = %command, "selected tool");
        info!(agent_id = %agent_id, tool = "shell.run_limited", command = %command, "tool execution start");
        let result = shell_tool::handle_run(&state.runtime, agent_id, &command).await;
        match result {
            Ok(value) => {
                info!(agent_id = %agent_id, tool = "shell.run_limited", command = %command, "tool execution success");
                return Ok(Some(value));
            }
            Err(err) => {
                error!(agent_id = %agent_id, tool = "shell.run_limited", command = %command, error = %err, "tool execution failure");
                return Err(err);
            }
        }
    }

    Ok(None)
}
