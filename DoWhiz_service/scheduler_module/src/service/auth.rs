use axum::extract::{Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Redirect};
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use base64::Engine;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::task;
use tracing::{error, info, warn};
use uuid::Uuid;

use crate::account_store::{AccountStore, AccountStoreError};
use crate::blob_store::BlobStore;
use crate::user_store::UserStore;
use crate::{load_tasks_with_status, TaskStatusSummary};

/// State for auth routes
#[derive(Clone)]
pub struct AuthState {
    pub account_store: Arc<AccountStore>,
    pub blob_store: Option<Arc<BlobStore>>,
    pub supabase_url: String,
    // Discord OAuth config
    pub discord_client_id: Option<String>,
    pub discord_client_secret: Option<String>,
    pub discord_redirect_uri: Option<String>,
    // Slack OAuth config
    pub slack_client_id: Option<String>,
    pub slack_client_secret: Option<String>,
    pub slack_redirect_uri: Option<String>,
    // Frontend URL for redirects after OAuth
    pub frontend_url: String,
    // User store and paths for task lookups
    pub user_store: Option<Arc<UserStore>>,
    pub users_root: Option<std::path::PathBuf>,
}

/// JWT Claims from Supabase token
#[derive(Debug, Deserialize)]
struct JwtClaims {
    sub: Uuid,           // User ID
    exp: usize,          // Expiration time
    #[serde(default)]
    aud: Option<String>, // Audience (optional)
    #[serde(default)]
    email: Option<String>, // User's email from Supabase
}

/// Authenticated user info extracted from token
struct AuthUser {
    id: Uuid,
    email: Option<String>,
}

/// Cached JWT secret for local verification
fn get_jwt_secret() -> Option<String> {
    std::env::var("SUPABASE_JWT_SECRET").ok()
}

/// Extract and validate Supabase JWT locally, returns the auth user ID and email
/// This avoids an HTTP round-trip to Supabase on every request.
async fn validate_supabase_token(
    supabase_url: &str,
    token: &str,
) -> Result<AuthUser, (StatusCode, String)> {
    // Try local JWT verification first (fast path)
    if let Some(secret) = get_jwt_secret() {
        let key = DecodingKey::from_secret(secret.as_bytes());
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_aud = false; // Supabase doesn't always set aud

        match decode::<JwtClaims>(token, &key, &validation) {
            Ok(token_data) => {
                return Ok(AuthUser {
                    id: token_data.claims.sub,
                    email: token_data.claims.email,
                });
            }
            Err(e) => {
                warn!("Local JWT validation failed: {}", e);
                // Fall through to remote validation
            }
        }
    }

    // Fallback: validate via Supabase API (slow path)
    // This handles cases where JWT_SECRET isn't configured or token format differs
    let client = reqwest::Client::new();
    let resp = client
        .get(format!("{}/auth/v1/user", supabase_url))
        .header("Authorization", format!("Bearer {}", token))
        .header(
            "apikey",
            std::env::var("SUPABASE_ANON_KEY").unwrap_or_default(),
        )
        .send()
        .await
        .map_err(|e| {
            error!("Failed to validate token with Supabase: {}", e);
            (
                StatusCode::BAD_GATEWAY,
                "Failed to validate token".to_string(),
            )
        })?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        error!("Supabase auth validation failed: {} - {}", status, body);
        return Err((
            StatusCode::UNAUTHORIZED,
            "Invalid or expired token".to_string(),
        ));
    }

    #[derive(Deserialize)]
    struct SupabaseUser {
        id: Uuid,
        email: Option<String>,
    }

    let user: SupabaseUser = resp.json().await.map_err(|e| {
        error!("Failed to parse Supabase user response: {}", e);
        (
            StatusCode::BAD_GATEWAY,
            "Invalid response from auth service".to_string(),
        )
    })?;

    Ok(AuthUser {
        id: user.id,
        email: user.email,
    })
}

/// Extract Bearer token from Authorization header
fn extract_bearer_token(headers: &HeaderMap) -> Option<String> {
    headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .map(|s| s.to_string())
}

// ============================================================================
// Signup
// ============================================================================

#[derive(Debug, Serialize)]
pub struct SignupResponse {
    pub account_id: Uuid,
    pub auth_user_id: Uuid,
    pub created: bool,
}

