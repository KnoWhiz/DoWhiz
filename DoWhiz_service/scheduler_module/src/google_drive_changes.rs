//! Google Drive Change API integration for real-time comment detection.
//!
//! This module provides push notification support using Google Drive's Changes API.
//! Instead of polling every 15 seconds, it sets up webhook channels to receive
//! notifications when files change, reducing latency to near-real-time.
//!
//! ## Architecture
//!
//! 1. Register a watch channel for each monitored file using `files.watch`
//! 2. Receive push notifications at `/webhooks/google-drive-changes`
//! 3. When a notification arrives, fetch comments for that specific file
//!
//! ## Requirements
//!
//! - A publicly accessible HTTPS endpoint for the webhook
//! - Environment variable: `GOOGLE_DRIVE_WEBHOOK_URL`
//! - Channel renewal (channels expire, typically after 1 hour)
//!
//! ## Usage
//!
//! ```ignore
//! // Enable push notifications instead of polling
//! GOOGLE_DRIVE_PUSH_ENABLED=true
//! GOOGLE_DRIVE_WEBHOOK_URL=https://your-domain.com/webhooks/google-drive-changes
//! ```

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info, warn};
use uuid::Uuid;

use crate::channel::AdapterError;
use crate::google_auth::GoogleAuth;

/// Default channel expiration time (1 hour).
const DEFAULT_CHANNEL_EXPIRATION_SECS: u64 = 3600;

/// Renew channels 5 minutes before expiration.
const CHANNEL_RENEWAL_BUFFER_SECS: u64 = 300;

/// HTTP client timeout for API calls.
const API_TIMEOUT_SECS: u64 = 30;

/// Configuration for Google Drive push notifications.
#[derive(Debug, Clone)]
pub struct GoogleDriveChangesConfig {
    /// Whether push notifications are enabled.
    pub enabled: bool,
    /// Webhook URL to receive notifications.
    pub webhook_url: Option<String>,
    /// Channel expiration time in seconds.
    pub channel_expiration_secs: u64,
}

impl Default for GoogleDriveChangesConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            webhook_url: None,
            channel_expiration_secs: DEFAULT_CHANNEL_EXPIRATION_SECS,
        }
    }
}

impl GoogleDriveChangesConfig {
    /// Load configuration from environment variables.
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        let enabled = std::env::var("GOOGLE_DRIVE_PUSH_ENABLED")
            .map(|v| v.to_lowercase() == "true" || v == "1")
            .unwrap_or(false);

        let webhook_url = std::env::var("GOOGLE_DRIVE_WEBHOOK_URL").ok();

        let channel_expiration_secs = std::env::var("GOOGLE_DRIVE_CHANNEL_EXPIRATION_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(DEFAULT_CHANNEL_EXPIRATION_SECS);

        Self {
            enabled,
            webhook_url,
            channel_expiration_secs,
        }
    }

    /// Check if the configuration is valid for use.
    pub fn is_valid(&self) -> bool {
        self.enabled && self.webhook_url.is_some()
    }
}

/// Response from Google Drive files.watch API.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WatchChannelResponse {
    /// Channel ID (UUID).
    pub id: String,
    /// Resource ID being watched.
    pub resource_id: String,
    /// Resource URI.
    pub resource_uri: Option<String>,
    /// Channel expiration time (milliseconds since epoch).
    pub expiration: Option<String>,
}

/// Tracked watch channel.
#[derive(Debug, Clone)]
pub struct WatchChannel {
    /// Channel ID.
    pub id: String,
    /// File ID being watched.
    pub file_id: String,
    /// Resource ID from Google.
    pub resource_id: String,
    /// Channel expiration time.
    pub expires_at: DateTime<Utc>,
    /// When the channel was created.
    pub created_at: DateTime<Utc>,
}

impl WatchChannel {
    /// Check if the channel needs renewal.
    pub fn needs_renewal(&self) -> bool {
        let now = Utc::now();
        let renewal_time = self.expires_at - chrono::Duration::seconds(CHANNEL_RENEWAL_BUFFER_SECS as i64);
        now >= renewal_time
    }

