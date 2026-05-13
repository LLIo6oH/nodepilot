use std::time::Duration;

use tokio::{process::Command, time::timeout};
use uuid::Uuid;

use crate::runtime::{RuntimeMode, RuntimeSettings};

const EXEC_TIMEOUT: Duration = Duration::from_secs(8);

pub fn container_name(agent_id: Uuid) -> String {
    format!("nodepilot-runtime-{agent_id}")
}

fn workspace_dir(agent_id: Uuid) -> String {
    format!("/workspaces/{agent_id}")
}

async fn docker_command(args: &[String]) -> Result<(i32, String, String), String> {
    let output = timeout(
        EXEC_TIMEOUT,
        Command::new("docker").args(args.iter().map(String::as_str)).output(),
    )
    .await
    .map_err(|_| "docker command timed out".to_string())
    .and_then(|result| result.map_err(|err| format!("failed to run docker CLI: {err}")))?;

    let code = output.status.code().unwrap_or(-1);
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    Ok((code, stdout, stderr))
}

pub async fn preflight(runtime: &RuntimeSettings) -> Result<(), String> {
    match runtime.mode {
        RuntimeMode::Simulated => return Ok(()),
        RuntimeMode::Docker | RuntimeMode::Auto => {}
    }

    let version_args = vec![
        "version".to_string(),
        "--format".to_string(),
        "{{.Server.Version}}".to_string(),
    ];
    let (code, _stdout, stderr) = docker_command(&version_args).await?;
    if code != 0 {
        return Err(format!("docker unavailable: {stderr}"));
    }

    let inspect_args = vec![
        "image".to_string(),
        "inspect".to_string(),
        runtime.image.clone(),
    ];
    let (inspect_code, _stdout, inspect_stderr) = docker_command(&inspect_args).await?;
    if inspect_code != 0 {
        return Err(format!(
            "runtime image '{}' not available: {}",
            runtime.image, inspect_stderr
        ));
    }

    Ok(())
}

pub async fn start_agent_runtime(runtime: &RuntimeSettings, agent_id: Uuid) -> Result<(), String> {
    let name = container_name(agent_id);
    let workdir = workspace_dir(agent_id);

    let _ = docker_command(&[
        "rm".to_string(),
        "-f".to_string(),
        name.clone(),
    ])
    .await;

    let run_args = vec![
        "run".to_string(),
        "-d".to_string(),
        "--name".to_string(),
        name.clone(),
        "--memory".to_string(),
        "256m".to_string(),
        "--cpus".to_string(),
        "0.5".to_string(),
        "-v".to_string(),
        format!("{}:/workspaces", runtime.workspaces_volume),
        "-w".to_string(),
        workdir,
        runtime.image.clone(),
        "sh".to_string(),
        "-lc".to_string(),
        "sleep infinity".to_string(),
    ];

    let (code, _stdout, stderr) = docker_command(&run_args).await?;
    if code != 0 {
        return Err(format!("failed to start runtime container: {stderr}"));
    }

    let inspect_args = vec![
        "inspect".to_string(),
        "-f".to_string(),
        "{{.State.Running}}".to_string(),
        name,
    ];
    let (running_code, running_stdout, running_stderr) = docker_command(&inspect_args).await?;
    if running_code != 0 || running_stdout != "true" {
        return Err(format!(
            "runtime container is not running: {}",
            if running_stderr.is_empty() {
                running_stdout
            } else {
                running_stderr
            }
        ));
    }

    Ok(())
}

pub async fn exec_in_agent_runtime(agent_id: Uuid, command: &str) -> Result<String, String> {
    let name = container_name(agent_id);
    let args = vec![
        "exec".to_string(),
        name,
        "sh".to_string(),
        "-lc".to_string(),
        command.to_string(),
    ];

    let (code, stdout, stderr) = docker_command(&args).await?;

    if code != 0 {
        let detail = if stderr.is_empty() { stdout } else { stderr };
        return Err(format!("runtime command failed: {detail}"));
    }

    if stdout.is_empty() {
        Ok("(no output)".to_string())
    } else {
        Ok(stdout)
    }
}
