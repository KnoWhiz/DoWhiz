use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value;

use super::errors::RunTaskError;
use super::types::UserIdentities;
use super::workspace::resolve_rel_dir;

const GITHUB_NOTIFICATIONS_ADDRESS: &str = "notifications@github.com";

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
    user_identities: &UserIdentities,
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
    let github_coauthor_section = build_github_coauthor_section(workspace_dir, input_email_dir);
    let user_identities_section = build_user_identities_section(user_identities);

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
  "💡 Tip: Link your DoWhiz account to sync your preferences and project info across all channels (email, Google Docs, Slack, etc.). Visit https://www.dowhiz.com/auth/index.html to get started."
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
{github_coauthor_section}

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

{user_identities_section}
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
        github_coauthor_section = github_coauthor_section,
        user_identities_section = user_identities_section,
        registration_section = registration_section,
    )
}

fn build_user_identities_section(identities: &UserIdentities) -> String {
    // Only show if user has linked channels
    let has_any = identities.account_id.is_some()
        || !identities.emails.is_empty()
        || !identities.slack_user_ids.is_empty()
        || !identities.discord_user_ids.is_empty()
        || !identities.phone_numbers.is_empty()
        || !identities.telegram_user_ids.is_empty();

    if !has_any {
        return String::new();
    }

    let mut lines = vec!["Cross-channel routing (user's linked channels):".to_string()];

    if let Some(account_id) = &identities.account_id {
        lines.push(format!("- DoWhiz Account ID: {}", account_id));
    }
    if !identities.emails.is_empty() {
        lines.push(format!("- Email: {}", identities.emails.join(", ")));
    }
    if !identities.slack_user_ids.is_empty() {
        lines.push(format!("- Slack User IDs: {}", identities.slack_user_ids.join(", ")));
    }
    if !identities.discord_user_ids.is_empty() {
        lines.push(format!("- Discord User IDs: {}", identities.discord_user_ids.join(", ")));
    }
    if !identities.phone_numbers.is_empty() {
        lines.push(format!("- Phone Numbers: {}", identities.phone_numbers.join(", ")));
    }
    if !identities.telegram_user_ids.is_empty() {
        lines.push(format!("- Telegram User IDs: {}", identities.telegram_user_ids.join(", ")));
    }

    lines.push(String::new());
    lines.push("Cross-channel Reply Routing:".to_string());
    lines.push("If the user requests a reply on a different channel than the inbound channel,".to_string());
    lines.push("write a `reply_routing.json` file in the workspace root to route the reply.".to_string());
    lines.push("If no routing file is written, the reply goes to the original inbound channel.".to_string());
    lines.push(String::new());
    lines.push("reply_routing.json schema:".to_string());
    lines.push("```json".to_string());
    lines.push(r#"{
  "channel": "email" | "slack" | "discord" | "telegram" | "sms" | "whatsapp" | "bluebubbles",
  "identifier": "<target identifier for the channel>"
}"#.to_string());
    lines.push("```".to_string());
    lines.push(String::new());
    lines.push("Identifier format per channel:".to_string());
    lines.push("- email: email address (e.g., \"user@example.com\")".to_string());
    lines.push("- slack: Slack user ID (e.g., \"U1234567890\")".to_string());
    lines.push("- discord: Discord user ID (e.g., \"123456789012345678\")".to_string());
    lines.push("- telegram: Telegram user ID (e.g., \"123456789\")".to_string());
    lines.push("- sms/whatsapp/bluebubbles: phone number (e.g., \"+15551234567\")".to_string());
    lines.push(String::new());
    lines.push("IMPORTANT: When using cross-channel routing, write the reply in the TARGET channel's format:".to_string());
    lines.push("- email target: reply_email_draft.html (HTML), attachments in reply_email_attachments/".to_string());
    lines.push("- slack target: reply_message.txt (Slack mrkdwn: *bold*, _italic_, `code`)".to_string());
    lines.push("- discord target: reply_message.txt (Discord markdown: **bold**, *italic*, `code`)".to_string());
    lines.push("- telegram target: reply_message.txt (MarkdownV2)".to_string());
    lines.push("- sms/whatsapp/bluebubbles target: reply_message.txt (plain text)".to_string());
    lines.push("- Attachments for non-email channels go in reply_attachments/".to_string());
    lines.push(String::new());
    lines.push("Example: Inbound is email, user says \"reply to my Discord instead\"".to_string());
    lines.push("1. Write reply_routing.json: {\"channel\": \"discord\", \"identifier\": \"123456789012345678\"}".to_string());
    lines.push("2. Write reply_message.txt (NOT reply_email_draft.html) with Discord markdown".to_string());
    lines.push(String::new());

    lines.join("\n")
}

