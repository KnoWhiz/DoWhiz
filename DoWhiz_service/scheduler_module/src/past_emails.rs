use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const DEFAULT_MAX_ATTACHMENT_BYTES: u64 = 50 * 1024 * 1024;

#[derive(Debug, thiserror::Error)]
pub enum PastEmailsError {
    #[error("io error: {0}")]
    Io(#[from] io::Error),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
}

#[derive(Debug, Default, Serialize)]
pub struct HydrateReport {
    pub entries_written: usize,
    pub attachments_total: usize,
    pub large_attachments: usize,
}

#[derive(Debug)]
struct ArchiveMessage {
    root_dir: PathBuf,
    incoming_email_dir: PathBuf,
    incoming_attachments_dir: PathBuf,
    payload: PostmarkPayload,
}

#[derive(Debug, Clone, Deserialize)]
struct PostmarkPayload {
    #[serde(rename = "From")]
    from: Option<String>,
    #[serde(rename = "To")]
    to: Option<String>,
    #[serde(rename = "Cc")]
    cc: Option<String>,
    #[serde(rename = "Bcc")]
    bcc: Option<String>,
    #[serde(rename = "Subject")]
    subject: Option<String>,
    #[serde(rename = "Date")]
    date: Option<String>,
    #[serde(rename = "MessageID", alias = "MessageId")]
    message_id: Option<String>,
    #[serde(rename = "Direction")]
    direction: Option<String>,
    #[serde(rename = "Attachments")]
    attachments: Option<Vec<PostmarkAttachment>>,
}

#[derive(Debug, Clone, Deserialize)]
struct PostmarkAttachment {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "ContentType")]
    content_type: String,
}

#[derive(Debug, Serialize)]
struct PastEmailsIndex {
    version: u32,
    generated_at: String,
    user_id: String,
    entries: Vec<PastEmailsIndexEntry>,
}

#[derive(Debug, Clone, Serialize)]
struct PastEmailsIndexEntry {
    entry_id: String,
    display_name: String,
    path: String,
    direction: String,
    subject: String,
    from: String,
    to: String,
    cc: String,
    bcc: String,
    date: Option<String>,
    message_id: String,
    attachments_manifest: String,
    attachments_count: usize,
    large_attachments_count: usize,
}

#[derive(Debug, Serialize)]
struct AttachmentsManifest {
    version: u32,
    generated_at: String,
    message_id: String,
    attachments: Vec<AttachmentEntry>,
}

#[derive(Debug, Serialize)]
struct AttachmentEntry {
    file_name: String,
    original_name: String,
    content_type: String,
    size_bytes: u64,
    storage: String,
    relative_path: Option<String>,
    azure_blob_url: Option<String>,
}

