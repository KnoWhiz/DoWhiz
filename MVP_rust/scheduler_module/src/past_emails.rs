use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
        let date = parse_payload_date(message.payload.date.as_deref());
        let date_str = date.map(|value| value.to_rfc3339());
        let date_prefix = date
            .map(|value| value.format("%Y-%m-%d").to_string())
            .unwrap_or_else(|| "unknown_date".to_string());
        let subject_slug = sanitize_token(&subject, "no_subject").to_lowercase();
        let subject_slug = truncate_ascii(&subject_slug, 60);
        let message_id = normalize_message_id(message.payload.message_id.as_deref()).unwrap_or_else(|| {
            let fallback = message
                .root_dir
                .file_name()
                .map(|value| value.to_string_lossy().to_string())
                .unwrap_or_else(|| "msg".to_string());
            sanitize_token(&fallback, "msg")
        });
        let short_id = truncate_ascii(&sanitize_token(&message_id, "msg"), 12);
        let base = format!("{}_{}_{}", date_prefix, subject_slug, short_id);
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
        fs::write(
            &manifest_path,
            serde_json::to_string_pretty(&manifest)?,
        )?;

        report.entries_written += 1;
        report.attachments_total += attachment_counts.total;
        report.large_attachments += attachment_counts.large;

        entries.push(PastEmailsIndexEntry {
            entry_id: message_id.clone(),
            display_name: display_name.clone(),
            path: display_name.clone(),
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
    fs::create_dir_all(dest_attachments_dir)?;

    let attachment_meta = payload
        .attachments
        .as_ref()
        .map(|items| {
            items
                .iter()
                .map(|item| {
                    let sanitized = sanitize_token(&item.name, "attachment");
                    (
                        sanitized,
                        (item.name.clone(), item.content_type.clone()),
                    )
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
                continue;
            }
            let path = entry.path();
            let metadata = entry.metadata()?;
            let size_bytes = metadata.len();
            let (original_name, content_type) = attachment_meta
                .get(&file_name)
                .cloned()
                .unwrap_or_else(|| (file_name.clone(), String::new()));

            if size_bytes > max_attachment_bytes {
                counts.large += 1;
                counts.total += 1;
                let azure_blob_url = read_azure_url(incoming_attachments_dir, &file_name);
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
                relative_path: Some(format!("incoming_attachments/{}", dest_path.file_name().unwrap().to_string_lossy())),
                azure_blob_url: None,
            });
        }
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

fn read_azure_url(dir: &Path, file_name: &str) -> Option<String> {
    let url_path = dir.join(format!("{}.azure_url", file_name));
    let content = fs::read_to_string(url_path).ok()?;
    let trimmed = content.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
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
