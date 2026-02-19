use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;

use axum::extract::{DefaultBodyLimit, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect};
use axum::routing::get;
use axum::Router;
use chrono::Utc;
use tracing::{error, info};

use crate::account_store::AccountStore;
use crate::index_store::IndexStore;
use crate::ingestion_queue::{IngestionQueue, PostgresIngestionQueue};
use crate::message_router::MessageRouter;
use crate::slack_store::{SlackInstallation, SlackStore};
use crate::user_store::UserStore;
use crate::{ModuleExecutor, Scheduler};
use tokio::task;

use super::auth::{auth_router, AuthState};

use super::config::ServiceConfig;
use super::ingestion::spawn_ingestion_consumer;
use super::scheduler::start_scheduler_threads;
use super::state::AppState;
use super::BoxError;

pub async fn run_server(
    config: ServiceConfig,
    shutdown: impl std::future::Future<Output = ()> + Send + 'static,
) -> Result<(), BoxError> {
    // Export SLACK_STORE_PATH so execute_slack_send can find the OAuth tokens
    std::env::set_var("SLACK_STORE_PATH", &config.slack_store_path);
    let config = Arc::new(config);
    let user_store = Arc::new(UserStore::new(&config.users_db_path)?);
    let index_store = Arc::new(IndexStore::new(&config.task_index_path)?);
    let slack_store = Arc::new(SlackStore::new(&config.slack_store_path)?);
    let ingestion_db_url = config.ingestion_db_url.clone();
    let ingestion_queue: Arc<dyn IngestionQueue> = Arc::new(
        task::spawn_blocking(move || PostgresIngestionQueue::new_from_url(&ingestion_db_url))
            .await
            .map_err(|err| -> BoxError { err.into() })??,
    );
    let message_router = Arc::new(MessageRouter::new());
    if let Ok(user_ids) = user_store.list_user_ids() {
        for user_id in user_ids {
            let paths = user_store.user_paths(&config.users_root, &user_id);
            let scheduler = Scheduler::load(&paths.tasks_db_path, ModuleExecutor::default());
            match scheduler {
                Ok(scheduler) => {
                    if let Err(err) = index_store.sync_user_tasks(&user_id, scheduler.tasks()) {
                        error!("index bootstrap failed for {}: {}", user_id, err);
                    }
                }
                Err(err) => {
                    error!("scheduler bootstrap failed for {}: {}", user_id, err);
                }
            }
        }
    }

    let mut scheduler_control =
        start_scheduler_threads(config.clone(), user_store.clone(), index_store.clone());

    info!(
        "Inbound webhooks are handled by the ingestion gateway; worker {} will only consume queued messages",
        config.employee_id
    );

    let mut ingestion_control = spawn_ingestion_consumer(
        config.clone(),
        ingestion_queue.clone(),
        user_store.clone(),
        index_store.clone(),
        slack_store.clone(),
        message_router.clone(),
    )?;

    let state = AppState {
        config: config.clone(),
        slack_store,
    };

    // Create account store for auth routes
    let account_store = Arc::new(
        task::spawn_blocking(AccountStore::from_env)
            .await
            .map_err(|err| -> BoxError { err.into() })??,
    );
    let supabase_url = std::env::var("SUPABASE_PROJECT_URL")
        .unwrap_or_else(|_| "https://resmseutzmwumflevfqw.supabase.co".to_string());
    let auth_state = AuthState {
        account_store,
        supabase_url,
    };

    let host: IpAddr = config
        .host
        .parse()
        .map_err(|_| format!("invalid host: {}", config.host))?;
    let addr = SocketAddr::new(host, config.port);
    info!("Rust email service listening on {}", addr);

    let app = Router::new()
        .route("/", get(health))
        .route("/health", get(health))
        .route("/slack/install", get(slack_install))
        .route("/slack/oauth/callback", get(slack_oauth_callback))
        .with_state(state)
        .merge(auth_router(auth_state))
        .layer(DefaultBodyLimit::max(config.inbound_body_max_bytes));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    let serve_result = axum::serve(listener, app)
        .with_graceful_shutdown(shutdown)
        .await;
    ingestion_control.stop_and_join();
    scheduler_control.stop_and_join();
    serve_result?;
    Ok(())
}

async fn health() -> impl IntoResponse {
    (StatusCode::OK, "ok")
}

