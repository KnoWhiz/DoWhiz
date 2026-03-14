use chrono::{Duration as ChronoDuration, Utc};
use mongodb::bson::{doc, Bson, DateTime as BsonDateTime, Document};
use mongodb::options::{FindOneOptions, FindOptions, UpdateOptions};
use mongodb::sync::Collection;
use mongodb::IndexModel;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicI64, Ordering};
use uuid::Uuid;

use crate::mongo_store::{create_client_from_env, database_from_env, ensure_index_compatible};

use super::super::types::{Schedule, ScheduledTask, SchedulerError};
use super::super::utils::{task_kind_channel, task_kind_label};
use super::TaskStatusSummary;

static EXECUTION_SEQ: AtomicI64 = AtomicI64::new(1);
const REQUEST_SUMMARY_MAX_CHARS: usize = 72;

#[derive(Debug)]
pub(crate) struct MongoSchedulerStore {
    tasks: Collection<Document>,
    executions: Collection<Document>,
    owner_kind: String,
    owner_id: String,
}

impl MongoSchedulerStore {
    pub(crate) fn new(tasks_db_path: &Path) -> Result<Self, SchedulerError> {
        let client = create_client_from_env().map_err(mongo_config_err)?;
        let db = database_from_env(&client);
        let (owner_kind, owner_id) = resolve_owner_scope(tasks_db_path);
        let tasks = db.collection::<Document>("tasks");
        ensure_index_compatible(
            &tasks,
            IndexModel::builder()
                .keys(doc! { "owner_scope.kind": 1, "owner_scope.id": 1, "task_id": 1 })
                .build(),
        )
        .map_err(mongo_err)?;
        ensure_index_compatible(
            &tasks,
            IndexModel::builder()
                .keys(doc! { "owner_scope.kind": 1, "owner_scope.id": 1, "created_at": 1 })
                .build(),
        )
        .map_err(mongo_err)?;
        let executions = db.collection::<Document>("task_executions");
        ensure_index_compatible(
            &executions,
            IndexModel::builder()
                .keys(doc! {
                    "owner_scope.kind": 1,
                    "owner_scope.id": 1,
                    "task_id": 1,
                    "started_at": -1
                })
                .build(),
        )
        .map_err(mongo_err)?;
        Ok(Self {
            tasks,
            executions,
            owner_kind,
            owner_id,
        })
    }

    pub(crate) fn load_tasks(&self) -> Result<Vec<ScheduledTask>, SchedulerError> {
        let cursor = self
            .tasks
            .find(
                self.owner_filter(),
                FindOptions::builder()
                    .sort(doc! { "created_at": -1 })
                    .build(),
            )
            .map_err(mongo_err)?;
        let mut seen_task_ids = HashSet::new();
        let mut tasks = Vec::new();
        for row in cursor {
            let document = row.map_err(mongo_err)?;
            if let Ok(task_id) = document.get_str("task_id") {
                if !seen_task_ids.insert(task_id.to_string()) {
                    continue;
                }
            }
            let task_json = document.get_str("task_json").map_err(|err| {
                SchedulerError::Storage(format!("missing task_json for task document: {err}"))
            })?;
            let task: ScheduledTask = serde_json::from_str(task_json)
                .map_err(|err| SchedulerError::Storage(format!("invalid task_json: {err}")))?;
            tasks.push(task);
        }
        tasks.sort_by_key(|task| task.created_at);
        Ok(tasks)
    }

    pub(crate) fn insert_task(&self, task: &ScheduledTask) -> Result<(), SchedulerError> {
        let task_json = serde_json::to_string(task)
            .map_err(|err| SchedulerError::Storage(format!("serialize task failed: {err}")))?;
        self.tasks
            .update_one(
                self.task_filter(&task.id.to_string()),
                doc! {
                    "$set": {
                        "owner_scope": self.owner_scope_doc(),
                        "task_id": task.id.to_string(),
                        "kind": task_kind_label(&task.kind),
                        "channel": task_kind_channel(&task.kind).to_string(),
                        "enabled": task.enabled,
                        "created_at": BsonDateTime::from_chrono(task.created_at),
                        "last_run": task.last_run.map(BsonDateTime::from_chrono).map(Bson::DateTime).unwrap_or(Bson::Null),
                        "schedule": schedule_doc(&task.schedule),
                        "task_json": task_json,
                    },
                    "$setOnInsert": {
                        "retry_count": 0i32,
                    },
                },
                UpdateOptions::builder().upsert(Some(true)).build(),
            )
            .map_err(mongo_err)?;
        Ok(())
    }