fn build_github_coauthor_section(workspace_dir: &Path, input_email_dir: &Path) -> String {
    let Some(login) = load_github_requester_login(workspace_dir, input_email_dir) else {
        return String::new();
    };
    let coauthor_email = format!("{login}@users.noreply.github.com");
    let trailer = format!("Co-authored-by: {login} <{coauthor_email}>");
    format!(
        r#"GitHub Attribution Requirement:
- This request came from GitHub user @{login}.
- If you create or amend any git commit for this task, append this trailer exactly once in each relevant commit message: `{trailer}`.
- If you open or update a PR, include `Requested-by: @{login}` in the PR body.
- Do not add co-author/requested-by lines when no commit or PR is created.

"#
    )
}

fn load_github_requester_login(workspace_dir: &Path, input_email_dir: &Path) -> Option<String> {
    let payload_path = workspace_dir
        .join(input_email_dir)
        .join("postmark_payload.json");
    let payload_raw = fs::read(payload_path).ok()?;
    let payload: Value = serde_json::from_slice(&payload_raw).ok()?;
    let from = payload
        .get("From")
        .or_else(|| payload.get("from"))
        .and_then(Value::as_str)
        .unwrap_or_default();
    if !looks_like_github_notifications_sender(from) {
        return None;
    }
    extract_github_sender_from_headers(&payload)
        .or_else(|| extract_github_sender_from_bodies(&payload))
}

fn looks_like_github_notifications_sender(from: &str) -> bool {
    from.to_ascii_lowercase()
        .contains(&GITHUB_NOTIFICATIONS_ADDRESS.to_ascii_lowercase())
}

fn extract_github_sender_from_headers(payload: &Value) -> Option<String> {
    let headers = payload
        .get("Headers")
        .or_else(|| payload.get("headers"))
        .and_then(Value::as_array)?;
    for header in headers {
        let name = header
            .get("Name")
            .or_else(|| header.get("name"))
            .and_then(Value::as_str)
            .unwrap_or_default();
        if !name.eq_ignore_ascii_case("X-GitHub-Sender") {
            continue;
        }
        let value = header
            .get("Value")
            .or_else(|| header.get("value"))
            .and_then(Value::as_str)
            .unwrap_or_default();
        if let Some(login) = normalize_github_login(value) {
            return Some(login);
        }
    }
    None
}

fn extract_github_sender_from_bodies(payload: &Value) -> Option<String> {
    for field in ["StrippedTextReply", "TextBody", "HtmlBody"] {
        if let Some(body) = payload.get(field).and_then(Value::as_str) {
            if let Some(login) = extract_github_sender_from_text(body) {
                return Some(login);
            }
        }
    }
    None
}

fn extract_github_sender_from_text(text: &str) -> Option<String> {
    if text.trim().is_empty() {
        return None;
    }

    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        if let Some(login) = extract_login_from_activity_line(trimmed) {
            return Some(login);
        }

        if let Some(login) = extract_login_from_html_activity_line(trimmed) {
            return Some(login);
        }
    }

    None
}

fn extract_login_from_activity_line(line: &str) -> Option<String> {
    let (candidate, rest) = line.split_once(char::is_whitespace)?;
    let rest = rest.trim_start().to_ascii_lowercase();
    let activity_prefixes = [
        "left a comment",
        "created an issue",
        "opened a pull request",
        "opened an issue",
        "closed an issue",
        "reopened an issue",
        "reviewed",
        "requested a review",
    ];
    if activity_prefixes
        .iter()
        .any(|prefix| rest.starts_with(prefix))
    {
        return normalize_github_login(candidate);
    }
    None
}

fn extract_login_from_html_activity_line(line: &str) -> Option<String> {
    let lower = line.to_ascii_lowercase();
    let start_idx = lower.find("<strong>")?;
    let after_start = &line[start_idx + "<strong>".len()..];
    let after_start_lower = after_start.to_ascii_lowercase();
    let end_idx = after_start_lower.find("</strong>")?;
    let candidate = &after_start[..end_idx];
    let rest = after_start[end_idx + "</strong>".len()..]
        .trim_start()
        .to_ascii_lowercase();
    let activity_prefixes = [
        "left a comment",
        "created an issue",
        "opened a pull request",
        "opened an issue",
    ];
    if activity_prefixes
        .iter()
        .any(|prefix| rest.starts_with(prefix))
    {
        return normalize_github_login(candidate);
    }
    None
}