    /// Check if the channel has expired.
    pub fn is_expired(&self) -> bool {
        Utc::now() >= self.expires_at
    }
}

/// Incoming change notification from Google.
#[derive(Debug, Clone, Deserialize)]
pub struct ChangeNotification {
    /// Channel ID that received the notification.
    pub channel_id: String,
    /// Resource ID that changed.
    pub resource_id: String,
    /// Resource state (sync, add, remove, update, trash, untrash, change).
    pub resource_state: String,
    /// Message number (for ordering).
    pub message_number: Option<String>,
}

/// Manager for Google Drive watch channels.
pub struct GoogleDriveChangesManager {
    config: GoogleDriveChangesConfig,
    auth: GoogleAuth,
    /// Active channels keyed by file_id.
    channels: Mutex<HashMap<String, WatchChannel>>,
    /// Reverse lookup: resource_id -> file_id.
    resource_to_file: Mutex<HashMap<String, String>>,
}

impl GoogleDriveChangesManager {
    /// Create a new manager.
    pub fn new(config: GoogleDriveChangesConfig, auth: GoogleAuth) -> Self {
        Self {
            config,
            auth,
            channels: Mutex::new(HashMap::new()),
            resource_to_file: Mutex::new(HashMap::new()),
        }
    }

    /// Check if push notifications are enabled and configured.
    pub fn is_enabled(&self) -> bool {
        self.config.is_valid()
    }

    /// Register a watch channel for a file.
    pub fn watch_file(&self, file_id: &str) -> Result<WatchChannel, AdapterError> {
        let webhook_url = self.config.webhook_url.as_ref()
            .ok_or_else(|| AdapterError::ConfigError("Webhook URL not configured".to_string()))?;

        let access_token = self.auth
            .get_access_token()
            .map_err(|e| AdapterError::ConfigError(e.to_string()))?;

        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(API_TIMEOUT_SECS))
            .build()
            .map_err(|e| AdapterError::ConfigError(format!("Failed to create HTTP client: {}", e)))?;

        let channel_id = Uuid::new_v4().to_string();
        let expiration_ms = (SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64)
            + (self.config.channel_expiration_secs * 1000);

        let url = format!(
            "https://www.googleapis.com/drive/v3/files/{}/watch",
            file_id
        );

        let payload = serde_json::json!({
            "id": channel_id,
            "type": "web_hook",
            "address": webhook_url,
            "expiration": expiration_ms.to_string(),
        });

        info!("Registering watch channel for file {}", file_id);

