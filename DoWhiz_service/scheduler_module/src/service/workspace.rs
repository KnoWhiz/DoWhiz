use std::io;
use std::path::{Path, PathBuf};

use chrono::Utc;
use serde::Serialize;
use tracing::error;

use crate::domain::workspace_blueprint::StartupWorkspaceBlueprint;
use crate::employee_config::EmployeeProfile;

use super::html::{strip_html_tags, truncate_preview};
use super::startup_workspace::{
    bootstrap_workspace_plan, build_workspace_home_snapshot, StartupWorkspaceBootstrapPlan,
};
use super::BoxError;

fn thread_workspace_name(thread_key: &str) -> String {
    let hash = format!("{:x}", md5::compute(thread_key.as_bytes()));
    format!("thread_{}", hash)
}

pub(super) fn copy_skills_directory(src: &Path, dest: &Path) -> std::io::Result<()> {
    if !src.exists() {
        return Ok(());
    }

    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let skill_src = entry.path();
        let skill_dest = dest.join(entry.file_name());

        if skill_src.is_dir() {
            copy_dir_recursive(&skill_src, &skill_dest)?;
        }
    }
    Ok(())
}

pub fn copy_dir_recursive(src: &Path, dest: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(dest)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dest_path = dest.join(entry.file_name());

        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dest_path)?;
        } else {
            copy_file_with_fallback(&src_path, &dest_path)?;
        }
    }
    Ok(())
}

pub(super) fn ensure_workspace_employee_files(
    workspace: &Path,
    employee: &EmployeeProfile,
) -> std::io::Result<()> {
    if let Some(path) = employee.agents_path.as_ref() {
        if path.exists() {
            copy_file_with_fallback(path, &workspace.join("AGENTS.md"))?;
        }
    }
    if let Some(path) = employee.claude_path.as_ref() {
        if path.exists() {
            copy_file_with_fallback(path, &workspace.join("CLAUDE.md"))?;
        }
    }
    if let Some(path) = employee.soul_path.as_ref() {
        if path.exists() {
            copy_file_with_fallback(path, &workspace.join("SOUL.md"))?;
        }
    }
    Ok(())
}

fn copy_file_with_fallback(src: &Path, dest: &Path) -> std::io::Result<()> {
    match std::fs::copy(src, dest) {
        Ok(_) => Ok(()),
        Err(err)
            if err.kind() == std::io::ErrorKind::PermissionDenied
                || err.raw_os_error() == Some(1) =>
        {
            // Some CIFS/Azure Files mounts reject the kernel fast-copy syscall.
            // Fall back to a stream copy that is broadly supported.
            let mut input = std::fs::File::open(src)?;
            let mut output = std::fs::File::create(dest)?;
            std::io::copy(&mut input, &mut output)?;
            Ok(())
        }
        Err(err) => Err(err),
    }
}

