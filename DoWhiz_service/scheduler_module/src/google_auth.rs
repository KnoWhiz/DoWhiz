//! Google OAuth 2.0 authentication management.
//!
//! This module provides OAuth 2.0 token management for Google APIs,
//! supporting both service account and user OAuth flows.

use serde::Deserialize;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use tracing::{debug, error, warn};

/// Google OAuth credentials and token management.
#[derive(Debug, Clone)]
pub struct GoogleAuth {
    inner: Arc<RwLock<GoogleAuthInner>>,
}

#[derive(Debug)]
struct GoogleAuthInner {
    /// OAuth client ID
    client_id: Option<String>,
    /// OAuth client secret
    client_secret: Option<String>,
    /// Refresh token (for user OAuth flow)
    refresh_token: Option<String>,
    /// Service account JSON credentials
    service_account_json: Option<String>,
    /// Current access token
    access_token: Option<String>,
    /// Token expiration time
    token_expires_at: Option<Instant>,
}

/// Configuration for Google OAuth.
#[derive(Debug, Clone, Default)]
pub struct GoogleAuthConfig {
    /// OAuth client ID (for user OAuth flow)
    pub client_id: Option<String>,
    /// OAuth client secret (for user OAuth flow)
    pub client_secret: Option<String>,
    /// Refresh token (for user OAuth flow)
    pub refresh_token: Option<String>,
    /// Service account JSON credentials (alternative to OAuth)
    pub service_account_json: Option<String>,
    /// Pre-generated access token (for sandbox environments without network access)
    pub access_token: Option<String>,
}

impl GoogleAuthConfig {
    /// Load configuration from environment variables.
    pub fn from_env() -> Self {
        Self {
            client_id: std::env::var("GOOGLE_CLIENT_ID").ok(),
            client_secret: std::env::var("GOOGLE_CLIENT_SECRET").ok(),
            refresh_token: std::env::var("GOOGLE_REFRESH_TOKEN").ok(),
            service_account_json: std::env::var("GOOGLE_SERVICE_ACCOUNT_JSON").ok(),
            // Pre-generated access token (for sandbox environments)
            access_token: std::env::var("GOOGLE_ACCESS_TOKEN").ok(),
        }
    }

    /// Load configuration with employee-specific OAuth credentials.
    ///
    /// Looks for employee-specific refresh token first (GOOGLE_REFRESH_TOKEN_{EMPLOYEE_ID_UPPERCASE}),
    /// then falls back to the global GOOGLE_REFRESH_TOKEN.
    ///
    /// Example: For employee_id "boiled_egg", looks for GOOGLE_REFRESH_TOKEN_BOILED_EGG first.
    pub fn from_env_for_employee(employee_id: Option<&str>) -> Self {
        let refresh_token = if let Some(emp_id) = employee_id {
            // Try employee-specific token first (convert to uppercase for env var)
            let env_var_name = format!("GOOGLE_REFRESH_TOKEN_{}", emp_id.to_uppercase());
            std::env::var(&env_var_name)
                .ok()
                .or_else(|| {
                    tracing::debug!(
                        "No employee-specific token {} found, falling back to GOOGLE_REFRESH_TOKEN",
                        env_var_name
                    );
                    std::env::var("GOOGLE_REFRESH_TOKEN").ok()
                })
        } else {
            std::env::var("GOOGLE_REFRESH_TOKEN").ok()
        };

        Self {
            client_id: std::env::var("GOOGLE_CLIENT_ID").ok(),
            client_secret: std::env::var("GOOGLE_CLIENT_SECRET").ok(),
            refresh_token,
            service_account_json: std::env::var("GOOGLE_SERVICE_ACCOUNT_JSON").ok(),
            // Pre-generated access token (for sandbox environments)
            access_token: std::env::var("GOOGLE_ACCESS_TOKEN").ok(),
        }
    }

    /// Check if the configuration is valid (has required credentials).
    pub fn is_valid(&self) -> bool {
        // Pre-generated access token is valid on its own (for sandbox environments)
        self.access_token.is_some()
            // Or service account credentials
            || self.service_account_json.is_some()
            // Or full OAuth credentials for refresh
            || (self.client_id.is_some()
                && self.client_secret.is_some()
                && self.refresh_token.is_some())
    }
}

/// Error types for Google authentication.
#[derive(Debug, thiserror::Error)]
pub enum GoogleAuthError {
    #[error("missing credentials: {0}")]
    MissingCredentials(String),
    #[error("token refresh failed: {0}")]
    TokenRefreshFailed(String),
    #[error("service account auth failed: {0}")]
    ServiceAccountAuthFailed(String),
    #[error("http error: {0}")]
    HttpError(String),
    #[error("json error: {0}")]
    JsonError(String),
}

impl GoogleAuth {
    /// Create a new GoogleAuth instance from configuration.
    pub fn new(config: GoogleAuthConfig) -> Result<Self, GoogleAuthError> {
        if !config.is_valid() {
            return Err(GoogleAuthError::MissingCredentials(
                "Either GOOGLE_ACCESS_TOKEN, GOOGLE_SERVICE_ACCOUNT_JSON, or (GOOGLE_CLIENT_ID + GOOGLE_CLIENT_SECRET + GOOGLE_REFRESH_TOKEN) must be set".to_string(),
            ));
        }

        // If a pre-generated access token is provided, use it directly
        // (useful for sandbox environments without network access)
        let (access_token, token_expires_at) = if let Some(ref token) = config.access_token {
            // Pre-generated tokens are assumed valid for 1 hour
            (Some(token.clone()), Some(Instant::now() + Duration::from_secs(3600)))
        } else {
            (None, None)
        };

        Ok(Self {
            inner: Arc::new(RwLock::new(GoogleAuthInner {
                client_id: config.client_id,
                client_secret: config.client_secret,
                refresh_token: config.refresh_token,
                service_account_json: config.service_account_json,
                access_token,
                token_expires_at,
            })),
        })
    }