/// POST /auth/signup
/// Creates a DoWhiz account for the authenticated Supabase user.
/// Requires: Authorization: Bearer <supabase_access_token>
pub async fn signup(State(state): State<AuthState>, headers: HeaderMap) -> impl IntoResponse {
    let token = match extract_bearer_token(&headers) {
        Some(t) => t,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "error": "Missing Authorization header"
                })),
            )
                .into_response();
        }
    };

    let auth_user = match validate_supabase_token(&state.supabase_url, &token).await {
        Ok(user) => user,
        Err((status, msg)) => {
            return (status, Json(serde_json::json!({ "error": msg }))).into_response();
        }
    };
    let auth_user_id = auth_user.id;
    let auth_email = auth_user.email.clone();

    // Check if account already exists (run on blocking thread)
    let store = state.account_store.clone();
    let existing = task::spawn_blocking(move || store.get_account_by_auth_user(auth_user_id))
        .await
        .map_err(|e| {
            error!("spawn_blocking panicked: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Internal error" })),
            )
        });

    let existing = match existing {
        Ok(Ok(existing)) => existing,
        Ok(Err(e)) => {
            error!("Failed to check existing account: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Database error"
                })),
            )
                .into_response();
        }
        Err(resp) => return resp.into_response(),
    };

    if let Some(existing) = existing {
        info!("Account already exists for auth_user_id={}", auth_user_id);
        return (
            StatusCode::OK,
            Json(SignupResponse {
                account_id: existing.id,
                auth_user_id: existing.auth_user_id,
                created: false,
            }),
        )
            .into_response();
    }

    // Create new account (run on blocking thread)
    let store = state.account_store.clone();
    let result = task::spawn_blocking(move || store.create_account(auth_user_id))
        .await
        .map_err(|e| {
            error!("spawn_blocking panicked: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Internal error" })),
            )
        });

    match result {
        Ok(Ok(account)) => {
            info!(
                "Created account {} for auth_user_id={}",
                account.id, auth_user_id
            );

            // Auto-link the auth email as an identifier
            if let Some(email) = auth_email {
                let store = state.account_store.clone();
                let account_id = account.id;
                let email_clone = email.clone();
                let link_result = task::spawn_blocking(move || {
                    store.create_identifier(account_id, "email", &email_clone)
                })
                .await;

                match link_result {
                    Ok(Ok(_)) => {
                        info!("Auto-linked email {} to account {}", email, account.id);
                    }
                    Ok(Err(AccountStoreError::IdentifierTaken)) => {
                        warn!("Email {} already linked to another account", email);
                    }
                    Ok(Err(e)) => {
                        warn!("Failed to auto-link email {}: {}", email, e);
                    }
                    Err(e) => {
                        warn!("spawn_blocking panicked during email link: {}", e);
                    }
                }
            }

            (
                StatusCode::CREATED,
                Json(SignupResponse {
                    account_id: account.id,
                    auth_user_id: account.auth_user_id,
                    created: true,
                }),
            )
                .into_response()
        }
        Ok(Err(e)) => {
            error!("Failed to create account: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to create account"
                })),
            )
                .into_response()
        }
        Err(resp) => resp.into_response(),
    }
}

// ============================================================================
// Get Account
// ============================================================================

#[derive(Debug, Serialize)]
pub struct AccountResponse {
    pub account_id: Uuid,
    pub auth_user_id: Uuid,
    pub identifiers: Vec<IdentifierResponse>,
}

#[derive(Debug, Serialize)]
pub struct IdentifierResponse {
    pub identifier_type: String,
    pub identifier: String,
    pub verified: bool,
}

/// GET /auth/account
/// Returns the current user's account and linked identifiers.
pub async fn get_account(State(state): State<AuthState>, headers: HeaderMap) -> impl IntoResponse {
    let token = match extract_bearer_token(&headers) {
        Some(t) => t,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "error": "Missing Authorization header"
                })),
            )
                .into_response();
        }
    };

    let auth_user_id = match validate_supabase_token(&state.supabase_url, &token).await {
        Ok(user) => user.id,
        Err((status, msg)) => {
            return (status, Json(serde_json::json!({ "error": msg }))).into_response();
        }
    };

    // Get account (run on blocking thread)
    let store = state.account_store.clone();
    let account_result = task::spawn_blocking(move || store.get_account_by_auth_user(auth_user_id))
        .await
        .map_err(|e| {
            error!("spawn_blocking panicked: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Internal error" })),
            )
        });

    let account = match account_result {
        Ok(Ok(Some(acc))) => acc,
        Ok(Ok(None)) => {
            return (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "error": "Account not found. Please sign up first."
                })),
            )
                .into_response();
        }
        Ok(Err(e)) => {
            error!("Failed to get account: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Database error"
                })),
            )
                .into_response();
        }
        Err(resp) => return resp.into_response(),
    };

    // List identifiers (run on blocking thread)
    let account_id = account.id;
    let store = state.account_store.clone();
    let identifiers_result = task::spawn_blocking(move || store.list_identifiers(account_id))
        .await
        .map_err(|e| {
            error!("spawn_blocking panicked: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Internal error" })),
            )
        });

    let identifiers = match identifiers_result {
        Ok(Ok(ids)) => ids,
        Ok(Err(e)) => {
            error!("Failed to list identifiers: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Database error"
                })),
            )
                .into_response();
        }
        Err(resp) => return resp.into_response(),
    };

    (
        StatusCode::OK,
        Json(AccountResponse {
            account_id: account.id,
            auth_user_id: account.auth_user_id,
            identifiers: identifiers
                .into_iter()
                .map(|i| IdentifierResponse {
                    identifier_type: i.identifier_type,
                    identifier: i.identifier,
                    verified: i.verified,
                })
                .collect(),
        }),
    )
        .into_response()
}

// ============================================================================
// Link Identifier
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct LinkRequest {
    pub identifier_type: String,
    pub identifier: String,
}

#[derive(Debug, Serialize)]
pub struct LinkResponse {
    pub identifier_type: String,
    pub identifier: String,
    pub verified: bool,
    pub message: String,
}