pub(crate) fn ensure_thread_workspace(
    user_paths: &crate::user_store::UserPaths,
    user_id: &str,
    thread_key: &str,
    employee: &EmployeeProfile,
    skills_source_dir: Option<&Path>,
) -> Result<PathBuf, BoxError> {
    std::fs::create_dir_all(&user_paths.workspaces_root).map_err(|err| {
        io::Error::other(format!(
            "create_dir_all workspaces_root failed path={} error={}",
            user_paths.workspaces_root.display(),
            err
        ))
    })?;

    let workspace_name = thread_workspace_name(thread_key);
    let workspace = user_paths.workspaces_root.join(workspace_name);
    let is_new = !workspace.exists();
    if is_new {
        std::fs::create_dir_all(&workspace).map_err(|err| {
            io::Error::other(format!(
                "create_dir_all workspace failed path={} error={}",
                workspace.display(),
                err
            ))
        })?;
    }

    let incoming_email = workspace.join("incoming_email");
    let incoming_attachments = workspace.join("incoming_attachments");
    let memory = workspace.join("memory");
    let references = workspace.join("references");

    std::fs::create_dir_all(&incoming_email).map_err(|err| {
        io::Error::other(format!(
            "create_dir_all incoming_email failed path={} error={}",
            incoming_email.display(),
            err
        ))
    })?;
    std::fs::create_dir_all(&incoming_attachments).map_err(|err| {
        io::Error::other(format!(
            "create_dir_all incoming_attachments failed path={} error={}",
            incoming_attachments.display(),
            err
        ))
    })?;
    std::fs::create_dir_all(&memory).map_err(|err| {
        io::Error::other(format!(
            "create_dir_all memory failed path={} error={}",
            memory.display(),
            err
        ))
    })?;
    std::fs::create_dir_all(&references).map_err(|err| {
        io::Error::other(format!(
            "create_dir_all references failed path={} error={}",
            references.display(),
            err
        ))
    })?;

    if is_new || !references.join("past_emails").exists() {
        if let Err(err) = crate::past_emails::hydrate_past_emails(
            &user_paths.mail_root,
            &references,
            user_id,
            None,
        ) {
            error!("failed to hydrate past_emails: {}", err);
        }
    }

    ensure_workspace_employee_files(&workspace, employee).map_err(|err| {
        io::Error::other(format!(
            "ensure_workspace_employee_files failed workspace={} error={}",
            workspace.display(),
            err
        ))
    })?;

    // Copy skills to workspace for Codex/Claude runners.
    let agents_skills_dir = workspace.join(".agents").join("skills");
    if let Some(skills_src) = skills_source_dir {
        if let Err(err) = copy_skills_directory(skills_src, &agents_skills_dir) {
            error!("failed to copy base skills to workspace: {}", err);
        }
    }
    if let Some(employee_skills) = employee.skills_dir.as_deref() {
        let should_copy = skills_source_dir
            .map(|base| base != employee_skills)
            .unwrap_or(true);
        if should_copy {
            if let Err(err) = copy_skills_directory(employee_skills, &agents_skills_dir) {
                error!("failed to copy employee skills to workspace: {}", err);
            }
        }
    }

    Ok(workspace)
}

/// Bootstrap a startup workspace plan and persist it as reviewable workspace artifacts.
pub fn bootstrap_startup_workspace_files(
    workspace: &Path,
    blueprint: StartupWorkspaceBlueprint,
) -> Result<StartupWorkspaceBootstrapPlan, BoxError> {
    let plan = bootstrap_workspace_plan(blueprint)?;
    persist_startup_workspace_files(workspace, &plan)?;
    Ok(plan)
}

