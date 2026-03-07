//! Google Drive API operations for file sharing and permissions.
//!
//! This module provides common Drive operations that work across all
//! Google Workspace file types (Docs, Slides, Sheets).

use tracing::{error, info};

use crate::channel::AdapterError;
use crate::google_auth::GoogleAuth;

/// Role for file permissions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionRole {
    /// Can view the file
    Reader,
    /// Can comment on the file
    Commenter,
    /// Can edit the file
    Writer,
}

impl PermissionRole {
    fn as_str(&self) -> &'static str {
        match self {
            PermissionRole::Reader => "reader",
            PermissionRole::Commenter => "commenter",
            PermissionRole::Writer => "writer",
        }
    }
}

/// Result of sharing a file.
#[derive(Debug, Clone)]
pub struct ShareResult {
    /// The permission ID created
    pub permission_id: String,
    /// Email address the file was shared with
    pub email: String,
    /// Role granted
    pub role: String,
}

/// File link information.
#[derive(Debug, Clone)]
pub struct FileLinks {
    /// Web view link (for viewing/editing in browser)
    pub web_view_link: Option<String>,
    /// Web content link (for downloading, if available)
    pub web_content_link: Option<String>,
}

/// Client for Google Drive operations.
#[derive(Debug, Clone)]
pub struct GoogleDriveClient {
    auth: GoogleAuth,
}

impl GoogleDriveClient {
    pub fn new(auth: GoogleAuth) -> Self {
        Self { auth }
    }

    /// Share a file with a user by email.
    ///
    /// # Arguments
    /// * `file_id` - The ID of the file (document, presentation, or spreadsheet)
    /// * `email` - The email address to share with
    /// * `role` - The permission role to grant
    /// * `send_notification` - Whether to send an email notification to the user
    ///
    /// # Returns
    /// The share result containing the permission ID.
    pub fn share_file(
        &self,
        file_id: &str,
        email: &str,
        role: PermissionRole,
        send_notification: bool,
    ) -> Result<ShareResult, AdapterError> {
        let access_token = self
            .auth
            .get_access_token()
            .map_err(|e| AdapterError::ConfigError(e.to_string()))?;

        let client = reqwest::blocking::Client::new();

        let url = format!(
            "https://www.googleapis.com/drive/v3/files/{}/permissions?sendNotificationEmail={}",
            file_id, send_notification
        );

        let payload = serde_json::json!({
            "type": "user",
            "role": role.as_str(),
            "emailAddress": email
        });

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
            error!(
                "Failed to share file {} with {}: {} - {}",
                file_id, email, status, body
            );
            return Err(AdapterError::SendError(format!("HTTP {}: {}", status, body)));
        }

        let json: serde_json::Value = response
            .json()
            .map_err(|e| AdapterError::ParseError(e.to_string()))?;

        let permission_id = json
            .get("id")
            .and_then(|id| id.as_str())
            .unwrap_or("unknown")
            .to_string();

        info!(
            "Shared file {} with {} as {} (permission: {})",
            file_id,
            email,
            role.as_str(),
            permission_id
        );

        Ok(ShareResult {
            permission_id,
            email: email.to_string(),
            role: role.as_str().to_string(),
        })
    }

    /// Get the sharing link for a file.
    ///
    /// Returns the web view link that can be used to view/edit the file in a browser.
    pub fn get_sharing_link(&self, file_id: &str) -> Result<FileLinks, AdapterError> {
        let access_token = self
            .auth
            .get_access_token()
            .map_err(|e| AdapterError::ConfigError(e.to_string()))?;

        let client = reqwest::blocking::Client::new();

        let url = format!(
            "https://www.googleapis.com/drive/v3/files/{}?fields=webViewLink,webContentLink",
            file_id
        );

        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .map_err(|e| AdapterError::SendError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            error!(
                "Failed to get sharing link for file {}: {} - {}",
                file_id, status, body
            );
            return Err(AdapterError::SendError(format!("HTTP {}: {}", status, body)));
        }

        let json: serde_json::Value = response
            .json()
            .map_err(|e| AdapterError::ParseError(e.to_string()))?;

        let web_view_link = json
            .get("webViewLink")
            .and_then(|v| v.as_str())
            .map(String::from);

        let web_content_link = json
            .get("webContentLink")
            .and_then(|v| v.as_str())
            .map(String::from);

        info!("Got sharing link for file {}: {:?}", file_id, web_view_link);

        Ok(FileLinks {
            web_view_link,
            web_content_link,
        })
    }

    /// Make a file publicly accessible to anyone with the link.
    ///
    /// # Arguments
    /// * `file_id` - The ID of the file
    /// * `role` - The permission role for public access (typically Reader)
    pub fn make_public(&self, file_id: &str, role: PermissionRole) -> Result<(), AdapterError> {
        let access_token = self
            .auth
            .get_access_token()
            .map_err(|e| AdapterError::ConfigError(e.to_string()))?;

        let client = reqwest::blocking::Client::new();

        let url = format!(
            "https://www.googleapis.com/drive/v3/files/{}/permissions",
            file_id
        );

        let payload = serde_json::json!({
            "type": "anyone",
            "role": role.as_str()
        });

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
            error!(
                "Failed to make file {} public: {} - {}",
                file_id, status, body
            );
            return Err(AdapterError::SendError(format!("HTTP {}: {}", status, body)));
        }

        info!("Made file {} public with {} access", file_id, role.as_str());

        Ok(())
    }

    /// List permissions on a file.
    pub fn list_permissions(&self, file_id: &str) -> Result<Vec<serde_json::Value>, AdapterError> {
        let access_token = self
            .auth
            .get_access_token()
            .map_err(|e| AdapterError::ConfigError(e.to_string()))?;

        let client = reqwest::blocking::Client::new();

        let url = format!(
            "https://www.googleapis.com/drive/v3/files/{}/permissions?fields=permissions(id,type,role,emailAddress,displayName)",
            file_id
        );

        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .map_err(|e| AdapterError::SendError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            error!(
                "Failed to list permissions for file {}: {} - {}",
                file_id, status, body
            );
            return Err(AdapterError::SendError(format!("HTTP {}: {}", status, body)));
        }

        let json: serde_json::Value = response
            .json()
            .map_err(|e| AdapterError::ParseError(e.to_string()))?;

        let permissions = json
            .get("permissions")
            .and_then(|p| p.as_array())
            .cloned()
            .unwrap_or_default();

        Ok(permissions)
    }

    /// Remove a permission from a file.
    pub fn remove_permission(
        &self,
        file_id: &str,
        permission_id: &str,
    ) -> Result<(), AdapterError> {
        let access_token = self
            .auth
            .get_access_token()
            .map_err(|e| AdapterError::ConfigError(e.to_string()))?;

        let client = reqwest::blocking::Client::new();

        let url = format!(
            "https://www.googleapis.com/drive/v3/files/{}/permissions/{}",
            file_id, permission_id
        );

        let response = client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .map_err(|e| AdapterError::SendError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            error!(
                "Failed to remove permission {} from file {}: {} - {}",
                permission_id, file_id, status, body
            );
            return Err(AdapterError::SendError(format!("HTTP {}: {}", status, body)));
        }

        info!(
            "Removed permission {} from file {}",
            permission_id, file_id
        );

        Ok(())
    }
}