    /// Create a new GoogleAuth instance from environment variables.
    pub fn from_env() -> Result<Self, GoogleAuthError> {
        let config = GoogleAuthConfig::from_env();
        Self::new(config)
    }

    /// Get a valid access token, refreshing if necessary.
    pub fn get_access_token(&self) -> Result<String, GoogleAuthError> {
        // Check if we have a valid cached token
        {
            let inner = self.inner.read().unwrap();
            if let (Some(token), Some(expires_at)) =
                (&inner.access_token, &inner.token_expires_at)
            {
                // Add 60 second buffer before expiration
                if *expires_at > Instant::now() + Duration::from_secs(60) {
                    return Ok(token.clone());
                }
            }
        }

        // Need to refresh the token
        self.refresh_access_token()
    }

    /// Force refresh the access token.
    pub fn refresh_access_token(&self) -> Result<String, GoogleAuthError> {
        let inner = self.inner.read().unwrap();

        // Try service account first
        if let Some(ref service_account_json) = inner.service_account_json {
            let service_account_json = service_account_json.clone();
            drop(inner); // Release read lock before acquiring write lock
            return self.refresh_via_service_account(&service_account_json);
        }

        // Fall back to OAuth refresh token
        if let (Some(ref client_id), Some(ref client_secret), Some(ref refresh_token)) = (
            &inner.client_id,
            &inner.client_secret,
            &inner.refresh_token,
        ) {
            let client_id = client_id.clone();
            let client_secret = client_secret.clone();
            let refresh_token = refresh_token.clone();
            drop(inner); // Release read lock before acquiring write lock
            return self.refresh_via_oauth(&client_id, &client_secret, &refresh_token);
        }

        Err(GoogleAuthError::MissingCredentials(
            "No valid credentials available".to_string(),
        ))
    }

    /// Refresh token using OAuth 2.0 refresh token flow.
    fn refresh_via_oauth(
        &self,
        client_id: &str,
        client_secret: &str,
        refresh_token: &str,
    ) -> Result<String, GoogleAuthError> {
        debug!("Refreshing Google OAuth token");

        let client = reqwest::blocking::Client::new();
        let response = client
            .post("https://oauth2.googleapis.com/token")
            .form(&[
                ("client_id", client_id),
                ("client_secret", client_secret),
                ("refresh_token", refresh_token),
                ("grant_type", "refresh_token"),
            ])
            .send()
            .map_err(|e| GoogleAuthError::HttpError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            error!("OAuth token refresh failed: {} - {}", status, body);
            return Err(GoogleAuthError::TokenRefreshFailed(format!(
                "HTTP {}: {}",
                status, body
            )));
        }

        let token_response: OAuthTokenResponse = response
            .json()
            .map_err(|e| GoogleAuthError::JsonError(e.to_string()))?;

        let expires_at = Instant::now() + Duration::from_secs(token_response.expires_in as u64);
        let access_token = token_response.access_token.clone();

        // Update cached token
        {
            let mut inner = self.inner.write().unwrap();
            inner.access_token = Some(token_response.access_token);
            inner.token_expires_at = Some(expires_at);
        }

        debug!("Google OAuth token refreshed successfully");
        Ok(access_token)
    }

    /// Refresh token using service account credentials.
    fn refresh_via_service_account(
        &self,
        _service_account_json: &str,
    ) -> Result<String, GoogleAuthError> {
        // TODO: Implement JWT signing for service account authentication
        // This requires:
        // 1. Parse the service account JSON to extract private key and email
        // 2. Create a JWT with claims for the desired scopes
        // 3. Sign the JWT with the private key
        // 4. Exchange the signed JWT for an access token

        warn!("Service account authentication not yet implemented, falling back to OAuth");
        Err(GoogleAuthError::ServiceAccountAuthFailed(
            "Service account authentication not yet implemented".to_string(),
        ))
    }

    /// Check if Google Docs integration is enabled (has valid credentials).
    pub fn is_enabled(&self) -> bool {
        let inner = self.inner.read().unwrap();
        inner.service_account_json.is_some()
            || (inner.client_id.is_some()
                && inner.client_secret.is_some()
                && inner.refresh_token.is_some())
    }
}

#[derive(Debug, Deserialize)]
struct OAuthTokenResponse {
    access_token: String,
    expires_in: i64,
    #[allow(dead_code)]
    token_type: String,
    #[allow(dead_code)]
    scope: Option<String>,
}

/// Scopes required for Google Docs collaboration.
pub const GOOGLE_DOCS_SCOPES: &[&str] = &[
    "https://www.googleapis.com/auth/documents",
    "https://www.googleapis.com/auth/drive",
    "https://www.googleapis.com/auth/drive.file",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_validation() {
        let empty_config = GoogleAuthConfig::default();
        assert!(!empty_config.is_valid());

        let oauth_config = GoogleAuthConfig {
            client_id: Some("client_id".to_string()),
            client_secret: Some("client_secret".to_string()),
            refresh_token: Some("refresh_token".to_string()),
            service_account_json: None,
            access_token: None,
        };
        assert!(oauth_config.is_valid());

        let service_account_config = GoogleAuthConfig {
            client_id: None,
            client_secret: None,
            refresh_token: None,
            service_account_json: Some("{}".to_string()),
            access_token: None,
        };
        assert!(service_account_config.is_valid());
    }
}