pub fn persist_startup_workspace_files(
    workspace: &Path,
    plan: &StartupWorkspaceBootstrapPlan,
) -> Result<PathBuf, BoxError> {
    std::fs::create_dir_all(workspace)?;

    let bootstrap_root = workspace.join("startup_workspace");
    std::fs::create_dir_all(&bootstrap_root)?;

    let workspace_home_snapshot = build_workspace_home_snapshot(plan);
    write_json_pretty(&bootstrap_root.join("blueprint.json"), &plan.blueprint)?;
    write_json_pretty(&bootstrap_root.join("resources.json"), &plan.resources)?;
    write_json_pretty(
        &bootstrap_root.join("agent_roster.json"),
        &plan.agent_roster,
    )?;
    write_json_pretty(
        &bootstrap_root.join("starter_tasks.json"),
        &plan.starter_tasks,
    )?;
    write_json_pretty(
        &bootstrap_root.join("artifact_queue.json"),
        &plan.artifact_queue,
    )?;
    write_json_pretty(
        &bootstrap_root.join("provisioning.json"),
        &plan.provisioning,
    )?;
    write_json_pretty(
        &bootstrap_root.join("workspace_home_snapshot.json"),
        &workspace_home_snapshot,
    )?;

    let placeholders_root = bootstrap_root.join("artifact_placeholders");
    std::fs::create_dir_all(&placeholders_root)?;

    let mut placeholder_index: Vec<String> = Vec::new();
    placeholder_index.push("# Startup Workspace Bootstrap".to_string());
    placeholder_index.push(String::new());
    placeholder_index.push(format!(
        "Generated at: {}",
        Utc::now().format("%Y-%m-%dT%H:%M:%SZ")
    ));
    placeholder_index.push(String::new());
    placeholder_index.push("Generated files:".to_string());
    placeholder_index.push("- blueprint.json".to_string());
    placeholder_index.push("- resources.json".to_string());
    placeholder_index.push("- agent_roster.json".to_string());
    placeholder_index.push("- starter_tasks.json".to_string());
    placeholder_index.push("- artifact_queue.json".to_string());
    placeholder_index.push("- provisioning.json".to_string());
    placeholder_index.push("- workspace_home_snapshot.json".to_string());
    placeholder_index.push(String::new());
    placeholder_index.push("Artifact placeholders:".to_string());

    for artifact in plan.artifact_queue.artifacts.iter() {
        let file_name = format!("{}.md", slugify_filename(&artifact.id));
        let placeholder_path = placeholders_root.join(&file_name);
        let content = render_artifact_placeholder(plan, artifact);
        std::fs::write(&placeholder_path, content)?;
        placeholder_index.push(format!("- artifact_placeholders/{file_name}"));
    }

    std::fs::write(
        bootstrap_root.join("README.md"),
        placeholder_index.join("\n"),
    )?;

    Ok(bootstrap_root)
}

fn write_json_pretty<T: Serialize>(path: &Path, value: &T) -> Result<(), BoxError> {
    let serialized = serde_json::to_string_pretty(value)?;
    std::fs::write(path, serialized)?;
    Ok(())
}

fn slugify_filename(value: &str) -> String {
    let mut output = String::new();
    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() {
            output.push(ch.to_ascii_lowercase());
        } else if !output.ends_with('_') {
            output.push('_');
        }
    }
    let trimmed = output.trim_matches('_');
    if trimmed.is_empty() {
        "artifact".to_string()
    } else {
        trimmed.to_string()
    }
}

fn render_artifact_placeholder(
    plan: &StartupWorkspaceBootstrapPlan,
    artifact: &crate::domain::artifact_queue::ArtifactPlaceholder,
) -> String {
    let status = match artifact.status {
        crate::domain::artifact_queue::ArtifactQueueStatus::Planned => "planned",
        crate::domain::artifact_queue::ArtifactQueueStatus::PendingReview => "pending_review",
    };

    let workspace_title = if plan.blueprint.venture.name.trim().is_empty() {
        "Founder Workspace".to_string()
    } else {
        plan.blueprint.venture.name.trim().to_string()
    };

    [
        format!("# {}", artifact.title),
        String::new(),
        format!("Workspace: {workspace_title}"),
        format!("Owner Role: {}", artifact.owner_role),
        format!("Surface: {}", artifact.surface),
        format!("Status: {status}"),
        String::new(),
        "## Rationale".to_string(),
        artifact.rationale.clone(),
        String::new(),
        "## Source Context".to_string(),
        format!("- Founder: {}", plan.blueprint.founder.name),
        format!("- Thesis: {}", plan.blueprint.venture.thesis),
        format!(
            "- Goals: {}",
            if plan.blueprint.goals_30_90_days.is_empty() {
                "None listed".to_string()
            } else {
                plan.blueprint.goals_30_90_days.join("; ")
            }
        ),
        String::new(),
        "## Draft".to_string(),
        "Fill in this placeholder during bootstrap execution.".to_string(),
        String::new(),
    ]
    .join("\n")
}

