//! Notion browser session management using WebDriver (fantoccini).
//!
//! This module provides a browser automation layer for interacting with Notion
//! as a real user. It handles:
//! - Session creation with persistent profiles
//! - Login flow
//! - Navigation to pages and notifications
//! - Posting replies to comments

use fantoccini::{Client, ClientBuilder, Locator};
use serde_json::json;
use std::path::PathBuf;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{debug, info};

use super::models::NotionReplyResult;
use super::NotionError;

/// Configuration for the Notion browser instance.
#[derive(Debug, Clone)]
pub struct NotionBrowserConfig {
    /// WebDriver server URL (e.g., "http://localhost:4444")
    pub webdriver_url: String,
    /// Browser profile directory for session persistence
    pub profile_dir: PathBuf,
    /// Whether to run headless (false recommended for anti-detection)
    pub headless: bool,
    /// Slow-mo delay between actions (ms)
    pub slow_mo_ms: u64,
    /// Page load timeout (seconds)
    pub page_load_timeout_secs: u64,
    /// Element wait timeout (seconds)
    pub element_timeout_secs: u64,
}

impl Default for NotionBrowserConfig {
    fn default() -> Self {
        Self {
            webdriver_url: "http://localhost:4444".to_string(),
            profile_dir: PathBuf::from("/tmp/notion_browser_profile"),
            headless: false,
            slow_mo_ms: 100,
            page_load_timeout_secs: 30,
            element_timeout_secs: 10,
        }
    }
}

/// Notion browser automation client.
pub struct NotionBrowser {
    client: Client,
    config: NotionBrowserConfig,
    logged_in: bool,
}

impl NotionBrowser {
    /// Create a new Notion browser instance.
    ///
    /// This will connect to a WebDriver server (geckodriver or chromedriver)
    /// and create a new browser session.
    pub async fn new(config: NotionBrowserConfig) -> Result<Self, NotionError> {
        // Ensure profile directory exists
        if !config.profile_dir.exists() {
            std::fs::create_dir_all(&config.profile_dir)?;
        }

        // Build WebDriver capabilities
        let mut caps = serde_json::Map::new();

        // Firefox-specific options (geckodriver)
        let firefox_options = json!({
            "args": if config.headless {
                vec!["-headless"]
            } else {
                vec![]
            },
            "prefs": {
                // Disable webdriver detection
                "dom.webdriver.enabled": false,
                // Use profile directory
                "profile": config.profile_dir.to_string_lossy()
            }
        });
        caps.insert("moz:firefoxOptions".to_string(), firefox_options);

        // Chrome-specific options (chromedriver) - uncomment if using Chrome
        // let chrome_options = json!({
        //     "args": [
        //         if config.headless { "--headless" } else { "" },
        //         "--disable-blink-features=AutomationControlled",
        //         "--no-sandbox",
        //         format!("--user-data-dir={}", config.profile_dir.to_string_lossy())
        //     ].into_iter().filter(|s| !s.is_empty()).collect::<Vec<_>>()
        // });
        // caps.insert("goog:chromeOptions".to_string(), chrome_options);

        let client = ClientBuilder::native()
            .capabilities(caps)
            .connect(&config.webdriver_url)
            .await?;

        info!("Connected to WebDriver at {}", config.webdriver_url);

        Ok(Self {
            client,
            config,
            logged_in: false,
        })
    }

    /// Check if the browser is logged into Notion.
    pub async fn is_logged_in(&mut self) -> Result<bool, NotionError> {
        // Navigate to Notion home
        self.client
            .goto("https://www.notion.so")
            .await
            .map_err(|e| NotionError::NavigationError(e.to_string()))?;

        self.human_delay().await;

        // Check for login indicators
        let current_url = self.client.current_url().await?;
        let url_str = current_url.as_str();

        // If redirected to login page, not logged in
        if url_str.contains("/login") || url_str.contains("/signup") {
            self.logged_in = false;
            return Ok(false);
        }

        // Check for sidebar or workspace selector (indicates logged in)
        match self
            .client
            .find(Locator::Css(".notion-sidebar"))
            .await
        {
            Ok(_) => {
                self.logged_in = true;
                Ok(true)
            }
            Err(_) => {
                // Try another indicator
                match self
                    .client
                    .find(Locator::Css("[data-block-id]"))
                    .await
                {
                    Ok(_) => {
                        self.logged_in = true;
                        Ok(true)
                    }
                    Err(_) => {
                        self.logged_in = false;
                        Ok(false)
                    }
                }
            }
        }
    }

