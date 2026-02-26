#[path = "inbound_gateway/config.rs"]
mod config;
#[path = "inbound_gateway/discord.rs"]
mod discord;
#[path = "inbound_gateway/google_drive_webhook.rs"]
mod google_drive_webhook;
#[path = "inbound_gateway/google_workspace.rs"]
mod google_workspace;
#[path = "inbound_gateway/handlers.rs"]
mod handlers;
#[path = "inbound_gateway/routes.rs"]
mod routes;
#[path = "inbound_gateway/state.rs"]
mod state;
#[path = "inbound_gateway/verify.rs"]
mod verify;

use axum::extract::DefaultBodyLimit;
use axum::routing::{get, post};
use axum::Router;
use std::env;
use std::sync::Arc;
use tokio::task;
use tracing::{info, warn};

use scheduler_module::account_store::AccountStore;
use scheduler_module::blob_store::get_blob_store;
use scheduler_module::employee_config::load_employee_directory;
use scheduler_module::google_auth::GoogleAuth;
use scheduler_module::google_drive_changes::{GoogleDriveChangesConfig, GoogleDriveChangesManager};
use scheduler_module::ingestion_queue::{
    build_servicebus_queue_from_env, resolve_ingestion_queue_backend, IngestionQueue,
};
use scheduler_module::service::auth::{auth_router, AuthState};

use config::{
    load_gateway_config, resolve_employee_config_path, resolve_gateway_config_path,
    GatewayConfigFile,
};
use discord::spawn_discord_gateway;
use google_drive_webhook::handle_google_drive_webhook;
use google_workspace::spawn_google_workspace_poller;
use handlers::{
    health, ingest_bluebubbles, ingest_postmark, ingest_slack, ingest_sms, ingest_telegram,
    ingest_whatsapp, verify_whatsapp_webhook,
};
use routes::normalize_routes;
use state::{build_address_map, GatewayConfig, GatewayState};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::fmt().with_target(false).init();
    dotenvy::dotenv().ok();

    let config_path = resolve_gateway_config_path()?;
    let config_file: GatewayConfigFile = load_gateway_config(&config_path)?;

    let employee_config_path = resolve_employee_config_path();
    let employee_directory = load_employee_directory(&employee_config_path)?;
    let address_to_employee = build_address_map(&employee_directory);

    let host = env::var("GATEWAY_HOST")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| {
            config_file
                .server
                .host
                .unwrap_or_else(|| "0.0.0.0".to_string())
        });
    let port = env::var("GATEWAY_PORT")
        .ok()
        .and_then(|value| value.parse::<u16>().ok())
        .unwrap_or_else(|| config_file.server.port.unwrap_or(9100));

    let backend = resolve_ingestion_queue_backend();
    if backend != "servicebus" && backend != "service_bus" {
        return Err(format!(
            "inbound gateway requires SCALE_OLIVER_INGESTION_QUEUE_BACKEND (or INGESTION_QUEUE_BACKEND)=servicebus (got '{}')",
            backend
        )
        .into());
    }

    let (routes, channel_defaults) = normalize_routes(&config_file.routes)?;

    let queue: Arc<dyn IngestionQueue> = task::spawn_blocking(build_servicebus_queue_from_env)
        .await
        .map_err(|err| -> Box<dyn std::error::Error + Send + Sync> { err.into() })??;

    // Initialize Google Drive push notifications if enabled
    let drive_changes_config = GoogleDriveChangesConfig::from_env();
    let (drive_changes_manager, drive_change_notifier) = if drive_changes_config.is_valid() {
        info!(
            "Google Drive push notifications enabled, webhook_url={}",
            drive_changes_config.webhook_url.as_deref().unwrap_or("")
        );
        match GoogleAuth::from_env() {
            Ok(auth) => {
                let manager = Arc::new(GoogleDriveChangesManager::new(drive_changes_config, auth));
                let (tx, _rx) = tokio::sync::broadcast::channel::<String>(100);
                (Some(manager), Some(tx))
            }
            Err(e) => {
                warn!("Google Drive push notifications disabled: failed to initialize auth: {}", e);
                (None, None)
            }
        }
    } else {
        info!("Google Drive push notifications disabled (set GOOGLE_DRIVE_PUSH_ENABLED=true to enable)");
        (None, None)
    };

    let state = Arc::new(GatewayState {
        config: GatewayConfig {
            defaults: config_file.defaults,
            routes,
            channel_defaults,
        },
        employee_directory,
        address_to_employee,
        queue,
        drive_changes_manager,
        drive_change_notifier,
    });

    info!(
        "ingestion gateway config path={}, host={}, port={}, backend={}",
        config_path.display(),
        host,
        port,
        backend
    );

    spawn_discord_gateway(state.clone()).await;
    // Unified poller handles Docs, Sheets, and Slides
    spawn_google_workspace_poller(state.clone());

    let max_body_bytes = env::var("GATEWAY_MAX_BODY_BYTES")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(25 * 1024 * 1024);

    // Create auth state for OAuth routes
    let account_store = Arc::new(
        task::spawn_blocking(AccountStore::from_env)
            .await
            .map_err(|err| -> Box<dyn std::error::Error + Send + Sync> { err.into() })??,
    );
    let supabase_url = env::var("SUPABASE_PROJECT_URL")
        .unwrap_or_else(|_| "https://resmseutzmwumflevfqw.supabase.co".to_string());
    let blob_store = get_blob_store();

    // Discord OAuth config (optional)
    let discord_client_id = env::var("DISCORD_CLIENT_ID").ok();
    let discord_client_secret = env::var("DISCORD_CLIENT_SECRET").ok();
    let discord_redirect_uri = env::var("DISCORD_REDIRECT_URI").ok();

    // Slack OAuth config (optional)
    let slack_client_id = env::var("SLACK_CLIENT_ID").ok();
    let slack_client_secret = env::var("SLACK_CLIENT_SECRET").ok();
    let slack_redirect_uri = env::var("SLACK_AUTH_REDIRECT_URI").ok();

    // Frontend URL for OAuth redirects
    let frontend_url =
        env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:5173".to_string());

    let auth_state = AuthState {
        account_store,
        blob_store,
        supabase_url,
        discord_client_id,
        discord_client_secret,
        discord_redirect_uri,
        slack_client_id,
        slack_client_secret,
        slack_redirect_uri,
        frontend_url,
        user_store: None, // Task lookups not available in inbound gateway
        users_root: None,
    };
    // Instantiate router in inbound_gateway to solve two ports, one tunnel issue
    let app = Router::new()
        .route("/health", get(health))
        .route("/postmark/inbound", post(ingest_postmark))
        .route("/slack/events", post(ingest_slack))
        .route("/bluebubbles/webhook", post(ingest_bluebubbles))
        .route("/telegram/webhook", post(ingest_telegram))
        .route("/sms/twilio", post(ingest_sms))
        .route("/whatsapp/webhook", get(verify_whatsapp_webhook))
        .route("/whatsapp/webhook", post(ingest_whatsapp))
        .route(
            "/webhooks/google-drive-changes",
            post(handle_google_drive_webhook),
        )
        .with_state(state)
        .merge(auth_router(auth_state))
        .layer(DefaultBodyLimit::max(max_body_bytes));

    let addr: std::net::SocketAddr = format!("{}:{}", host, port).parse()?;
    info!("ingestion gateway listening on {}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;

    Ok(())
}