/// POST /auth/link
/// Start linking a channel identifier to the account.
/// For now, creates an unverified link. Verification can be added later.
pub async fn link_identifier(
    State(state): State<AuthState>,
    headers: HeaderMap,
    Json(req): Json<LinkRequest>,
) -> impl IntoResponse {
    let token = match extract_bearer_token(&headers) {
        Some(t) => t,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "error": "Missing Authorization header"
                })),
            )
                .into_response();
        }
    };

    let auth_user_id = match validate_supabase_token(&state.supabase_url, &token).await {
        Ok(user) => user.id,
        Err((status, msg)) => {
            return (status, Json(serde_json::json!({ "error": msg }))).into_response();
        }
    };

    // Get account (run on blocking thread)
    let store = state.account_store.clone();
    let account_result = task::spawn_blocking(move || store.get_account_by_auth_user(auth_user_id))
        .await
        .map_err(|e| {
            error!("spawn_blocking panicked: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Internal error" })),
            )
        });

    let account = match account_result {
        Ok(Ok(Some(acc))) => acc,
        Ok(Ok(None)) => {
            return (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "error": "Account not found. Please sign up first."
                })),
            )
                .into_response();
        }
        Ok(Err(e)) => {
            error!("Failed to get account: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Database error"
                })),
            )
                .into_response();
        }
        Err(resp) => return resp.into_response(),
    };

    // Create identifier (run on blocking thread)
    let account_id = account.id;
    let identifier_type = req.identifier_type.clone();
    let identifier = req.identifier.clone();
    let store = state.account_store.clone();
    let create_result = task::spawn_blocking(move || {
        store.create_identifier(account_id, &identifier_type, &identifier)
    })
    .await
    .map_err(|e| {
        error!("spawn_blocking panicked: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": "Internal error" })),
        )
    });

    match create_result {
        Ok(Ok(identifier)) => {
            info!(
                "Linked identifier {}:{} to account {}",
                req.identifier_type, req.identifier, account.id
            );
            // TODO: Send verification code for phone/email channels
            (
                StatusCode::CREATED,
                Json(LinkResponse {
                    identifier_type: identifier.identifier_type,
                    identifier: identifier.identifier,
                    verified: identifier.verified,
                    message: "Identifier linked. Verification may be required.".to_string(),
                }),
            )
                .into_response()
        }
        Ok(Err(AccountStoreError::IdentifierTaken)) => (
            StatusCode::CONFLICT,
            Json(serde_json::json!({
                "error": "This identifier is already linked to another account"
            })),
        )
            .into_response(),
        Ok(Err(e)) => {
            error!("Failed to link identifier: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to link identifier"
                })),
            )
                .into_response()
        }
        Err(resp) => resp.into_response(),
    }
}

// ============================================================================
// Verify Identifier
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct VerifyRequest {
    pub identifier_type: String,
    pub identifier: String,
    pub code: String,
}

/// POST /auth/verify
/// Verify an identifier with a verification code.
/// For now, accepts any code and marks as verified (placeholder).
pub async fn verify_identifier(
    State(state): State<AuthState>,
    headers: HeaderMap,
    Json(req): Json<VerifyRequest>,
) -> impl IntoResponse {
    let token = match extract_bearer_token(&headers) {
        Some(t) => t,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "error": "Missing Authorization header"
                })),
            )
                .into_response();
        }
    };

    let auth_user_id = match validate_supabase_token(&state.supabase_url, &token).await {
        Ok(user) => user.id,
        Err((status, msg)) => {
            return (status, Json(serde_json::json!({ "error": msg }))).into_response();
        }
    };

    // Get account (run on blocking thread)
    let store = state.account_store.clone();
    let account_result = task::spawn_blocking(move || store.get_account_by_auth_user(auth_user_id))
        .await
        .map_err(|e| {
            error!("spawn_blocking panicked: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Internal error" })),
            )
        });

    let account = match account_result {
        Ok(Ok(Some(acc))) => acc,
        Ok(Ok(None)) => {
            return (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "error": "Account not found"
                })),
            )
                .into_response();
        }
        Ok(Err(e)) => {
            error!("Failed to get account: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Database error"
                })),
            )
                .into_response();
        }
        Err(resp) => return resp.into_response(),
    };

    // TODO: Actually validate the verification code
    // For now, just mark as verified
    let account_id = account.id;
    let identifier_type = req.identifier_type.clone();
    let identifier = req.identifier.clone();
    let store = state.account_store.clone();
    let verify_result = task::spawn_blocking(move || {
        store.verify_identifier(account_id, &identifier_type, &identifier)
    })
    .await
    .map_err(|e| {
        error!("spawn_blocking panicked: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": "Internal error" })),
        )
    });

    match verify_result {
        Ok(Ok(())) => {
            info!(
                "Verified identifier {}:{} for account {}",
                req.identifier_type, req.identifier, account.id
            );
            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "verified": true,
                    "message": "Identifier verified successfully"
                })),
            )
                .into_response()
        }
        Ok(Err(AccountStoreError::NotFound)) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": "Identifier not found"
            })),
        )
            .into_response(),
        Ok(Err(e)) => {
            error!("Failed to verify identifier: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to verify identifier"
                })),
            )
                .into_response()
        }
        Err(resp) => resp.into_response(),
    }
}

// ============================================================================
// Unlink Identifier
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct UnlinkRequest {
    pub identifier_type: String,
    pub identifier: String,
}