    pub(crate) fn update_task(&self, task: &ScheduledTask) -> Result<(), SchedulerError> {
        let task_json = serde_json::to_string(task)
            .map_err(|err| SchedulerError::Storage(format!("serialize task failed: {err}")))?;
        self.tasks
            .update_one(
                self.task_filter(&task.id.to_string()),
                doc! {
                    "$set": {
                        "enabled": task.enabled,
                        "last_run": task.last_run.map(BsonDateTime::from_chrono).map(Bson::DateTime).unwrap_or(Bson::Null),
                        "schedule": schedule_doc(&task.schedule),
                        "task_json": task_json,
                    }
                },
                None,
            )
            .map_err(mongo_err)?;
        Ok(())
    }

    pub(crate) fn record_execution_start(
        &self,
        task_id: Uuid,
        started_at: chrono::DateTime<Utc>,
    ) -> Result<i64, SchedulerError> {
        let execution_id = EXECUTION_SEQ.fetch_add(1, Ordering::Relaxed);
        self.executions
            .insert_one(
                doc! {
                    "owner_scope": self.owner_scope_doc(),
                    "execution_id": execution_id,
                    "task_id": task_id.to_string(),
                    "started_at": BsonDateTime::from_chrono(started_at),
                    "finished_at": Bson::Null,
                    "status": "running",
                    "error_message": Bson::Null,
                },
                None,
            )
            .map_err(mongo_err)?;
        Ok(execution_id)
    }

    pub(crate) fn record_execution_finish(
        &self,
        task_id: Uuid,
        execution_id: i64,
        finished_at: chrono::DateTime<Utc>,
        status: &str,
        error_message: Option<&str>,
    ) -> Result<(), SchedulerError> {
        self.executions
            .update_one(
                doc! {
                    "owner_scope.kind": &self.owner_kind,
                    "owner_scope.id": &self.owner_id,
                    "task_id": task_id.to_string(),
                    "execution_id": execution_id,
                },
                doc! {
                    "$set": {
                        "finished_at": BsonDateTime::from_chrono(finished_at),
                        "status": status,
                        "error_message": error_message.map(Bson::from).unwrap_or(Bson::Null),
                    }
                },
                None,
            )
            .map_err(mongo_err)?;
        Ok(())
    }

    pub(crate) fn get_retry_count(&self, task_id: &str) -> Result<u32, SchedulerError> {
        let document = self
            .tasks
            .find_one(self.task_filter(task_id), None)
            .map_err(mongo_err)?;
        Ok(document
            .as_ref()
            .and_then(|doc| numeric_field_to_u32(doc, "retry_count"))
            .unwrap_or(0))
    }

    pub(crate) fn increment_retry_count(&self, task_id: &str) -> Result<u32, SchedulerError> {
        self.tasks
            .update_one(
                self.task_filter(task_id),
                doc! { "$inc": { "retry_count": 1i32 } },
                None,
            )
            .map_err(mongo_err)?;
        self.get_retry_count(task_id)
    }

    pub(crate) fn reset_retry_count(&self, task_id: &str) -> Result<(), SchedulerError> {
        self.tasks
            .update_one(
                self.task_filter(task_id),
                doc! { "$set": { "retry_count": 0i32 } },
                None,
            )
            .map_err(mongo_err)?;
        Ok(())
    }

