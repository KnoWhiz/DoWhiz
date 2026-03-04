//! Notion browser poller for monitoring @mentions and comments.
//!
//! This module provides the main polling loop that:
//! 1. Navigates to the Notion notifications page
//! 2. Parses @mentions from the HTML
//! 3. Filters out already-processed notifications
//! 4. Extracts context from mentioned pages
//! 5. Creates InboundMessages for the task queue

use chrono::Utc;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::broadcast;
use tokio::time::sleep;
use tracing::{debug, error, info, warn};

use crate::channel::{Channel, ChannelMetadata, InboundMessage};
use crate::service_bus_queue::ServiceBusIngestionQueue;

use super::browser::{NotionBrowser, NotionBrowserConfig};
use super::models::{NotionMention, NotionNotification, NotionPageContext};
use super::parser::{parse_notifications, parse_page_content};
use super::store::MongoNotionProcessedStore;
use super::NotionError;

/// Configuration for the Notion browser poller.
#[derive(Debug, Clone)]
pub struct NotionPollerConfig {
    /// Employee ID (e.g., "little_bear")
    pub employee_id: String,
    /// Email for Notion login
    pub notion_email: String,
    /// Password for Notion login
    pub notion_password: String,
    /// Poll interval in seconds (recommended: 30-60)
    pub poll_interval_secs: u64,
    /// Browser configuration
    pub browser_config: NotionBrowserConfig,
    /// Employee's display name for filtering self-comments
    pub employee_name: String,
    /// Tenant ID for routing
    pub tenant_id: String,
}

impl NotionPollerConfig {
    /// Create configuration from environment variables.
    pub fn from_env(employee_id: &str) -> Result<Self, NotionError> {
        use std::env;

        let notion_email = env::var("NOTION_EMPLOYEE_EMAIL")
            .map_err(|_| NotionError::ConfigError("NOTION_EMPLOYEE_EMAIL not set".to_string()))?;

        let notion_password = env::var("NOTION_EMPLOYEE_PASSWORD")
            .map_err(|_| NotionError::ConfigError("NOTION_EMPLOYEE_PASSWORD not set".to_string()))?;

        let poll_interval_secs = env::var("NOTION_POLL_INTERVAL_SECS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(45);

        let headless = env::var("NOTION_BROWSER_HEADLESS")
            .ok()
            .map(|s| s == "true" || s == "1")
            .unwrap_or(false);

        let slow_mo_ms = env::var("NOTION_BROWSER_SLOW_MO")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(100);

        let profile_dir = env::var("NOTION_BROWSER_PROFILE_DIR")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|_| {
                dirs::home_dir()
                    .unwrap_or_else(|| std::path::PathBuf::from("/tmp"))
                    .join(".dowhiz")
                    .join("notion_profile")
                    .join(employee_id)
            });

        let webdriver_url = env::var("WEBDRIVER_URL")
            .unwrap_or_else(|_| "http://localhost:4444".to_string());

        let employee_name = env::var("NOTION_EMPLOYEE_NAME")
            .unwrap_or_else(|_| "Oliver".to_string());

        let tenant_id = env::var("TENANT_ID")
            .unwrap_or_else(|_| "default".to_string());

        Ok(Self {
            employee_id: employee_id.to_string(),
            notion_email,
            notion_password,
            poll_interval_secs,
            browser_config: NotionBrowserConfig {
                webdriver_url,
                profile_dir,
                headless,
                slow_mo_ms,
                ..Default::default()
            },
            employee_name,
            tenant_id,
        })
    }
}

/// Notion browser poller that monitors for @mentions.
pub struct NotionBrowserPoller {
    config: NotionPollerConfig,
    browser: Option<NotionBrowser>,
    processed_store: MongoNotionProcessedStore,
    queue: Option<Arc<ServiceBusIngestionQueue>>,
    shutdown_rx: Option<broadcast::Receiver<()>>,
}

impl NotionBrowserPoller {
    /// Create a new Notion browser poller.
    pub fn new(
        config: NotionPollerConfig,
        processed_store: MongoNotionProcessedStore,
        queue: Option<Arc<ServiceBusIngestionQueue>>,
    ) -> Self {
        Self {
            config,
            browser: None,
            processed_store,
            queue,
            shutdown_rx: None,
        }
    }

    /// Set a shutdown signal receiver.
    pub fn with_shutdown(mut self, shutdown_rx: broadcast::Receiver<()>) -> Self {
        self.shutdown_rx = Some(shutdown_rx);
        self
    }

