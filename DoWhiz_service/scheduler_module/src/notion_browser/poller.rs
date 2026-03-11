//! Notion browser poller for monitoring @mentions and comments.
//!
//! **NOTE**: This browser-based polling is now a fallback approach.
//! The preferred method is email-based detection via `notion_email_detector.rs`.
//!
//! ## Recommended Architecture (Email-based)
//!
//! 1. Notion sends email notifications to your service address
//! 2. `notion_email_detector.rs` parses the email and extracts Notion context
//! 3. Agent uses Notion API (preferred) or browser-use (fallback) to interact
//!
//! ## Legacy Architecture (This module)
//!
//! This module provides browser-based polling that:
//! 1. Navigates to the Notion notifications page
//! 2. Parses @mentions from browser state (via LLM or regex patterns)
//! 3. Filters out already-processed notifications
//! 4. Extracts context from mentioned pages
//! 5. Creates InboundMessages for the task queue
//! 6. Processes pending reply requests from the queue
//!
//! ## Detection Modes
//!
//! The poller supports two detection modes:
//! - `AgentDriven`: Uses LLM (Claude Haiku) to analyze inbox screenshots (recommended)
//! - `Hardcoded`: Uses regex patterns to parse browser state (legacy, fallback)
//!
//! Set `NOTION_DETECTION_MODE=agent_driven` (default) or `hardcoded` to switch.

use chrono::Utc;
use regex::Regex;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::broadcast;
use tokio::time::sleep;
use tracing::{debug, error, info, warn};

use uuid::Uuid;

use crate::channel::{Channel, ChannelMetadata, InboundMessage};
use crate::ingestion::{IngestionEnvelope, IngestionPayload};
use crate::service_bus_queue::ServiceBusIngestionQueue;

use super::agent_detector::{AgentDetector, DetectedMention, UiAction};
use super::browser::{NotionBrowser, NotionBrowserConfig};
use super::models::{NotionMention, NotionNotification, NotionPageContext};
use super::store::MongoNotionProcessedStore;
use super::NotionError;

/// Detection mode for inbox mentions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DetectionMode {
    /// Use LLM to analyze inbox screenshots (recommended)
    AgentDriven,
    /// Use hardcoded regex patterns (legacy fallback)
    Hardcoded,
}

impl Default for DetectionMode {
    fn default() -> Self {
        Self::AgentDriven
    }
}