/// DELETE /auth/unlink
/// Remove a linked identifier from the account.
pub async fn unlink_identifier(
    State(state): State<AuthState>,
    headers: HeaderMap,
    Json(req): Json<UnlinkRequest>,
) -> impl IntoResponse {
    let token = match extract_bearer_token(&headers) {
        Some(t) => t,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "error": "Missing Authorization header"
                })),
            )
                .into_response();
        }
    };

    let auth_user_id = match validate_supabase_token(&state.supabase_url, &token).await {
        Ok(user) => user.id,
        Err((status, msg)) => {
            return (status, Json(serde_json::json!({ "error": msg }))).into_response();
        }
    };

    // Get account (run on blocking thread)
    let store = state.account_store.clone();
    let account_result = task::spawn_blocking(move || store.get_account_by_auth_user(auth_user_id))
        .await
        .map_err(|e| {
            error!("spawn_blocking panicked: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Internal error" })),
            )
        });

    let account = match account_result {
        Ok(Ok(Some(acc))) => acc,
        Ok(Ok(None)) => {
            return (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "error": "Account not found"
                })),
            )
                .into_response();
        }
        Ok(Err(e)) => {
            error!("Failed to get account: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Database error"
                })),
            )
                .into_response();
        }
        Err(resp) => return resp.into_response(),
    };

    // Delete identifier (run on blocking thread)
    let account_id = account.id;
    let identifier_type = req.identifier_type.clone();
    let identifier = req.identifier.clone();
    let store = state.account_store.clone();
    let delete_result = task::spawn_blocking(move || {
        store.delete_identifier(account_id, &identifier_type, &identifier)
    })
    .await
    .map_err(|e| {
        error!("spawn_blocking panicked: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": "Internal error" })),
        )
    });

    match delete_result {
        Ok(Ok(())) => {
            info!(
                "Unlinked identifier {}:{} from account {}",
                req.identifier_type, req.identifier, account.id
            );
            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "message": "Identifier unlinked successfully"
                })),
            )
                .into_response()
        }
        Ok(Err(AccountStoreError::NotFound)) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": "Identifier not found"
            })),
        )
            .into_response(),
        Ok(Err(e)) => {
            error!("Failed to unlink identifier: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to unlink identifier"
                })),
            )
                .into_response()
        }
        Err(resp) => resp.into_response(),
    }
}

// ============================================================================
// Delete Account
// ============================================================================

/// DELETE /auth/account
/// Delete the current user's DoWhiz account and all linked identifiers.
pub async fn delete_account(
    State(state): State<AuthState>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let token = match extract_bearer_token(&headers) {
        Some(t) => t,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "error": "Missing Authorization header"
                })),
            )
                .into_response();
        }
    };

    let auth_user_id = match validate_supabase_token(&state.supabase_url, &token).await {
        Ok(user) => user.id,
        Err((status, msg)) => {
            return (status, Json(serde_json::json!({ "error": msg }))).into_response();
        }
    };

    // Get account (run on blocking thread)
    let store = state.account_store.clone();
    let account_result = task::spawn_blocking(move || store.get_account_by_auth_user(auth_user_id))
        .await
        .map_err(|e| {
            error!("spawn_blocking panicked: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Internal error" })),
            )
        });

    let account = match account_result {
        Ok(Ok(Some(acc))) => acc,
        Ok(Ok(None)) => {
            return (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "error": "Account not found"
                })),
            )
                .into_response();
        }
        Ok(Err(e)) => {
            error!("Failed to get account: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Database error"
                })),
            )
                .into_response();
        }
        Err(resp) => return resp.into_response(),
    };

    // Delete account (run on blocking thread)
    let account_id = account.id;
    let store = state.account_store.clone();
    let delete_result = task::spawn_blocking(move || store.delete_account(account_id))
        .await
        .map_err(|e| {
            error!("spawn_blocking panicked: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Internal error" })),
            )
        });

    match delete_result {
        Ok(Ok(())) => {
            info!(
                "Deleted account {} for auth_user_id={}",
                account_id, auth_user_id
            );
            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "message": "Account deleted successfully"
                })),
            )
                .into_response()
        }
        Ok(Err(AccountStoreError::NotFound)) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": "Account not found"
            })),
        )
            .into_response(),
        Ok(Err(e)) => {
            error!("Failed to delete account: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to delete account"
                })),
            )
                .into_response()
        }
        Err(resp) => resp.into_response(),
    }
}

// ============================================================================
// Get Memo
// ============================================================================

