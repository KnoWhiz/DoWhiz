//! Standalone Notion browser poller for local testing.
//!
//! This binary runs the Notion browser poller independently,
//! without requiring the full inbound gateway infrastructure.
//!
//! Usage:
//!   IN_DOCKER=true cargo run --release -p scheduler_module --bin notion_poller

use std::env;
use tokio::sync::broadcast;
use tracing::{error, info, warn};

use scheduler_module::notion_browser::{
    MongoNotionProcessedStore, NotionBrowserPoller, NotionPollerConfig,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::fmt()
        .with_target(false)
        .init();
    dotenvy::dotenv().ok();

    info!("=== Notion Browser Poller (Local Test Mode) ===");

    // Check required environment variables
    let notion_email = env::var("NOTION_EMPLOYEE_EMAIL")
        .map_err(|_| "NOTION_EMPLOYEE_EMAIL not set")?;
    let _notion_password = env::var("NOTION_EMPLOYEE_PASSWORD")
        .map_err(|_| "NOTION_EMPLOYEE_PASSWORD not set")?;
    let employee_id = env::var("EMPLOYEE_ID").unwrap_or_else(|_| "little_bear".to_string());

    // Check IN_DOCKER environment variable (required for WSL)
    if env::var("IN_DOCKER").ok().as_deref() != Some("true") {
        warn!("IN_DOCKER is not set to 'true'. This is required for WSL/root environments.");
        warn!("Setting IN_DOCKER=true automatically...");
        std::env::set_var("IN_DOCKER", "true");
    }

    info!("Configuration:");
    info!("  Employee ID: {}", employee_id);
    info!("  Notion Email: {}", notion_email);
    info!("  Browser Mode: {}", env::var("NOTION_BROWSER_MODE").unwrap_or_else(|_| "chromium".to_string()));
    info!("  Poll Interval: {}s", env::var("NOTION_POLL_INTERVAL_SECS").unwrap_or_else(|_| "45".to_string()));
    info!("  Headless: {}", env::var("NOTION_BROWSER_HEADLESS").unwrap_or_else(|_| "true".to_string()));

    // Check browser-use availability
    info!("Checking browser-use CLI...");
    let bu_path = shellexpand::tilde("~/.local/bin/browser-use").to_string();
    if !std::path::Path::new(&bu_path).exists() {
        error!("browser-use CLI not found at {}", bu_path);
        error!("Please install browser-use: pipx install browser-use");
        return Err("browser-use not available".into());
    }
    info!("  browser-use found at {}", bu_path);

    // Initialize MongoDB store for deduplication
    info!("Initializing MongoDB connection...");
    let processed_store = match MongoNotionProcessedStore::from_env(&employee_id) {
        Ok(store) => {
            info!("  MongoDB connected successfully");
            store
        }
        Err(e) => {
            warn!("MongoDB not available: {}. Running without deduplication.", e);
            warn!("Note: Without MongoDB, notifications will be reprocessed on restart.");
            MongoNotionProcessedStore::noop()
        }
    };

    // Create poller config
    let config = NotionPollerConfig::from_env(&employee_id)?;

    info!("");
    info!("Starting Notion browser poller...");
    info!("The browser will open and log into Notion.");
    info!("");
    info!("To test:");
    info!("  1. Ensure agent@dowhiz.com is invited to your Notion workspace");
    info!("  2. Go to any Notion page and @mention agent@dowhiz.com");
    info!("  3. Watch this terminal for detection logs");
    info!("");
    info!("Press Ctrl+C to stop.");
    info!("");

    // Setup shutdown handler
    let (shutdown_tx, _) = broadcast::channel::<()>(1);
    let shutdown_tx_clone = shutdown_tx.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.ok();
        info!("Received shutdown signal...");
        let _ = shutdown_tx_clone.send(());
    });

    // Run the poller (without Service Bus queue - local mode)
    let mut poller = NotionBrowserPoller::new(config, processed_store, None)
        .with_shutdown(shutdown_tx.subscribe());

    if let Err(e) = poller.run().await {
        error!("Poller error: {}", e);
        return Err(e.into());
    }

    info!("Notion poller stopped.");
    Ok(())
}
