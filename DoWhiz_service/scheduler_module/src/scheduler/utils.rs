use chrono::{DateTime, Utc};
use std::path::Path;

use crate::channel::Channel;
use crate::google_auth::{GoogleAuth, GoogleAuthConfig};

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

/// Load or dynamically generate a Google access token for agent sandbox use.
///
/// This function first checks for a pre-generated `GOOGLE_ACCESS_TOKEN` in the environment.
/// If not found, it attempts to use OAuth credentials (`GOOGLE_CLIENT_ID`, `GOOGLE_CLIENT_SECRET`,
/// and `GOOGLE_REFRESH_TOKEN` or employee-specific `GOOGLE_REFRESH_TOKEN_{EMPLOYEE_ID}`) to
/// dynamically obtain a fresh access token.
///
/// This ensures that agents running in sandboxed environments (like Codex) can use the
/// `google-docs` CLI without requiring browser-based authentication.
pub fn load_google_access_token_from_service_env() -> Option<String> {
    // First, try to load a pre-generated access token from .env file
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let service_root = manifest_dir.parent().unwrap_or(manifest_dir);
    let env_path = service_root.join(".env");
    if let Ok(iter) = dotenvy::from_path_iter(&env_path) {
        for item in iter {
            if let Ok((key, value)) = item {
                if key == "GOOGLE_ACCESS_TOKEN" {
                    let value = value.trim();
                    if !value.is_empty() {
                        tracing::debug!("Using pre-generated GOOGLE_ACCESS_TOKEN from .env");
                        return Some(value.to_string());
                    }
                }
            }
        }
    }

    // Also check environment variable directly (may be set by container/process)
    if let Ok(token) = std::env::var("GOOGLE_ACCESS_TOKEN") {
        let token = token.trim();
        if !token.is_empty() {
            tracing::debug!("Using GOOGLE_ACCESS_TOKEN from environment variable");
            return Some(token.to_string());
        }
    }

    // No pre-generated token found, try to dynamically obtain one using OAuth credentials
    tracing::debug!("No GOOGLE_ACCESS_TOKEN found, attempting dynamic refresh via OAuth");

    // Get employee ID if available for employee-specific refresh tokens
    let employee_id = std::env::var("EMPLOYEE_ID").ok();
    let config = GoogleAuthConfig::from_env_for_employee(employee_id.as_deref());

    if !config.is_valid() {
        tracing::debug!(
            "Google OAuth credentials not configured (need GOOGLE_CLIENT_ID + \
             GOOGLE_CLIENT_SECRET + GOOGLE_REFRESH_TOKEN)"
        );
        return None;
    }

    match GoogleAuth::new(config) {
        Ok(auth) => match auth.get_access_token() {
            Ok(token) => {
                tracing::info!(
                    "Successfully obtained Google access token via OAuth refresh{}",
                    employee_id
                        .as_ref()
                        .map(|id| format!(" for employee {}", id))
                        .unwrap_or_default()
                );
                Some(token)
            }
            Err(e) => {
                tracing::warn!("Failed to refresh Google access token: {}", e);
                None
            }
        },
        Err(e) => {
            tracing::warn!("Failed to initialize Google auth: {}", e);
            None
        }
    }
}