        let response = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", access_token))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .map_err(|e| AdapterError::SendError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            error!("Failed to create watch channel for {}: {} - {}", file_id, status, body);
            return Err(AdapterError::SendError(format!("HTTP {}: {}", status, body)));
        }

        let watch_response: WatchChannelResponse = response
            .json()
            .map_err(|e| AdapterError::ParseError(e.to_string()))?;

        let expires_at = watch_response.expiration
            .as_ref()
            .and_then(|e| e.parse::<i64>().ok())
            .map(|ms| DateTime::from_timestamp_millis(ms))
            .flatten()
            .unwrap_or_else(|| Utc::now() + chrono::Duration::seconds(self.config.channel_expiration_secs as i64));

        let channel = WatchChannel {
            id: watch_response.id,
            file_id: file_id.to_string(),
            resource_id: watch_response.resource_id.clone(),
            expires_at,
            created_at: Utc::now(),
        };

        // Store the channel
        if let Ok(mut channels) = self.channels.lock() {
            channels.insert(file_id.to_string(), channel.clone());
        }
        if let Ok(mut resource_map) = self.resource_to_file.lock() {
            resource_map.insert(watch_response.resource_id, file_id.to_string());
        }

        info!("Created watch channel {} for file {}, expires at {}", channel.id, file_id, expires_at);

        Ok(channel)
    }

    /// Stop watching a file.
    pub fn stop_watching(&self, file_id: &str) -> Result<(), AdapterError> {
        let channel = {
            let channels = self.channels.lock()
                .map_err(|_| AdapterError::ConfigError("Failed to lock channels".to_string()))?;
            channels.get(file_id).cloned()
        };

        let Some(channel) = channel else {
            debug!("No watch channel found for file {}", file_id);
            return Ok(());
        };

        let access_token = self.auth
            .get_access_token()
            .map_err(|e| AdapterError::ConfigError(e.to_string()))?;

        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(API_TIMEOUT_SECS))
            .build()
            .map_err(|e| AdapterError::ConfigError(format!("Failed to create HTTP client: {}", e)))?;

        let url = "https://www.googleapis.com/drive/v3/channels/stop";

        let payload = serde_json::json!({
            "id": channel.id,
            "resourceId": channel.resource_id,
        });

        let response = client
            .post(url)
            .header("Authorization", format!("Bearer {}", access_token))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .map_err(|e| AdapterError::SendError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            warn!("Failed to stop watch channel {} for {}: {} - {}", channel.id, file_id, status, body);
            // Don't return error - channel may have already expired
        }

        // Remove from maps
        if let Ok(mut channels) = self.channels.lock() {
            channels.remove(file_id);
        }
        if let Ok(mut resource_map) = self.resource_to_file.lock() {
            resource_map.remove(&channel.resource_id);
        }

        info!("Stopped watch channel {} for file {}", channel.id, file_id);

        Ok(())
    }

    /// Handle an incoming change notification.
    /// Returns the file_id that changed, if the notification is valid.
    pub fn handle_notification(&self, notification: &ChangeNotification) -> Option<String> {
        // Ignore sync notifications (sent when channel is created)
        if notification.resource_state == "sync" {
            debug!("Ignoring sync notification for channel {}", notification.channel_id);
            return None;
        }

        // Look up the file_id from the resource_id
        let file_id = {
            let resource_map = self.resource_to_file.lock().ok()?;
            resource_map.get(&notification.resource_id).cloned()
        };

        if let Some(ref file_id) = file_id {
            info!(
                "Change notification for file {}: state={}",
                file_id, notification.resource_state
            );
        } else {
            warn!(
                "Unknown resource_id {} in change notification",
                notification.resource_id
            );
        }

        file_id
    }

    /// Renew channels that are about to expire.
    pub fn renew_expiring_channels(&self) -> Result<Vec<String>, AdapterError> {
        let files_to_renew: Vec<String> = {
            let channels = self.channels.lock()
                .map_err(|_| AdapterError::ConfigError("Failed to lock channels".to_string()))?;
            channels.values()
                .filter(|c| c.needs_renewal())
                .map(|c| c.file_id.clone())
                .collect()
        };

        let mut renewed = Vec::new();

        for file_id in files_to_renew {
            // Stop old channel
            let _ = self.stop_watching(&file_id);

            // Create new channel
            match self.watch_file(&file_id) {
                Ok(_) => {
                    info!("Renewed watch channel for file {}", file_id);
                    renewed.push(file_id);
                }
                Err(e) => {
                    error!("Failed to renew watch channel for {}: {}", file_id, e);
                }
            }
        }

        Ok(renewed)
    }

    /// Get all active channels.
    pub fn get_active_channels(&self) -> Vec<WatchChannel> {
        self.channels.lock()
            .map(|c| c.values().cloned().collect())
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_from_env() {
        let config = GoogleDriveChangesConfig::default();
        assert!(!config.enabled);
        assert!(config.webhook_url.is_none());
        assert_eq!(config.channel_expiration_secs, DEFAULT_CHANNEL_EXPIRATION_SECS);
    }

    #[test]
    fn test_channel_needs_renewal() {
        let channel = WatchChannel {
            id: "test".to_string(),
            file_id: "file1".to_string(),
            resource_id: "res1".to_string(),
            expires_at: Utc::now() + chrono::Duration::seconds(100),
            created_at: Utc::now(),
        };
        // Channel with 100 seconds left should need renewal (buffer is 300 seconds)
        assert!(channel.needs_renewal());

        let channel2 = WatchChannel {
            id: "test2".to_string(),
            file_id: "file2".to_string(),
            resource_id: "res2".to_string(),
            expires_at: Utc::now() + chrono::Duration::seconds(600),
            created_at: Utc::now(),
        };
        // Channel with 600 seconds left should NOT need renewal
        assert!(!channel2.needs_renewal());
    }
}