    pub(crate) fn disable_task_by_id(&self, task_id: &str) -> Result<(), SchedulerError> {
        self.tasks
            .update_one(
                self.task_filter(task_id),
                doc! { "$set": { "enabled": false } },
                None,
            )
            .map_err(mongo_err)?;
        Ok(())
    }

    pub(crate) fn list_tasks_with_status(&self) -> Result<Vec<TaskStatusSummary>, SchedulerError> {
        let created_after = BsonDateTime::from_chrono(Utc::now() - ChronoDuration::hours(24));
        let cursor = self
            .tasks
            .find(
                doc! {
                    "owner_scope.kind": &self.owner_kind,
                    "owner_scope.id": &self.owner_id,
                    "created_at": { "$gte": created_after },
                },
                FindOptions::builder()
                    .sort(doc! { "created_at": -1 })
                    .build(),
            )
            .map_err(mongo_err)?;
        let mut summaries = Vec::new();
        let mut seen_task_ids = HashSet::new();
        for row in cursor {
            let task_doc = row.map_err(mongo_err)?;
            let task_id = task_doc
                .get_str("task_id")
                .map_err(|err| SchedulerError::Storage(format!("missing task_id: {err}")))?;
            if !seen_task_ids.insert(task_id.to_string()) {
                continue;
            }
            let request_summary = derive_request_summary(&task_doc);
            let schedule = task_doc.get_document("schedule").ok();
            let execution = self
                .executions
                .find_one(
                    doc! {
                        "owner_scope.kind": &self.owner_kind,
                        "owner_scope.id": &self.owner_id,
                        "task_id": task_id,
                    },
                    FindOneOptions::builder()
                        .sort(doc! { "started_at": -1 })
                        .build(),
                )
                .map_err(mongo_err)?;
            summaries.push(TaskStatusSummary {
                id: task_id.to_string(),
                kind: task_doc.get_str("kind").unwrap_or("unknown").to_string(),
                channel: task_doc.get_str("channel").unwrap_or("email").to_string(),
                request_summary,
                enabled: task_doc.get_bool("enabled").unwrap_or(false),
                created_at: datetime_field_to_rfc3339(&task_doc, "created_at").unwrap_or_default(),
                last_run: datetime_field_to_rfc3339(&task_doc, "last_run"),
                schedule_type: schedule
                    .and_then(|doc| doc.get_str("type").ok())
                    .unwrap_or("one_shot")
                    .to_string(),
                next_run: schedule.and_then(|doc| datetime_field_to_rfc3339(doc, "next_run")),
                run_at: schedule.and_then(|doc| datetime_field_to_rfc3339(doc, "run_at")),
                execution_status: execution
                    .as_ref()
                    .and_then(|doc| doc.get_str("status").ok())
                    .map(|value| value.to_string()),
                error_message: execution.as_ref().and_then(|doc| {
                    doc.get_str("error_message")
                        .ok()
                        .map(|value| value.to_string())
                }),
                execution_started_at: execution
                    .as_ref()
                    .and_then(|doc| datetime_field_to_rfc3339(doc, "started_at")),
            });
        }
        Ok(summaries)
    }

    fn owner_filter(&self) -> Document {
        doc! {
            "owner_scope.kind": &self.owner_kind,
            "owner_scope.id": &self.owner_id,
        }
    }

    fn task_filter(&self, task_id: &str) -> Document {
        doc! {
            "owner_scope.kind": &self.owner_kind,
            "owner_scope.id": &self.owner_id,
            "task_id": task_id,
        }
    }

    fn owner_scope_doc(&self) -> Document {
        doc! {
            "kind": &self.owner_kind,
            "id": &self.owner_id,
        }
    }
}

fn schedule_doc(schedule: &Schedule) -> Document {
    match schedule {
        Schedule::Cron {
            expression,
            next_run,
        } => doc! {
            "type": "cron",
            "cron_expression": expression,
            "next_run": BsonDateTime::from_chrono(*next_run),
            "run_at": Bson::Null,
        },
        Schedule::OneShot { run_at } => doc! {
            "type": "one_shot",
            "cron_expression": Bson::Null,
            "next_run": Bson::Null,
            "run_at": BsonDateTime::from_chrono(*run_at),
        },
    }
}