#[derive(Debug, Serialize)]
pub struct MemoResponse {
    pub account_id: Uuid,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct MemoUpdateRequest {
    pub content: String,
}

/// GET /auth/memo
/// Returns the memo.md content for the current user's account.
pub async fn get_memo(State(state): State<AuthState>, headers: HeaderMap) -> impl IntoResponse {
    let token = match extract_bearer_token(&headers) {
        Some(t) => t,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "error": "Missing Authorization header"
                })),
            )
                .into_response();
        }
    };

    let auth_user_id = match validate_supabase_token(&state.supabase_url, &token).await {
        Ok(user) => user.id,
        Err((status, msg)) => {
            return (status, Json(serde_json::json!({ "error": msg }))).into_response();
        }
    };

    // Get account (run on blocking thread)
    let store = state.account_store.clone();
    let account_result = task::spawn_blocking(move || store.get_account_by_auth_user(auth_user_id))
        .await
        .map_err(|e| {
            error!("spawn_blocking panicked: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Internal error" })),
            )
        });

    let account = match account_result {
        Ok(Ok(Some(acc))) => acc,
        Ok(Ok(None)) => {
            return (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "error": "Account not found. Please sign up first."
                })),
            )
                .into_response();
        }
        Ok(Err(e)) => {
            error!("Failed to get account: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Database error"
                })),
            )
                .into_response();
        }
        Err(resp) => return resp.into_response(),
    };

    // Check if blob store is available
    let blob_store = match &state.blob_store {
        Some(store) => store.clone(),
        None => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(serde_json::json!({
                    "error": "Memo storage not configured"
                })),
            )
                .into_response();
        }
    };

    // Read memo from blob storage
    let account_id = account.id;
    match blob_store.read_memo(account_id).await {
        Ok(content) => {
            info!("Retrieved memo for account {}", account_id);
            (
                StatusCode::OK,
                Json(MemoResponse {
                    account_id,
                    content,
                }),
            )
                .into_response()
        }
        Err(e) => {
            error!("Failed to read memo for account {}: {}", account_id, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to read memo"
                })),
            )
                .into_response()
        }
    }
}

/// POST /auth/memo
/// Updates the memo.md content for the current user's account.
pub async fn update_memo(
    State(state): State<AuthState>,
    headers: HeaderMap,
    Json(payload): Json<MemoUpdateRequest>,
) -> impl IntoResponse {
    let token = match extract_bearer_token(&headers) {
        Some(t) => t,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "error": "Missing Authorization header"
                })),
            )
                .into_response();
        }
    };

    let auth_user_id = match validate_supabase_token(&state.supabase_url, &token).await {
        Ok(user) => user.id,
        Err((status, msg)) => {
            return (status, Json(serde_json::json!({ "error": msg }))).into_response();
        }
    };

    // Get account (run on blocking thread)
    let store = state.account_store.clone();
    let account_result = task::spawn_blocking(move || store.get_account_by_auth_user(auth_user_id))
        .await
        .map_err(|e| {
            error!("spawn_blocking panicked: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Internal error" })),
            )
        });

    let account = match account_result {
        Ok(Ok(Some(acc))) => acc,
        Ok(Ok(None)) => {
            return (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "error": "Account not found. Please sign up first."
                })),
            )
                .into_response();
        }
        Ok(Err(e)) => {
            error!("Failed to get account: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Database error"
                })),
            )
                .into_response();
        }
        Err(resp) => return resp.into_response(),
    };

    // Check if blob store is available
    let blob_store = match &state.blob_store {
        Some(store) => store.clone(),
        None => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(serde_json::json!({
                    "error": "Memo storage not configured"
                })),
            )
                .into_response();
        }
    };

    // Write memo directly to blob storage
    let account_id = account.id;
    match blob_store.write_memo(account_id, &payload.content).await {
        Ok(()) => {
            info!("Updated memo for account {}", account_id);
            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "success": true,
                    "account_id": account_id
                })),
            )
                .into_response()
        }
        Err(e) => {
            error!("Failed to write memo for account {}: {}", account_id, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to save memo"
                })),
            )
                .into_response()
        }
    }
}

// ============================================================================
// Discord OAuth
// ============================================================================

/// Query params for Discord OAuth callback
#[derive(Debug, Deserialize)]
pub struct DiscordCallbackQuery {
    pub code: String,
    pub state: String,
}

/// Discord token response
#[derive(Debug, Deserialize)]
struct DiscordTokenResponse {
    access_token: String,
    token_type: String,
}

/// Discord user response
#[derive(Debug, Deserialize)]
struct DiscordUser {
    id: String,
    username: String,
}

/// GET /auth/discord
/// Initiates Discord OAuth flow - redirects to Discord's authorization page.
pub async fn discord_oauth_start(
    State(state): State<AuthState>,
    headers: HeaderMap,
) -> impl IntoResponse {
    // Check if Discord OAuth is configured
    let (client_id, redirect_uri) = match (&state.discord_client_id, &state.discord_redirect_uri) {
        (Some(id), Some(uri)) => (id.clone(), uri.clone()),
        _ => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(serde_json::json!({
                    "error": "Discord OAuth not configured"
                })),
            )
                .into_response();
        }
    };

    // Extract and validate Supabase token
    let token = match extract_bearer_token(&headers) {
        Some(t) => t,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "error": "Missing Authorization header"
                })),
            )
                .into_response();
        }
    };

    // Validate the token to ensure user is authenticated
    if let Err((status, msg)) = validate_supabase_token(&state.supabase_url, &token).await {
        return (status, Json(serde_json::json!({ "error": msg }))).into_response();
    }

    // Encode the Supabase token in state so we can identify the user on callback
    let encoded_state = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(token.as_bytes());

    // Build Discord OAuth URL
    let discord_auth_url = format!(
        "https://discord.com/api/oauth2/authorize?client_id={}&redirect_uri={}&response_type=code&scope=identify&state={}",
        client_id,
        urlencoding::encode(&redirect_uri),
        encoded_state
    );

    // Return the URL for the frontend to redirect to
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "redirect_url": discord_auth_url
        })),
    )
        .into_response()
}

