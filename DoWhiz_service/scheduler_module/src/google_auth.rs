//! Google OAuth 2.0 authentication management.
//!
//! This module provides OAuth 2.0 token management for Google APIs,
//! supporting both service account (with Domain-Wide Delegation) and user OAuth flows.

use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tracing::{debug, error, info};

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
    /// Subject email for Domain-Wide Delegation (the user to impersonate)
    subject: Option<String>,
    /// OAuth scopes to request
    scopes: Vec<String>,
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
    /// Subject email for Domain-Wide Delegation (the user to impersonate)
    /// Required when using service account with DWD to act as a specific user.
    pub subject: Option<String>,
    /// OAuth scopes to request (defaults to GOOGLE_WORKSPACE_SCOPES if not specified)
    pub scopes: Option<Vec<String>>,
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
            // Subject for Domain-Wide Delegation (user to impersonate)
            subject: std::env::var("GOOGLE_SERVICE_ACCOUNT_SUBJECT").ok(),
            scopes: None, // Use default scopes
            // Pre-generated access token (for sandbox environments)
            access_token: std::env::var("GOOGLE_ACCESS_TOKEN").ok(),
        }
    }

    /// Load configuration with employee-specific OAuth credentials.
    ///
    /// For service account with DWD: looks for GOOGLE_SERVICE_ACCOUNT_SUBJECT_{EMPLOYEE_ID}
    /// to determine which user to impersonate. Falls back to GOOGLE_SERVICE_ACCOUNT_SUBJECT.
    ///
    /// For OAuth: looks for employee-specific refresh token first (GOOGLE_REFRESH_TOKEN_{EMPLOYEE_ID_UPPERCASE}),
    /// then falls back to the global GOOGLE_REFRESH_TOKEN.
    ///
    /// Example: For employee_id "little_bear", looks for:
    ///   - Service account subject: GOOGLE_SERVICE_ACCOUNT_SUBJECT_LITTLE_BEAR or GOOGLE_SERVICE_ACCOUNT_SUBJECT
    ///   - OAuth refresh token: GOOGLE_REFRESH_TOKEN_LITTLE_BEAR or GOOGLE_REFRESH_TOKEN
    pub fn from_env_for_employee(employee_id: Option<&str>) -> Self {
        let (refresh_token, subject) = if let Some(emp_id) = employee_id {
            let emp_id_upper = emp_id.to_uppercase();

            // Try employee-specific refresh token
            let refresh_env = format!("GOOGLE_REFRESH_TOKEN_{}", emp_id_upper);
            let refresh = std::env::var(&refresh_env).ok().or_else(|| {
                tracing::debug!(
                    "No employee-specific token {} found, falling back to GOOGLE_REFRESH_TOKEN",
                    refresh_env
                );
                std::env::var("GOOGLE_REFRESH_TOKEN").ok()
            });

            // Try employee-specific subject for DWD
            let subject_env = format!("GOOGLE_SERVICE_ACCOUNT_SUBJECT_{}", emp_id_upper);
            let subj = std::env::var(&subject_env).ok().or_else(|| {
                tracing::debug!(
                    "No employee-specific subject {} found, falling back to GOOGLE_SERVICE_ACCOUNT_SUBJECT",
                    subject_env
                );
                std::env::var("GOOGLE_SERVICE_ACCOUNT_SUBJECT").ok()
            });

            (refresh, subj)
        } else {
            (
                std::env::var("GOOGLE_REFRESH_TOKEN").ok(),
                std::env::var("GOOGLE_SERVICE_ACCOUNT_SUBJECT").ok(),
            )
        };

        Self {
            client_id: std::env::var("GOOGLE_CLIENT_ID").ok(),
            client_secret: std::env::var("GOOGLE_CLIENT_SECRET").ok(),
            refresh_token,
            service_account_json: std::env::var("GOOGLE_SERVICE_ACCOUNT_JSON").ok(),
            subject,
            scopes: None, // Use default scopes
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
            (
                Some(token.clone()),
                Some(Instant::now() + Duration::from_secs(3600)),
            )
        } else {
            (None, None)
        };

        // Use provided scopes or default to workspace scopes
        let scopes = config
            .scopes
            .unwrap_or_else(|| GOOGLE_WORKSPACE_SCOPES.iter().map(|s| s.to_string()).collect());

        Ok(Self {
            inner: Arc::new(RwLock::new(GoogleAuthInner {
                client_id: config.client_id,
                client_secret: config.client_secret,
                refresh_token: config.refresh_token,
                service_account_json: config.service_account_json,
                subject: config.subject,
                scopes,
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
            if let (Some(token), Some(expires_at)) = (&inner.access_token, &inner.token_expires_at)
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

        // Try service account first (preferred for production)
        if let Some(ref service_account_json) = inner.service_account_json {
            let service_account_json = service_account_json.clone();
            let subject = inner.subject.clone();
            let scopes = inner.scopes.clone();
            drop(inner); // Release read lock before acquiring write lock
            return self.refresh_via_service_account(&service_account_json, subject.as_deref(), &scopes);
        }

        // Fall back to OAuth refresh token
        if let (Some(ref client_id), Some(ref client_secret), Some(ref refresh_token)) =
            (&inner.client_id, &inner.client_secret, &inner.refresh_token)
        {
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

    /// Refresh token using service account credentials with optional Domain-Wide Delegation.
    ///
    /// When `subject` is provided, the service account will impersonate that user
    /// (requires DWD to be configured in Google Workspace Admin Console).
    fn refresh_via_service_account(
        &self,
        service_account_json: &str,
        subject: Option<&str>,
        scopes: &[String],
    ) -> Result<String, GoogleAuthError> {
        debug!(
            "Refreshing Google token via service account{}",
            subject.map_or(String::new(), |s| format!(" (impersonating {})", s))
        );

        // Parse service account JSON
        let sa_info: ServiceAccountInfo = serde_json::from_str(service_account_json)
            .map_err(|e| GoogleAuthError::JsonError(format!("Invalid service account JSON: {}", e)))?;

        // Create JWT claims
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| GoogleAuthError::ServiceAccountAuthFailed(e.to_string()))?
            .as_secs() as i64;

        let scope_str = scopes.join(" ");

        let claims = ServiceAccountClaims {
            iss: sa_info.client_email.clone(),
            sub: subject.map(|s| s.to_string()),
            scope: scope_str,
            aud: "https://oauth2.googleapis.com/token".to_string(),
            iat: now,
            exp: now + 3600, // 1 hour
        };

        // Sign JWT with RS256
        let header = Header::new(Algorithm::RS256);
        let key = EncodingKey::from_rsa_pem(sa_info.private_key.as_bytes())
            .map_err(|e| GoogleAuthError::ServiceAccountAuthFailed(format!("Invalid private key: {}", e)))?;

        let jwt = encode(&header, &claims, &key)
            .map_err(|e| GoogleAuthError::ServiceAccountAuthFailed(format!("JWT signing failed: {}", e)))?;

        // Exchange JWT for access token
        let client = reqwest::blocking::Client::new();
        let response = client
            .post("https://oauth2.googleapis.com/token")
            .form(&[
                ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
                ("assertion", &jwt),
            ])
            .send()
            .map_err(|e| GoogleAuthError::HttpError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            error!("Service account token exchange failed: {} - {}", status, body);
            return Err(GoogleAuthError::ServiceAccountAuthFailed(format!(
                "HTTP {}: {}",
                status, body
            )));
        }

        let token_response: ServiceAccountTokenResponse = response
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

        info!(
            "Service account token refreshed successfully{}",
            subject.map_or(String::new(), |s| format!(" (as {})", s))
        );
        Ok(access_token)
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

/// Service account JSON structure.
#[derive(Debug, Deserialize)]
struct ServiceAccountInfo {
    client_email: String,
    private_key: String,
    #[allow(dead_code)]
    project_id: Option<String>,
}

/// JWT claims for service account authentication.
#[derive(Debug, Serialize)]
struct ServiceAccountClaims {
    iss: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    sub: Option<String>,
    scope: String,
    aud: String,
    iat: i64,
    exp: i64,
}

/// Token response from service account JWT exchange.
#[derive(Debug, Deserialize)]
struct ServiceAccountTokenResponse {
    access_token: String,
    expires_in: i64,
    #[allow(dead_code)]
    token_type: String,
}

/// Scopes required for Google Docs collaboration (legacy alias).
pub const GOOGLE_DOCS_SCOPES: &[&str] = &[
    "https://www.googleapis.com/auth/documents",
    "https://www.googleapis.com/auth/drive",
    "https://www.googleapis.com/auth/drive.file",
];

/// Full scopes for Google Workspace operations (Docs, Sheets, Slides, Drive).
pub const GOOGLE_WORKSPACE_SCOPES: &[&str] = &[
    "https://www.googleapis.com/auth/documents",
    "https://www.googleapis.com/auth/drive",
    "https://www.googleapis.com/auth/spreadsheets",
    "https://www.googleapis.com/auth/presentations",
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
            subject: None,
            scopes: None,
            access_token: None,
        };
        assert!(oauth_config.is_valid());

        let service_account_config = GoogleAuthConfig {
            client_id: None,
            client_secret: None,
            refresh_token: None,
            service_account_json: Some("{}".to_string()),
            subject: Some("user@example.com".to_string()),
            scopes: None,
            access_token: None,
        };
        assert!(service_account_config.is_valid());
    }

    #[test]
    fn test_service_account_claims_serialization() {
        let claims = ServiceAccountClaims {
            iss: "sa@project.iam.gserviceaccount.com".to_string(),
            sub: Some("user@example.com".to_string()),
            scope: "https://www.googleapis.com/auth/drive".to_string(),
            aud: "https://oauth2.googleapis.com/token".to_string(),
            iat: 1234567890,
            exp: 1234571490,
        };
        let json = serde_json::to_string(&claims).unwrap();
        assert!(json.contains("\"sub\":\"user@example.com\""));
        assert!(json.contains("\"iss\":\"sa@project.iam.gserviceaccount.com\""));
    }

    #[test]
    fn test_service_account_claims_without_subject() {
        let claims = ServiceAccountClaims {
            iss: "sa@project.iam.gserviceaccount.com".to_string(),
            sub: None,
            scope: "https://www.googleapis.com/auth/drive".to_string(),
            aud: "https://oauth2.googleapis.com/token".to_string(),
            iat: 1234567890,
            exp: 1234571490,
        };
        let json = serde_json::to_string(&claims).unwrap();
        // sub should be omitted when None
        assert!(!json.contains("\"sub\""));
    }
}