pub fn hydrate_past_emails(
    archive_root: &Path,
    references_dir: &Path,
    user_id: &str,
    max_attachment_bytes: Option<u64>,
) -> Result<HydrateReport, PastEmailsError> {
    let mut report = HydrateReport::default();
    let max_attachment_bytes = max_attachment_bytes.unwrap_or(DEFAULT_MAX_ATTACHMENT_BYTES);
    let past_root = references_dir.join("past_emails");
    fs::create_dir_all(&past_root)?;

    let mut entries = Vec::new();
    if !archive_root.exists() {
        write_index(&past_root, user_id, &entries)?;
        return Ok(report);
    }

    let mut messages = collect_archive_messages(archive_root)?;
    messages.sort_by(|a, b| {
        let a_date = parse_payload_date(a.payload.date.as_deref());
        let b_date = parse_payload_date(b.payload.date.as_deref());
        b_date.cmp(&a_date)
    });

    for message in messages {
        let subject = message.payload.subject.clone().unwrap_or_default();
        let direction = message
            .payload
            .direction
            .clone()
            .unwrap_or_else(|| "inbound".to_string());
        let date = parse_payload_date(message.payload.date.as_deref());
        let date_str = date.map(|value| value.to_rfc3339());
        let date_prefix = date
            .map(|value| value.format("%Y-%m-%d").to_string())
            .unwrap_or_else(|| "unknown_date".to_string());
        let direction = direction.to_ascii_lowercase();
        let message_id = normalize_message_id(message.payload.message_id.as_deref())
            .unwrap_or_else(|| {
                let fallback = message
                    .root_dir
                    .file_name()
                    .map(|value| value.to_string_lossy().to_string())
                    .unwrap_or_else(|| "msg".to_string());
                sanitize_token(&fallback, "msg")
            });
        let base = build_past_email_dir_name(&date_prefix, &subject, &direction, &message_id);
        let entry_dir = create_unique_dir(&past_root, &base)?;
        let display_name = entry_dir
            .file_name()
            .map(|value| value.to_string_lossy().to_string())
            .unwrap_or_else(|| base.clone());

        let dest_email_dir = entry_dir.join("incoming_email");
        copy_dir_recursive(&message.incoming_email_dir, &dest_email_dir)?;

        let dest_attachments_dir = entry_dir.join("incoming_attachments");
        let (manifest, attachment_counts) = hydrate_attachments(
            &message.payload,
            &message.incoming_attachments_dir,
            &dest_attachments_dir,
            max_attachment_bytes,
        )?;
        let manifest_path = entry_dir.join("attachments_manifest.json");
        fs::write(&manifest_path, serde_json::to_string_pretty(&manifest)?)?;

        report.entries_written += 1;
        report.attachments_total += attachment_counts.total;
        report.large_attachments += attachment_counts.large;

        entries.push(PastEmailsIndexEntry {
            entry_id: message_id.clone(),
            display_name: display_name.clone(),
            path: display_name.clone(),
            direction: direction.clone(),
            subject: subject.clone(),
            from: message.payload.from.clone().unwrap_or_default(),
            to: message.payload.to.clone().unwrap_or_default(),
            cc: message.payload.cc.clone().unwrap_or_default(),
            bcc: message.payload.bcc.clone().unwrap_or_default(),
            date: date_str,
            message_id,
            attachments_manifest: format!("{}/attachments_manifest.json", display_name),
            attachments_count: attachment_counts.total,
            large_attachments_count: attachment_counts.large,
        });
    }

    write_index(&past_root, user_id, &entries)?;
    Ok(report)
}

pub fn archive_outbound(
    archive_root: &Path,
    subject: &str,
    html_path: &Path,
    attachments_dir: &Path,
    to: &[String],
    cc: &[String],
    bcc: &[String],
    in_reply_to: Option<&str>,
    references: Option<&str>,
    message_id: &str,
    submitted_at: &str,
    from: &str,
) -> Result<(), PastEmailsError> {
    let html_body = fs::read_to_string(html_path)?;
    let mut text_body = strip_html_tags(&html_body);
    if text_body.trim().is_empty() {
        text_body = "(no content)".to_string();
    }

    let submitted_at = submitted_at.trim();
    let parsed_date = parse_payload_date(Some(submitted_at)).unwrap_or_else(Utc::now);
    let date_value = if submitted_at.is_empty() {
        parsed_date.to_rfc3339()
    } else {
        submitted_at.to_string()
    };
    let year = parsed_date.format("%Y").to_string();
    let month = parsed_date.format("%m").to_string();

    let fallback = format!("email_{}", parsed_date.timestamp());
    let message_id_value = if message_id.trim().is_empty() {
        fallback.clone()
    } else {
        message_id.trim().to_string()
    };
    let base = sanitize_token(&message_id_value, &fallback);

    let mail_root = archive_root.join(year).join(month);
    fs::create_dir_all(&mail_root)?;
    let mail_dir = create_unique_dir(&mail_root, &base)?;
    let incoming_email = mail_dir.join("incoming_email");
    let incoming_attachments = mail_dir.join("incoming_attachments");
    fs::create_dir_all(&incoming_email)?;
    fs::create_dir_all(&incoming_attachments)?;

    let attachments = archive_attachments(attachments_dir, &incoming_attachments)?;

    let headers = build_headers(in_reply_to, references);
    let payload = serde_json::json!({
        "From": from,
        "To": join_recipients(to),
        "Cc": join_recipients(cc),
        "Bcc": join_recipients(bcc),
        "Subject": subject,
        "Date": date_value,
        "MessageID": message_id_value,
        "TextBody": text_body,
        "HtmlBody": html_body,
        "Headers": headers,
        "Attachments": attachments,
        "Direction": "outbound",
    });
    fs::write(
        incoming_email.join("postmark_payload.json"),
        serde_json::to_string_pretty(&payload)?,
    )?;

    let email_html = render_email_html(&html_body, &text_body);
    fs::write(incoming_email.join("email.html"), email_html)?;
    Ok(())
}