fn resolve_owner_scope(path: &Path) -> (String, String) {
    let mut components: Vec<String> = Vec::new();
    for component in path.components() {
        if let Some(value) = component.as_os_str().to_str() {
            components.push(value.to_string());
        }
    }

    for (idx, value) in components.iter().enumerate() {
        if value == "users" {
            if let Some(owner_id) = components.get(idx + 1) {
                return ("user".to_string(), owner_id.to_string());
            }
        }
    }

    if path.file_name().and_then(|v| v.to_str()) == Some("tasks.db") {
        if let Some(state_dir) = path.parent() {
            if state_dir.file_name().and_then(|v| v.to_str()) == Some("state") {
                if let Some(owner_dir) = state_dir.parent() {
                    if let Some(owner_id) = owner_dir.file_name().and_then(|v| v.to_str()) {
                        return ("user".to_string(), owner_id.to_string());
                    }
                }
            }
        }
    }

    let hashed = format!("{:x}", md5::compute(path.to_string_lossy().as_bytes()));
    ("path_scope".to_string(), hashed)
}

fn datetime_field_to_rfc3339(document: &Document, key: &str) -> Option<String> {
    match document.get(key) {
        Some(Bson::DateTime(value)) => Some(value.to_chrono().to_rfc3339()),
        Some(Bson::String(value)) => Some(value.to_string()),
        _ => None,
    }
}

fn numeric_field_to_u32(document: &Document, key: &str) -> Option<u32> {
    match document.get(key) {
        Some(Bson::Int32(value)) if *value >= 0 => Some(*value as u32),
        Some(Bson::Int64(value)) if *value >= 0 => Some(*value as u32),
        _ => None,
    }
}

fn derive_request_summary(task_doc: &Document) -> Option<String> {
    let task_json = task_doc.get_str("task_json").ok()?;
    let task_value: serde_json::Value = serde_json::from_str(task_json).ok()?;
    let task_kind = task_value.pointer("/kind/type").and_then(|v| v.as_str())?;

    match task_kind {
        "send_email" => task_value
            .pointer("/kind/subject")
            .and_then(|v| v.as_str())
            .and_then(normalize_summary_text),
        "run_task" => {
            let workspace_dir = task_value
                .pointer("/kind/workspace_dir")
                .and_then(|v| v.as_str())?;
            let channel = task_value
                .pointer("/kind/channel")
                .and_then(|v| v.as_str())
                .or_else(|| task_doc.get_str("channel").ok())
                .unwrap_or("");
            derive_run_task_summary(Path::new(workspace_dir), channel)
        }
        _ => None,
    }
}

fn derive_run_task_summary(workspace_dir: &Path, channel: &str) -> Option<String> {
    let incoming_dir = workspace_dir.join("incoming_email");
    if !incoming_dir.exists() {
        return None;
    }

    match channel {
        "email" => derive_email_summary(&incoming_dir),
        "google_docs" => derive_google_workspace_summary(&incoming_dir, "gdocs"),
        "google_sheets" => derive_google_workspace_summary(&incoming_dir, "gsheets"),
        "google_slides" => derive_google_workspace_summary(&incoming_dir, "gslides"),
        "discord" => derive_discord_summary(&incoming_dir),
        "slack" => derive_text_file_summary(&incoming_dir, &["_slack_message.txt"]),
        "sms" => derive_text_file_summary(&incoming_dir, &["_sms_message.txt"]),
        "bluebubbles" => derive_text_file_summary(&incoming_dir, &["_bluebubbles_message.txt"]),
        "telegram" => derive_header_text_file_summary(&incoming_dir, &["_telegram.txt"]),
        "whatsapp" => derive_header_text_file_summary(&incoming_dir, &["_whatsapp.txt"]),
        "wechat" => derive_header_text_file_summary(&incoming_dir, &["_wechat.txt"]),
        _ => None,
    }
}