/// GET /auth/discord/callback
/// Handles Discord OAuth callback - exchanges code for token, gets user info, links account.
pub async fn discord_oauth_callback(
    State(state): State<AuthState>,
    Query(params): Query<DiscordCallbackQuery>,
) -> impl IntoResponse {
    // Helper to build redirect URLs to the frontend
    let frontend_url = state.frontend_url.clone();
    let redirect_to = |path: &str| -> axum::response::Response {
        Redirect::to(&format!("{}{}", frontend_url, path)).into_response()
    };

    // Check if Discord OAuth is configured
    let (client_id, client_secret, redirect_uri) = match (
        &state.discord_client_id,
        &state.discord_client_secret,
        &state.discord_redirect_uri,
    ) {
        (Some(id), Some(secret), Some(uri)) => (id.clone(), secret.clone(), uri.clone()),
        _ => {
            return redirect_to("/auth/index.html?discord=error&reason=not_configured");
        }
    };

    // Decode state to get the Supabase token
    let token = match base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(&params.state) {
        Ok(bytes) => match String::from_utf8(bytes) {
            Ok(t) => t,
            Err(_) => {
                return redirect_to("/auth/index.html?discord=error&reason=invalid_state");
            }
        },
        Err(_) => {
            return redirect_to("/auth/index.html?discord=error&reason=invalid_state");
        }
    };

    // Validate Supabase token and get user
    let auth_user_id = match validate_supabase_token(&state.supabase_url, &token).await {
        Ok(user) => user.id,
        Err(_) => {
            return redirect_to("/auth/index.html?discord=error&reason=invalid_token");
        }
    };

    // Exchange code for Discord access token
    let client = reqwest::Client::new();
    let token_res = client
        .post("https://discord.com/api/oauth2/token")
        .form(&[
            ("client_id", client_id.as_str()),
            ("client_secret", client_secret.as_str()),
            ("code", params.code.as_str()),
            ("grant_type", "authorization_code"),
            ("redirect_uri", redirect_uri.as_str()),
        ])
        .send()
        .await;

    let discord_token = match token_res {
        Ok(res) if res.status().is_success() => match res.json::<DiscordTokenResponse>().await {
            Ok(t) => t,
            Err(e) => {
                error!("Failed to parse Discord token response: {}", e);
                return redirect_to("/auth/index.html?discord=error&reason=token_parse_error");
            }
        },
        Ok(res) => {
            error!("Discord token exchange failed: {}", res.status());
            return redirect_to("/auth/index.html?discord=error&reason=token_exchange_failed");
        }
        Err(e) => {
            error!("Discord token request failed: {}", e);
            return redirect_to("/auth/index.html?discord=error&reason=token_request_failed");
        }
    };

    // Get Discord user info
    let user_res = client
        .get("https://discord.com/api/users/@me")
        .header(
            "Authorization",
            format!("{} {}", discord_token.token_type, discord_token.access_token),
        )
        .send()
        .await;

    let discord_user = match user_res {
        Ok(res) if res.status().is_success() => match res.json::<DiscordUser>().await {
            Ok(u) => u,
            Err(e) => {
                error!("Failed to parse Discord user response: {}", e);
                return redirect_to("/auth/index.html?discord=error&reason=user_parse_error");
            }
        },
        Ok(res) => {
            error!("Discord user request failed: {}", res.status());
            return redirect_to("/auth/index.html?discord=error&reason=user_request_failed");
        }
        Err(e) => {
            error!("Discord user request failed: {}", e);
            return redirect_to("/auth/index.html?discord=error&reason=user_request_failed");
        }
    };

    info!(
        "Discord OAuth successful for user {} (Discord: {} / {})",
        auth_user_id, discord_user.id, discord_user.username
    );

    // Get user's account
    let store = state.account_store.clone();
    let account_result =
        task::spawn_blocking(move || store.get_account_by_auth_user(auth_user_id)).await;

    let account = match account_result {
        Ok(Ok(Some(acc))) => acc,
        Ok(Ok(None)) => {
            return redirect_to("/auth/index.html?discord=error&reason=account_not_found");
        }
        Ok(Err(e)) => {
            error!("Failed to get account: {}", e);
            return redirect_to("/auth/index.html?discord=error&reason=db_error");
        }
        Err(e) => {
            error!("spawn_blocking panicked: {}", e);
            return redirect_to("/auth/index.html?discord=error&reason=internal_error");
        }
    };

    // Link Discord ID to account
    let store = state.account_store.clone();
    let discord_id = discord_user.id.clone();
    let link_result = task::spawn_blocking(move || {
        store.create_identifier(account.id, "discord", &discord_id)
    })
    .await;

    match link_result {
        Ok(Ok(_identifier)) => {
            info!(
                "Linked Discord {} to account {}",
                discord_user.id, account.id
            );
            redirect_to("/auth/index.html?discord=success")
        }
        Ok(Err(AccountStoreError::IdentifierTaken)) => {
            redirect_to("/auth/index.html?discord=error&reason=already_linked")
        }
        Ok(Err(e)) => {
            error!("Failed to link Discord: {}", e);
            redirect_to("/auth/index.html?discord=error&reason=link_failed")
        }
        Err(e) => {
            error!("spawn_blocking panicked: {}", e);
            redirect_to("/auth/index.html?discord=error&reason=internal_error")
        }
    }
}