fn write_index(
    past_root: &Path,
    user_id: &str,
    entries: &[PastEmailsIndexEntry],
) -> Result<(), PastEmailsError> {
    let index = PastEmailsIndex {
        version: 1,
        generated_at: Utc::now().to_rfc3339(),
        user_id: user_id.to_string(),
        entries: entries.to_vec(),
    };
    fs::write(
        past_root.join("index.json"),
        serde_json::to_string_pretty(&index)?,
    )?;
    Ok(())
}

fn collect_archive_messages(root: &Path) -> Result<Vec<ArchiveMessage>, PastEmailsError> {
    let mut messages = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let incoming_email = dir.join("incoming_email");
        let payload_path = incoming_email.join("postmark_payload.json");
        if incoming_email.is_dir() && payload_path.is_file() {
            let payload_data = fs::read_to_string(&payload_path)?;
            let payload: PostmarkPayload = serde_json::from_str(&payload_data)?;
            messages.push(ArchiveMessage {
                root_dir: dir.clone(),
                incoming_email_dir: incoming_email,
                incoming_attachments_dir: dir.join("incoming_attachments"),
                payload,
            });
            continue;
        }
        if let Ok(entries) = fs::read_dir(&dir) {
            for entry in entries.flatten() {
                if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                    stack.push(entry.path());
                }
            }
        }
    }
    Ok(messages)
}

fn hydrate_attachments(
    payload: &PostmarkPayload,
    incoming_attachments_dir: &Path,
    dest_attachments_dir: &Path,
    max_attachment_bytes: u64,
) -> Result<(AttachmentsManifest, AttachmentCounts), PastEmailsError> {
    let mut entries = Vec::new();
    let mut counts = AttachmentCounts::default();
    let mut seen_files = HashSet::new();
    let mut azure_urls = HashMap::new();
    fs::create_dir_all(dest_attachments_dir)?;

    let attachment_meta = payload
        .attachments
        .as_ref()
        .map(|items| {
            items
                .iter()
                .map(|item| {
                    let sanitized = sanitize_token(&item.name, "attachment");
                    (sanitized, (item.name.clone(), item.content_type.clone()))
                })
                .collect::<HashMap<_, _>>()
        })
        .unwrap_or_default();

    if incoming_attachments_dir.is_dir() {
        for entry in fs::read_dir(incoming_attachments_dir)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            if !file_type.is_file() {
                continue;
            }
            let file_name = entry.file_name().to_string_lossy().to_string();
            if file_name.ends_with(".azure_url") {
                let base = file_name.trim_end_matches(".azure_url").to_string();
                let content = fs::read_to_string(entry.path())?;
                let trimmed = content.trim();
                if !trimmed.is_empty() {
                    azure_urls.insert(base, trimmed.to_string());
                }
                continue;
            }
        }
    }

    if incoming_attachments_dir.is_dir() {
        for entry in fs::read_dir(incoming_attachments_dir)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            if !file_type.is_file() {
                continue;
            }
            let file_name = entry.file_name().to_string_lossy().to_string();
            if file_name.ends_with(".azure_url") {
                continue;
            }
            let path = entry.path();
            let metadata = entry.metadata()?;
            let size_bytes = metadata.len();
            let (original_name, content_type) = attachment_meta
                .get(&file_name)
                .cloned()
                .unwrap_or_else(|| (file_name.clone(), String::new()));

            seen_files.insert(file_name.clone());

            if size_bytes > max_attachment_bytes {
                counts.large += 1;
                counts.total += 1;
                let azure_blob_url = azure_urls.get(&file_name).cloned();
                entries.push(AttachmentEntry {
                    file_name,
                    original_name,
                    content_type,
                    size_bytes,
                    storage: "azure".to_string(),
                    relative_path: None,
                    azure_blob_url,
                });
                continue;
            }

            let dest_path = dest_attachments_dir.join(&file_name);
            fs::copy(&path, &dest_path)?;
            counts.total += 1;
            entries.push(AttachmentEntry {
                file_name,
                original_name,
                content_type,
                size_bytes,
                storage: "local".to_string(),
                relative_path: Some(format!(
                    "incoming_attachments/{}",
                    dest_path.file_name().unwrap().to_string_lossy()
                )),
                azure_blob_url: None,
            });
        }
    }

    for (file_name, azure_blob_url) in azure_urls {
        if seen_files.contains(&file_name) {
            continue;
        }
        let (original_name, content_type) = attachment_meta
            .get(&file_name)
            .cloned()
            .unwrap_or_else(|| (file_name.clone(), String::new()));
        counts.large += 1;
        counts.total += 1;
        entries.push(AttachmentEntry {
            file_name,
            original_name,
            content_type,
            size_bytes: 0,
            storage: "azure".to_string(),
            relative_path: None,
            azure_blob_url: Some(azure_blob_url),
        });
    }

    let manifest = AttachmentsManifest {
        version: 1,
        generated_at: Utc::now().to_rfc3339(),
        message_id: normalize_message_id(payload.message_id.as_deref()).unwrap_or_default(),
        attachments: entries,
    };
    Ok((manifest, counts))
}