pub(super) fn write_thread_history(
    incoming_email: &Path,
    incoming_attachments: &Path,
) -> Result<(), BoxError> {
    let entries_email = incoming_email.join("entries");
    if !entries_email.exists() {
        return Ok(());
    }

    let mut entry_dirs: Vec<PathBuf> = Vec::new();
    for entry in std::fs::read_dir(&entries_email)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            entry_dirs.push(entry.path());
        }
    }
    entry_dirs.sort_by_key(|path| {
        path.file_name()
            .map(|value| value.to_string_lossy().to_string())
            .unwrap_or_default()
    });

    let mut output = String::new();
    output.push_str("# Thread history (inbound)\n");
    output.push_str("Auto-generated from incoming_email/entries. Latest entry is last.\n\n");

    for entry_dir in entry_dirs {
        let entry_name = entry_dir
            .file_name()
            .map(|value| value.to_string_lossy().to_string())
            .unwrap_or_else(|| "entry".to_string());
        let payload_path = entry_dir.join("postmark_payload.json");
        let summary = load_payload_summary(&payload_path);
        let attachments_dir = incoming_attachments.join("entries").join(&entry_name);
        let attachments = list_attachment_names(&attachments_dir).unwrap_or_default();
        let email_file = if entry_dir.join("email.html").exists() {
            "email.html"
        } else if entry_dir.join("email.txt").exists() {
            "email.txt"
        } else {
            "email.html"
        };

        output.push_str(&format!("## {entry_name}\n"));
        if let Some(summary) = summary {
            output.push_str(&format!("Subject: {}\n", summary.subject));
            output.push_str(&format!("From: {}\n", summary.from));
            output.push_str(&format!("To: {}\n", summary.to));
            if !summary.cc.is_empty() {
                output.push_str(&format!("Cc: {}\n", summary.cc));
            }
            if !summary.bcc.is_empty() {
                output.push_str(&format!("Bcc: {}\n", summary.bcc));
            }
            if let Some(date) = summary.date.as_deref() {
                output.push_str(&format!("Date: {}\n", date));
            }
            if !summary.message_id.is_empty() {
                output.push_str(&format!("Message-ID: {}\n", summary.message_id));
            }
            let preview = build_preview(&summary);
            if let Some(preview) = preview {
                output.push_str("Preview:\n```text\n");
                output.push_str(&preview);
                output.push_str("\n```\n");
            }
        }

        output.push_str("Files:\n");
        output.push_str(&format!(
            "- incoming_email/entries/{entry_name}/{email_file}\n"
        ));
        output.push_str(&format!(
            "- incoming_email/entries/{entry_name}/postmark_payload.json\n"
        ));
        if !attachments.is_empty() {
            output.push_str(&format!(
                "- incoming_attachments/entries/{entry_name}/ ({})\n",
                attachments.join(", ")
            ));
        } else {
            output.push_str("- incoming_attachments/entries/(none)\n");
        }
        output.push('\n');
    }

    std::fs::write(incoming_email.join("thread_history.md"), output)?;
    Ok(())
}

#[derive(Default)]
struct PayloadSummary {
    subject: String,
    from: String,
    to: String,
    cc: String,
    bcc: String,
    date: Option<String>,
    message_id: String,
    text_body: Option<String>,
    html_body: Option<String>,
}

fn load_payload_summary(payload_path: &Path) -> Option<PayloadSummary> {
    let payload_data = std::fs::read_to_string(payload_path).ok()?;
    let payload_json: serde_json::Value = serde_json::from_str(&payload_data).ok()?;
    Some(PayloadSummary {
        subject: json_string(&payload_json, "Subject").unwrap_or_default(),
        from: json_string(&payload_json, "From").unwrap_or_default(),
        to: json_string(&payload_json, "To").unwrap_or_default(),
        cc: json_string(&payload_json, "Cc").unwrap_or_default(),
        bcc: json_string(&payload_json, "Bcc").unwrap_or_default(),
        date: json_string(&payload_json, "Date")
            .or_else(|| json_string(&payload_json, "ReceivedAt")),
        message_id: json_string(&payload_json, "MessageID")
            .or_else(|| json_string(&payload_json, "MessageId"))
            .unwrap_or_default(),
        text_body: json_string(&payload_json, "TextBody")
            .or_else(|| json_string(&payload_json, "StrippedTextReply")),
        html_body: json_string(&payload_json, "HtmlBody"),
    })
}