fn derive_email_summary(incoming_dir: &Path) -> Option<String> {
    let payload_path = incoming_dir.join("postmark_payload.json");
    let raw_payload = fs::read_to_string(payload_path).ok()?;
    let payload_value: serde_json::Value = serde_json::from_str(&raw_payload).ok()?;

    payload_value
        .get("Subject")
        .and_then(|v| v.as_str())
        .and_then(normalize_summary_text)
        .or_else(|| {
            payload_value
                .get("StrippedTextReply")
                .and_then(|v| v.as_str())
                .and_then(normalize_summary_text)
        })
        .or_else(|| {
            payload_value
                .get("TextBody")
                .and_then(|v| v.as_str())
                .and_then(normalize_summary_text)
        })
}

fn derive_google_workspace_summary(incoming_dir: &Path, file_prefix: &str) -> Option<String> {
    let comment_suffix = format!("_{}_comment.json", file_prefix);
    if let Some(comment_path) = latest_file_with_suffix(incoming_dir, &[comment_suffix.as_str()]) {
        if let Ok(raw_comment) = fs::read_to_string(comment_path) {
            if let Ok(comment) = serde_json::from_str::<serde_json::Value>(&raw_comment) {
                if let Some(summary) = comment
                    .get("content")
                    .and_then(|v| v.as_str())
                    .and_then(normalize_summary_text)
                {
                    return Some(summary);
                }
            }
        }
    }

    let meta_suffix = format!("_{}_meta.json", file_prefix);
    let meta_path = latest_file_with_suffix(incoming_dir, &[meta_suffix.as_str()])?;
    let raw_meta = fs::read_to_string(meta_path).ok()?;
    let meta: serde_json::Value = serde_json::from_str(&raw_meta).ok()?;
    let file_name = meta.get("file_name").and_then(|v| v.as_str())?;

    normalize_summary_text(&format!("Comment on {}", file_name))
}

fn derive_discord_summary(incoming_dir: &Path) -> Option<String> {
    let raw = read_latest_text_by_suffix(incoming_dir, &["_discord_message.txt"])?;
    if let Some((_, user_section)) = raw.split_once("User message:\n") {
        if let Some(summary) = normalize_summary_text(user_section) {
            return Some(summary);
        }
    }
    normalize_summary_text(&raw)
}

fn derive_text_file_summary(incoming_dir: &Path, suffixes: &[&str]) -> Option<String> {
    let raw = read_latest_text_by_suffix(incoming_dir, suffixes)?;
    normalize_summary_text(&raw)
}

fn derive_header_text_file_summary(incoming_dir: &Path, suffixes: &[&str]) -> Option<String> {
    let raw = read_latest_text_by_suffix(incoming_dir, suffixes)?;
    extract_header_file_body_summary(&raw).or_else(|| normalize_summary_text(&raw))
}

fn read_latest_text_by_suffix(incoming_dir: &Path, suffixes: &[&str]) -> Option<String> {
    let path = latest_file_with_suffix(incoming_dir, suffixes)?;
    fs::read_to_string(path).ok()
}

fn latest_file_with_suffix(incoming_dir: &Path, suffixes: &[&str]) -> Option<PathBuf> {
    let mut matches: Vec<(String, PathBuf)> = Vec::new();

    for entry in fs::read_dir(incoming_dir).ok()? {
        let entry = entry.ok()?;
        if !entry.file_type().ok()?.is_file() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().to_string();
        if suffixes.iter().any(|suffix| name.ends_with(suffix)) {
            matches.push((name, entry.path()));
        }
    }

    matches.sort_by(|a, b| a.0.cmp(&b.0));
    matches.pop().map(|(_, path)| path)
}

fn extract_header_file_body_summary(raw: &str) -> Option<String> {
    let mut body_started = false;

    for line in raw.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            body_started = true;
            continue;
        }

        if !body_started
            && (trimmed.starts_with("From:")
                || trimmed.starts_with("Date:")
                || trimmed.starts_with("To:")
                || trimmed.starts_with("Subject:"))
        {
            continue;
        }

        return clean_summary_line(trimmed);
    }

    None
}