impl DetectionMode {
    /// Parse from environment variable or string.
    pub fn from_env() -> Self {
        match env::var("NOTION_DETECTION_MODE")
            .unwrap_or_default()
            .to_lowercase()
            .as_str()
        {
            "hardcoded" | "regex" | "legacy" => Self::Hardcoded,
            _ => Self::AgentDriven,
        }
    }
}

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
    /// Detection mode (agent-driven or hardcoded)
    pub detection_mode: DetectionMode,
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
            .unwrap_or(true);

        let browser_mode = env::var("NOTION_BROWSER_MODE")
            .unwrap_or_else(|_| "chromium".to_string());

        let session_name = format!("notion_{}", employee_id);

        let employee_name = env::var("NOTION_EMPLOYEE_NAME")
            .unwrap_or_else(|_| "Oliver".to_string());

        let tenant_id = env::var("TENANT_ID")
            .unwrap_or_else(|_| "default".to_string());

        let detection_mode = DetectionMode::from_env();

        Ok(Self {
            employee_id: employee_id.to_string(),
            notion_email,
            notion_password,
            poll_interval_secs,
            browser_config: NotionBrowserConfig {
                session_name,
                browser_mode,
                headless,
                profile: None,
                command_timeout_secs: 60,
                page_load_wait_secs: 3,
            },
            employee_name,
            tenant_id,
            detection_mode,
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
    agent_detector: Option<AgentDetector>,
    detection_mode: DetectionMode,
}

impl NotionBrowserPoller {
    /// Create a new Notion browser poller.
    pub fn new(
        config: NotionPollerConfig,
        processed_store: MongoNotionProcessedStore,
        queue: Option<Arc<ServiceBusIngestionQueue>>,
    ) -> Self {
        let detection_mode = config.detection_mode;

        // Initialize agent detector if using agent-driven mode
        let agent_detector = if detection_mode == DetectionMode::AgentDriven {
            match AgentDetector::from_env(&config.employee_name) {
                Ok(detector) => {
                    info!("Agent detector initialized for employee: {}", config.employee_name);
                    Some(detector)
                }
                Err(e) => {
                    warn!(
                        "Failed to initialize agent detector, falling back to hardcoded mode: {}",
                        e
                    );
                    None
                }
            }
        } else {
            info!("Using hardcoded detection mode");
            None
        };

        // Adjust detection mode if agent detector failed to initialize
        let effective_mode = if detection_mode == DetectionMode::AgentDriven && agent_detector.is_none() {
            DetectionMode::Hardcoded
        } else {
            detection_mode
        };

        Self {
            config,
            browser: None,
            processed_store,
            queue,
            shutdown_rx: None,
            agent_detector,
            detection_mode: effective_mode,
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
            let mut browser = NotionBrowser::new(self.config.browser_config.clone());
            browser.start().await?;
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
            "Starting Notion browser poller for employee {} with {}s interval (mode: {:?})",
            self.config.employee_id, self.config.poll_interval_secs, self.detection_mode
        );

        loop {
            // Check for shutdown signal
            if let Some(ref mut rx) = self.shutdown_rx {
                if rx.try_recv().is_ok() {
                    info!("Received shutdown signal, stopping Notion poller");
                    break;
                }
            }

            // Process inbound notifications
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

            // Process outbound reply queue
            match self.process_reply_queue().await {
                Ok(count) => {
                    if count > 0 {
                        info!("Posted {} Notion replies from queue", count);
                    }
                }
                Err(e) => {
                    error!("Notion reply queue error: {}", e);
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

    /// Perform a single poll iteration across all workspaces.
    pub async fn poll_once(&mut self) -> Result<usize, NotionError> {
        // Ensure browser is ready
        self.ensure_browser().await?;

        // Collect all notifications from all workspaces first
        let all_mentions = self.collect_notifications_from_all_workspaces().await?;

        let total_count = all_mentions.len();
        info!("Found {} total new @mentions across all workspaces", total_count);

        // Process each new mention
        for notification in all_mentions {
            match self.process_notification(notification).await {
                Ok(()) => {}
                Err(e) => {
                    warn!("Failed to process notification: {}", e);
                    // Continue with other notifications
                }
            }
        }

        Ok(total_count)
    }

    /// Collect notifications from all workspaces.
    /// Strategy:
    /// 1. First check the current workspace's inbox (no switching needed)
    /// 2. Then list other workspaces and switch to each one
    /// This ensures we always process the current workspace even if listing fails.
    async fn collect_notifications_from_all_workspaces(&mut self) -> Result<Vec<NotionNotification>, NotionError> {
        let browser = self
            .browser
            .as_ref()
            .ok_or_else(|| NotionError::BrowserError("Browser not initialized".to_string()))?;

        let mut all_mentions = Vec::new();

        // Step 1: Check current workspace first (no switching needed)
        // This ensures we don't miss notifications even if workspace listing fails
        info!("Checking inbox for current workspace first");
        let current_ws_mentions = self.check_current_workspace_inbox(browser, "current").await;
        all_mentions.extend(current_ws_mentions);

        // Step 2: List all workspaces and check others
        let workspaces = match browser.list_workspaces().await {
            Ok(ws) => {
                info!("Found {} workspaces in dropdown", ws.len());
                ws
            }
            Err(e) => {
                warn!("Failed to list workspaces: {}. Only current workspace was checked.", e);
                return Ok(all_mentions);
            }
        };

        // Check inbox for each workspace (skip if we can't switch)
        for (workspace_name, _workspace_idx) in &workspaces {
            info!("Checking inbox for workspace: {}", workspace_name);

            // Switch to this workspace by name
            if let Err(e) = browser.switch_workspace_by_name(workspace_name).await {
                warn!("Failed to switch to workspace {}: {}", workspace_name, e);
                continue;
            }

            // Open inbox for this workspace
            if let Err(e) = browser.open_inbox().await {
                warn!("Failed to open inbox for {}: {}", workspace_name, e);
                // Try direct navigation as fallback
                if let Err(e2) = browser.go_to_notifications().await {
                    warn!("Fallback navigation also failed: {}", e2);
                    continue;
                }
            }

            // Get browser state
            let state = match browser.get_state().await {
                Ok(s) => s,
                Err(e) => {
                    warn!("Failed to get state for {}: {}", workspace_name, e);
                    continue;
                }
            };

            // Debug: save state to file for analysis
            let debug_file = format!("/tmp/notion_state_{}.txt", workspace_name.replace(' ', "_").replace("'", ""));
            if let Err(e) = std::fs::write(&debug_file, &state.raw) {
                warn!("Failed to save state for debug: {}", e);
            } else {
                debug!("Saved {} inbox state to {} ({} bytes)", workspace_name, debug_file, state.raw.len());
            }

            // Parse notifications using appropriate detection method
            let notifications = match self.detection_mode {
                DetectionMode::AgentDriven => {
                    self.detect_mentions_with_agent(browser, workspace_name).await
                        .unwrap_or_else(|e| {
                            warn!("Agent detection failed for {}, falling back to hardcoded: {}", workspace_name, e);
                            self.parse_notifications_from_state(&state.raw)
                        })
                }
                DetectionMode::Hardcoded => {
                    self.parse_notifications_from_state(&state.raw)
                }
            };
            info!("Detected {} notifications from {} inbox (mode: {:?})", notifications.len(), workspace_name, self.detection_mode);

            // Filter to unprocessed mentions
            // NOTE: We do NOT skip based on is_read because opening the inbox panel
            // itself can trigger Notion to mark notifications as read. We rely solely
            // on processed_store for deduplication.
            for mut notification in notifications {
                // Add workspace info to notification
                notification.workspace_name = Some(workspace_name.clone());

                // Skip if already processed (this is our primary deduplication)
                let full_id = format!("{}:{}", workspace_name, notification.id);
                if self.processed_store.is_processed(&full_id).unwrap_or(false) {
                    debug!("Skipping already processed notification: {}", full_id);
                    continue;
                }

                // Process mentions and comments (be more inclusive)
                let preview_lower = notification.preview_text.as_ref()
                    .map(|t| t.to_lowercase())
                    .unwrap_or_default();

                if notification.notification_type != "mention"
                    && notification.notification_type != "comment"
                    && !preview_lower.contains("mentioned")
                    && !preview_lower.contains("commented")
                    && !preview_lower.contains("@")
                {
                    debug!("Skipping non-mention notification: {} (type: {})",
                        notification.id, notification.notification_type);
                    continue;
                }

                // Update ID to include workspace
                notification.id = full_id;

                info!("Found new mention in {}: {} from {:?}",
                    workspace_name, notification.id, notification.actor_name);
                all_mentions.push(notification);
            }

            // Small delay between workspaces
            sleep(Duration::from_secs(1)).await;
        }

        Ok(all_mentions)
    }

    /// Check inbox for current workspace without switching.
    /// Returns collected notifications (empty vec on error).
    async fn check_current_workspace_inbox(
        &self,
        browser: &NotionBrowser,
        workspace_name: &str,
    ) -> Vec<NotionNotification> {
        // Open inbox for current workspace
        if let Err(e) = browser.open_inbox().await {
            warn!("Failed to open inbox for current workspace: {}", e);
            // Try direct navigation as fallback
            if let Err(e2) = browser.go_to_notifications().await {
                warn!("Fallback navigation also failed: {}", e2);
                return vec![];
            }
        }

        // Get browser state
        let state = match browser.get_state().await {
            Ok(s) => s,
            Err(e) => {
                warn!("Failed to get state for current workspace: {}", e);
                return vec![];
            }
        };

        // Debug: save state to file for analysis
        let debug_file = format!("/tmp/notion_state_{}.txt", workspace_name.replace(' ', "_").replace("'", ""));
        if let Err(e) = std::fs::write(&debug_file, &state.raw) {
            warn!("Failed to save state for debug: {}", e);
        } else {
            debug!("Saved {} inbox state to {} ({} bytes)", workspace_name, debug_file, state.raw.len());
        }

        // Parse notifications using appropriate detection method
        let notifications = match self.detection_mode {
            DetectionMode::AgentDriven => {
                self.detect_mentions_with_agent(browser, workspace_name).await
                    .unwrap_or_else(|e| {
                        warn!("Agent detection failed for {}, falling back to hardcoded: {}", workspace_name, e);
                        self.parse_notifications_from_state(&state.raw)
                    })
            }
            DetectionMode::Hardcoded => {
                self.parse_notifications_from_state(&state.raw)
            }
        };
        info!("Detected {} notifications from {} inbox (mode: {:?})", notifications.len(), workspace_name, self.detection_mode);

        // Filter to unprocessed mentions
        let mut result = Vec::new();
        for mut notification in notifications {
            notification.workspace_name = Some(workspace_name.to_string());

            let full_id = format!("{}:{}", workspace_name, notification.id);
            if self.processed_store.is_processed(&full_id).unwrap_or(false) {
                debug!("Skipping already processed notification: {}", full_id);
                continue;
            }

            let preview_lower = notification.preview_text.as_ref()
                .map(|t| t.to_lowercase())
                .unwrap_or_default();

            if notification.notification_type != "mention"
                && notification.notification_type != "comment"
                && !preview_lower.contains("mentioned")
                && !preview_lower.contains("commented")
                && !preview_lower.contains("@")
            {
                debug!("Skipping non-mention notification: {} (type: {})",
                    notification.id, notification.notification_type);
                continue;
            }

            notification.id = full_id;
            info!("Found new mention in {}: {} from {:?}",
                workspace_name, notification.id, notification.actor_name);
            result.push(notification);
        }

        result
    }

    /// Detect mentions using the agent (LLM-based analysis).
    async fn detect_mentions_with_agent(
        &self,
        browser: &NotionBrowser,
        workspace_name: &str,
    ) -> Result<Vec<NotionNotification>, NotionError> {
        let agent = self.agent_detector.as_ref().ok_or_else(|| {
            NotionError::ConfigError("Agent detector not initialized".to_string())
        })?;

        // Take a screenshot of the inbox
        let screenshot_path = format!(
            "/tmp/notion_inbox_{}_{}.png",
            workspace_name.replace(' ', "_").replace("'", ""),
            chrono::Utc::now().timestamp()
        );
        browser.screenshot(&screenshot_path).await?;

        // Get current browser state for element indices
        let state = browser.get_state().await?;

        // Check for UI blockers first
        let ui_state = agent.check_ui_state(Path::new(&screenshot_path), &state.raw).await?;

        if ui_state.blocked {
            info!(
                "UI blocked by {:?}, attempting to dismiss",
                ui_state.blocker_type
            );
            match ui_state.dismiss_action {
                UiAction::Click(idx) => {
                    browser.click(idx).await?;
                    sleep(Duration::from_secs(1)).await;
                    // Retake screenshot after dismissing
                    browser.screenshot(&screenshot_path).await?;
                }
                UiAction::PressEscape => {
                    browser.send_keys("Escape").await?;
                    sleep(Duration::from_secs(1)).await;
                    browser.screenshot(&screenshot_path).await?;
                }
                UiAction::Refresh => {
                    browser.navigate("https://www.notion.so").await?;
                    sleep(Duration::from_secs(2)).await;
                    browser.open_inbox().await?;
                    sleep(Duration::from_secs(2)).await;
                    browser.screenshot(&screenshot_path).await?;
                }
                UiAction::Wait(secs) => {
                    sleep(Duration::from_secs(secs as u64)).await;
                    browser.screenshot(&screenshot_path).await?;
                }
                UiAction::None => {}
            }
        }

        // Get updated state after any UI fixes
        let state = browser.get_state().await?;

        // Analyze the inbox with the agent
        let analysis = agent.analyze_inbox(Path::new(&screenshot_path), &state.raw).await?;

        if let Some(error) = analysis.error {
            warn!("Agent analysis returned error: {}", error);
        }

        if analysis.inbox_empty {
            debug!("Agent detected empty inbox for {}", workspace_name);
            return Ok(vec![]);
        }

        // Convert detected mentions to NotionNotification
        let notifications: Vec<NotionNotification> = analysis
            .mentions
            .into_iter()
            .filter(|m| m.confidence >= 0.5) // Filter low-confidence detections
            .map(|m| self.detected_mention_to_notification(m, workspace_name))
            .collect();

        info!(
            "Agent detected {} mentions in {} (scroll_needed: {})",
            notifications.len(),
            workspace_name,
            analysis.scroll_needed
        );

        // TODO: Handle scroll_needed - scroll down and analyze again if true

        // Keep screenshots for debugging - comment out when done
        // let _ = std::fs::remove_file(&screenshot_path);
        debug!("Screenshot saved to: {}", screenshot_path);

        Ok(notifications)
    }

    /// Convert a DetectedMention from agent analysis to NotionNotification.
    fn detected_mention_to_notification(
        &self,
        mention: DetectedMention,
        workspace_name: &str,
    ) -> NotionNotification {
        // Build ID from element index and page title for deduplication
        let id = if let Some(idx) = mention.element_index {
            format!(
                "agent_{}_{}_{}",
                idx,
                mention.page_title.replace(' ', "_").replace("'", ""),
                workspace_name.replace(' ', "_")
            )
        } else {
            format!(
                "agent_{}_{}",
                mention.page_title.replace(' ', "_").replace("'", ""),
                chrono::Utc::now().timestamp_millis()
            )
        };

        NotionNotification {
            id,
            notification_type: "mention".to_string(),
            workspace_id: None,
            workspace_name: Some(workspace_name.to_string()),
            page_id: String::new(), // Will be extracted when navigating to page
            block_id: None,
            actor_id: None,
            actor_name: Some(mention.mentioner),
            preview_text: Some(mention.snippet),
            url: mention.page_url.unwrap_or_default(),
            created_at: None,
            is_read: false,
        }
    }

    /// Parse notifications from browser state output (hardcoded patterns).
    fn parse_notifications_from_state(&self, raw: &str) -> Vec<NotionNotification> {
        let mut notifications = Vec::new();
        let mut seen_ids = std::collections::HashSet::new();
        // Content-based deduplication to prevent multiple patterns matching the same notification
        let mut seen_content = std::collections::HashSet::new();

        // Pattern 0: Inbox panel format from browser-use state
        // Format in state output (with tabs):
        //   *[12541]<a role=link />
        //   \t*[12509]<div role=img aria-label=D />
        //   \tDowhiz
        //   \tcommented in
        //   \tImprove website copy
        //   \t2d OR Mar 2 OR Yesterday
        //   \t*[12535]<span />
        //   \t\tOliver:
        //   这个任务的描述已经很清晰了！...
        //
        // We look for: [index]<a role=link /> followed by actor, "commented in" or "评论于", page name, and @mention
        // Time format can be: "2d", "5h", "Mar 2", "Yesterday", "3小时", "1天", etc.
        // Actor names can be Chinese or English (e.g., "Liu Xintong", "Dowhiz")
        if let Ok(inbox_re) = Regex::new(
            r#"\*?\[(\d+)\]<a\s+role=link[^>]*>\s*\n\t*\*?\[\d+\]<[^>]+>\s*\n\t*([^\n\t\[*]+?)\s*\n\t*(?:commented in|评论于)\s*\n\t*([^\n]+)\s*\n\t*(?:\d+[dhm小时天]?|[A-Z][a-z]{2}\s+\d+|\d+月\d+日|Yesterday|Today|今天|昨天)\s*\n(?:[^\n]*\n)*?\t+@?([A-Za-z][A-Za-z\s]+?)(?:\s+at\s+[^:]+)?:\s*\n([^\n\[*]+)"#
        ) {
            for caps in inbox_re.captures_iter(raw) {
                if let (Some(idx), Some(actor), Some(page), Some(mentioned), Some(preview)) =
                    (caps.get(1), caps.get(2), caps.get(3), caps.get(4), caps.get(5))
                {
                    let idx_str = idx.as_str();
                    let actor_name = actor.as_str().trim().to_string();
                    let page_title = page.as_str().trim().to_string();
                    let mentioned_name = mentioned.as_str().trim().to_string();
                    let preview_text = preview.as_str().trim().to_string();

                    // Only process if this mentions our employee
                    // Match if mentioned_name contains employee_name (handles "@Oliver at DoWhiz" matching "Oliver")
                    let employee_name = &self.config.employee_name;
                    let mentioned_lower = mentioned_name.to_lowercase();
                    let employee_lower = employee_name.to_lowercase();
                    if !mentioned_lower.contains(&employee_lower) && !mentioned_lower.eq(&employee_lower) {
                        debug!("Skipping notification mentioning {} (looking for {})", mentioned_name, employee_name);
                        continue;
                    }

                    let id = format!("inbox_{}_{}", idx_str, page_title.replace(' ', "_").replace("'", ""));

                    if seen_ids.contains(&id) {
                        continue;
                    }
                    seen_ids.insert(id.clone());

                    // Content-based deduplication (different patterns may capture same notification with different element indices)
                    let content_key = format!("{}_{}_{}",
                        actor_name.to_lowercase(),
                        page_title.to_lowercase().replace(' ', "_"),
                        mentioned_name.to_lowercase());
                    if seen_content.contains(&content_key) {
                        debug!("Skipping duplicate notification (content match): {}", content_key);
                        continue;
                    }
                    seen_content.insert(content_key);

                    info!("Found inbox notification: {} commented in {} mentioning @{}: {}",
                        actor_name, page_title, mentioned_name, preview_text);

                    notifications.push(NotionNotification {
                        id,
                        notification_type: "comment".to_string(),
                        workspace_id: None,
                        workspace_name: None,
                        page_id: String::new(),
                        block_id: None,
                        actor_id: None,
                        actor_name: Some(actor_name),
                        preview_text: Some(format!("@{}: {}", mentioned_name, preview_text)),
                        url: String::new(),
                        created_at: None,
                        is_read: false,
                    });
                }
            }
        }

        // Pattern 0b: Inbox panel format with tabs - captures link element index
        // Format (supports both English and Chinese UI):
        //   *[index]<a role=link />   <- this index for clicking
        //   \t*[index]<div role=img ...>
        //   \tLiu Xintong OR Dowhiz
        //   \tcommented in OR 评论于
        //   \tDowhiz testing OR Improve website copy
        //   \t3小时 OR 2d OR Mar 2 OR Yesterday
        //   \t*[index]<span />
        //   \t\t@Oliver at DoWhiz: OR Oliver:
        //   change this line to heading format OR 这个任务的描述...
        if let Ok(simple_re) = Regex::new(
            r#"\*?\[(\d+)\]<a[^>]*role=link[^>]*/?>\s*\n(?:[^\n]*\n)*?\t*([^\n\t\[*]+?)\s*\n\t*(?:commented in|评论于)\s*\n\t*([^\n]+)\s*\n\t*(?:\d+[dhm小时天]?|[A-Z][a-z]{2}\s+\d+|\d+月\d+日|Yesterday|Today|今天|昨天)\s*\n(?:[^\n]*\n)*?\t*@?([A-Za-z][A-Za-z\s]+?)(?:\s+at\s+[^:]+)?:\s*\n([^\n\[*]+)"#
        ) {
            for caps in simple_re.captures_iter(raw) {
                if let (Some(idx), Some(actor), Some(page), Some(mentioned), Some(preview)) =
                    (caps.get(1), caps.get(2), caps.get(3), caps.get(4), caps.get(5))
                {
                    let idx_str = idx.as_str();
                    let actor_name = actor.as_str().trim().to_string();
                    let page_title = page.as_str().trim().to_string();
                    let mentioned_name = mentioned.as_str().trim().to_string();
                    let preview_text = preview.as_str().trim().to_string();

                    // Match if mentioned_name contains employee_name (handles "@Oliver at DoWhiz" matching "Oliver")
                    let employee_name = &self.config.employee_name;
                    let mentioned_lower = mentioned_name.to_lowercase();
                    let employee_lower = employee_name.to_lowercase();
                    if !mentioned_lower.contains(&employee_lower) && !mentioned_lower.eq(&employee_lower) {
                        continue;
                    }

                    // Use inbox_{idx}_{page} format so process_notification can extract the index
                    let id = format!("inbox_{}_{}", idx_str, page_title.replace(' ', "_").replace("'", ""));

                    if seen_ids.contains(&id) {
                        continue;
                    }
                    seen_ids.insert(id.clone());

                    // Content-based deduplication (same notification may match different patterns)
                    let content_key = format!("{}_{}_{}",
                        actor_name.to_lowercase(),
                        page_title.to_lowercase().replace(' ', "_"),
                        mentioned_name.to_lowercase());
                    if seen_content.contains(&content_key) {
                        debug!("Skipping duplicate notification (content match): {}", content_key);
                        continue;
                    }
                    seen_content.insert(content_key);

                    info!("Found inbox notification (simple): {} commented in {} mentioning @{}: {}",
                        actor_name, page_title, mentioned_name, preview_text);

                    notifications.push(NotionNotification {
                        id,
                        notification_type: "comment".to_string(),
                        workspace_id: None,
                        workspace_name: None,
                        page_id: String::new(),
                        block_id: None,
                        actor_id: None,
                        actor_name: Some(actor_name),
                        preview_text: Some(format!("@{}: {}", mentioned_name, preview_text)),
                        url: String::new(),
                        created_at: None,
                        is_read: false,
                    });
                }
            }
        }

        // Pattern 1: Look for "mentioned" text with element indices
        // Format: [123]<tag>...
        //              Person Name mentioned you...
        if let Ok(mention_re) = Regex::new(r"\[(\d+)\][^\n]*\n\s*([A-Z][a-z]+(?:\s+[A-Z][a-z]+)?)\s+mentioned") {
            for caps in mention_re.captures_iter(raw) {
                if let (Some(idx), Some(actor)) = (caps.get(1), caps.get(2)) {
                    let id = format!("mention_{}_{}", idx.as_str(),
                        actor.as_str().replace(' ', "_"));

                    if seen_ids.contains(&id) {
                        continue;
                    }
                    seen_ids.insert(id.clone());

                    // Try to find URL nearby
                    let url = self.extract_nearby_url(raw, idx.start());

                    notifications.push(NotionNotification {
                        id,
                        notification_type: "mention".to_string(),
                        workspace_id: None,
                        workspace_name: None,
                        page_id: extract_page_id_from_url(&url),
                        block_id: None,
                        actor_id: None,
                        actor_name: Some(actor.as_str().to_string()),
                        preview_text: Some(format!("{} mentioned you", actor.as_str())),
                        url,
                        created_at: None,
                        is_read: false,
                    });
                }
            }
        }

        // Pattern 2: Direct "mentioned" or "commented" in element text
        if let Ok(direct_re) = Regex::new(r"\[(\d+)\]<[^>]+>\s*\n?\s*([^\n\[]+(?:mentioned|commented)[^\n]*)") {
            for caps in direct_re.captures_iter(raw) {
                if let (Some(idx), Some(text)) = (caps.get(1), caps.get(2)) {
                    let text_str = text.as_str().trim();
                    let id = format!("notif_{}", idx.as_str());

                    if seen_ids.contains(&id) {
                        continue;
                    }
                    seen_ids.insert(id.clone());

                    // Extract actor name (usually first capitalized words)
                    let actor_name = Regex::new(r"^([A-Z][a-z]+(?:\s+[A-Z][a-z]+)?)")
                        .ok()
                        .and_then(|r| r.captures(text_str))
                        .and_then(|c| c.get(1))
                        .map(|m| m.as_str().to_string());

                    let notification_type = if text_str.to_lowercase().contains("mentioned") {
                        "mention"
                    } else {
                        "comment"
                    };

                    let url = self.extract_nearby_url(raw, idx.start());

                    notifications.push(NotionNotification {
                        id,
                        notification_type: notification_type.to_string(),
                        workspace_id: None,
                        workspace_name: None,
                        page_id: extract_page_id_from_url(&url),
                        block_id: None,
                        actor_id: None,
                        actor_name,
                        preview_text: Some(text_str.to_string()),
                        url,
                        created_at: None,
                        is_read: false,
                    });
                }
            }
        }

        // Pattern 3: Clickable notification items with role=button
        if let Ok(button_re) = Regex::new(r"\[(\d+)\]<(?:div|a)[^>]*role=button[^>]*>\s*\n?\s*([^\n\[]+)") {
            for caps in button_re.captures_iter(raw) {
                if let (Some(idx), Some(text)) = (caps.get(1), caps.get(2)) {
                    let text_str = text.as_str().trim();

                    // Only include if it looks like a notification
                    if !text_str.to_lowercase().contains("mention")
                        && !text_str.to_lowercase().contains("comment")
                        && !text_str.to_lowercase().contains("replied") {
                        continue;
                    }

                    let id = format!("btn_{}", idx.as_str());

                    if seen_ids.contains(&id) {
                        continue;
                    }
                    seen_ids.insert(id.clone());

                    notifications.push(NotionNotification {
                        id,
                        notification_type: "mention".to_string(),
                        workspace_id: None,
                        workspace_name: None,
                        page_id: String::new(),
                        block_id: None,
                        actor_id: None,
                        actor_name: None,
                        preview_text: Some(text_str.to_string()),
                        url: String::new(),
                        created_at: None,
                        is_read: false,
                    });
                }
            }
        }

        notifications
    }

    /// Extract URL from text near a position.
    fn extract_nearby_url(&self, raw: &str, pos: usize) -> String {
        // Look for href= or notion.so URL within 500 chars of position
        let start = pos.saturating_sub(200);
        let end = (pos + 500).min(raw.len());
        let slice = &raw[start..end];

        // Try href first
        if let Ok(href_re) = Regex::new(r#"href="([^"]+notion\.so[^"]*)""#) {
            if let Some(caps) = href_re.captures(slice) {
                if let Some(url) = caps.get(1) {
                    return url.as_str().to_string();
                }
            }
        }

        // Try bare URL
        if let Ok(url_re) = Regex::new(r#"https://(?:www\.)?notion\.so/[^\s<>"']+"#) {
            if let Some(m) = url_re.find(slice) {
                return m.as_str().to_string();
            }
        }

        String::new()
    }

    /// Process a single notification.
    async fn process_notification(&mut self, notification: NotionNotification) -> Result<(), NotionError> {
        info!(
            "Processing notification {} from {:?}",
            notification.id, notification.actor_name
        );

        let browser = self
            .browser
            .as_ref()
            .ok_or_else(|| NotionError::BrowserError("Browser not initialized".to_string()))?;

        // Navigate to the mentioned page
        if !notification.url.is_empty() {
            browser.navigate(&notification.url).await?;
        } else if !notification.page_id.is_empty() {
            browser.go_to_page(&notification.page_id).await?;
        } else {
            // Try to click on the notification element if we have an index
            if let Some(idx_str) = notification.id.split('_').nth(1) {
                if let Ok(idx) = idx_str.parse::<u32>() {
                    info!("No URL available, clicking notification element {}", idx);
                    browser.click(idx).await?;
                    sleep(Duration::from_secs(2)).await;
                } else {
                    return Err(NotionError::ParseError(
                        "Notification has no URL, page ID, or clickable element".to_string(),
                    ));
                }
            } else {
                return Err(NotionError::ParseError(
                    "Notification has no URL or page ID".to_string(),
                ));
            }
        }

        // Get page state for context
        let state = browser.get_state().await?;
        let page_context = self.extract_page_context(&state.raw, &state.url);

        // Build the mention object
        let mention = NotionMention {
            id: notification.id.clone(),
            workspace_id: notification.workspace_id.clone().unwrap_or_default(),
            workspace_name: notification.workspace_name.clone().unwrap_or_default(),
            page_id: notification.page_id.clone(),
            page_title: page_context.title.clone(),
            block_id: notification.block_id.clone(),
            comment_id: None,
            sender_name: notification.actor_name.clone().unwrap_or_default(),
            sender_id: notification.actor_id.clone(),
            comment_text: notification.preview_text.clone().unwrap_or_default(),
            thread_context: page_context.comment_thread.clone(),
            url: state.url.clone(),
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

    /// Extract page context from browser state.
    fn extract_page_context(&self, raw: &str, url: &str) -> NotionPageContext {
        // Extract title from first heading or title-like element
        let title = Regex::new(r"(?:title|h1|page-title)[^>]*>\s*\n?\s*([^\n<\[]+)")
            .ok()
            .and_then(|r| r.captures(raw))
            .and_then(|c| c.get(1))
            .map(|m| m.as_str().trim().to_string())
            .unwrap_or_else(|| "Untitled".to_string());

        // Extract text content (simplified - just grab visible text)
        let mut content_parts = Vec::new();
        if let Ok(text_re) = Regex::new(r"\]<[^>]+>\s*\n?\s*([A-Za-z][^\n\[]{10,})") {
            for caps in text_re.captures_iter(raw) {
                if let Some(text) = caps.get(1) {
                    let t = text.as_str().trim();
                    if !t.is_empty() && !content_parts.contains(&t.to_string()) {
                        content_parts.push(t.to_string());
                    }
                }
            }
        }

        let content_text = content_parts.join("\n");

        NotionPageContext {
            title,
            page_id: extract_page_id_from_url(url),
            url: url.to_string(),
            content_text,
            parent_page_id: None,
            database_id: None,
            comment_thread: Vec::new(),
        }
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
        let Some(ref queue) = self.queue else {
            warn!(
                "No queue configured, Notion message from {} will not be processed",
                message.sender
            );
            return Ok(());
        };

        // Create dedupe key from sender + page + timestamp prefix (minute granularity)
        let dedupe_key = format!(
            "notion:{}:{}:{}",
            message.sender,
            message.thread_id,
            Utc::now().format("%Y%m%d%H%M")
        );

        let envelope = IngestionEnvelope {
            envelope_id: Uuid::new_v4(),
            received_at: Utc::now(),
            tenant_id: None,
            employee_id: self.config.employee_id.clone(),
            channel: Channel::Notion,
            external_message_id: message.message_id.clone(),
            dedupe_key,
            payload: IngestionPayload::from_inbound(&message),
            raw_payload_ref: None,
        };

        queue
            .enqueue(&envelope)
            .map_err(|e| NotionError::QueueError(e.to_string()))?;

        info!(
            "Enqueued Notion message: {} from {} about {}",
            envelope.envelope_id,
            message.sender,
            message.subject.as_deref().unwrap_or("(no subject)")
        );

        Ok(())
    }

    /// Get the reply queue directory for this employee.
    fn reply_queue_dir(&self) -> PathBuf {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("/tmp"))
            .join(".dowhiz")
            .join("notion_reply_queue")
            .join(&self.config.employee_id)
    }

    /// Process pending reply requests from the queue.
    pub async fn process_reply_queue(&mut self) -> Result<usize, NotionError> {
        let queue_dir = self.reply_queue_dir();
        if !queue_dir.exists() {
            return Ok(0);
        }

        let entries = match fs::read_dir(&queue_dir) {
            Ok(entries) => entries,
            Err(e) => {
                warn!("Failed to read reply queue directory: {}", e);
                return Ok(0);
            }
        };

        let mut processed_count = 0;

        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_file() || path.extension().map_or(true, |ext| ext != "json") {
                continue;
            }

            let content = match fs::read_to_string(&path) {
                Ok(c) => c,
                Err(e) => {
                    warn!("Failed to read reply request {:?}: {}", path, e);
                    continue;
                }
            };

            let request: serde_json::Value = match serde_json::from_str(&content) {
                Ok(v) => v,
                Err(e) => {
                    warn!("Failed to parse reply request {:?}: {}", path, e);
                    let _ = self.move_request_to_failed(&path, &format!("Parse error: {}", e));
                    continue;
                }
            };

            let status = request.get("status").and_then(|v| v.as_str()).unwrap_or("");
            if status != "pending" {
                continue;
            }

            match self.process_reply_request(&request).await {
                Ok(()) => {
                    info!("Successfully posted Notion reply for request {:?}", path.file_name());
                    if let Err(e) = fs::remove_file(&path) {
                        warn!("Failed to remove processed request {:?}: {}", path, e);
                    }
                    processed_count += 1;
                }
                Err(e) => {
                    warn!("Failed to process reply request {:?}: {}", path, e);
                    let _ = self.move_request_to_failed(&path, &e.to_string());
                }
            }
        }

        Ok(processed_count)
    }

    /// Process a single reply request.
    async fn process_reply_request(&mut self, request: &serde_json::Value) -> Result<(), NotionError> {
        let reply_text = request
            .get("reply_text")
            .and_then(|v| v.as_str())
            .ok_or_else(|| NotionError::ParseError("Missing reply_text".to_string()))?;

        let url = request.get("url").and_then(|v| v.as_str());
        let page_id = request.get("page_id").and_then(|v| v.as_str());
        let comment_id = request.get("comment_id").and_then(|v| v.as_str());

        self.ensure_browser().await?;

        let browser = self
            .browser
            .as_ref()
            .ok_or_else(|| NotionError::BrowserError("Browser not initialized".to_string()))?;

        if let Some(u) = url {
            if !u.is_empty() {
                browser.navigate(u).await?;
            }
        } else if let Some(pid) = page_id {
            if !pid.is_empty() {
                browser.go_to_page(pid).await?;
            }
        } else {
            return Err(NotionError::ParseError(
                "No URL or page_id in reply request".to_string(),
            ));
        }

        let result = browser.reply_to_comment(comment_id, reply_text).await?;

        if !result.success {
            return Err(NotionError::BrowserError(
                result.error.unwrap_or_else(|| "Unknown error".to_string()),
            ));
        }

        Ok(())
    }

    /// Move a failed request to a failed subdirectory.
    fn move_request_to_failed(&self, path: &PathBuf, error: &str) -> Result<(), NotionError> {
        let queue_dir = self.reply_queue_dir();
        let failed_dir = queue_dir.join("failed");
        fs::create_dir_all(&failed_dir)?;

        if let Ok(content) = fs::read_to_string(path) {
            if let Ok(mut request) = serde_json::from_str::<serde_json::Value>(&content) {
                if let Some(obj) = request.as_object_mut() {
                    obj.insert("status".to_string(), serde_json::json!("failed"));
                    obj.insert("error".to_string(), serde_json::json!(error));
                    obj.insert("failed_at".to_string(), serde_json::json!(Utc::now().to_rfc3339()));
                }

                let failed_path = failed_dir.join(path.file_name().unwrap_or_default());
                let _ = fs::write(&failed_path, serde_json::to_string_pretty(&request).unwrap_or_default());
            }
        }

        let _ = fs::remove_file(path);

        Ok(())
    }
}

/// Extract page ID from a Notion URL.
fn extract_page_id_from_url(url: &str) -> String {
    if url.is_empty() {
        return String::new();
    }

    // Remove query params and trailing slash
    let url = url.split('?').next().unwrap_or(url).trim_end_matches('/');

    // Get last segment
    if let Some(last_segment) = url.split('/').last() {
        // Notion IDs are 32 hex chars, sometimes at end of title
        // Format: Page-Title-abc123def456789012345678901234ab
        if let Some(id_part) = last_segment.split('-').last() {
            if id_part.len() == 32 && id_part.chars().all(|c| c.is_ascii_hexdigit()) {
                return id_part.to_string();
            }
        }

        // Or the whole segment could be the ID (with dashes)
        let clean = last_segment.replace('-', "");
        if clean.len() == 32 && clean.chars().all(|c| c.is_ascii_hexdigit()) {
            return clean;
        }
    }

    String::new()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_page_id_from_url() {
        assert_eq!(
            extract_page_id_from_url("https://www.notion.so/workspace/My-Page-abc123def456789012345678901234ab"),
            "abc123def456789012345678901234ab"
        );

        assert_eq!(
            extract_page_id_from_url("https://www.notion.so/abc123def456789012345678901234ab"),
            "abc123def456789012345678901234ab"
        );

        assert_eq!(
            extract_page_id_from_url("https://www.notion.so/abc12-3def-4567-8901-2345678901234ab"),
            "abc123def456789012345678901234ab"
        );

        assert_eq!(
            extract_page_id_from_url(""),
            ""
        );
    }

    #[test]
    fn test_parse_notifications_from_state() {
        let config = NotionPollerConfig {
            employee_id: "test".to_string(),
            notion_email: "test@example.com".to_string(),
            notion_password: "password".to_string(),
            poll_interval_secs: 30,
            browser_config: NotionBrowserConfig::default(),
            employee_name: "Test".to_string(),
            tenant_id: "default".to_string(),
            detection_mode: DetectionMode::Hardcoded, // Use hardcoded for unit tests
        };

        let poller = NotionBrowserPoller::new(
            config,
            MongoNotionProcessedStore::noop(),
            None,
        );

        let state = r#"
viewport: 1920x1080
url: https://www.notion.so/notifications
[50]<div role=button />
    John Doe mentioned you in a comment
[51]<a href="https://www.notion.so/workspace/Test-Page-abc123def456789012345678901234ab" />
    View page
[52]<div />
    Jane Smith commented on your page
"#;

        let notifications = poller.parse_notifications_from_state(state);
        assert!(!notifications.is_empty());

        // Should find "mentioned" notification
        let mention = notifications.iter().find(|n| n.notification_type == "mention");
        assert!(mention.is_some());
    }
}
