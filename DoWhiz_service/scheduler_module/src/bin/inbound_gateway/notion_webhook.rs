//! Notion webhook handler for real-time comment notifications.
//!
//! Receives `comment.created` events from Notion webhooks and triggers
//! task processing. This is more reliable than email notifications
//! because webhooks have retry guarantees and don't batch/merge events.
//!
//! ## Setup
//!
//! 1. Configure webhook in Notion integration settings
//! 2. Set `NOTION_WEBHOOK_SECRET` env var for signature verification
//! 3. Subscribe to `comment.created` events
//!
//! ## Event Flow
//!
//! 1. Notion sends POST to `/webhook/notion` with event payload
//! 2. Verify signature using HMAC-SHA256
//! 3. For `comment.created`: extract page_id, fetch OAuth token
//! 4. Schedule RunTask for the agent to handle the comment

use std::sync::Arc;

use axum::body::Bytes;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use chrono::Utc;
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::Sha256;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

use scheduler_module::channel::{Channel, ChannelMetadata};
use scheduler_module::ingestion::{IngestionEnvelope, IngestionPayload};
use scheduler_module::raw_payload_store;

use super::state::GatewayState;

type HmacSha256 = Hmac<Sha256>;

/// Notion webhook event types we handle.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum NotionEventType {
    #[serde(rename = "comment.created")]
    CommentCreated,
    #[serde(rename = "page.created")]
    PageCreated,
    #[serde(rename = "page.content_updated")]
    PageContentUpdated,
    #[serde(other)]
    Unknown,
}

/// Entity reference in Notion webhook event.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NotionEntity {
    pub id: String,
    #[serde(rename = "type")]
    pub entity_type: Option<String>,
}

/// Parent reference in Notion webhook data.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NotionDataParent {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub parent_type: Option<String>,
}

/// Data payload for webhook events (sparse - only contains parent reference).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NotionWebhookData {
    pub parent: Option<NotionDataParent>,
}

/// Author in webhook event.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NotionWebhookAuthor {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub author_type: Option<String>,
}

/// Generic Notion webhook event payload.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NotionWebhookEvent {
    #[serde(rename = "type")]
    pub event_type: NotionEventType,
    pub data: Option<NotionWebhookData>,
    pub entity: Option<NotionEntity>,
    pub timestamp: Option<String>,
    pub workspace_id: Option<String>,
    pub subscription_id: Option<String>,
    pub integration_id: Option<String>,
    pub authors: Option<Vec<NotionWebhookAuthor>>,
}

/// Verification request from Notion during webhook setup.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NotionVerificationRequest {
    pub verification_token: Option<String>,
}

/// Verify Notion webhook signature.
///
/// Notion signs webhooks with HMAC-SHA256 using the webhook secret.
/// The signature is in the `X-Notion-Signature` header.
fn verify_notion_signature(headers: &HeaderMap, body: &[u8]) -> Result<(), &'static str> {
    let secret = match std::env::var("NOTION_WEBHOOK_SECRET") {
        Ok(s) if !s.is_empty() => s,
        _ => {
            warn!("NOTION_WEBHOOK_SECRET not set, skipping signature verification");
            return Ok(()); // Allow in development, but log warning
        }
    };

    let signature = headers
        .get("X-Notion-Signature")
        .and_then(|v| v.to_str().ok())
        .ok_or("missing X-Notion-Signature header")?;

    // Notion signature format: "v1=<hmac_hex>"
    let expected_sig = signature
        .strip_prefix("v1=")
        .ok_or("invalid signature format")?;

    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .map_err(|_| "invalid webhook secret")?;
    mac.update(body);
    let computed = hex::encode(mac.finalize().into_bytes());

    if computed != expected_sig {
        warn!(
            "Notion webhook signature mismatch: expected={}, computed={}",
            expected_sig, computed
        );
        return Err("signature mismatch");
    }

    debug!("Notion webhook signature verified");
    Ok(())
}

/// Extract plain text from Notion rich_text array.
fn extract_plain_text(rich_text: &[serde_json::Value]) -> String {
    rich_text
        .iter()
        .filter_map(|item| item.get("plain_text").and_then(|v| v.as_str()))
        .collect::<Vec<_>>()
        .join("")
}

