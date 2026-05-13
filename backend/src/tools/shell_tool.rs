use tokio::process::Command;
use tracing::info;

use crate::tools::router::ToolExecutionResult;

const ALLOWED_COMMANDS: [&str; 5] = ["pwd", "ls", "whoami", "date", "uname"];

pub fn extract_shell_command(message: &str) -> Option<String> {
    let trimmed = message.trim();
    if trimmed.is_empty() {
        return None;
    }

    let lower = trimmed.to_lowercase();

    for prefix in ["run ", "execute ", "shell "] {
        if lower.starts_with(prefix) {
            let raw_cmd = trimmed[prefix.len()..].trim();
            if raw_cmd.is_empty() {
                return None;
            }
            return Some(raw_cmd.to_string());
        }
    }

    if ALLOWED_COMMANDS.contains(&lower.as_str()) {
        return Some(lower);
    }

    None
}

pub async fn handle_run(command_input: &str) -> Result<ToolExecutionResult, String> {
    let normalized = command_input.trim().to_lowercase();

    if !ALLOWED_COMMANDS.contains(&normalized.as_str()) {
        return Err("shell command is not allowed".to_string());
    }

    info!(command = %normalized, "selected shell command");

    let output = Command::new(&normalized)
        .output()
        .await
        .map_err(|err| format!("shell execution failed: {err}"))?;

    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    let response = if !stdout.is_empty() {
        stdout
    } else if !stderr.is_empty() {
        stderr
    } else {
        "(no output)".to_string()
    };

    Ok(ToolExecutionResult {
        tool_used: "shell.run_limited".to_string(),
        response,
        events: vec![
            "Tool selected: shell.run_limited".to_string(),
            format!("Shell command executed: {}", normalized),
        ],
    })
}
