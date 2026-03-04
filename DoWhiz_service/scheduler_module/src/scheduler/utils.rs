use chrono::{DateTime, Utc};
use std::path::Path;

use crate::channel::Channel;

use super::types::{SchedulerError, TaskKind};

pub(crate) fn task_kind_label(kind: &TaskKind) -> &'static str {
    match kind {
        TaskKind::SendReply(_) => "send_email",
        TaskKind::RunTask(_) => "run_task",
        TaskKind::Noop => "noop",
    }
}

pub(crate) fn task_kind_channel(kind: &TaskKind) -> Channel {
    match kind {
        TaskKind::SendReply(send) => send.channel.clone(),
        TaskKind::RunTask(run) => run.channel.clone(),
        TaskKind::Noop => Channel::default(),
    }
}

pub(crate) fn parse_datetime(value: &str) -> Result<DateTime<Utc>, SchedulerError> {
    Ok(DateTime::parse_from_rfc3339(value)?.with_timezone(&Utc))
}

pub fn load_google_access_token_from_service_env() -> Option<String> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let service_root = manifest_dir.parent().unwrap_or(manifest_dir);
    let env_path = service_root.join(".env");
    let iter = dotenvy::from_path_iter(&env_path).ok()?;
    for item in iter {
        if let Ok((key, value)) = item {
            if key == "GOOGLE_ACCESS_TOKEN" {
                let value = value.trim();
                if value.is_empty() {
                    return None;
                }
                return Some(value.to_string());
            }
        }
    }
    None
}