/// Handle incoming Notion webhook.
pub(super) async fn handle_notion_webhook(
    State(state): State<Arc<GatewayState>>,
    headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    // Log receipt
    info!(
        "Notion webhook received: {} bytes",
        body.len()
    );

    // Verify signature
    if let Err(reason) = verify_notion_signature(&headers, &body) {
        error!("Notion webhook signature verification failed: {}", reason);
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({"status": "unauthorized", "reason": reason})),
        );
    }

    // Try to parse as verification request first
    if let Ok(verification) = serde_json::from_slice::<NotionVerificationRequest>(&body) {
        if let Some(token) = verification.verification_token {
            info!("Notion webhook verification token received: {}", token);
            // Return the token for manual verification in Notion UI
            // Note: Notion expects you to paste this token in their UI
            return (
                StatusCode::OK,
                Json(json!({
                    "status": "verification_received",
                    "verification_token": token,
                    "message": "Copy this token to Notion integration settings to complete verification"
                })),
            );
        }
    }

    // Parse the event payload
    let event: NotionWebhookEvent = match serde_json::from_slice(&body) {
        Ok(e) => e,
        Err(e) => {
            warn!(
                "Failed to parse Notion webhook payload: {} - body preview: {}",
                e,
                String::from_utf8_lossy(&body[..body.len().min(500)])
            );
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"status": "bad_json", "error": e.to_string()})),
            );
        }
    };

    info!(
        "Notion webhook event: type={:?} workspace_id={:?}",
        event.event_type, event.workspace_id
    );

    // Handle based on event type
    match event.event_type {
        NotionEventType::CommentCreated => {
            handle_comment_created(state, &event, &body).await
        }
        NotionEventType::PageCreated | NotionEventType::PageContentUpdated => {
            // Acknowledge but don't process page events for now
            info!("Notion page event received but not processed: {:?}", event.event_type);
            (StatusCode::OK, Json(json!({"status": "acknowledged"})))
        }
        NotionEventType::Unknown => {
            debug!("Unknown Notion event type, acknowledging");
            (StatusCode::OK, Json(json!({"status": "unknown_event"})))
        }
    }
}

/// Handle comment.created event.
async fn handle_comment_created(
    state: Arc<GatewayState>,
    event: &NotionWebhookEvent,
    _raw_body: &[u8],
) -> (StatusCode, Json<serde_json::Value>) {
    // Extract comment ID from entity
    let comment_id = match event.entity.as_ref() {
        Some(entity) => entity.id.clone(),
        None => {
            warn!("comment.created event missing entity field");
            return (
                StatusCode::OK,
                Json(json!({"status": "missing_entity"})),
            );
        }
    };

    // Extract page_id from data.parent
    let page_id = event
        .data
        .as_ref()
        .and_then(|d| d.parent.as_ref())
        .and_then(|p| p.id.clone());

    let Some(page_id) = page_id else {
        warn!("comment.created event missing page_id in data.parent");
        return (
            StatusCode::OK,
            Json(json!({"status": "missing_page_id"})),
        );
    };

    // Extract workspace_id from event
    let workspace_id = event.workspace_id.clone();

    // Extract author ID from authors array
    let author_id = event
        .authors
        .as_ref()
        .and_then(|authors| authors.first())
        .and_then(|a| a.id.clone());

    // Note: Notion webhooks use sparse payloads - we don't have comment text here
    // The agent will need to fetch the comment via API using the comment_id

    info!(
        "Notion comment.created: comment_id={} page_id={} author_id={:?} workspace_id={:?}",
        comment_id,
        page_id,
        author_id,
        workspace_id
    );

    // Find employee to route to
    // For Notion webhooks, we use workspace_id or a default employee
    let employee_id = resolve_notion_employee(&state, workspace_id.as_deref());

    let Some(employee_id) = employee_id else {
        info!("No employee configured for Notion webhook");
        return (
            StatusCode::OK,
            Json(json!({"status": "no_route"})),
        );
    };

    // Build ingestion envelope
    // Note: comment_text is empty - agent will fetch via Notion API
    let envelope = build_notion_envelope(
        &employee_id,
        &page_id,
        workspace_id.as_deref(),
        None, // author_name not available in sparse payload
        author_id.as_deref(),
        "", // comment_text not available - agent fetches via API
        &comment_id,
    )
    .await;

    let envelope = match envelope {
        Ok(e) => e,
        Err(e) => {
            error!("Failed to build Notion envelope: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "envelope_error", "error": e.to_string()})),
            );
        }
    };

    // Enqueue for processing
    match state.queue.enqueue(&envelope) {
        Ok(_result) => {
            info!(
                "Notion webhook enqueued: employee={} page_id={} comment_id={}",
                employee_id, page_id, comment_id
            );
            (
                StatusCode::OK,
                Json(json!({
                    "status": "enqueued",
                    "employee_id": employee_id,
                    "page_id": page_id,
                    "comment_id": comment_id
                })),
            )
        }
        Err(e) => {
            error!("Failed to enqueue Notion webhook: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "enqueue_failed", "error": format!("{}", e)})),
            )
        }
    }
}

