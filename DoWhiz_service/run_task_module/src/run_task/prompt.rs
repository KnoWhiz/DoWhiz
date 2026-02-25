use std::fs;
use std::path::{Path, PathBuf};

use super::errors::RunTaskError;
use super::workspace::resolve_rel_dir;

/// Check if we've already prompted user to register in this thread.
/// Returns true if a marker file exists, indicating we've already prompted.
fn has_prompted_registration(workspace_dir: &Path) -> bool {
    workspace_dir.join(".registration_prompted").exists()
}

/// Mark that we've prompted the user to register in this thread.
fn mark_registration_prompted(workspace_dir: &Path) {
    let _ = fs::write(workspace_dir.join(".registration_prompted"), "1");
}

pub(super) fn build_prompt(
    input_email_dir: &Path,
    input_attachments_dir: &Path,
    memory_dir: &Path,
    reference_dir: &Path,
    workspace_dir: &Path,
    runner: &str,
    memory_context: &str,
    reply_required: bool,
    channel: &str,
    has_unified_account: bool,
) -> String {
    let memory_section = if memory_context.trim().is_empty() {
        "Memory context (from memory/*.md):\n- (no memory files found)\n\n".to_string()
    } else {
        format!(
            "Memory context (from memory/*.md):\n{memory_context}\n\n",
            memory_context = memory_context.trim_end()
        )
    };

    // Channel-specific reply instructions
    let reply_instruction = if !reply_required {
        "2. After finishing the task (step one), do not write any reply. This inbound message is from a non-replyable address, so skip creating any reply files."
    } else {
        match channel.to_lowercase().as_str() {
            "slack" => {
                "2. After finishing the task (step one), write a plain text reply in reply_message.txt in the workspace root. Use Slack mrkdwn formatting: *bold*, _italic_, `code`, ```code blocks```. Keep the reply concise and conversational. Do not use HTML. If there are files to attach, put them in reply_attachments/ and mention them in the reply. Do not pretend the job has been done without actually doing it."
            }
            "discord" => {
                "2. After finishing the task (step one), write a plain text reply in reply_message.txt in the workspace root. Use Discord markdown formatting: **bold**, *italic*, `code`, ```code blocks```. Keep the reply concise and conversational. Do not use HTML. If there are files to attach, put them in reply_attachments/ and mention them in the reply. Do not pretend the job has been done without actually doing it."
            }
            "telegram" => {
                "2. After finishing the task (step one), write a plain text reply in reply_message.txt in the workspace root. Use Telegram MarkdownV2 formatting. Keep the reply concise. Do not use HTML. If there are files to attach, put them in reply_attachments/. Do not pretend the job has been done without actually doing it."
            }
            "sms" => {
                "2. After finishing the task (step one), write a plain text reply in reply_message.txt in the workspace root. Keep the reply concise and conversational. Do not use HTML. If there are files to attach, put them in reply_attachments/ and mention them in the reply. Do not pretend the job has been done without actually doing it."
            }
            "bluebubbles" => {
                "2. After finishing the task (step one), write a plain text reply in reply_message.txt in the workspace root. Keep the reply concise and conversational. Do not use HTML or markdown. If there are files to attach, put them in reply_attachments/ and mention them in the reply. Do not pretend the job has been done without actually doing it."
            }
            "whatsapp" => {
                "2. After finishing the task (step one), write a plain text reply in reply_message.txt in the workspace root. Keep the reply concise and conversational. Do not use HTML. If there are files to attach, put them in reply_attachments/ and mention them in the reply. Do not pretend the job has been done without actually doing it."
            }
            _ => {
                // Default to email (HTML)
                "2. After finishing the task (step one), make sure you write a proper HTML email draft in reply_email_draft.html in the workspace root. If there are files to attach, put them in reply_email_attachments/ and reference them in the email draft. Do not pretend the job has been done without actually doing it, and do not write the email draft until the task is done. If you are not sure about the task, send another email to ask for clarification (and if any, attach information about why did you fail to get the task done, what is the exact error you encountered)."
            }
        }
    };
    let guidance_section = build_guidance_section(workspace_dir, runner);
    let discord_context_section = if channel.eq_ignore_ascii_case("discord") {
        build_discord_context_section(workspace_dir)
    } else {
        String::new()
    };

    // Build registration prompt section if user doesn't have a unified account
    // and we haven't prompted them yet in this thread
    let registration_section = if !has_unified_account && !has_prompted_registration(workspace_dir)
    {
        // Mark that we've prompted so we don't repeat
        mark_registration_prompted(workspace_dir);
        r#"
Account Registration Notice:
- This user does not have a DoWhiz unified account linked.
- At the END of your reply (after completing the task), add a brief note like:
  "💡 Tip: Link your DoWhiz account to sync your preferences and project info across all channels (email, Google Docs, Slack, etc.). Visit https://dowhiz.com/link to get started."
- Only mention this once - do not repeat in subsequent messages.
"#
    } else {
        ""
    };

    format!(
        r#"You are a DoWhiz digital employee. Follow the employee guidance provided below. Your task is to read incoming emails, understand the user's intent, finish the task, and draft appropriate email replies. You can also use memory and reference materials for context (already saved under current workspace). Always be cute, patient, friendly and helpful in your replies.

Employee guidance (from workspace files):
{guidance_section}

You main goal is
1. Most importantly, understand the task described in the incoming email and get the task done.
{reply_instruction}

Inputs (relative to workspace root):
- Incoming email dir: {input_email} (email.html, postmark_payload.json, thread_history.md, entries/)
- For incoming email, all previous emails in current thread: /incoming_email/entries/
- Incoming attachments dir: {input_attachments}
- Memory dir (memory about the current user): {memory}
- Reference dir (contain all past emails with the current user): {reference}

{discord_context_section}

Memory about the current user:
```{memory_section}```

Memory management and maintain policy:
- Read all Markdown files under memory/ before starting; they are long-term, per-user memory.
- Persist durable facts only (identity, preferences, recurring tasks, projects, contacts,
  decisions, and working processes). Do not store transient email-specific details.
- Default file is memory/memo.md (Markdown).
- If memo.md exceeds 500 lines, split by info type into multiple files (for example:
  memo_profile.md, memo_preferences.md, memo_projects.md, memo_contacts.md,
  memo_decisions.md, memo_processes.md). Keep every file <= 500 lines.
- When split, replace memo.md with a short index or highlights so it stays <= 500 lines.
- Update memory files at the end if new durable info is learned; otherwise leave unchanged.

Scheduling:
- For any scheduling (email or task), you MUST use the skill "scheduler_maintain".

Rules:
- Each workspace includes a `.env` file at the workspace root. You may edit it to manage per-user secrets; updates are synced back after the task completes.
- Do not modify input directories. Any file editing requests should be done on the copied version of attachments and save into reply_email_attachments/ to be sent back to the user. Mark version updates as "_v2", "_v3", etc. in the filename.
- You may create or modify other files and folders in the workspace as needed to complete the task.
  Prefer creating a work/ directory for clones, patches, and build artifacts.
- If attachments include version suffixes like _v1, _v2, the highest version should be the latest version.
- Avoid interactive commands; use non-interactive flags for git/gh (for example, `gh pr create --title ... --body ...`).
{registration_section}"#,
        input_email = input_email_dir.display(),
        input_attachments = input_attachments_dir.display(),
        memory = memory_dir.display(),
        reference = reference_dir.display(),
        memory_section = memory_section,
        guidance_section = guidance_section,
        reply_instruction = reply_instruction,
        discord_context_section = discord_context_section,
        registration_section = registration_section,
    )
}