/// Redirect to Slack OAuth authorization page.
/// GET /slack/install
async fn slack_install(State(state): State<AppState>) -> impl IntoResponse {
    let client_id = match &state.config.slack_client_id {
        Some(id) => id.clone(),
        None => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                "Slack OAuth not configured (missing SLACK_CLIENT_ID)",
            )
                .into_response();
        }
    };

    let redirect_uri = state.config.slack_redirect_uri.clone().unwrap_or_else(|| {
        format!(
            "http://localhost:{}/slack/oauth/callback",
            state.config.port
        )
    });

    let scopes = "chat:write,channels:history,groups:history,im:history,mpim:history";

    let auth_url = format!(
        "https://slack.com/oauth/v2/authorize?client_id={}&scope={}&redirect_uri={}",
        urlencoding::encode(&client_id),
        urlencoding::encode(scopes),
        urlencoding::encode(&redirect_uri)
    );

    Redirect::temporary(&auth_url).into_response()
}

/// Query parameters for OAuth callback.
#[derive(Debug, serde::Deserialize)]
struct SlackOAuthCallbackParams {
    code: Option<String>,
    error: Option<String>,
}

/// Handle Slack OAuth callback.
/// GET /slack/oauth/callback?code=...
async fn slack_oauth_callback(
    State(state): State<AppState>,
    Query(params): Query<SlackOAuthCallbackParams>,
) -> impl IntoResponse {
    // Check for OAuth errors
    if let Some(error) = params.error {
        return (
            StatusCode::BAD_REQUEST,
            format!("Slack OAuth error: {}", error),
        )
            .into_response();
    }

    let code = match params.code {
        Some(c) => c,
        None => {
            return (StatusCode::BAD_REQUEST, "Missing OAuth code").into_response();
        }
    };

    let client_id = match &state.config.slack_client_id {
        Some(id) => id.clone(),
        None => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                "SLACK_CLIENT_ID not configured",
            )
                .into_response();
        }
    };

    let client_secret = match &state.config.slack_client_secret {
        Some(secret) => secret.clone(),
        None => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                "SLACK_CLIENT_SECRET not configured",
            )
                .into_response();
        }
    };

    let redirect_uri = state.config.slack_redirect_uri.clone().unwrap_or_else(|| {
        format!(
            "http://localhost:{}/slack/oauth/callback",
            state.config.port
        )
    });

    // Exchange code for token
    let client = reqwest::Client::new();
    let token_response = match client
        .post("https://slack.com/api/oauth.v2.access")
        .form(&[
            ("client_id", client_id.as_str()),
            ("client_secret", client_secret.as_str()),
            ("code", code.as_str()),
            ("redirect_uri", redirect_uri.as_str()),
        ])
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(e) => {
            error!("Slack OAuth token exchange failed: {}", e);
            return (StatusCode::BAD_GATEWAY, "Failed to contact Slack API").into_response();
        }
    };

    let token_json: serde_json::Value = match token_response.json().await {
        Ok(v) => v,
        Err(e) => {
            error!("Failed to parse Slack OAuth response: {}", e);
            return (StatusCode::BAD_GATEWAY, "Invalid response from Slack").into_response();
        }
    };

    // Check for API errors
    if token_json.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        let error_msg = token_json
            .get("error")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");
        error!("Slack OAuth error: {}", error_msg);
        return (
            StatusCode::BAD_REQUEST,
            format!("Slack API error: {}", error_msg),
        )
            .into_response();
    }

    // Extract installation details
    let team_id = token_json
        .get("team")
        .and_then(|t| t.get("id"))
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let team_name = token_json
        .get("team")
        .and_then(|t| t.get("name"))
        .and_then(|v| v.as_str());
    let bot_token = token_json
        .get("access_token")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let bot_user_id = token_json
        .get("bot_user_id")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    if team_id.is_empty() || bot_token.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            "Missing team_id or access_token in Slack response",
        )
            .into_response();
    }

    // Save installation
    let installation = SlackInstallation {
        team_id: team_id.to_string(),
        team_name: team_name.map(|s| s.to_string()),
        bot_token: bot_token.to_string(),
        bot_user_id: bot_user_id.to_string(),
        installed_at: Utc::now(),
    };

    if let Err(e) = state.slack_store.upsert_installation(&installation) {
        error!("Failed to save Slack installation: {}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to save installation",
        )
            .into_response();
    }

    info!(
        "Slack app installed for team {} ({})",
        team_id,
        team_name.unwrap_or("unknown")
    );

    // Return success page
    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
  <meta charset="UTF-8" />
  <meta
    name="description"
    content="DoWhiz Slack integration is installed. Confirm your workspace, learn next steps, and start chatting with digital employees right away."
  />
  <title>DoWhiz Slack Integration | Install Complete and Next Steps</title>
</head>
<body style="font-family: sans-serif; text-align: center; padding: 50px;">
    <h1>Installation Complete!</h1>
    <p>DoWhiz has been successfully installed to <strong>{}</strong>.</p>
    <p>You can now close this window and start chatting with the bot in Slack.</p>
</body>
</html>"#,
        team_name.unwrap_or(team_id)
    );

    (StatusCode::OK, axum::response::Html(html)).into_response()
}