/// Resolve which employee should handle this Notion webhook.
fn resolve_notion_employee(state: &GatewayState, workspace_id: Option<&str>) -> Option<String> {
    // Check for workspace-specific mapping via env var
    if let Some(ws_id) = workspace_id {
        let env_key = format!(
            "NOTION_WORKSPACE_{}_EMPLOYEE",
            ws_id.replace('-', "_").to_uppercase()
        );
        if let Ok(employee) = std::env::var(&env_key) {
            if !employee.is_empty() {
                return Some(employee);
            }
        }
    }

    // Check for default Notion employee
    if let Ok(employee) = std::env::var("NOTION_DEFAULT_EMPLOYEE") {
        if !employee.is_empty() {
            return Some(employee);
        }
    }

    // Fall back to gateway default
    state
        .config
        .defaults
        .employee_id
        .clone()
        .or_else(|| state.employee_directory.default_employee_id.clone())
}

/// Build an ingestion envelope for a Notion comment event.
async fn build_notion_envelope(
    employee_id: &str,
    page_id: &str,
    workspace_id: Option<&str>,
    author_name: Option<&str>,
    author_id: Option<&str>,
    comment_text: &str,
    comment_id: &str,
) -> Result<IngestionEnvelope, Box<dyn std::error::Error + Send + Sync>> {
    let envelope_id = Uuid::new_v4();
    let now = Utc::now();

    // Build a NotionMention JSON structure that the worker expects
    // This matches the NotionMention struct in notion_browser/models.rs
    let notion_mention = json!({
        "id": comment_id,
        "workspace_id": workspace_id.unwrap_or("unknown"),
        "workspace_name": workspace_id.unwrap_or("Unknown Workspace"),
        "page_id": page_id,
        "page_title": format!("Page {}", page_id),
        "block_id": null,
        "comment_id": comment_id,
        "sender_name": author_name.unwrap_or("Unknown Notion User"),
        "sender_id": author_id,
        "comment_text": comment_text,
        "thread_context": [],
        "url": format!("https://notion.so/{}", page_id.replace('-', "")),
        "detected_at": now.to_rfc3339()
    });

    let mention_bytes = serde_json::to_vec(&notion_mention)?;

    // Store the NotionMention JSON as the raw payload
    let raw_payload_ref = match raw_payload_store::upload_raw_payload(
        envelope_id,
        now,
        &mention_bytes,
    )
    .await
    {
        Ok(url) => Some(url),
        Err(e) => {
            warn!("Failed to store Notion webhook raw payload: {}", e);
            None
        }
    };

    // Build channel metadata
    let metadata = ChannelMetadata {
        notion_workspace_id: workspace_id.map(String::from),
        notion_page_id: Some(page_id.to_string()),
        notion_comment_id: Some(comment_id.to_string()),
        ..Default::default()
    };

    // Build payload
    let sender = author_name.unwrap_or("Unknown Notion User");
    let payload = IngestionPayload {
        sender: sender.to_string(),
        sender_name: author_name.map(String::from),
        recipient: employee_id.to_string(),
        subject: Some(format!("Notion comment on page {}", page_id)),
        text_body: Some(comment_text.to_string()),
        html_body: None,
        thread_id: format!("notion:page:{}", page_id),
        message_id: Some(comment_id.to_string()),
        attachments: Vec::new(),
        reply_to: vec![sender.to_string()],
        metadata,
    };

    Ok(IngestionEnvelope {
        envelope_id,
        received_at: now,
        tenant_id: Some("default".to_string()),
        employee_id: employee_id.to_string(),
        channel: Channel::Notion,
        external_message_id: Some(comment_id.to_string()),
        dedupe_key: format!("notion:comment:{}", comment_id),
        payload,
        raw_payload_ref,
        account_id: None,
    })
}