#[derive(Debug, Default)]
struct AttachmentCounts {
    total: usize,
    large: usize,
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let dest = dst.join(entry.file_name());
        if file_type.is_dir() {
            copy_dir_recursive(&entry.path(), &dest)?;
        } else if file_type.is_file() {
            fs::copy(entry.path(), dest)?;
        }
    }
    Ok(())
}

fn archive_attachments(
    src_dir: &Path,
    dest_dir: &Path,
) -> Result<Vec<serde_json::Value>, PastEmailsError> {
    let mut attachments = Vec::new();
    if !src_dir.is_dir() {
        return Ok(attachments);
    }

    let mut entries: Vec<_> = fs::read_dir(src_dir)?.collect::<Result<_, _>>()?;
    entries.sort_by_key(|entry| entry.path());

    for entry in entries {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let file_name = entry.file_name().to_string_lossy().to_string();
        if file_name.ends_with(".azure_url") {
            fs::copy(&path, dest_dir.join(&file_name))?;
            continue;
        }
        fs::copy(&path, dest_dir.join(&file_name))?;
        attachments.push(serde_json::json!({
            "Name": file_name,
            "ContentType": "",
        }));
    }

    Ok(attachments)
}

fn build_headers(in_reply_to: Option<&str>, references: Option<&str>) -> Vec<serde_json::Value> {
    let mut headers = Vec::new();
    if let Some(value) = clean_header_value(in_reply_to) {
        headers.push(serde_json::json!({
            "Name": "In-Reply-To",
            "Value": value,
        }));
    }
    if let Some(value) = clean_header_value(references) {
        headers.push(serde_json::json!({
            "Name": "References",
            "Value": value,
        }));
    }
    headers
}

fn clean_header_value(value: Option<&str>) -> Option<String> {
    value
        .map(str::trim)
        .filter(|trimmed| !trimmed.is_empty())
        .map(|trimmed| trimmed.to_string())
}

fn join_recipients(values: &[String]) -> String {
    values
        .iter()
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
        .collect::<Vec<_>>()
        .join(", ")
}

fn render_email_html(html_body: &str, text_body: &str) -> String {
    if !html_body.trim().is_empty() {
        return html_body.to_string();
    }
    if text_body.trim().is_empty() {
        return "<pre>(no content)</pre>".to_string();
    }
    wrap_text_as_html(text_body)
}

fn wrap_text_as_html(input: &str) -> String {
    format!("<pre>{}</pre>", escape_html(input))
}

fn escape_html(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for ch in input.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(ch),
        }
    }
    out
}

fn strip_html_tags(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut in_tag = false;
    for ch in input.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => out.push(ch),
            _ => {}
        }
    }
    out
}

