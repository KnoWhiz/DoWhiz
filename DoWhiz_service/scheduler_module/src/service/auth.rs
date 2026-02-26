use axum::extract::{Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Redirect};
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use base64::Engine;
use chrono::Utc;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::task;
use tracing::{error, info, warn};
use uuid::Uuid;

use crate::account_store::{AccountStore, AccountStoreError};
use crate::blob_store::BlobStore;
use crate::gtm_agents::{
    AccountSignal, AgentId, AgentTaskEnvelope, ChannelPolicy, ClaimRisk, GtmChannel,
    HubspotModeAExecutor, IcpScoutInput, IcpTier, MessageBundle, MessageVariant, ModeAAgentEngine,
    ModeAOutboundDispatchInput, ModeAOutboundDispatchOutput, OutboundSdrInput, Phase1AgentEngine,
    PolicyPack, SegmentContact, SequencePolicy, TaskPriority,
};
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
    sub: Uuid,  // User ID
    exp: usize, // Expiration time
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

async fn resolve_authenticated_account_id(
    state: &AuthState,
    headers: &HeaderMap,
) -> Result<Uuid, axum::response::Response> {
    let token = match extract_bearer_token(headers) {
        Some(t) => t,
        None => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "error": "Missing Authorization header"
                })),
            )
                .into_response());
        }
    };

    let auth_user = match validate_supabase_token(&state.supabase_url, &token).await {
        Ok(user) => user,
        Err((status, msg)) => {
            return Err((status, Json(serde_json::json!({ "error": msg }))).into_response());
        }
    };

    let store = state.account_store.clone();
    let account_lookup = task::spawn_blocking(move || store.get_account_by_auth_user(auth_user.id))
        .await
        .map_err(|err| {
            error!("spawn_blocking panicked: {}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Internal error" })),
            )
                .into_response()
        })?;

    match account_lookup {
        Ok(Some(account)) => Ok(account.id),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": "No DoWhiz account found. Complete sign-in again to provision account."
            })),
        )
            .into_response()),
        Err(err) => {
            error!("Failed to resolve account: {}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Database error" })),
            )
                .into_response())
        }
    }
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

    // For email type, create a verification token and send email
    if req.identifier_type == "email" {
        let account_id = account.id;
        let email = req.identifier.clone();
        let store = state.account_store.clone();
        let frontend_url = state.frontend_url.clone();

        // Create verification token
        let token_result =
            task::spawn_blocking(move || store.create_email_verification_token(account_id, &email))
                .await
                .map_err(|e| {
                    error!("spawn_blocking panicked: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({ "error": "Internal error" })),
                    )
                });

        let verification_token = match token_result {
            Ok(Ok(token)) => token,
            Ok(Err(e)) => {
                error!("Failed to create verification token: {}", e);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({
                        "error": "Failed to create verification token"
                    })),
                )
                    .into_response();
            }
            Err(resp) => return resp.into_response(),
        };

        // Send verification email
        let verify_url = format!(
            "{}/auth/index.html?verify_email={}",
            frontend_url, verification_token.token
        );

        if let Err(e) = send_verification_email(&verification_token.email, &verify_url).await {
            error!("Failed to send verification email: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to send verification email"
                })),
            )
                .into_response();
        }

        info!(
            "Sent verification email to {} for account {}",
            verification_token.email, account.id
        );

        return (
            StatusCode::OK,
            Json(LinkResponse {
                identifier_type: req.identifier_type,
                identifier: req.identifier,
                verified: false,
                message: "Verification email sent. Please check your inbox.".to_string(),
            }),
        )
            .into_response();
    }

    // For other types (discord, slack, phone, etc.), create identifier directly
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
            (
                StatusCode::CREATED,
                Json(LinkResponse {
                    identifier_type: identifier.identifier_type,
                    identifier: identifier.identifier,
                    verified: identifier.verified,
                    message: "Identifier linked.".to_string(),
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
            format!(
                "{} {}",
                discord_token.token_type, discord_token.access_token
            ),
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
    let link_result =
        task::spawn_blocking(move || store.create_identifier(account.id, "discord", &discord_id))
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
        let error_msg = slack_response
            .error
            .unwrap_or_else(|| "unknown".to_string());
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
    let link_result =
        task::spawn_blocking(move || store.create_identifier(account.id, "slack", &slack_id)).await;

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
// Email Verification
// ============================================================================

/// Send a verification email with a magic link
async fn send_verification_email(
    email: &str,
    verify_url: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let postmark_token = std::env::var("POSTMARK_SERVER_TOKEN")
        .map_err(|_| "POSTMARK_SERVER_TOKEN not configured")?;
    let from_email =
        std::env::var("POSTMARK_FROM_EMAIL").unwrap_or_else(|_| "noreply@dowhiz.com".to_string());

    let html_body = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Verify your email</title>
</head>
<body style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; padding: 40px; background: #f5f5f5;">
    <div style="max-width: 500px; margin: 0 auto; background: white; border-radius: 8px; padding: 40px; box-shadow: 0 2px 8px rgba(0,0,0,0.1);">
        <h1 style="margin: 0 0 20px; color: #333;">Verify your email</h1>
        <p style="color: #666; line-height: 1.6;">Click the button below to verify your email address and link it to your DoWhiz account.</p>
        <a href="{}" style="display: inline-block; margin: 20px 0; padding: 12px 24px; background: #333; color: white; text-decoration: none; border-radius: 6px; font-weight: 500;">Verify Email</a>
        <p style="color: #999; font-size: 14px; margin-top: 30px;">This link expires in 24 hours. If you didn't request this, you can ignore this email.</p>
    </div>
</body>
</html>"#,
        verify_url
    );

    let text_body = format!(
        "Verify your email\n\nClick the link below to verify your email address:\n{}\n\nThis link expires in 24 hours.",
        verify_url
    );

    let client = reqwest::Client::new();
    let res = client
        .post("https://api.postmarkapp.com/email")
        .header("X-Postmark-Server-Token", &postmark_token)
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "From": from_email,
            "To": email,
            "Subject": "Verify your email for DoWhiz",
            "HtmlBody": html_body,
            "TextBody": text_body,
            "MessageStream": "outbound"
        }))
        .send()
        .await?;

    if !res.status().is_success() {
        let error_text = res.text().await.unwrap_or_default();
        return Err(format!("Postmark error: {}", error_text).into());
    }

    Ok(())
}