    /// Initialize the browser and ensure logged in.
    async fn ensure_browser(&mut self) -> Result<(), NotionError> {
        if self.browser.is_none() {
            info!("Initializing Notion browser...");
            let browser = NotionBrowser::new(self.config.browser_config.clone()).await?;
            self.browser = Some(browser);
        }

        if let Some(ref mut browser) = self.browser {
            browser
                .ensure_logged_in(&self.config.notion_email, &self.config.notion_password)
                .await?;
        }

        Ok(())
    }

    /// Run the polling loop.
    pub async fn run(&mut self) -> Result<(), NotionError> {
        info!(
            "Starting Notion browser poller for employee {} with {}s interval",
            self.config.employee_id, self.config.poll_interval_secs
        );

        loop {
            // Check for shutdown signal
            if let Some(ref mut rx) = self.shutdown_rx {
                if rx.try_recv().is_ok() {
                    info!("Received shutdown signal, stopping Notion poller");
                    break;
                }
            }

            match self.poll_once().await {
                Ok(count) => {
                    if count > 0 {
                        info!("Processed {} new Notion notifications", count);
                    } else {
                        debug!("No new Notion notifications");
                    }
                }
                Err(e) => {
                    error!("Notion poll error: {}", e);
                    // Reset browser on error to force re-login
                    if let Some(browser) = self.browser.take() {
                        let _ = browser.close().await;
                    }
                }
            }

            sleep(Duration::from_secs(self.config.poll_interval_secs)).await;
        }

        // Cleanup
        if let Some(browser) = self.browser.take() {
            let _ = browser.close().await;
        }

        Ok(())
    }

    /// Perform a single poll iteration.
    pub async fn poll_once(&mut self) -> Result<usize, NotionError> {
        // Ensure browser is ready
        self.ensure_browser().await?;

        let browser = self
            .browser
            .as_mut()
            .ok_or_else(|| NotionError::BrowserError("Browser not initialized".to_string()))?;

        // Navigate to notifications page
        browser.go_to_notifications().await?;

        // Get page HTML
        let html = browser.get_page_html().await?;

        // Parse notifications
        let notifications = parse_notifications(&html)?;
        debug!("Found {} notifications in HTML", notifications.len());

        // Filter to unprocessed, unread mentions
        let mut new_mentions = Vec::new();
        for notification in notifications {
            // Skip if already processed
            if self.processed_store.is_processed(&notification.id)? {
                continue;
            }

            // Skip if already read (user may have handled manually)
            if notification.is_read {
                continue;
            }

            // Only process mentions (not all notifications)
            if notification.notification_type != "mention"
                && !notification.preview_text.as_ref().map_or(false, |t| {
                    t.to_lowercase().contains("mentioned")
                })
            {
                continue;
            }

            new_mentions.push(notification);
        }

        let count = new_mentions.len();
        debug!("Found {} new unprocessed mentions", count);

        // Process each new mention
        for notification in new_mentions {
            match self.process_notification(notification).await {
                Ok(()) => {}
                Err(e) => {
                    warn!("Failed to process notification: {}", e);
                    // Continue with other notifications
                }
            }
        }

        Ok(count)
    }

    /// Process a single notification.
    async fn process_notification(&mut self, notification: NotionNotification) -> Result<(), NotionError> {
        info!(
            "Processing notification {} from workspace {:?}",
            notification.id, notification.workspace_name
        );

        let browser = self
            .browser
            .as_mut()
            .ok_or_else(|| NotionError::BrowserError("Browser not initialized".to_string()))?;

        // Navigate to the mentioned page
        if !notification.url.is_empty() {
            browser.go_to_url(&notification.url).await?;
        } else if !notification.page_id.is_empty() {
            browser.go_to_page(&notification.page_id).await?;
        } else {
            return Err(NotionError::ParseError(
                "Notification has no URL or page ID".to_string(),
            ));
        }

        // Get page content for context
        let page_html = browser.get_page_html().await?;
        let current_url = browser.current_url().await?;

        let page_context = parse_page_content(&page_html, &notification.page_id, &current_url)?;

        // Build the mention object
        let mention = NotionMention {
            id: notification.id.clone(),
            workspace_id: notification.workspace_id.clone().unwrap_or_default(),
            workspace_name: notification.workspace_name.clone().unwrap_or_default(),
            page_id: notification.page_id.clone(),
            page_title: page_context.title.clone(),
            block_id: notification.block_id.clone(),
            comment_id: None, // Would need to extract from DOM
            sender_name: notification.actor_name.clone().unwrap_or_default(),
            sender_id: notification.actor_id.clone(),
            comment_text: notification.preview_text.clone().unwrap_or_default(),
            thread_context: page_context.comment_thread.clone(),
            url: current_url.clone(),
            detected_at: Utc::now(),
        };

        // Build InboundMessage
        let inbound_message = self.build_inbound_message(&mention, &page_context)?;

        // Enqueue for processing
        self.enqueue_message(inbound_message).await?;

        // Mark as processed
        self.processed_store.mark_processed(
            &notification.id,
            notification.workspace_id.as_deref(),
            Some(&notification.page_id),
        )?;

        Ok(())
    }

