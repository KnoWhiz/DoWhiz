//! Standalone Notion browser poller for local testing and E2E.
//!
//! **DEPRECATED**: This browser-based polling approach is now a fallback option.
//! The recommended way to detect Notion @mentions is via email notifications:
//!
//! 1. Configure Notion to send email notifications to your service email
//! 2. The email handler (`notion_email_detector.rs`) will automatically detect and
//!    process Notion notification emails
//! 3. Tasks are created with Notion context, and agents can use the Notion API
//!    (preferred) or browser-use (fallback) to interact with pages
//!
//! This poller is still useful for:
//! - Users who haven't set up email notifications in Notion
//! - Testing and debugging browser automation
//! - Environments where email integration is not available
//!
//! This binary runs the Notion browser poller independently.
//! If SERVICE_BUS_CONNECTION_STRING is set, it will enqueue messages for worker processing.
//!
//! Usage:
//!   IN_DOCKER=true cargo run -p scheduler_module --bin notion_poller

use std::env;
use std::sync::Arc;
use tokio::sync::broadcast;
use tracing::{error, info, warn};

use scheduler_module::notion_browser::{
    MongoNotionProcessedStore, NotionBrowserPoller, NotionPollerConfig,
};
use scheduler_module::service_bus_queue::ServiceBusIngestionQueue;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::fmt()
        .with_target(false)
        .init();
    dotenvy::dotenv().ok();

    tracing::info!("=== Notion Browser Poller ===");

    // Now enter async runtime
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(async_main())
}

async fn async_main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("=== Starting Notion Browser Poller ===");

    // Initialize Service Bus queue if configured
    let queue: Option<Arc<ServiceBusIngestionQueue>> = match env::var("SERVICE_BUS_CONNECTION_STRING") {
        Ok(conn_str) if !conn_str.is_empty() => {
            let queue_name = env::var("SERVICE_BUS_QUEUE_NAME")
                .unwrap_or_else(|_| "ingestion-test".to_string());
            info!("Connecting to Service Bus queue: {}", queue_name);
            match ServiceBusIngestionQueue::from_env() {
                Ok(q) => {
                    info!("Service Bus queue connected successfully");
                    Some(Arc::new(q))
                }
                Err(e) => {
                    warn!("Failed to connect to Service Bus: {}. Running without queue.", e);
                    None
                }
            }
        }
        _ => {
            info!("No SERVICE_BUS_CONNECTION_STRING set. Running in local test mode (no queue).");
            None
        }
    };

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

    // Run the poller (queue was created before async runtime)
    let mut poller = NotionBrowserPoller::new(config, processed_store, queue)
        .with_shutdown(shutdown_tx.subscribe());

    if let Err(e) = poller.run().await {
        error!("Poller error: {}", e);
        return Err(e.into());
    }

    info!("Notion poller stopped.");
    Ok(())
}