#[derive(Debug, Deserialize)]
pub struct VerifyEmailQuery {
    pub token: String,
}

/// GET /auth/verify-email?token=<token>
/// Verify an email address via magic link.
pub async fn verify_email(
    State(state): State<AuthState>,
    Query(query): Query<VerifyEmailQuery>,
) -> impl IntoResponse {
    let frontend_url = state.frontend_url.clone();
    let redirect_to = |path: &str| -> axum::response::Response {
        Redirect::to(&format!("{}{}", frontend_url, path)).into_response()
    };

    let store = state.account_store.clone();
    let token = query.token.clone();

    let verify_result = task::spawn_blocking(move || store.verify_email_token(&token))
        .await
        .map_err(|e| {
            error!("spawn_blocking panicked: {}", e);
            "Internal error"
        });

    match verify_result {
        Ok(Ok(identifier)) => {
            info!(
                "Email {} verified for account {}",
                identifier.identifier, identifier.account_id
            );
            redirect_to("/auth/index.html?email_verified=success")
        }
        Ok(Err(AccountStoreError::TokenInvalid)) => {
            warn!("Invalid or expired email verification token");
            redirect_to("/auth/index.html?email_verified=error&reason=invalid_token")
        }
        Ok(Err(e)) => {
            error!("Failed to verify email: {}", e);
            redirect_to("/auth/index.html?email_verified=error&reason=database_error")
        }
        Err(_) => redirect_to("/auth/index.html?email_verified=error&reason=internal_error"),
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
// GTM Mode A API
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct GtmIcpPreviewRequest {
    pub leads: Vec<GtmLeadInput>,
    pub min_sample_size: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct GtmLeadInput {
    pub recipient_id: Option<Uuid>,
    pub account_id: Option<Uuid>,
    pub email: String,
    pub first_name: Option<String>,
    pub job_title: Option<String>,
    pub company_name: Option<String>,
    pub timezone: Option<String>,
    pub company_size: Option<u32>,
    pub industry: Option<String>,
    pub region: Option<String>,
    pub product_events_14d: Option<u32>,
    pub support_tickets_30d: Option<u32>,
    pub won_deals_12m: Option<u32>,
    pub lost_deals_12m: Option<u32>,
    pub churned: Option<bool>,
    pub activation_days: Option<u32>,
    pub ltv_usd: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct GtmIcpPreviewLead {
    pub recipient_id: Uuid,
    pub account_id: Uuid,
    pub email: String,
    pub first_name: Option<String>,
    pub job_title: Option<String>,
    pub company_name: Option<String>,
    pub timezone: Option<String>,
    pub company_size: u32,
    pub industry: String,
    pub region: String,
    pub score_0_100: u8,
    pub tier: IcpTier,
    pub recommended: bool,
    pub top_drivers: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct GtmIcpPreviewResponse {
    pub status: String,
    pub errors: Vec<String>,
    pub segment_definitions: Vec<crate::gtm_agents::SegmentDefinition>,
    pub anti_icp_rules: Vec<String>,
    pub leads: Vec<GtmIcpPreviewLead>,
}

#[derive(Debug, Deserialize)]
pub struct GtmOutboundPlanRequest {
    pub leads: Vec<GtmOutboundLead>,
    pub segment_id: Option<String>,
    pub message_subject: String,
    pub message_body: String,
    pub claim_risk: Option<ClaimRisk>,
    pub max_touches: Option<u8>,
    pub cadence_days: Option<u16>,
    pub stop_conditions: Option<Vec<String>>,
    pub approval_required: Option<bool>,
    pub assignee_team: Option<String>,
    pub reviewer_group: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GtmOutboundLead {
    #[serde(default = "default_true")]
    pub selected: bool,
    pub recipient_id: Option<Uuid>,
    pub account_id: Option<Uuid>,
    pub email: String,
    pub first_name: Option<String>,
    pub job_title: Option<String>,
    pub company_name: Option<String>,
    pub timezone: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct GtmOutboundPlanResponse {
    pub outbound_status: String,
    pub outbound_errors: Vec<String>,
    pub dispatch_status: String,
    pub dispatch_errors: Vec<String>,
    pub mode_a_output: ModeAOutboundDispatchOutput,
}

#[derive(Debug, Deserialize)]
pub struct GtmHubspotPushRequest {
    pub mode_a_output: ModeAOutboundDispatchOutput,
}

#[derive(Debug, Serialize)]
pub struct GtmHubspotPushResponse {
    pub report: crate::gtm_agents::HubspotDispatchReport,
}

pub async fn gtm_icp_preview(
    State(state): State<AuthState>,
    headers: HeaderMap,
    Json(payload): Json<GtmIcpPreviewRequest>,
) -> impl IntoResponse {
    let account_id = match resolve_authenticated_account_id(&state, &headers).await {
        Ok(id) => id,
        Err(response) => return response,
    };

    if payload.leads.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "At least one lead is required"
            })),
        )
            .into_response();
    }

    let engine = Phase1AgentEngine;
    let mut envelope = AgentTaskEnvelope::new(AgentId::RachelOrchestrator);
    envelope.tenant_id = account_id;
    envelope.priority = TaskPriority::High;
    envelope.input_refs = vec!["ui://gtm_linkedin_console".to_string()];

    let mut leads = Vec::new();
    let mut accounts = Vec::new();
    for lead in payload.leads {
        let normalized_email = lead.email.trim().to_lowercase();
        if normalized_email.is_empty() {
            continue;
        }
        let account_id = lead.account_id.unwrap_or_else(Uuid::new_v4);
        let recipient_id = lead.recipient_id.unwrap_or_else(Uuid::new_v4);
        let company_size = lead.company_size.unwrap_or(50);
        let industry = lead.industry.unwrap_or_else(|| "unknown".to_string());
        let region = lead.region.unwrap_or_else(|| "US".to_string());
        let product_events_14d = lead.product_events_14d.unwrap_or(3);
        let support_tickets_30d = lead.support_tickets_30d.unwrap_or(1);
        let won_deals_12m = lead.won_deals_12m.unwrap_or(1);
        let lost_deals_12m = lead.lost_deals_12m.unwrap_or(1);
        let churned = lead.churned.unwrap_or(false);
        let activation_days = lead.activation_days.unwrap_or(14);
        let ltv_usd = lead.ltv_usd.unwrap_or(1500.0);

        leads.push(GtmIcpPreviewLead {
            recipient_id,
            account_id,
            email: normalized_email.clone(),
            first_name: lead.first_name,
            job_title: lead.job_title,
            company_name: lead.company_name,
            timezone: lead.timezone,
            company_size,
            industry: industry.clone(),
            region: region.clone(),
            score_0_100: 0,
            tier: IcpTier::D,
            recommended: false,
            top_drivers: Vec::new(),
        });

        accounts.push(AccountSignal {
            entity_id: account_id,
            company_size,
            industry,
            region,
            product_events_14d,
            support_tickets_30d,
            won_deals_12m,
            lost_deals_12m,
            churned,
            activation_days,
            ltv_usd,
        });
    }

    if accounts.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "No valid leads provided"
            })),
        )
            .into_response();
    }

    let min_sample_size = payload
        .min_sample_size
        .unwrap_or_else(|| accounts.len().min(25).max(2));
    let result = match engine.run_icp_scout(
        envelope.with_agent(AgentId::RachelIcpScout),
        IcpScoutInput {
            accounts,
            current_segment_ids: Vec::new(),
            min_sample_size,
        },
    ) {
        Ok(value) => value,
        Err(err) => {
            error!("GTM ICP preview failed: {}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to run ICP preview"
                })),
            )
                .into_response();
        }
    };

    let scores_by_entity = result
        .output_payload
        .icp_scores
        .iter()
        .map(|score| (score.entity_id, score))
        .collect::<std::collections::HashMap<_, _>>();
    for lead in &mut leads {
        if let Some(score) = scores_by_entity.get(&lead.account_id) {
            lead.score_0_100 = score.score_0_100;
            lead.tier = score.tier;
            lead.top_drivers = score.top_drivers.clone();
            lead.recommended = matches!(score.tier, IcpTier::A | IcpTier::B);
        }
    }

    let status = match result.status {
        crate::gtm_agents::TaskStatus::Succeeded => "succeeded",
        crate::gtm_agents::TaskStatus::NeedsHuman => "needs_human",
        crate::gtm_agents::TaskStatus::Failed => "failed",
        crate::gtm_agents::TaskStatus::Partial => "partial",
    };

    (
        StatusCode::OK,
        Json(GtmIcpPreviewResponse {
            status: status.to_string(),
            errors: result.errors,
            segment_definitions: result.output_payload.segment_definitions,
            anti_icp_rules: result.output_payload.anti_icp_rules,
            leads,
        }),
    )
        .into_response()
}