    /// Build an InboundMessage from a Notion mention.
    fn build_inbound_message(
        &self,
        mention: &NotionMention,
        page_context: &NotionPageContext,
    ) -> Result<InboundMessage, NotionError> {
        // Build text body with context
        let mut text_body = String::new();

        // Add the triggering comment
        text_body.push_str(&format!("From: {}\n", mention.sender_name));
        text_body.push_str(&format!("Message: {}\n", mention.comment_text));
        text_body.push('\n');

        // Add thread context if available
        if !mention.thread_context.is_empty() {
            text_body.push_str("--- Previous conversation ---\n");
            for comment in &mention.thread_context {
                text_body.push_str(&format!("{}: {}\n", comment.author_name, comment.text));
            }
            text_body.push('\n');
        }

        // Add page content summary
        text_body.push_str(&format!("--- Page: {} ---\n", page_context.title));
        let content_preview = if page_context.content_text.len() > 2000 {
            format!("{}...", &page_context.content_text[..2000])
        } else {
            page_context.content_text.clone()
        };
        text_body.push_str(&content_preview);

        // Thread ID for tracking conversation
        let thread_id = format!("notion:{}:{}", mention.page_id, mention.id);

        // Message ID for deduplication
        let message_id = Some(mention.id.clone());

        Ok(InboundMessage {
            channel: Channel::Notion,
            sender: mention.sender_id.clone().unwrap_or_else(|| mention.sender_name.clone()),
            sender_name: Some(mention.sender_name.clone()),
            recipient: format!("{}@notion", self.config.employee_id),
            subject: Some(format!("Mention on: {}", page_context.title)),
            text_body: Some(text_body),
            html_body: None,
            thread_id,
            message_id,
            attachments: vec![],
            reply_to: vec![mention.sender_id.clone().unwrap_or_default()],
            raw_payload: serde_json::to_vec(mention).unwrap_or_default(),
            metadata: ChannelMetadata {
                notion_workspace_id: Some(mention.workspace_id.clone()),
                notion_workspace_name: Some(mention.workspace_name.clone()),
                notion_page_id: Some(mention.page_id.clone()),
                notion_page_title: Some(page_context.title.clone()),
                notion_comment_id: mention.comment_id.clone(),
                notion_block_id: mention.block_id.clone(),
                notion_notification_id: Some(mention.id.clone()),
                ..Default::default()
            },
        })
    }

    /// Enqueue an inbound message for processing.
    async fn enqueue_message(&self, message: InboundMessage) -> Result<(), NotionError> {
        // For now, log the message. Full integration would use ServiceBus queue.
        info!(
            "Enqueueing Notion message from {} about {}",
            message.sender,
            message.subject.as_deref().unwrap_or("(no subject)")
        );

        // TODO: Integrate with actual queue
        // if let Some(ref queue) = self.queue {
        //     let envelope = IngestionEnvelope { ... };
        //     queue.enqueue(&envelope).await?;
        // }

        let _ = self.queue; // Suppress unused warning

        Ok(())
    }
}

/// Spawn the Notion browser poller in a background task.
pub fn spawn_notion_browser_poller(
    config: NotionPollerConfig,
    processed_store: MongoNotionProcessedStore,
    queue: Option<Arc<ServiceBusIngestionQueue>>,
    shutdown_tx: broadcast::Sender<()>,
) -> tokio::task::JoinHandle<()> {
    let shutdown_rx = shutdown_tx.subscribe();

    tokio::spawn(async move {
        let mut poller = NotionBrowserPoller::new(config, processed_store, queue)
            .with_shutdown(shutdown_rx);

        if let Err(e) = poller.run().await {
            error!("Notion browser poller error: {}", e);
        }
    })
}