fn parse_payload_date(raw: Option<&str>) -> Option<DateTime<Utc>> {
    let raw = raw?.trim();
    if raw.is_empty() {
        return None;
    }
    if let Ok(parsed) = DateTime::parse_from_rfc2822(raw) {
        return Some(parsed.with_timezone(&Utc));
    }
    if let Ok(parsed) = DateTime::parse_from_rfc3339(raw) {
        return Some(parsed.with_timezone(&Utc));
    }
    None
}

fn normalize_message_id(raw: Option<&str>) -> Option<String> {
    let trimmed = raw?.trim().trim_matches(|ch| matches!(ch, '<' | '>'));
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_ascii_lowercase())
    }
}

fn sanitize_token(value: &str, fallback: &str) -> String {
    let trimmed = value.trim().trim_start_matches('<').trim_end_matches('>');
    let mut out = String::new();
    for ch in trimmed.chars() {
        if ch.is_ascii_alphanumeric() || matches!(ch, '.' | '_' | '-') {
            out.push(ch);
        } else {
            out.push('_');
        }
    }
    let cleaned = out.trim_matches(&['.', '_', '-'][..]);
    if cleaned.is_empty() {
        fallback.to_string()
    } else {
        cleaned.to_string()
    }
}

fn truncate_ascii(value: &str, max_len: usize) -> String {
    if value.len() <= max_len {
        return value.to_string();
    }
    value[..max_len].to_string()
}

fn build_past_email_dir_name(
    date_prefix: &str,
    subject: &str,
    direction: &str,
    message_id: &str,
) -> String {
    let (action, topic) = derive_action_topic(subject, direction);
    let action = sanitize_token(&action, "message").to_lowercase();
    let action = truncate_slug(&action, 24);
    let topic = sanitize_token(&topic, "topic").to_lowercase();
    let topic = truncate_slug(&topic, 60);
    let short_id = truncate_ascii(&sanitize_token(message_id, "msg"), 12);
    format!("{}_{}_{}_{}", date_prefix, action, topic, short_id)
}

fn derive_action_topic(subject: &str, direction: &str) -> (String, String) {
    let direction = normalize_direction(direction);
    let prefix_action = subject_prefix_action(subject);
    let mut tokens = extract_ascii_tokens(subject);
    strip_reply_prefixes(&mut tokens);

    let keyword_action = find_action_keyword(&tokens);
    let action = prefix_action
        .or(keyword_action)
        .unwrap_or_else(|| default_action(direction));

    let topic_tokens: Vec<String> = tokens
        .into_iter()
        .filter(|token| !is_reply_prefix(token))
        .filter(|token| token != action)
        .collect();

    let fallback_topic = default_topic(direction, action);
    let topic = slug_from_tokens(&topic_tokens, fallback_topic, 60);

    (action.to_string(), topic)
}

fn normalize_direction(direction: &str) -> &'static str {
    if direction.eq_ignore_ascii_case("outbound") {
        "outbound"
    } else {
        "inbound"
    }
}

fn subject_prefix_action(subject: &str) -> Option<&'static str> {
    let lower = subject.trim_start().to_ascii_lowercase();
    if lower.starts_with("re:") {
        Some("reply")
    } else if lower.starts_with("fw:") || lower.starts_with("fwd:") {
        Some("forward")
    } else {
        None
    }
}

fn default_action(direction: &str) -> &'static str {
    if direction == "outbound" {
        "reply"
    } else {
        "message"
    }
}

fn default_topic(direction: &str, action: &str) -> &'static str {
    let fallback = if direction == "outbound" {
        "response"
    } else {
        "request"
    };
    if fallback == action {
        "topic"
    } else {
        fallback
    }
}

fn slug_from_tokens(tokens: &[String], fallback: &str, max_len: usize) -> String {
    let mut slug = tokens
        .iter()
        .filter(|token| !token.is_empty())
        .map(|token| token.to_ascii_lowercase())
        .collect::<Vec<_>>()
        .join("-");
    if slug.is_empty() {
        slug = fallback.to_string();
    }
    let slug = sanitize_token(&slug, fallback).to_lowercase();
    let slug = truncate_slug(&slug, max_len);
    if slug.is_empty() {
        fallback.to_string()
    } else {
        slug
    }
}

fn truncate_slug(value: &str, max_len: usize) -> String {
    let truncated = truncate_ascii(value, max_len);
    truncated.trim_matches(&['-', '_', '.'][..]).to_string()
}