pub async fn gtm_outbound_plan(
    State(state): State<AuthState>,
    headers: HeaderMap,
    Json(payload): Json<GtmOutboundPlanRequest>,
) -> impl IntoResponse {
    let account_id = match resolve_authenticated_account_id(&state, &headers).await {
        Ok(id) => id,
        Err(response) => return response,
    };

    if payload.message_subject.trim().is_empty() || payload.message_body.trim().is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "message_subject and message_body are required"
            })),
        )
            .into_response();
    }

    let segment_manifest = payload
        .leads
        .iter()
        .filter(|lead| lead.selected)
        .filter_map(|lead| {
            let email = lead.email.trim().to_lowercase();
            if email.is_empty() {
                return None;
            }
            Some(SegmentContact {
                recipient_id: lead.recipient_id.unwrap_or_else(Uuid::new_v4),
                account_id: lead.account_id.unwrap_or_else(Uuid::new_v4),
                email,
                first_name: lead.first_name.clone(),
                job_title: lead.job_title.clone(),
                company_name: lead.company_name.clone(),
                timezone: lead.timezone.clone(),
            })
        })
        .collect::<Vec<_>>();

    if segment_manifest.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "Select at least one lead to plan outbound outreach"
            })),
        )
            .into_response();
    }

    let claim_risk = payload.claim_risk.unwrap_or(ClaimRisk::Low);
    let segment_id = payload
        .segment_id
        .clone()
        .unwrap_or_else(|| "linkedin_icp_selected".to_string());
    let outbound_input = OutboundSdrInput {
        segment_manifest,
        message_bundle: MessageBundle {
            segment_id: segment_id.clone(),
            variants: vec![MessageVariant {
                template_id: format!("linkedin_dm_{}", Utc::now().timestamp()),
                subject: payload.message_subject.clone(),
                body: payload.message_body.clone(),
                claim_risk,
            }],
        },
        sequence_policy: SequencePolicy {
            max_touches: payload.max_touches.unwrap_or(2).clamp(1, 6),
            cadence_days: payload.cadence_days.unwrap_or(2),
            stop_conditions: payload
                .stop_conditions
                .clone()
                .unwrap_or_else(|| vec!["positive_reply".to_string()]),
        },
        channel_policy: ChannelPolicy {
            email_enabled: false,
            linkedin_ads_enabled: false,
            linkedin_dm_enabled: true,
        },
    };

    let approval_required = payload.approval_required.unwrap_or(true);
    let mut base_envelope = AgentTaskEnvelope::new(AgentId::RachelOrchestrator);
    base_envelope.tenant_id = account_id;
    base_envelope.priority = TaskPriority::High;
    base_envelope.input_refs = vec![format!("ui://gtm/segment/{}", segment_id)];
    base_envelope.policy_pack = PolicyPack::default();
    base_envelope.policy_pack.human_approval_required = approval_required;
    base_envelope.policy_pack.allowed_channels =
        vec![GtmChannel::LinkedinDm, GtmChannel::HubspotWorkflow];

    let phase1 = Phase1AgentEngine;
    let outbound_result = match phase1.run_outbound_sdr(
        base_envelope.with_agent(AgentId::RachelOutboundSdr),
        outbound_input.clone(),
    ) {
        Ok(value) => value,
        Err(err) => {
            error!("Failed to run outbound planner: {}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to generate outbound sequence"
                })),
            )
                .into_response();
        }
    };

    let mode_a = ModeAAgentEngine;
    let dispatch = match mode_a.run_workflow(crate::gtm_agents::ModeAWorkflowInput {
        base_envelope,
        dispatch: ModeAOutboundDispatchInput {
            outbound_input,
            outbound_output: outbound_result.output_payload,
            assignee_team: payload
                .assignee_team
                .clone()
                .unwrap_or_else(|| "sdr_team".to_string()),
            reviewer_group: payload
                .reviewer_group
                .clone()
                .unwrap_or_else(|| "gtm_ops".to_string()),
            approval_required,
        },
    }) {
        Ok(value) => value,
        Err(err) => {
            error!("Failed to run Mode A dispatch planner: {}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to generate Mode A dispatch plan"
                })),
            )
                .into_response();
        }
    };

    let outbound_status = match outbound_result.status {
        crate::gtm_agents::TaskStatus::Succeeded => "succeeded",
        crate::gtm_agents::TaskStatus::NeedsHuman => "needs_human",
        crate::gtm_agents::TaskStatus::Failed => "failed",
        crate::gtm_agents::TaskStatus::Partial => "partial",
    };
    let dispatch_status = match dispatch.dispatch.status {
        crate::gtm_agents::TaskStatus::Succeeded => "succeeded",
        crate::gtm_agents::TaskStatus::NeedsHuman => "needs_human",
        crate::gtm_agents::TaskStatus::Failed => "failed",
        crate::gtm_agents::TaskStatus::Partial => "partial",
    };

    (
        StatusCode::OK,
        Json(GtmOutboundPlanResponse {
            outbound_status: outbound_status.to_string(),
            outbound_errors: outbound_result.errors,
            dispatch_status: dispatch_status.to_string(),
            dispatch_errors: dispatch.dispatch.errors,
            mode_a_output: dispatch.dispatch.output_payload,
        }),
    )
        .into_response()
}

