#[path = "inbound_gateway/config.rs"]
mod config;
#[path = "inbound_gateway/discord.rs"]
mod discord;
#[path = "inbound_gateway/google_docs.rs"]
mod google_docs;
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
use tracing::info;

use scheduler_module::employee_config::load_employee_directory;
use scheduler_module::ingestion_queue::{build_queue_from_env, resolve_ingestion_queue_backend, IngestionQueue};

use config::{
    load_gateway_config, resolve_employee_config_path, resolve_gateway_config_path,
    resolve_ingestion_db_url, GatewayConfigFile,
};
use discord::spawn_discord_gateway;
use google_docs::spawn_google_docs_poller;
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
    let db_url = if backend == "postgres" {
        Some(resolve_ingestion_db_url(&config_file.storage)?)
    } else {
        None
    };

    let (routes, channel_defaults) = normalize_routes(&config_file.routes)?;

    let queue: Arc<dyn IngestionQueue> =
        task::spawn_blocking(move || build_queue_from_env(db_url))
            .await
            .map_err(|err| -> Box<dyn std::error::Error + Send + Sync> { err.into() })??;

    let state = Arc::new(GatewayState {
        config: GatewayConfig {
            defaults: config_file.defaults,
            routes,
            channel_defaults,
        },
        employee_directory,
        address_to_employee,
        queue,
    });

    info!(
        "ingestion gateway config path={}, host={}, port={}, db_url=***",
        config_path.display(),
        host,
        port
    );

    spawn_discord_gateway(state.clone()).await;
    spawn_google_docs_poller(state.clone());

    let max_body_bytes = env::var("GATEWAY_MAX_BODY_BYTES")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(25 * 1024 * 1024);

    let app = Router::new()
        .route("/health", get(health))
        .route("/postmark/inbound", post(ingest_postmark))
        .route("/slack/events", post(ingest_slack))
        .route("/bluebubbles/webhook", post(ingest_bluebubbles))
        .route("/telegram/webhook", post(ingest_telegram))
        .route("/sms/twilio", post(ingest_sms))
        .route("/whatsapp/webhook", get(verify_whatsapp_webhook))
        .route("/whatsapp/webhook", post(ingest_whatsapp))
        .with_state(state)
        .layer(DefaultBodyLimit::max(max_body_bytes));

    let addr: std::net::SocketAddr = format!("{}:{}", host, port).parse()?;
    info!("ingestion gateway listening on {}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;

    Ok(())
}