fn normalize_summary_text(raw: &str) -> Option<String> {
    let first_line = raw.lines().map(str::trim).find(|line| !line.is_empty())?;
    clean_summary_line(first_line)
}

fn clean_summary_line(line: &str) -> Option<String> {
    let compact = line.split_whitespace().collect::<Vec<_>>().join(" ");
    if compact.is_empty() {
        return None;
    }
    Some(truncate_summary(&compact, REQUEST_SUMMARY_MAX_CHARS))
}

fn truncate_summary(value: &str, max_chars: usize) -> String {
    let mut chars = value.chars();
    let mut output = String::new();

    for _ in 0..max_chars {
        match chars.next() {
            Some(ch) => output.push(ch),
            None => return output,
        }
    }

    if chars.next().is_some() {
        output.push_str("...");
    }

    output
}

fn mongo_err(err: mongodb::error::Error) -> SchedulerError {
    SchedulerError::Storage(format!("mongodb error: {err}"))
}

fn mongo_config_err(err: crate::mongo_store::MongoStoreError) -> SchedulerError {
    SchedulerError::Storage(err.to_string())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use mongodb::bson::doc;
    use tempfile::TempDir;

    use super::{derive_request_summary, resolve_owner_scope};

    #[test]
    fn resolve_owner_scope_extracts_user_id() {
        let path = PathBuf::from("/tmp/runtime/users/user-123/state/tasks.db");
        let scope = resolve_owner_scope(&path);
        assert_eq!(scope.0, "user");
        assert_eq!(scope.1, "user-123");
    }

    #[test]
    fn derive_request_summary_prefers_send_email_subject() {
        let task_json = serde_json::json!({
            "kind": {
                "type": "send_email",
                "subject": "Weekly analytics summary and next actions"
            }
        })
        .to_string();
        let doc = doc! {
            "task_json": task_json,
            "channel": "email",
        };

        let summary = derive_request_summary(&doc);
        assert_eq!(
            summary.as_deref(),
            Some("Weekly analytics summary and next actions")
        );
    }

    #[test]
    fn derive_request_summary_reads_latest_slack_message() {
        let temp = TempDir::new().expect("tempdir");
        let incoming_dir = temp.path().join("incoming_email");
        fs::create_dir_all(&incoming_dir).expect("create incoming_email");
        fs::write(
            incoming_dir.join("00001_slack_message.txt"),
            "Earlier message",
        )
        .expect("write old message");
        fs::write(
            incoming_dir.join("00002_slack_message.txt"),
            "Please draft a concise project update for the team.",
        )
        .expect("write latest message");

        let task_json = serde_json::json!({
            "kind": {
                "type": "run_task",
                "workspace_dir": temp.path().to_string_lossy(),
                "channel": "slack"
            }
        })
        .to_string();
        let doc = doc! {
            "task_json": task_json,
            "channel": "slack",
        };

        let summary = derive_request_summary(&doc);
        assert_eq!(
            summary.as_deref(),
            Some("Please draft a concise project update for the team.")
        );
    }

    #[test]
    fn derive_request_summary_skips_header_lines_for_telegram_text() {
        let temp = TempDir::new().expect("tempdir");
        let incoming_dir = temp.path().join("incoming_email");
        fs::create_dir_all(&incoming_dir).expect("create incoming_email");
        fs::write(
            incoming_dir.join("0001_telegram.txt"),
            "From: User (123)\nDate: 2026-03-13T20:00:00Z\n\nReview the attached budget and flag risks.",
        )
        .expect("write telegram message");

        let task_json = serde_json::json!({
            "kind": {
                "type": "run_task",
                "workspace_dir": temp.path().to_string_lossy(),
                "channel": "telegram"
            }
        })
        .to_string();
        let doc = doc! {
            "task_json": task_json,
            "channel": "telegram",
        };

        let summary = derive_request_summary(&doc);
        assert_eq!(
            summary.as_deref(),
            Some("Review the attached budget and flag risks.")
        );
    }
}