fn build_guidance_section(workspace_dir: &Path, runner: &str) -> String {
    let mut blocks = Vec::new();

    if let Some(content) = load_optional_text(&workspace_dir.join("SOUL.md")) {
        blocks.push(format_guidance_block("SOUL.md", &content));
    }
    if let Some(content) = load_optional_text(&workspace_dir.join("AGENTS.md")) {
        blocks.push(format_guidance_block("AGENTS.md", &content));
    }
    if runner.eq_ignore_ascii_case("claude") {
        if let Some(content) = load_optional_text(&workspace_dir.join("CLAUDE.md")) {
            blocks.push(format_guidance_block("CLAUDE.md", &content));
        }
    }

    if blocks.is_empty() {
        "- (no employee guidance files found)\n".to_string()
    } else {
        blocks.join("\n")
    }
}

fn load_optional_text(path: &Path) -> Option<String> {
    let content = fs::read_to_string(path).ok()?;
    let trimmed = content.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn format_guidance_block(label: &str, content: &str) -> String {
    format!("{label}:\n```\n{content}\n```\n")
}

fn build_discord_context_section(workspace_dir: &Path) -> String {
    let path = workspace_dir
        .join("incoming_email")
        .join("discord_context_for_agent.md");
    let Ok(content) = fs::read_to_string(path) else {
        return String::new();
    };
    let trimmed = content.trim();
    if trimmed.is_empty() {
        return String::new();
    }
    let max_chars = 12_000usize;
    let mut clipped: String = trimmed.chars().take(max_chars).collect();
    if trimmed.chars().count() > max_chars {
        clipped.push_str("\n\n(Truncated. Use incoming_email/discord_thread_context_full.json and incoming_email/discord_channel_last_24h.json for complete context.)");
    }
    format!(
        "Discord context snapshot (auto-generated; full history is stored in local files):\n```markdown\n{}\n```\n",
        clipped
    )
}

pub(super) fn load_memory_context(
    workspace_dir: &Path,
    memory_dir: &Path,
) -> Result<String, RunTaskError> {
    let resolved = resolve_rel_dir(workspace_dir, memory_dir, "memory_dir")?;
    let mut files: Vec<PathBuf> = Vec::new();
    for entry in fs::read_dir(&resolved)? {
        let entry = entry?;
        if entry.file_type()?.is_file() && is_markdown_file(&entry.path()) {
            files.push(entry.path());
        }
    }
    files.sort_by(|left, right| left.file_name().cmp(&right.file_name()));

    let mut sections = Vec::new();
    for path in files {
        let content = fs::read_to_string(&path)?;
        let rel_path = path.strip_prefix(workspace_dir).unwrap_or(&path);
        sections.push(format!(
            "--- {path} ---\n{content}",
            path = rel_path.display(),
            content = content.trim_end()
        ));
    }
    Ok(sections.join("\n\n"))
}

fn is_markdown_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| matches!(ext.to_ascii_lowercase().as_str(), "md" | "markdown"))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn load_memory_context_sorts_and_includes_markdown() {
        let temp = TempDir::new().expect("tempdir");
        let workspace = temp.path().join("workspace");
        let memory_dir = workspace.join("memory");
        fs::create_dir_all(&memory_dir).expect("memory dir");
        fs::write(memory_dir.join("b.md"), "second").expect("b.md");
        fs::write(memory_dir.join("a.md"), "first").expect("a.md");
        fs::write(memory_dir.join("note.txt"), "ignore").expect("note.txt");

        let context = load_memory_context(&workspace, Path::new("memory")).expect("context");

        let first_idx = context.find("--- memory/a.md ---").expect("a.md marker");
        let second_idx = context.find("--- memory/b.md ---").expect("b.md marker");
        assert!(first_idx < second_idx, "expected a.md before b.md");
        assert!(context.contains("first"));
        assert!(context.contains("second"));
        assert!(!context.contains("note.txt"));
    }

    #[test]
    fn build_prompt_includes_memory_policy_and_section() {
        let prompt = build_prompt(
            Path::new("incoming_email"),
            Path::new("incoming_attachments"),
            Path::new("memory"),
            Path::new("references"),
            Path::new("."),
            "codex",
            "--- memory/memo.md ---\nHello",
            true,
            "email",
            true, // has_unified_account
        );

        assert!(prompt.contains("Memory context"));
        assert!(prompt.contains("memory/memo.md"));
        assert!(prompt.contains("Memory management"));
        assert!(prompt.contains("memo.md"));
        assert!(prompt.contains("500 lines"));
    }

    #[test]
    fn build_prompt_skips_reply_instruction_for_non_replyable() {
        let prompt = build_prompt(
            Path::new("incoming_email"),
            Path::new("incoming_attachments"),
            Path::new("memory"),
            Path::new("references"),
            Path::new("."),
            "codex",
            "",
            false,
            "email",
            true, // has_unified_account
        );

        assert!(prompt.contains("non-replyable"));
        assert!(!prompt.contains("write a proper HTML email draft"));
    }

    #[test]
    fn build_prompt_includes_registration_notice_for_unregistered_user() {
        let temp = TempDir::new().expect("tempdir");
        let workspace = temp.path();

        let prompt = build_prompt(
            Path::new("incoming_email"),
            Path::new("incoming_attachments"),
            Path::new("memory"),
            Path::new("references"),
            workspace,
            "codex",
            "",
            true,
            "email",
            false, // has_unified_account = false
        );

        assert!(prompt.contains("Account Registration Notice"));
        assert!(prompt.contains("dowhiz.com/link"));

        // Second call should NOT include the notice (already prompted)
        let prompt2 = build_prompt(
            Path::new("incoming_email"),
            Path::new("incoming_attachments"),
            Path::new("memory"),
            Path::new("references"),
            workspace,
            "codex",
            "",
            true,
            "email",
            false,
        );

        assert!(!prompt2.contains("Account Registration Notice"));
    }

    #[test]
    fn build_prompt_includes_discord_context_snapshot_when_available() {
        let temp = TempDir::new().expect("tempdir");
        let workspace = temp.path();
        let incoming_dir = workspace.join("incoming_email");
        fs::create_dir_all(&incoming_dir).expect("incoming_email");
        fs::write(
            incoming_dir.join("discord_context_for_agent.md"),
            "# Discord Context Snapshot\nQuoted + thread context",
        )
        .expect("context file");

        let prompt = build_prompt(
            Path::new("incoming_email"),
            Path::new("incoming_attachments"),
            Path::new("memory"),
            Path::new("references"),
            workspace,
            "codex",
            "",
            true,
            "discord",
            true,
        );

        assert!(prompt.contains("Discord context snapshot (auto-generated"));
        assert!(prompt.contains("Quoted + thread context"));
    }
}
