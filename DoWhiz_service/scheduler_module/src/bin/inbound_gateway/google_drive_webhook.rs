//! Google Drive push notification webhook handler.
//!
//! Receives notifications from Google Drive when watched files change,
//! then triggers immediate comment polling for the affected file.

use std::sync::Arc;

use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;
use tracing::{debug, info, warn};

use scheduler_module::google_drive_changes::ChangeNotification;

use super::state::GatewayState;

/// Handle Google Drive push notification webhook.
///
/// Google sends notifications with headers:
/// - `X-Goog-Channel-ID`: The channel ID we registered
/// - `X-Goog-Resource-ID`: The resource ID being watched
/// - `X-Goog-Resource-State`: sync, add, remove, update, trash, untrash, change
/// - `X-Goog-Message-Number`: Message sequence number
///
/// On receiving a notification, we trigger an immediate poll for comments
/// on the affected file, reducing latency from 15s polling to near-real-time.
pub(super) async fn handle_google_drive_webhook(
    State(state): State<Arc<GatewayState>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    // Extract notification details from headers
    let channel_id = headers
        .get("X-Goog-Channel-ID")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();

    let resource_id = headers
        .get("X-Goog-Resource-ID")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();

    let resource_state = headers
        .get("X-Goog-Resource-State")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();

    let message_number = headers
        .get("X-Goog-Message-Number")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    if channel_id.is_empty() || resource_id.is_empty() {
        warn!("Google Drive webhook missing required headers");
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"status": "missing_headers"})),
        );
    }

    let notification = ChangeNotification {
        channel_id,
        resource_id,
        resource_state,
        message_number,
    };

    debug!(
        "Google Drive notification: channel={}, resource={}, state={}",
        notification.channel_id, notification.resource_id, notification.resource_state
    );

    // Check if we have a drive changes manager
    let Some(ref manager) = state.drive_changes_manager else {
        // Push notifications not enabled, just acknowledge
        return (StatusCode::OK, Json(json!({"status": "ok"})));
    };

    // Handle the notification - returns the file_id if valid
    let file_id = manager.handle_notification(&notification);

    if let Some(file_id) = file_id {
        info!(
            "Google Drive change detected for file {}, triggering immediate poll",
            file_id
        );

        // Notify the workspace poller to poll this file immediately
        if let Some(ref notifier) = state.drive_change_notifier {
            if let Err(e) = notifier.send(file_id.clone()) {
                warn!("Failed to notify workspace poller of file change: {}", e);
            }
        }

        (
            StatusCode::OK,
            Json(json!({"status": "ok", "file_id": file_id})),
        )
    } else {
        // Unknown resource or sync notification
        (StatusCode::OK, Json(json!({"status": "ok"})))
    }
}