    /// Log into Notion with email and password.
    pub async fn login(&mut self, email: &str, password: &str) -> Result<(), NotionError> {
        info!("Attempting to log into Notion as {}", email);

        // Navigate to login page
        self.client
            .goto("https://www.notion.so/login")
            .await
            .map_err(|e| NotionError::NavigationError(e.to_string()))?;

        self.human_delay().await;

        // Find and fill email field
        let email_input = self
            .wait_for_element(Locator::Css("input[type='email'], input[placeholder*='email' i]"))
            .await?;

        self.type_like_human(&email_input, email).await?;
        self.human_delay().await;

        // Click continue/next button
        let continue_btn = self
            .wait_for_element(Locator::Css("button[type='submit'], div[role='button']"))
            .await?;
        continue_btn.click().await?;
        self.human_delay().await;

        // Wait for password field (Notion shows it after email)
        sleep(Duration::from_secs(2)).await;

        let password_input = self
            .wait_for_element(Locator::Css("input[type='password']"))
            .await?;

        self.type_like_human(&password_input, password).await?;
        self.human_delay().await;

        // Click login button
        let login_btn = self
            .wait_for_element(Locator::Css("button[type='submit']"))
            .await?;
        login_btn.click().await?;

        // Wait for login to complete
        sleep(Duration::from_secs(5)).await;

        // Verify login succeeded
        if self.is_logged_in().await? {
            info!("Successfully logged into Notion as {}", email);
            Ok(())
        } else {
            Err(NotionError::LoginFailed(
                "Login appeared to fail - could not verify logged in state".to_string(),
            ))
        }
    }

    /// Ensure logged in, performing login if necessary.
    pub async fn ensure_logged_in(&mut self, email: &str, password: &str) -> Result<(), NotionError> {
        if !self.is_logged_in().await? {
            self.login(email, password).await?;
        }
        Ok(())
    }

    /// Navigate to the Notion notifications page.
    pub async fn go_to_notifications(&mut self) -> Result<(), NotionError> {
        debug!("Navigating to notifications page");

        self.client
            .goto("https://www.notion.so/notifications")
            .await
            .map_err(|e| NotionError::NavigationError(e.to_string()))?;

        self.human_delay().await;

        // Wait for notifications to load
        self.wait_for_element(Locator::Css("[data-block-id], .notion-page-content"))
            .await?;

        Ok(())
    }

    /// Navigate to a specific Notion page by ID.
    pub async fn go_to_page(&mut self, page_id: &str) -> Result<(), NotionError> {
        debug!("Navigating to page: {}", page_id);

        let url = format!("https://www.notion.so/{}", page_id.replace("-", ""));
        self.client
            .goto(&url)
            .await
            .map_err(|e| NotionError::NavigationError(e.to_string()))?;

        self.human_delay().await;

        // Wait for page content to load
        self.wait_for_element(Locator::Css("[data-block-id], .notion-page-content"))
            .await?;

        Ok(())
    }

    /// Navigate to a specific URL.
    pub async fn go_to_url(&mut self, url: &str) -> Result<(), NotionError> {
        debug!("Navigating to URL: {}", url);

        self.client
            .goto(url)
            .await
            .map_err(|e| NotionError::NavigationError(e.to_string()))?;

        self.human_delay().await;
        Ok(())
    }

    /// Get the current page's HTML content.
    pub async fn get_page_html(&self) -> Result<String, NotionError> {
        let html = self
            .client
            .source()
            .await
            .map_err(|e| NotionError::BrowserError(e.to_string()))?;
        Ok(html)
    }

    /// Get the current URL.
    pub async fn current_url(&self) -> Result<String, NotionError> {
        let url = self
            .client
            .current_url()
            .await
            .map_err(|e| NotionError::BrowserError(e.to_string()))?;
        Ok(url.to_string())
    }

