use std::path::Path;

use chrono::Utc;
use tokio::fs;
use uuid::Uuid;

use crate::runtime::workspace::ensure_agent_workspace;
use crate::tools::router::ToolExecutionResult;

pub async fn handle_compose(
    workspace_root: &Path,
    agent_id: Uuid,
    prompt: &str,
) -> Result<ToolExecutionResult, String> {
    let workspace = ensure_agent_workspace(workspace_root, agent_id)
        .await
        .map_err(|err| format!("workspace init failed: {err}"))?;

    let drafts_dir = workspace.join("drafts");
    fs::create_dir_all(&drafts_dir)
        .await
        .map_err(|err| format!("draft directory init failed: {err}"))?;

    let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
    let filename = format!("email_{}.md", timestamp);
    let draft_path = drafts_dir.join(&filename);

    let draft = format!(
        "# Email Draft\n\nSubject: Follow-up\n\nHello,\n\n{}\n\nBest,\nNodePilot Agent\n",
        prompt
    );

    fs::write(&draft_path, draft.as_bytes())
        .await
        .map_err(|err| format!("draft write failed: {err}"))?;

    Ok(ToolExecutionResult {
        tool_used: "email.compose".to_string(),
        response: draft,
        events: vec![
            "Tool selected: email.compose".to_string(),
            format!("Draft created: drafts/{}", filename),
        ],
    })
}
