use super::claude::run_claude_task;
use super::codex::run_codex_task;
use super::errors::RunTaskError;
use super::types::{RunTaskOutput, RunTaskParams, RunTaskRequest};
use super::workspace::{prepare_workspace, remap_workspace_dir, write_placeholder_reply};

pub fn run_task(params: &RunTaskParams) -> Result<RunTaskOutput, RunTaskError> {
    let workspace_dir = remap_workspace_dir(&params.workspace_dir)?;
    let runner = normalize_runner(&params.runner);
    let request = RunTaskRequest {
        workspace_dir: &workspace_dir,
        input_email_dir: &params.input_email_dir,
        input_attachments_dir: &params.input_attachments_dir,
        memory_dir: &params.memory_dir,
        reference_dir: &params.reference_dir,
        model_name: params.model_name.as_str(),
        reply_to: &params.reply_to,
        channel: &params.channel,
        google_access_token: params.google_access_token.as_deref(),
    };

    let (reply_html_path, reply_attachments_dir) = prepare_workspace(&request)?;

    if params.codex_disabled {
        if !params.reply_to.is_empty() {
            write_placeholder_reply(&reply_html_path)?;
        }
        return Ok(RunTaskOutput {
            reply_html_path,
            reply_attachments_dir,
            codex_output: "codex disabled".to_string(),
            scheduled_tasks: Vec::new(),
            scheduled_tasks_error: None,
            scheduler_actions: Vec::new(),
            scheduler_actions_error: None,
        });
    }

    match runner.as_str() {
        "claude" => run_claude_task(request, &runner, reply_html_path, reply_attachments_dir),
        _ => run_codex_task(request, &runner, reply_html_path, reply_attachments_dir),
    }
}

fn normalize_runner(raw: &str) -> String {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        "codex".to_string()
    } else {
        trimmed.to_ascii_lowercase()
    }
}
