use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::task;
use tracing::{error, info};
use uuid::Uuid;

use crate::account_store::{AccountStore, AccountStoreError};
use crate::blob_store::BlobStore;

/// State for auth routes
#[derive(Clone)]
pub struct AuthState {
    pub account_store: Arc<AccountStore>,
    pub blob_store: Option<Arc<BlobStore>>,
    pub supabase_url: String,
}

/// Response from Supabase /auth/v1/user endpoint
#[derive(Debug, Deserialize)]
struct SupabaseUser {
    id: Uuid,
    email: Option<String>,
}

/// Extract and validate Supabase JWT, returns the auth user ID
async fn validate_supabase_token(
    supabase_url: &str,
    token: &str,
) -> Result<Uuid, (StatusCode, String)> {
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

    let user: SupabaseUser = resp.json().await.map_err(|e| {
        error!("Failed to parse Supabase user response: {}", e);
        (
            StatusCode::BAD_GATEWAY,
            "Invalid response from auth service".to_string(),
        )
    })?;

    Ok(user.id)
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

    let auth_user_id = match validate_supabase_token(&state.supabase_url, &token).await {
        Ok(id) => id,
        Err((status, msg)) => {
            return (status, Json(serde_json::json!({ "error": msg }))).into_response();
        }
    };

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
        Ok(id) => id,
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
        Ok(id) => id,
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
        Ok(id) => id,
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
        Ok(id) => id,
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
        Ok(id) => id,
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
        Ok(id) => id,
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
        .route("/auth/memo", get(get_memo))
        .with_state(state)
}