fn json_string(payload: &serde_json::Value, key: &str) -> Option<String> {
    payload
        .get(key)
        .and_then(|value| value.as_str())
        .map(|value| value.to_string())
}

fn list_attachment_names(dir: &Path) -> Result<Vec<String>, std::io::Error> {
    if !dir.exists() {
        return Ok(Vec::new());
    }
    let mut names = Vec::new();
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            names.push(entry.file_name().to_string_lossy().to_string());
        }
    }
    names.sort();
    Ok(names)
}

fn build_preview(summary: &PayloadSummary) -> Option<String> {
    let mut preview = summary
        .text_body
        .as_deref()
        .unwrap_or("")
        .trim()
        .to_string();
    if preview.is_empty() {
        preview = summary
            .html_body
            .as_deref()
            .map(strip_html_tags)
            .unwrap_or_default();
    }
    let preview = preview.trim();
    if preview.is_empty() {
        return None;
    }
    Some(truncate_preview(preview, 1200))
}

pub(super) fn create_unique_dir(root: &Path, base: &str) -> Result<PathBuf, std::io::Error> {
    let mut candidate = root.join(base);
    if !candidate.exists() {
        std::fs::create_dir_all(&candidate)?;
        return Ok(candidate);
    }
    for idx in 1..1000 {
        let name = format!("{}_{}", base, idx);
        candidate = root.join(name);
        if !candidate.exists() {
            std::fs::create_dir_all(&candidate)?;
            return Ok(candidate);
        }
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::AlreadyExists,
        "failed to create unique workspace directory",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::workspace_blueprint::StartupWorkspaceBlueprint;
    use tempfile::tempdir;

    #[test]
    fn bootstrap_startup_workspace_files_writes_plan_and_placeholders() {
        let workspace_root = tempdir().expect("tempdir should be created");
        let workspace_dir = workspace_root.path().join("workspace");

        let mut blueprint = StartupWorkspaceBlueprint::default();
        blueprint.founder.name = "Founder".to_string();
        blueprint.founder.email = "founder@example.com".to_string();
        blueprint.venture.name = "Acme".to_string();
        blueprint.venture.thesis = "Build an agent-native startup workspace".to_string();
        blueprint.goals_30_90_days = vec!["Launch alpha".to_string()];

        let plan = bootstrap_startup_workspace_files(&workspace_dir, blueprint)
            .expect("bootstrap workspace files should succeed");

        assert!(!plan.resources.resources.is_empty());
        assert!(!plan.agent_roster.assignments.is_empty());
        assert!(!plan.artifact_queue.artifacts.is_empty());

        let startup_workspace_dir = workspace_dir.join("startup_workspace");
        assert!(startup_workspace_dir.join("blueprint.json").exists());
        assert!(startup_workspace_dir.join("resources.json").exists());
        assert!(startup_workspace_dir.join("agent_roster.json").exists());
        assert!(startup_workspace_dir.join("starter_tasks.json").exists());
        assert!(startup_workspace_dir.join("artifact_queue.json").exists());
        assert!(startup_workspace_dir.join("provisioning.json").exists());
        assert!(startup_workspace_dir
            .join("workspace_home_snapshot.json")
            .exists());
        assert!(startup_workspace_dir.join("README.md").exists());

        let placeholder_count =
            std::fs::read_dir(startup_workspace_dir.join("artifact_placeholders"))
                .expect("artifact placeholders directory should exist")
                .filter_map(Result::ok)
                .count();
        assert!(placeholder_count > 0);

        let resources_json = std::fs::read_to_string(startup_workspace_dir.join("resources.json"))
            .expect("resources.json should exist");
        assert!(resources_json.contains("manual_next_step"));
    }
}