fn extract_ascii_tokens(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut buf = String::new();
    for ch in input.chars() {
        if ch.is_ascii_alphanumeric() {
            buf.push(ch.to_ascii_lowercase());
        } else if !buf.is_empty() {
            tokens.push(std::mem::take(&mut buf));
        }
    }
    if !buf.is_empty() {
        tokens.push(buf);
    }
    tokens
}

fn strip_reply_prefixes(tokens: &mut Vec<String>) {
    while tokens
        .first()
        .map(|token| is_reply_prefix(token))
        .unwrap_or(false)
    {
        tokens.remove(0);
    }
}

fn is_reply_prefix(token: &str) -> bool {
    matches!(token, "re" | "fw" | "fwd")
}

fn find_action_keyword(tokens: &[String]) -> Option<&'static str> {
    for token in tokens {
        match token.as_str() {
            "review" => return Some("review"),
            "draft" => return Some("draft"),
            "reply" | "respond" | "response" => return Some("reply"),
            "followup" | "follow" => return Some("followup"),
            "schedule" | "meeting" => return Some("schedule"),
            "invoice" => return Some("invoice"),
            "payment" => return Some("payment"),
            "refund" => return Some("refund"),
            "update" => return Some("update"),
            "bug" | "issue" => return Some("bugfix"),
            "support" => return Some("support"),
            "proposal" => return Some("proposal"),
            "summary" => return Some("summary"),
            "report" => return Some("report"),
            "contract" => return Some("contract"),
            "onboarding" => return Some("onboarding"),
            "interview" => return Some("interview"),
            _ => {}
        }
    }
    None
}

fn create_unique_dir(root: &Path, base: &str) -> Result<PathBuf, io::Error> {
    let mut candidate = root.join(base);
    if !candidate.exists() {
        fs::create_dir_all(&candidate)?;
        return Ok(candidate);
    }
    for idx in 1..1000 {
        let name = format!("{}_{}", base, idx);
        candidate = root.join(name);
        if !candidate.exists() {
            fs::create_dir_all(&candidate)?;
            return Ok(candidate);
        }
    }
    Err(io::Error::new(
        io::ErrorKind::AlreadyExists,
        "failed to create unique past_emails directory",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn find_payload(root: &Path) -> Option<PathBuf> {
        let mut stack = vec![root.to_path_buf()];
        while let Some(dir) = stack.pop() {
            let payload = dir.join("incoming_email").join("postmark_payload.json");
            if payload.is_file() {
                return Some(payload);
            }
            if let Ok(entries) = fs::read_dir(&dir) {
                for entry in entries.flatten() {
                    if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                        stack.push(entry.path());
                    }
                }
            }
        }
        None
    }

    #[test]
    fn archive_outbound_writes_payload_and_attachments() {
        let temp = TempDir::new().expect("tempdir");
        let archive_root = temp.path().join("mail");
        fs::create_dir_all(&archive_root).expect("archive root");

        let html_path = temp.path().join("reply.html");
        fs::write(&html_path, "<html><body>Hello</body></html>").expect("html");
        let attachments_dir = temp.path().join("attachments");
        fs::create_dir_all(&attachments_dir).expect("attachments dir");
        fs::write(attachments_dir.join("note.txt"), "hello").expect("attachment");

        archive_outbound(
            &archive_root,
            "Subject",
            &html_path,
            &attachments_dir,
            &[String::from("user@example.com")],
            &[],
            &[],
            None,
            None,
            "msg-123@example.com",
            "2026-02-03T20:10:44Z",
            "agent@example.com",
        )
        .expect("archive outbound");

        let payload_path = find_payload(&archive_root).expect("payload");
        let payload_data = fs::read_to_string(&payload_path).expect("payload read");
        let payload_json: serde_json::Value =
            serde_json::from_str(&payload_data).expect("payload json");
        assert_eq!(payload_json["Direction"], "outbound");

        let mail_dir = payload_path
            .parent()
            .and_then(|value| value.parent())
            .expect("mail dir");
        assert!(mail_dir.join("incoming_email").join("email.html").exists());
        assert!(mail_dir
            .join("incoming_attachments")
            .join("note.txt")
            .exists());
    }
}
