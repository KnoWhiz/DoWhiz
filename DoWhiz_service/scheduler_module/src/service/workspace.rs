use std::path::{Path, PathBuf};

use tracing::error;

use crate::employee_config::EmployeeProfile;

use super::html::{strip_html_tags, truncate_preview};
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
            std::fs::copy(&src_path, &dest_path)?;
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
            std::fs::copy(path, workspace.join("AGENTS.md"))?;
        }
    }
    if let Some(path) = employee.claude_path.as_ref() {
        if path.exists() {
            std::fs::copy(path, workspace.join("CLAUDE.md"))?;
        }
    }
    if let Some(path) = employee.soul_path.as_ref() {
        if path.exists() {
            std::fs::copy(path, workspace.join("SOUL.md"))?;
        }
    }
    Ok(())
}

pub(super) fn ensure_thread_workspace(
    user_paths: &crate::user_store::UserPaths,
    user_id: &str,
    thread_key: &str,
    employee: &EmployeeProfile,
    skills_source_dir: Option<&Path>,
) -> Result<PathBuf, BoxError> {
    std::fs::create_dir_all(&user_paths.workspaces_root)?;

    let workspace_name = thread_workspace_name(thread_key);
    let workspace = user_paths.workspaces_root.join(workspace_name);
    let is_new = !workspace.exists();
    if is_new {
        std::fs::create_dir_all(&workspace)?;
    }

    let incoming_email = workspace.join("incoming_email");
    let incoming_attachments = workspace.join("incoming_attachments");
    let memory = workspace.join("memory");
    let references = workspace.join("references");

    std::fs::create_dir_all(&incoming_email)?;
    std::fs::create_dir_all(&incoming_attachments)?;
    std::fs::create_dir_all(&memory)?;
    std::fs::create_dir_all(&references)?;

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

    ensure_workspace_employee_files(&workspace, employee)?;

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