fn normalize_github_login(raw: &str) -> Option<String> {
    let trimmed = raw
        .trim()
        .trim_start_matches('@')
        .trim_matches(|ch: char| matches!(ch, '"' | '\'' | '<' | '>' | '`'));
    if trimmed.is_empty() {
        return None;
    }

    let lower = trimmed.to_ascii_lowercase();
    let (base, bot_suffix) = if lower.ends_with("[bot]") {
        (&lower[..lower.len() - "[bot]".len()], true)
    } else {
        (lower.as_str(), false)
    };
    if base.is_empty() || base.len() > 39 {
        return None;
    }
    let mut chars = base.chars();
    let first = chars.next()?;
    if !first.is_ascii_alphanumeric() {
        return None;
    }
    if chars.any(|ch| !(ch.is_ascii_alphanumeric() || ch == '-')) {
        return None;
    }
    if base.ends_with('-') {
        return None;
    }
    if bot_suffix {
        Some(format!("{base}[bot]"))
    } else {
        Some(base.to_string())
    }
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
            &UserIdentities::default(),
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
            &UserIdentities::default(),
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
            &UserIdentities::default(),
        );

        assert!(prompt.contains("Account Registration Notice"));
        assert!(prompt.contains("www.dowhiz.com/auth/index.html"));

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
            &UserIdentities::default(),
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
            &UserIdentities::default(),
        );

        assert!(prompt.contains("Discord context snapshot (auto-generated"));
        assert!(prompt.contains("Quoted + thread context"));
    }

    #[test]
    fn build_prompt_includes_github_coauthor_guidance_when_sender_detected() {
        let temp = TempDir::new().expect("tempdir");
        let workspace = temp.path();
        let incoming_dir = workspace.join("incoming_email");
        fs::create_dir_all(&incoming_dir).expect("incoming_email");
        fs::write(
            incoming_dir.join("postmark_payload.json"),
            r#"{
  "From": "Bingran You <notifications@github.com>",
  "Headers": [{"Name":"X-GitHub-Sender","Value":"bingran-you"}]
}"#,
        )
        .expect("postmark payload");

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
            true,
            &UserIdentities::default(),
        );

        assert!(prompt.contains("GitHub Attribution Requirement"));
        assert!(
            prompt.contains("Co-authored-by: bingran-you <bingran-you@users.noreply.github.com>")
        );
        assert!(prompt.contains("Requested-by: @bingran-you"));
    }

    #[test]
    fn build_prompt_omits_github_coauthor_guidance_for_non_github_email() {
        let temp = TempDir::new().expect("tempdir");
        let workspace = temp.path();
        let incoming_dir = workspace.join("incoming_email");
        fs::create_dir_all(&incoming_dir).expect("incoming_email");
        fs::write(
            incoming_dir.join("postmark_payload.json"),
            r#"{
  "From": "Alice <alice@example.com>",
  "TextBody": "hello"
}"#,
        )
        .expect("postmark payload");

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
            true,
            &UserIdentities::default(),
        );

        assert!(!prompt.contains("GitHub Attribution Requirement"));
        assert!(!prompt.contains("Co-authored-by:"));
    }

    #[test]
    fn build_user_identities_section_empty_for_default() {
        let identities = UserIdentities::default();
        let section = build_user_identities_section(&identities);
        assert!(section.is_empty());
    }

    #[test]
    fn build_user_identities_section_includes_account_id() {
        let identities = UserIdentities {
            account_id: Some("test-account-123".to_string()),
            ..Default::default()
        };
        let section = build_user_identities_section(&identities);
        assert!(section.contains("DoWhiz Account ID: test-account-123"));
        assert!(section.contains("Cross-channel routing"));
    }

    #[test]
    fn build_user_identities_section_includes_all_channels() {
        let identities = UserIdentities {
            account_id: Some("acct-123".to_string()),
            emails: vec!["user@example.com".to_string()],
            slack_user_ids: vec!["U123456".to_string()],
            discord_user_ids: vec!["987654321".to_string()],
            phone_numbers: vec!["+15551234567".to_string()],
            telegram_user_ids: vec!["12345678".to_string()],
        };
        let section = build_user_identities_section(&identities);

        assert!(section.contains("Email: user@example.com"));
        assert!(section.contains("Slack User IDs: U123456"));
        assert!(section.contains("Discord User IDs: 987654321"));
        assert!(section.contains("Phone Numbers: +15551234567"));
        assert!(section.contains("Telegram User IDs: 12345678"));
    }

    #[test]
    fn build_user_identities_section_includes_routing_instructions() {
        let identities = UserIdentities {
            account_id: Some("acct-123".to_string()),
            ..Default::default()
        };
        let section = build_user_identities_section(&identities);

        assert!(section.contains("reply_routing.json"));
        assert!(section.contains("IMPORTANT: When using cross-channel routing"));
        assert!(section.contains("email target: reply_email_draft.html"));
        assert!(section.contains("discord target: reply_message.txt"));
    }

    #[test]
    fn build_prompt_includes_user_identities_when_present() {
        let temp = TempDir::new().expect("tempdir");
        let identities = UserIdentities {
            account_id: Some("test-acct".to_string()),
            emails: vec!["test@example.com".to_string()],
            discord_user_ids: vec!["123456789".to_string()],
            ..Default::default()
        };

        let prompt = build_prompt(
            Path::new("incoming_email"),
            Path::new("incoming_attachments"),
            Path::new("memory"),
            Path::new("references"),
            temp.path(),
            "codex",
            "",
            true,
            "email",
            true,
            &identities,
        );

        assert!(prompt.contains("Cross-channel routing"));
        assert!(prompt.contains("test@example.com"));
        assert!(prompt.contains("123456789"));
        assert!(prompt.contains("reply_routing.json"));
    }
}
