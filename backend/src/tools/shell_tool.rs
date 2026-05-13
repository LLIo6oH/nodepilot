use tokio::{process::Command, time::timeout};
use tracing::info;
use uuid::Uuid;

use crate::runtime::{RuntimeMode, RuntimeSettings, docker};
use crate::tools::router::ToolExecutionResult;

const ALLOWED_PREFIXES: [&str; 7] = ["pwd", "ls", "cat ", "echo ", "whoami", "date", "uname"];
const BLOCKED_FRAGMENTS: [&str; 11] = [
    "rm -rf /",
    "shutdown",
    "reboot",
    "docker",
    "169.254.169.254",
    "metadata.google.internal",
    ";",
    "&&",
    "||",
    "$(",
    "`",
];

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

    if ALLOWED_PREFIXES.iter().any(|prefix| {
        lower == *prefix || (prefix.ends_with(' ') && lower.starts_with(prefix))
    }) {
        return Some(lower);
    }

    None
}

fn is_allowed_shell_command(command_input: &str) -> bool {
    let normalized = command_input.trim().to_lowercase();
    if normalized.is_empty() || normalized.contains('\n') {
        return false;
    }

    if BLOCKED_FRAGMENTS
        .iter()
        .any(|fragment| normalized.contains(fragment))
    {
        return false;
    }

    ALLOWED_PREFIXES.iter().any(|prefix| {
        normalized == *prefix || (prefix.ends_with(' ') && normalized.starts_with(prefix))
    })
}

async fn run_local(command: &str) -> Result<String, String> {
    let output = timeout(
        std::time::Duration::from_secs(5),
        Command::new("sh").arg("-lc").arg(command).output(),
    )
    .await
    .map_err(|_| "shell command timed out".to_string())
    .and_then(|result| result.map_err(|err| format!("shell execution failed: {err}")))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(if stderr.is_empty() {
            "shell command failed".to_string()
        } else {
            stderr
        });
    }

    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if stdout.is_empty() {
        Ok("(no output)".to_string())
    } else {
        Ok(stdout)
    }
}

pub async fn handle_run(
    runtime: &RuntimeSettings,
    agent_id: Uuid,
    command_input: &str,
) -> Result<ToolExecutionResult, String> {
    let normalized = command_input.trim().to_lowercase();

    if !is_allowed_shell_command(&normalized) {
        return Err("shell command is blocked by safety policy".to_string());
    }

    info!(command = %normalized, "selected shell command");

    let response = match runtime.mode {
        RuntimeMode::Docker => docker::exec_in_agent_runtime(agent_id, &normalized).await?,
        RuntimeMode::Auto => match docker::exec_in_agent_runtime(agent_id, &normalized).await {
            Ok(output) => output,
            Err(_) => run_local(&normalized).await?,
        },
        RuntimeMode::Simulated => run_local(&normalized).await?,
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