pub async fn gtm_outbound_push_hubspot(
    State(state): State<AuthState>,
    headers: HeaderMap,
    Json(payload): Json<GtmHubspotPushRequest>,
) -> impl IntoResponse {
    if let Err(response) = resolve_authenticated_account_id(&state, &headers).await {
        return response;
    }

    let executor = match HubspotModeAExecutor::from_env() {
        Ok(value) => value,
        Err(err) => {
            let status = if matches!(
                err,
                crate::gtm_agents::HubspotDispatchError::MissingAccessToken
            ) {
                StatusCode::SERVICE_UNAVAILABLE
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            };
            return (
                status,
                Json(serde_json::json!({
                    "error": format!("HubSpot executor not available: {}", err)
                })),
            )
                .into_response();
        }
    };

    let report = executor.dispatch_mode_a_drafts(&payload.mode_a_output);
    (StatusCode::OK, Json(GtmHubspotPushResponse { report })).into_response()
}

fn default_true() -> bool {
    true
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
        .route("/auth/verify-email", get(verify_email))
        .route("/auth/unlink", delete(unlink_identifier))
        .route("/auth/memo", get(get_memo).post(update_memo))
        .route("/auth/discord", get(discord_oauth_start))
        .route("/auth/discord/callback", get(discord_oauth_callback))
        .route("/auth/slack", get(slack_oauth_start))
        .route("/auth/slack/callback", get(slack_oauth_callback))
        .route("/api/tasks", get(get_tasks))
        .route("/api/gtm/icp/preview", post(gtm_icp_preview))
        .route("/api/gtm/outbound/plan", post(gtm_outbound_plan))
        .route(
            "/api/gtm/outbound/push-hubspot",
            post(gtm_outbound_push_hubspot),
        )
        .with_state(state)
}