    /// Reply to a comment in the current page.
    ///
    /// This assumes we're already on the page containing the comment.
    pub async fn reply_to_comment(
        &mut self,
        comment_id: Option<&str>,
        reply_text: &str,
    ) -> Result<NotionReplyResult, NotionError> {
        info!("Replying to comment: {:?}", comment_id);

        // If we have a specific comment ID, try to locate and click it
        if let Some(cid) = comment_id {
            // Try to find the comment by ID or nearby elements
            let selector = format!(
                "[data-comment-id='{}'], [data-block-id='{}']",
                cid, cid
            );
            if let Ok(comment_el) = self.client.find(Locator::Css(&selector)).await {
                comment_el.click().await.ok();
                self.human_delay().await;
            }
        }

        // Look for the reply input or comment box
        let reply_input = match self
            .wait_for_element(Locator::Css(
                "[contenteditable='true'][data-placeholder*='Reply'], \
                 [contenteditable='true'][placeholder*='reply' i], \
                 .notion-comment-input [contenteditable='true']",
            ))
            .await
        {
            Ok(el) => el,
            Err(_) => {
                // Try clicking "Reply" button first
                if let Ok(reply_btn) = self
                    .client
                    .find(Locator::Css("button:contains('Reply'), [role='button']:contains('Reply')"))
                    .await
                {
                    reply_btn.click().await.ok();
                    self.human_delay().await;
                }

                // Now try to find the input again
                self.wait_for_element(Locator::Css("[contenteditable='true']"))
                    .await?
            }
        };

        // Type the reply with human-like delays
        self.type_like_human(&reply_input, reply_text).await?;
        self.human_delay().await;

        // Find and click the submit/send button
        let submit_btn = self
            .wait_for_element(Locator::Css(
                "button[type='submit'], \
                 button:contains('Send'), \
                 [role='button'][aria-label*='send' i], \
                 .notion-comment-submit",
            ))
            .await?;

        submit_btn.click().await?;

        // Wait for the reply to be posted
        sleep(Duration::from_secs(2)).await;

        info!("Reply posted successfully");
        Ok(NotionReplyResult {
            success: true,
            comment_id: None, // Could try to extract from DOM
            error: None,
        })
    }

    /// Close the browser session.
    pub async fn close(self) -> Result<(), NotionError> {
        self.client
            .close()
            .await
            .map_err(|e| NotionError::BrowserError(e.to_string()))?;
        Ok(())
    }

    // =========================================================================
    // Private helper methods
    // =========================================================================

    /// Wait for an element to be present in the DOM.
    async fn wait_for_element(
        &self,
        locator: Locator<'_>,
    ) -> Result<fantoccini::elements::Element, NotionError> {
        let timeout = Duration::from_secs(self.config.element_timeout_secs);
        let poll_interval = Duration::from_millis(500);
        let start = std::time::Instant::now();

        loop {
            match self.client.find(locator.clone()).await {
                Ok(el) => return Ok(el),
                Err(_) if start.elapsed() < timeout => {
                    sleep(poll_interval).await;
                }
                Err(e) => {
                    return Err(NotionError::ElementNotFound(format!(
                        "Element not found after {:?}: {}",
                        timeout, e
                    )));
                }
            }
        }
    }

    /// Add a random human-like delay between actions.
    async fn human_delay(&self) {
        use rand::Rng;
        let base = self.config.slow_mo_ms;
        let jitter = rand::thread_rng().gen_range(0..base / 2);
        sleep(Duration::from_millis(base + jitter)).await;
    }

    /// Type text with human-like delays between keystrokes.
    async fn type_like_human(
        &self,
        element: &fantoccini::elements::Element,
        text: &str,
    ) -> Result<(), NotionError> {
        use rand::Rng;

        // Clear existing content first
        element.clear().await.ok();

        // Type each character with random delays
        for ch in text.chars() {
            element
                .send_keys(&ch.to_string())
                .await
                .map_err(|e| NotionError::BrowserError(e.to_string()))?;

            let delay = rand::thread_rng().gen_range(50..150);
            sleep(Duration::from_millis(delay)).await;
        }

        Ok(())
    }
}