// ============================================================================
// Slack OAuth
// ============================================================================

/// Query params for Slack OAuth callback
#[derive(Debug, Deserialize)]
pub struct SlackCallbackQuery {
    pub code: String,
    pub state: String,
}

/// Slack OAuth token response
#[derive(Debug, Deserialize)]
struct SlackTokenResponse {
    ok: bool,
    error: Option<String>,
    authed_user: Option<SlackAuthedUser>,
}

#[derive(Debug, Deserialize)]
struct SlackAuthedUser {
    id: String,
    access_token: Option<String>,
}

/// GET /auth/slack
/// Initiates Slack OAuth flow - redirects to Slack's authorization page.
pub async fn slack_oauth_start(
    State(state): State<AuthState>,
    headers: HeaderMap,
) -> impl IntoResponse {
    // Check if Slack OAuth is configured
    let (client_id, redirect_uri) = match (&state.slack_client_id, &state.slack_redirect_uri) {
        (Some(id), Some(uri)) => (id.clone(), uri.clone()),
        _ => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(serde_json::json!({
                    "error": "Slack OAuth not configured"
                })),
            )
                .into_response();
        }
    };

    // Extract and validate Supabase token
    let token = match extract_bearer_token(&headers) {
        Some(t) => t,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "error": "Missing Authorization header"
                })),
            )
                .into_response();
        }
    };

    // Validate the token to ensure user is authenticated
    if let Err((status, msg)) = validate_supabase_token(&state.supabase_url, &token).await {
        return (status, Json(serde_json::json!({ "error": msg }))).into_response();
    }

    // Encode the Supabase token in state so we can identify the user on callback
    let encoded_state = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(token.as_bytes());

    // Build Slack OAuth URL - using user_scope for user identity
    let slack_auth_url = format!(
        "https://slack.com/oauth/v2/authorize?client_id={}&user_scope=identity.basic&redirect_uri={}&state={}",
        client_id,
        urlencoding::encode(&redirect_uri),
        encoded_state
    );

    // Return the URL for the frontend to redirect to
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "redirect_url": slack_auth_url
        })),
    )
        .into_response()
}

/// GET /auth/slack/callback
/// Handles Slack OAuth callback - exchanges code for token, gets user info, links account.
pub async fn slack_oauth_callback(
    State(state): State<AuthState>,
    Query(params): Query<SlackCallbackQuery>,
) -> impl IntoResponse {
    // Helper to build redirect URLs to the frontend
    let frontend_url = state.frontend_url.clone();
    let redirect_to = |path: &str| -> axum::response::Response {
        Redirect::to(&format!("{}{}", frontend_url, path)).into_response()
    };

    // Check if Slack OAuth is configured
    let (client_id, client_secret, redirect_uri) = match (
        &state.slack_client_id,
        &state.slack_client_secret,
        &state.slack_redirect_uri,
    ) {
        (Some(id), Some(secret), Some(uri)) => (id.clone(), secret.clone(), uri.clone()),
        _ => {
            return redirect_to("/auth/index.html?slack=error&reason=not_configured");
        }
    };

    // Decode state to get the Supabase token
    let token = match base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(&params.state) {
        Ok(bytes) => match String::from_utf8(bytes) {
            Ok(t) => t,
            Err(_) => {
                return redirect_to("/auth/index.html?slack=error&reason=invalid_state");
            }
        },
        Err(_) => {
            return redirect_to("/auth/index.html?slack=error&reason=invalid_state");
        }
    };

    // Validate Supabase token and get user
    let auth_user_id = match validate_supabase_token(&state.supabase_url, &token).await {
        Ok(user) => user.id,
        Err(_) => {
            return redirect_to("/auth/index.html?slack=error&reason=invalid_token");
        }
    };

    // Exchange code for Slack access token
    let client = reqwest::Client::new();
    let token_res = client
        .post("https://slack.com/api/oauth.v2.access")
        .form(&[
            ("client_id", client_id.as_str()),
            ("client_secret", client_secret.as_str()),
            ("code", params.code.as_str()),
            ("redirect_uri", redirect_uri.as_str()),
        ])
        .send()
        .await;

    let slack_response = match token_res {
        Ok(res) => match res.json::<SlackTokenResponse>().await {
            Ok(r) => r,
            Err(e) => {
                error!("Failed to parse Slack token response: {}", e);
                return redirect_to("/auth/index.html?slack=error&reason=token_parse_error");
            }
        },
        Err(e) => {
            error!("Slack token request failed: {}", e);
            return redirect_to("/auth/index.html?slack=error&reason=token_request_failed");
        }
    };

    // Check if Slack returned an error
    if !slack_response.ok {
        let error_msg = slack_response.error.unwrap_or_else(|| "unknown".to_string());
        error!("Slack OAuth error: {}", error_msg);
        return redirect_to(&format!(
            "/auth/index.html?slack=error&reason={}",
            urlencoding::encode(&error_msg)
        ));
    }

    // Get the user ID from the response
    let slack_user = match slack_response.authed_user {
        Some(user) => user,
        None => {
            error!("Slack response missing authed_user");
            return redirect_to("/auth/index.html?slack=error&reason=missing_user");
        }
    };

    info!(
        "Slack OAuth successful for user {} (Slack ID: {})",
        auth_user_id, slack_user.id
    );

    // Get user's account
    let store = state.account_store.clone();
    let account_result =
        task::spawn_blocking(move || store.get_account_by_auth_user(auth_user_id)).await;

    let account = match account_result {
        Ok(Ok(Some(acc))) => acc,
        Ok(Ok(None)) => {
            return redirect_to("/auth/index.html?slack=error&reason=account_not_found");
        }
        Ok(Err(e)) => {
            error!("Failed to get account: {}", e);
            return redirect_to("/auth/index.html?slack=error&reason=db_error");
        }
        Err(e) => {
            error!("spawn_blocking panicked: {}", e);
            return redirect_to("/auth/index.html?slack=error&reason=internal_error");
        }
    };

    // Link Slack ID to account
    let store = state.account_store.clone();
    let slack_id = slack_user.id.clone();
    let link_result = task::spawn_blocking(move || {
        store.create_identifier(account.id, "slack", &slack_id)
    })
    .await;

    match link_result {
        Ok(Ok(_identifier)) => {
            info!("Linked Slack {} to account {}", slack_user.id, account.id);
            redirect_to("/auth/index.html?slack=success")
        }
        Ok(Err(AccountStoreError::IdentifierTaken)) => {
            redirect_to("/auth/index.html?slack=error&reason=already_linked")
        }
        Ok(Err(e)) => {
            error!("Failed to link Slack: {}", e);
            redirect_to("/auth/index.html?slack=error&reason=link_failed")
        }
        Err(e) => {
            error!("spawn_blocking panicked: {}", e);
            redirect_to("/auth/index.html?slack=error&reason=internal_error")
        }
    }
}

// ============================================================================
// Tasks
// ============================================================================

#[derive(Debug, Serialize)]
pub struct TasksResponse {
    pub tasks: Vec<TaskStatusSummary>,
}

#[derive(Debug, Deserialize)]
pub struct TasksQuery {
    pub channel: Option<String>,
    pub identifier: Option<String>,
}

/// GET /api/tasks?channel=discord&identifier=123456789
/// Returns tasks for a specific channel identifier.
pub async fn get_tasks(
    State(state): State<AuthState>,
    headers: HeaderMap,
    Query(query): Query<TasksQuery>,
) -> impl IntoResponse {
    // Validate auth token
    let token = match extract_bearer_token(&headers) {
        Some(t) => t,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "error": "Missing Authorization header"
                })),
            )
                .into_response();
        }
    };

    if let Err((status, msg)) = validate_supabase_token(&state.supabase_url, &token).await {
        return (status, Json(serde_json::json!({ "error": msg }))).into_response();
    }

    // Require channel and identifier
    let (channel, identifier) = match (query.channel, query.identifier) {
        (Some(c), Some(i)) => (c, i),
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": "Missing required query params: channel and identifier"
                })),
            )
                .into_response();
        }
    };

    // Check if user_store is configured
    let (user_store, users_root) = match (&state.user_store, &state.users_root) {
        (Some(store), Some(root)) => (store.clone(), root.clone()),
        _ => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(serde_json::json!({
                    "error": "Task storage not configured"
                })),
            )
                .into_response();
        }
    };

    // Look up user by channel + identifier
    let user_result = task::spawn_blocking({
        let user_store = user_store.clone();
        move || user_store.get_user_by_identifier(&channel, &identifier)
    })
    .await;

    let user_record = match user_result {
        Ok(Ok(Some(record))) => record,
        Ok(Ok(None)) => {
            // No user found - return empty tasks (user hasn't interacted with bot yet)
            return (StatusCode::OK, Json(TasksResponse { tasks: Vec::new() })).into_response();
        }
        Ok(Err(e)) => {
            error!("Error looking up user: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Failed to look up user" })),
            )
                .into_response();
        }
        Err(e) => {
            error!("spawn_blocking panicked: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Internal error" })),
            )
                .into_response();
        }
    };

    // Load tasks for this user
    let paths = user_store.user_paths(&users_root, &user_record.user_id);
    let tasks = load_tasks_with_status(&paths.tasks_db_path);

    (StatusCode::OK, Json(TasksResponse { tasks })).into_response()
}

// ============================================================================
// Router
// ============================================================================

pub fn auth_router(state: AuthState) -> Router {
    Router::new()
        .route("/auth/signup", post(signup))
        .route("/auth/account", get(get_account).delete(delete_account))
        .route("/auth/link", post(link_identifier))
        .route("/auth/verify", post(verify_identifier))
        .route("/auth/unlink", delete(unlink_identifier))
        .route("/auth/memo", get(get_memo).post(update_memo))
        .route("/auth/discord", get(discord_oauth_start))
        .route("/auth/discord/callback", get(discord_oauth_callback))
        .route("/auth/slack", get(slack_oauth_start))
        .route("/auth/slack/callback", get(slack_oauth_callback))
        .route("/api/tasks", get(get_tasks))
        .with_state(state)
}
