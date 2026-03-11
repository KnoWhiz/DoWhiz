//! Notion browser session management using browser-use CLI.
//!
//! This module provides a browser automation layer for interacting with Notion
//! as a real user. It uses the browser-use CLI tool for:
//! - Session creation with persistent profiles
//! - Login flow (Google OAuth)
//! - Navigation to pages and notifications
//! - Posting replies to comments

use regex::Regex;
use std::collections::HashMap;
use std::process::Command;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{debug, info, warn};

use super::models::NotionReplyResult;
use super::NotionError;

/// Configuration for the Notion browser instance.
#[derive(Debug, Clone)]
pub struct NotionBrowserConfig {
    /// Session name for browser-use
    pub session_name: String,
    /// Browser mode: "chromium", "real", or "remote"
    pub browser_mode: String,
    /// Whether to run headless
    pub headless: bool,
    /// Browser profile for "real" mode
    pub profile: Option<String>,
    /// Command timeout in seconds
    pub command_timeout_secs: u64,
    /// Page load wait time in seconds
    pub page_load_wait_secs: u64,
}

impl Default for NotionBrowserConfig {
    fn default() -> Self {
        Self {
            session_name: "notion".to_string(),
            browser_mode: "chromium".to_string(),
            headless: true,
            profile: None,
            command_timeout_secs: 60,
            page_load_wait_secs: 3,
        }
    }
}

/// Browser state returned by `browser-use state`
#[derive(Debug, Clone, Default)]
pub struct BrowserState {
    /// Current page URL
    pub url: String,
    /// Page title
    pub title: String,
    /// Viewport dimensions
    pub viewport: (u32, u32),
    /// Raw state output (contains element indices)
    pub raw: String,
    /// Extracted clickable elements: index -> text/description
    pub elements: HashMap<u32, String>,
}

/// Notion browser automation client using browser-use CLI.
pub struct NotionBrowser {
    config: NotionBrowserConfig,
    session_started: bool,
}

impl NotionBrowser {
    /// Create a new Notion browser instance.
    pub fn new(config: NotionBrowserConfig) -> Self {
        Self {
            config,
            session_started: false,
        }
    }

    /// Start the browser session.
    pub async fn start(&mut self) -> Result<(), NotionError> {
        if self.session_started {
            return Ok(());
        }

        info!("Starting browser-use session: {}", self.config.session_name);

        // Open a blank page to start the session
        let args = self.build_open_args("about:blank");
        let (success, output) = self.run_browser_use(&args).await?;

        if !success {
            return Err(NotionError::BrowserError(format!(
                "Failed to start browser session: {}",
                output
            )));
        }

        self.session_started = true;
        info!("Browser session started successfully");
        Ok(())
    }

    /// Check if the browser is logged into Notion.
    pub async fn is_logged_in(&self) -> Result<bool, NotionError> {
        info!("Checking if logged into Notion...");

        // Navigate to Notion home
        self.navigate("https://www.notion.so").await?;
        sleep(Duration::from_secs(self.config.page_load_wait_secs)).await;

        // Get current state
        let state = self.get_state().await?;

        // Check URL for login indicators
        if state.url.contains("/login") || state.url.contains("/signup") {
            info!("Not logged in: URL contains /login or /signup");
            return Ok(false);
        }

        // Check for login page elements (in browser-use state output format)
        let login_page_indicators = [
            "Log in with",
            "Enter your email",
            "Continue with Google",
            "Continue with Apple",
            "Sign up",
            "Log in",
        ];
        for indicator in &login_page_indicators {
            if state.raw.contains(indicator) {
                info!("Not logged in: found login page element '{}'", indicator);
                return Ok(false);
            }
        }

        // Check for logged-in indicators (text that appears in browser-use state when logged in)
        // These are UI elements only visible when logged in
        let logged_in_indicators = [
            "New page",           // aria-label for new page button
            "Search",             // Search button in sidebar
            "Inbox",              // Inbox button
            "Settings",           // Settings button
            "at DoWhiz",          // Workspace name pattern
            "'s Space",           // Workspace name pattern
            "'s Notion",          // Workspace name pattern
            "Home",               // Home link in sidebar
            "Teamspaces",         // Teamspaces section
            "Private",            // Private section
        ];

        for indicator in &logged_in_indicators {
            if state.raw.contains(indicator) {
                info!("Logged in: found indicator '{}'", indicator);
                return Ok(true);
            }
        }

        // If on notion.so without login redirect and not showing login page, likely logged in
        if state.url.contains("notion.so") && !state.url.contains("/login") {
            // Double-check by looking for workspace button which always exists when logged in
            if state.raw.contains("role=button expanded=") {
                info!("Logged in: on notion.so with interactive elements");
                return Ok(true);
            }
        }

        info!("Not logged in: no indicators found. URL: {}", state.url);
        debug!("State preview: {}", &state.raw[..state.raw.len().min(500)]);
        Ok(false)
    }

    /// Log into Notion with Google OAuth (email is Google account, password is Google password).
    ///
    /// IMPORTANT: This method implements rate limiting to avoid triggering Google's
    /// security mechanisms. If cookies exist and are valid, they will be used instead
    /// of performing a fresh login.
    pub async fn login(&mut self, email: &str, password: &str) -> Result<(), NotionError> {
        // First try to import existing cookies - this is the preferred method
        let cookie_path = shellexpand::tilde("~/.dowhiz/notion/cookies.json").to_string();
        if std::path::Path::new(&cookie_path).exists() {
            info!("Found existing cookies, attempting to import...");
            if self.import_cookies(&cookie_path).await.is_ok() {
                // Navigate to Notion to check if cookies work
                self.navigate("https://www.notion.so").await?;
                sleep(Duration::from_secs(5)).await;

                if self.is_logged_in().await? {
                    info!("Successfully logged in using saved cookies");
                    return Ok(());
                }
                info!("Saved cookies expired or invalid");

                // Check if we should attempt Google OAuth at all
                // To avoid rate limiting, we only attempt fresh login if explicitly requested
                let force_login = std::env::var("NOTION_FORCE_LOGIN").ok().as_deref() == Some("true");
                if !force_login {
                    return Err(NotionError::LoginFailed(
                        "Cookies expired. Set NOTION_FORCE_LOGIN=true to attempt Google OAuth, \
                         or manually login and export cookies.".to_string()
                    ));
                }
            }
        } else {
            // No cookies exist - check if we should attempt automated login
            let force_login = std::env::var("NOTION_FORCE_LOGIN").ok().as_deref() == Some("true");
            if !force_login {
                return Err(NotionError::LoginFailed(
                    "No saved cookies found. Please manually login to Notion first using: \
                     browser-use --session notion --browser chromium --headed open https://notion.so/login \
                     Then export cookies with: browser-use --session notion cookies export ~/.dowhiz/notion/cookies.json".to_string()
                ));
            }
        }

        info!("Logging into Notion as {} via Google OAuth (NOTION_FORCE_LOGIN=true)", email);

        // Navigate to login page
        self.navigate("https://www.notion.so/login").await?;
        sleep(Duration::from_secs(5)).await;

        // Get state and click Google login button
        let state = self.get_state().await?;
        info!("Login page URL: {}", state.url);
        info!("Login page elements count: {}", state.elements.len());

        // Check if we're actually on the login page
        if !state.url.contains("/login") {
            info!("Warning: Not on login page, URL is: {}", state.url);
            // Try navigating again
            self.navigate("https://www.notion.so/login").await?;
            sleep(Duration::from_secs(5)).await;
        }

        let google_idx = self.find_element_index(&state.raw, &["Google"])
            .ok_or_else(|| {
                info!("Available elements: {:?}", state.elements.values().take(20).collect::<Vec<_>>());
                NotionError::ElementNotFound("Google login button".to_string())
            })?;

        info!("Found Google button at index {}", google_idx);
        self.click(google_idx).await?;
        sleep(Duration::from_secs(5)).await;

        // Switch to Google popup tab
        if let Err(e) = self.switch_tab(1).await {
            info!("Failed to switch to tab 1, trying to find email input on current page: {}", e);
        }
        sleep(Duration::from_secs(3)).await;

        // Get Google login state and find email input
        let state = self.get_state().await?;
        debug!("Google login page URL: {}", state.url);
        debug!("Google login state (first 500 chars): {}", &state.raw[..state.raw.len().min(500)]);

        let email_idx = self.find_input_index(&state.raw, &["email", "Email", "identifier", "identifierId"])
            .ok_or_else(|| {
                info!("Google page elements (first 10): {:?}", state.elements.values().take(10).collect::<Vec<_>>());
                NotionError::ElementNotFound("Google email input".to_string())
            })?;

        info!("Found Google email input at index {}", email_idx);
        self.input(email_idx, email).await?;
        sleep(Duration::from_secs(1)).await;

        // Find and click Next button
        let state = self.get_state().await?;
        let next_idx = self.find_element_index(&state.raw, &["Next"])
            .ok_or_else(|| NotionError::ElementNotFound("Next button".to_string()))?;

        info!("Found Next button at index {}", next_idx);
        self.click(next_idx).await?;
        sleep(Duration::from_secs(5)).await;

        // Get password input
        let state = self.get_state().await?;
        let pwd_idx = self.find_input_index(&state.raw, &["password", "Password", "Passwd"])
            .ok_or_else(|| NotionError::ElementNotFound("Google password input".to_string()))?;

        info!("Found Google password input at index {}", pwd_idx);
        self.input(pwd_idx, password).await?;
        sleep(Duration::from_secs(1)).await;

        // Find and click Next for password
        let state = self.get_state().await?;
        let next_idx = self.find_element_index(&state.raw, &["Next"])
            .ok_or_else(|| NotionError::ElementNotFound("Password Next button".to_string()))?;

        info!("Found password Next button at index {}", next_idx);
        self.click(next_idx).await?;
        sleep(Duration::from_secs(8)).await;

        // Switch back to main tab (Notion)
        self.switch_tab(0).await?;
        sleep(Duration::from_secs(3)).await;

        // Verify login success
        if self.is_logged_in().await? {
            info!("Successfully logged into Notion via Google OAuth");

            // Export cookies for future use
            if let Err(e) = self.export_cookies(&cookie_path).await {
                info!("Warning: Failed to export cookies: {}", e);
            } else {
                info!("Cookies saved for future sessions");
            }

            Ok(())
        } else {
            let state = self.get_state().await?;
            Err(NotionError::LoginFailed(format!(
                "Google OAuth login failed. Current URL: {}",
                state.url
            )))
        }
    }

    /// Ensure logged in, performing login if necessary.
    pub async fn ensure_logged_in(&mut self, email: &str, password: &str) -> Result<(), NotionError> {
        if !self.is_logged_in().await? {
            self.login(email, password).await?;
        }
        Ok(())
    }

    /// Navigate to the Notion notifications/inbox.
    /// Note: Notion's inbox is a sidebar panel, not a separate page.
    /// This method navigates to home first, then clicks the Inbox button.
    pub async fn go_to_notifications(&self) -> Result<(), NotionError> {
        info!("Opening Notion inbox panel");

        // First ensure we're on Notion home
        self.navigate("https://www.notion.so").await?;
        sleep(Duration::from_secs(self.config.page_load_wait_secs)).await;

        // Click the Inbox button in sidebar
        self.open_inbox().await?;

        Ok(())
    }

    /// List all workspaces the user has access to.
    /// Returns a list of (workspace_name, element_index) tuples.
    pub async fn list_workspaces(&self) -> Result<Vec<(String, u32)>, NotionError> {
        info!("Listing all workspaces");

        // First ensure we're on Notion
        self.navigate("https://www.notion.so").await?;
        sleep(Duration::from_secs(2)).await;

        // Get state and find workspace switcher
        let state = self.get_state().await?;

        // Find the workspace switcher button (usually shows current workspace name)
        let workspace_switcher_idx = self.find_workspace_switcher(&state.raw);

        if let Some(idx) = workspace_switcher_idx {
            info!("Found workspace switcher at index {}", idx);
            self.click(idx).await?;

            // Wait for dropdown to fully load (Notion dropdowns can be slow)
            sleep(Duration::from_secs(2)).await;

            // Get updated state with workspace list
            let state = self.get_state().await?;

            // Verify dropdown opened by looking for typical dropdown indicators
            let dropdown_opened = state.raw.contains("Add another account")
                || state.raw.contains("添加其他账户")
                || state.raw.contains("Log out")
                || state.raw.contains("退出登录")
                || state.raw.contains("Invite members");

            if !dropdown_opened {
                warn!("Workspace dropdown may not have opened properly, retrying click");
                self.click(idx).await?;
                sleep(Duration::from_secs(2)).await;
            }

            let state = self.get_state().await?;

            // Save full state for debugging
            let debug_file = "/tmp/notion_workspace_dropdown_state.txt";
            if let Err(e) = std::fs::write(debug_file, &state.raw) {
                warn!("Failed to save dropdown state: {}", e);
            } else {
                debug!("Saved workspace dropdown state to {} ({} bytes)", debug_file, state.raw.len());
            }

            let workspaces = self.parse_workspace_list(&state.raw);
            info!("Found {} workspaces", workspaces.len());

            // Close dropdown by pressing Escape
            self.send_keys("Escape").await?;
            sleep(Duration::from_millis(500)).await;

            return Ok(workspaces);
        }

        // Fallback: try to find workspaces in sidebar directly
        let workspaces = self.parse_workspace_list(&state.raw);
        if !workspaces.is_empty() {
            return Ok(workspaces);
        }

        Err(NotionError::ElementNotFound("Workspace switcher not found".to_string()))
    }

    /// Switch to a specific workspace by name.
    /// Opens the workspace switcher dropdown and clicks the workspace.
    pub async fn switch_workspace_by_name(&self, workspace_name: &str) -> Result<(), NotionError> {
        info!("Switching to workspace: {}", workspace_name);

        // First ensure we're on Notion home
        self.navigate("https://www.notion.so").await?;
        sleep(Duration::from_secs(2)).await;

        // Open workspace switcher dropdown
        let state = self.get_state().await?;
        let workspace_switcher_idx = self.find_workspace_switcher(&state.raw);

        if let Some(idx) = workspace_switcher_idx {
            info!("Found workspace switcher at index {}, clicking to open dropdown", idx);
            self.click(idx).await?;
            sleep(Duration::from_secs(1)).await;

            // Get updated state with workspace list
            let state = self.get_state().await?;

            // Find the workspace by name in the dropdown
            let workspaces = self.parse_workspace_list(&state.raw);
            let target_ws = workspaces.iter().find(|(name, _)| name == workspace_name);

            if let Some((_, ws_idx)) = target_ws {
                info!("Found workspace '{}' at index {}, clicking", workspace_name, ws_idx);
                self.click(*ws_idx).await?;
                sleep(Duration::from_secs(2)).await;

                // Press Escape to close any remaining menus
                self.send_keys("Escape").await?;
                sleep(Duration::from_secs(1)).await;

                return Ok(());
            } else {
                // Close dropdown and return error
                self.send_keys("Escape").await?;
                return Err(NotionError::ElementNotFound(format!(
                    "Workspace '{}' not found in dropdown",
                    workspace_name
                )));
            }
        }

        Err(NotionError::ElementNotFound("Workspace switcher not found".to_string()))
    }

    /// Switch to a specific workspace by clicking its element (DEPRECATED - use switch_workspace_by_name).
    pub async fn switch_workspace(&self, workspace_idx: u32) -> Result<(), NotionError> {
        info!("Switching to workspace at index {} (deprecated method)", workspace_idx);
        self.click(workspace_idx).await?;
        sleep(Duration::from_secs(2)).await;

        // Press Escape to close any open menus (like workspace switcher)
        self.send_keys("Escape").await?;
        sleep(Duration::from_secs(1)).await;

        Ok(())
    }

    /// Open the inbox panel in the current workspace.
    pub async fn open_inbox(&self) -> Result<(), NotionError> {
        info!("Opening inbox panel");

        // Try up to 3 times to open inbox
        for attempt in 1..=3 {
            let state = self.get_state().await?;

            // Check if inbox is already open (look for inbox-specific content)
            if self.is_inbox_panel_open(&state.raw) {
                info!("Inbox panel already open");
                return Ok(());
            }

            // Find Inbox button - try multiple patterns
            let inbox_idx = self.find_element_index(&state.raw, &["Inbox", "收件箱"])
                .or_else(|| {
                    // Also try looking for role=button with Inbox text
                    if let Ok(re) = Regex::new(r"\[(\d+)\]<div[^>]*role=button[^>]*>\s*\n\s*Inbox") {
                        re.captures(&state.raw)
                            .and_then(|c| c.get(1))
                            .and_then(|m| m.as_str().parse().ok())
                    } else {
                        None
                    }
                });

            if let Some(idx) = inbox_idx {
                info!("Found inbox button at index {} (attempt {})", idx, attempt);
                self.click(idx).await?;
                sleep(Duration::from_secs(2)).await;

                // Verify inbox opened
                let new_state = self.get_state().await?;
                if self.is_inbox_panel_open(&new_state.raw) {
                    info!("Inbox panel opened successfully");
                    return Ok(());
                }

                // If not opened, try clicking again or with different approach
                if attempt < 3 {
                    warn!("Inbox panel not detected after click, retrying...");
                    sleep(Duration::from_secs(1)).await;
                }
            } else {
                info!("Available elements (first 30): {:?}",
                    state.elements.iter().take(30).collect::<Vec<_>>());
                return Err(NotionError::ElementNotFound("Inbox button not found".to_string()));
            }
        }

        // If we get here, inbox didn't open after retries
        warn!("Failed to verify inbox panel opened after 3 attempts, proceeding anyway");
        Ok(())
    }

    /// Check if the inbox panel is currently open.
    fn is_inbox_panel_open(&self, raw: &str) -> bool {
        // First, exclude workspace switcher menu (common false positive)
        // Workspace switcher has patterns like "Add workspace", "Log out", "Free Plan", "Guest"
        let workspace_switcher_indicators = [
            "Add workspace",
            "Add another account",
            "Log out",
            "Free Plan",
            "Plus Plan",
            "Business Plan",
            "Get iOS & Android",
            "Try the new sidebar",
        ];

        let mut workspace_switcher_count = 0;
        for indicator in &workspace_switcher_indicators {
            if raw.contains(indicator) {
                workspace_switcher_count += 1;
            }
        }
        // If we see multiple workspace switcher indicators, this is NOT the inbox
        if workspace_switcher_count >= 2 {
            return false;
        }

        // Inbox panel indicators (must have at least 2 to be confident):
        // - Section headers: "This week" / "Last week" / "Earlier"
        // - Action verbs: "mentioned you", "commented on", "invited you"
        let inbox_indicators = [
            "This week",
            "Last week",
            "Earlier",
            "今天",
            "本周",
            "上周",
            "mentioned you",
            "commented on",
            "invited you",
            "replied to your",
            "assigned you",
            "Email updated",
        ];

        let mut inbox_indicator_count = 0;
        for indicator in &inbox_indicators {
            if raw.contains(indicator) {
                inbox_indicator_count += 1;
            }
        }

        // Need at least one strong indicator
        inbox_indicator_count >= 1
    }

    /// Find the workspace switcher button in the state.
    fn find_workspace_switcher(&self, raw: &str) -> Option<u32> {
        // Pattern: workspace name followed by expanded=false or with aria-label containing workspace
        // Usually at top of sidebar

        // Look for button with workspace-like content at start of sidebar
        // Supports Unicode names (Chinese, etc.)
        let patterns = [
            // Pattern: expanded=false button with name ending in 's or -
            r"\[(\d+)\]<div[^>]*role=button[^>]*expanded=false[^>]*>\s*\n\s+[^\n\[\]<>]{3,}(?:'s|-)",
            // Pattern: button followed by plan tier (Free, Plus, Business, 免费)
            r"\[(\d+)\]<div[^>]*role=button[^>]*>\s*\n\s+[^\n\[\]<>]{3,}\s*\n\s+(?:免费|Free|Plus|Business)",
        ];

        for pattern in &patterns {
            if let Ok(re) = Regex::new(pattern) {
                if let Some(caps) = re.captures(raw) {
                    if let Some(idx) = caps.get(1) {
                        if let Ok(n) = idx.as_str().parse::<u32>() {
                            return Some(n);
                        }
                    }
                }
            }
        }

        None
    }

    /// Parse workspace list from state after opening workspace switcher dropdown.
    /// Only parses entries that follow the workspace dropdown format:
    /// - Workspace name followed by email address
    /// - Workspace name followed by plan type (Free, Plus, Guest, 免费, etc.)
    fn parse_workspace_list(&self, raw: &str) -> Vec<(String, u32)> {
        let mut workspaces = Vec::new();
        let mut seen = std::collections::HashSet::new();

        // Helper to validate workspace name (filter out page content)
        let is_valid_workspace_name = |name: &str| -> bool {
            let trimmed = name.trim();
            // Must be reasonably short (workspace names are typically < 50 chars)
            if trimmed.len() > 60 || trimmed.len() < 2 {
                return false;
            }
            // Should not contain sentence punctuation (page content often has these)
            if trimmed.contains('？') || trimmed.contains('！') || trimmed.contains('。')
                || trimmed.contains('?') || trimmed.contains('!')
                || (trimmed.contains('.') && !trimmed.contains("'s"))
                || trimmed.contains('：') || trimmed.contains(':')
            {
                return false;
            }
            // Should not be a common UI element or instruction
            let blocklist = [
                "这些是帮助", "点击", "输入", "高亮", "尝试", "探索",
                "Click", "Enter", "Try", "Explore", "Learn", "Get started",
                "Skip to content", "Search", "Home", "Settings", "Trash",
                "New page", "Add new", "More", "Library", "Inbox",
            ];
            for blocked in &blocklist {
                if trimmed.contains(blocked) {
                    return false;
                }
            }
            true
        };

        // Pattern 1: Workspace entries with email below (most reliable)
        // Format with tabs:
        // [idx]<div role=button>
        // \tWorkspace Name
        // \temail@domain.com
        if let Ok(re) = Regex::new(r"\[(\d+)\]<div[^>]*role=button[^>]*>\s*\n[\t\s]+([^\n\[\]<>]{2,60})\s*\n[\t\s]+[a-z0-9._%+-]+@[a-z0-9.-]+") {
            for caps in re.captures_iter(raw) {
                if let (Some(idx), Some(name)) = (caps.get(1), caps.get(2)) {
                    if let Ok(n) = idx.as_str().parse::<u32>() {
                        let workspace_name = name.as_str().trim().to_string();
                        if !seen.contains(&workspace_name) && is_valid_workspace_name(&workspace_name) {
                            debug!("Found workspace (email pattern): {}", workspace_name);
                            seen.insert(workspace_name.clone());
                            workspaces.push((workspace_name, n));
                        }
                    }
                }
            }
        }

        // Pattern 2: Workspace entries with plan tier below (Free, Plus, Business, 免费, etc.)
        if let Ok(re) = Regex::new(r"\[(\d+)\]<div[^>]*role=button[^>]*>\s*\n[\t\s]+([^\n\[\]<>]{2,60})\s*\n[\t\s]+(?:Free|Plus|Business|Enterprise|Guest|免费|付费|专业版)") {
            for caps in re.captures_iter(raw) {
                if let (Some(idx), Some(name)) = (caps.get(1), caps.get(2)) {
                    if let Ok(n) = idx.as_str().parse::<u32>() {
                        let workspace_name = name.as_str().trim().to_string();
                        if !seen.contains(&workspace_name) && is_valid_workspace_name(&workspace_name) {
                            debug!("Found workspace (plan pattern): {}", workspace_name);
                            seen.insert(workspace_name.clone());
                            workspaces.push((workspace_name, n));
                        }
                    }
                }
            }
        }

        // Pattern 3: Workspace entries with role=menuitem (dropdown menu context)
        // The actual format has workspace name deeply nested:
        // [idx]<div role=menuitem id=:xxx: />
        //     ... (nested divs)
        //             Workspace Name
        //         ... (optional Guest label)
        // We use a two-step approach: find menuitems, then extract names nearby
        if let Ok(menuitem_re) = Regex::new(r"\[(\d+)\]<div[^>]*role=menuitem[^>]*>") {
            for caps in menuitem_re.captures_iter(raw) {
                if let Some(idx) = caps.get(1) {
                    if let Ok(n) = idx.as_str().parse::<u32>() {
                        // Find the position and look for workspace name in next ~30 lines
                        let match_end = caps.get(0).unwrap().end();
                        let search_area = &raw[match_end..raw.len().min(match_end + 1500)];

                        // Look for workspace name pattern (Name's Notion, 的 Notion, or capitalized name)
                        if let Ok(name_re) = Regex::new(r"[\t\s]+([A-Z][^\n\[\]<>]{2,40}(?:'s\s+(?:Space|Notion|Workspace))?|[^\n\[\]<>]{2,30}的\s*Notion(?:\s*总部)?)\s*\n") {
                            if let Some(name_caps) = name_re.captures(search_area) {
                                if let Some(name_match) = name_caps.get(1) {
                                    let workspace_name = name_match.as_str().trim().to_string();
                                    if !seen.contains(&workspace_name) && is_valid_workspace_name(&workspace_name) {
                                        debug!("Found workspace (menuitem pattern): {} at index {}", workspace_name, n);
                                        seen.insert(workspace_name.clone());
                                        workspaces.push((workspace_name, n));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Pattern 4: Workspace with typical naming patterns (Name's Space/Notion, 的 Notion)
        // Followed by email or plan to ensure it's from the dropdown
        if let Ok(re) = Regex::new(r"\[(\d+)\]<[^>]+>\s*\n[\t\s]+([^\n\[\]<>]{2,40}(?:'s\s+(?:Space|Notion|Workspace)|的\s*Notion(?:\s*总部)?))\s*\n[\t\s]+(?:[a-z0-9._%+-]+@|Free|Plus|Guest|免费)") {
            for caps in re.captures_iter(raw) {
                if let (Some(idx), Some(name)) = (caps.get(1), caps.get(2)) {
                    if let Ok(n) = idx.as_str().parse::<u32>() {
                        let workspace_name = name.as_str().trim().to_string();
                        if !seen.contains(&workspace_name) && is_valid_workspace_name(&workspace_name) {
                            debug!("Found workspace (naming pattern): {}", workspace_name);
                            seen.insert(workspace_name.clone());
                            workspaces.push((workspace_name, n));
                        }
                    }
                }
            }
        }

        // Pattern 5 (fallback): Any button followed by text that looks like a workspace name
        // and then followed by an email (less strict but more reliable for varied formats)
        if workspaces.is_empty() {
            debug!("Trying fallback pattern for workspaces");
            if let Ok(re) = Regex::new(r"\[(\d+)\]<[^>]*role=button[^>]*/?>\s*\n[\t\s]*([A-Za-z][^\n\[\]<>]{1,50}(?:'s\s+\w+|的\s*\w+)?)\s*\n[\t\s]*[a-z0-9._%+-]+@") {
                for caps in re.captures_iter(raw) {
                    if let (Some(idx), Some(name)) = (caps.get(1), caps.get(2)) {
                        if let Ok(n) = idx.as_str().parse::<u32>() {
                            let workspace_name = name.as_str().trim().to_string();
                            if !seen.contains(&workspace_name) && is_valid_workspace_name(&workspace_name) {
                                debug!("Found workspace (fallback pattern): {}", workspace_name);
                                seen.insert(workspace_name.clone());
                                workspaces.push((workspace_name, n));
                            }
                        }
                    }
                }
            }
        }

        if workspaces.is_empty() {
            warn!("No workspaces found with any patterns. Raw state preview:\n{}", &raw[..raw.len().min(1000)]);
        } else {
            info!("Parsed {} workspaces from dropdown", workspaces.len());
        }

        workspaces
    }

    /// Navigate to a specific Notion page by ID.
    pub async fn go_to_page(&self, page_id: &str) -> Result<(), NotionError> {
        let clean_id = page_id.replace("-", "");
        let url = format!("https://www.notion.so/{}", clean_id);
        self.navigate(&url).await?;
        sleep(Duration::from_secs(self.config.page_load_wait_secs)).await;
        Ok(())
    }

    /// Navigate to a specific URL.
    pub async fn navigate(&self, url: &str) -> Result<(), NotionError> {
        debug!("Navigating to: {}", url);

        let args = self.build_open_args(url);
        let (success, output) = self.run_browser_use(&args).await?;

        if !success {
            return Err(NotionError::NavigationError(format!(
                "Failed to navigate to {}: {}",
                url, output
            )));
        }

        Ok(())
    }

    /// Get the current browser state.
    pub async fn get_state(&self) -> Result<BrowserState, NotionError> {
        let args = vec!["state".to_string()];
        let (success, output) = self.run_browser_use(&args).await?;

        if !success {
            return Err(NotionError::BrowserError(format!(
                "Failed to get state: {}",
                output
            )));
        }

        Ok(self.parse_state(&output))
    }

    /// Get the current page's HTML content.
    pub async fn get_page_html(&self) -> Result<String, NotionError> {
        let args = vec!["get".to_string(), "html".to_string()];
        let (success, output) = self.run_browser_use(&args).await?;

        if !success {
            return Err(NotionError::BrowserError(format!(
                "Failed to get HTML: {}",
                output
            )));
        }

        Ok(output)
    }

    /// Click an element by index.
    pub async fn click(&self, index: u32) -> Result<(), NotionError> {
        debug!("Clicking element {}", index);

        let args = vec!["click".to_string(), index.to_string()];
        let (success, output) = self.run_browser_use(&args).await?;

        if !success {
            return Err(NotionError::BrowserError(format!(
                "Failed to click element {}: {}",
                index, output
            )));
        }

        Ok(())
    }

    /// Input text into an element by index.
    pub async fn input(&self, index: u32, text: &str) -> Result<(), NotionError> {
        debug!("Inputting text into element {}", index);

        let args = vec!["input".to_string(), index.to_string(), text.to_string()];
        let (success, output) = self.run_browser_use(&args).await?;

        if !success {
            return Err(NotionError::BrowserError(format!(
                "Failed to input into element {}: {}",
                index, output
            )));
        }

        Ok(())
    }

    /// Type text (into currently focused element).
    pub async fn type_text(&self, text: &str) -> Result<(), NotionError> {
        debug!("Typing text");

        let args = vec!["type".to_string(), text.to_string()];
        let (success, output) = self.run_browser_use(&args).await?;

        if !success {
            return Err(NotionError::BrowserError(format!(
                "Failed to type text: {}",
                output
            )));
        }

        Ok(())
    }

    /// Send keyboard keys.
    pub async fn send_keys(&self, keys: &str) -> Result<(), NotionError> {
        debug!("Sending keys: {}", keys);

        let args = vec!["keys".to_string(), keys.to_string()];
        let (success, output) = self.run_browser_use(&args).await?;

        if !success {
            return Err(NotionError::BrowserError(format!(
                "Failed to send keys {}: {}",
                keys, output
            )));
        }

        Ok(())
    }

    /// Take a screenshot and save to file.
    pub async fn screenshot(&self, path: &str) -> Result<(), NotionError> {
        debug!("Taking screenshot: {}", path);

        let args = vec!["screenshot".to_string(), path.to_string()];
        let (success, output) = self.run_browser_use(&args).await?;

        if !success {
            return Err(NotionError::BrowserError(format!(
                "Failed to take screenshot: {}",
                output
            )));
        }

        Ok(())
    }

    /// Reply to a comment (assumes we're on the correct page).
    pub async fn reply_to_comment(
        &self,
        _comment_id: Option<&str>,
        reply_text: &str,
    ) -> Result<NotionReplyResult, NotionError> {
        info!("Replying to comment");

        // Get state to find reply input
        let state = self.get_state().await?;

        // Look for reply input or comment box
        let reply_idx = self.find_input_index(&state.raw, &["reply", "Reply", "comment", "Comment"])
            .or_else(|| self.find_contenteditable_index(&state.raw));

        if let Some(idx) = reply_idx {
            // Click to focus the input
            self.click(idx).await?;
            sleep(Duration::from_millis(500)).await;

            // Type the reply
            self.type_text(reply_text).await?;
            sleep(Duration::from_millis(500)).await;

            // Try to find and click submit button
            let state = self.get_state().await?;
            if let Some(submit_idx) = self.find_element_index(&state.raw, &["Send", "Submit", "Post"]) {
                self.click(submit_idx).await?;
                sleep(Duration::from_secs(2)).await;

                return Ok(NotionReplyResult {
                    success: true,
                    comment_id: None,
                    error: None,
                });
            }

            // Try pressing Enter to submit
            self.send_keys("Enter").await?;
            sleep(Duration::from_secs(2)).await;

            return Ok(NotionReplyResult {
                success: true,
                comment_id: None,
                error: None,
            });
        }

        Ok(NotionReplyResult {
            success: false,
            comment_id: None,
            error: Some("Could not find reply input".to_string()),
        })
    }

    /// Close the browser session.
    pub async fn close(&self) -> Result<(), NotionError> {
        info!("Closing browser session");

        let args = vec!["close".to_string()];
        let (_, _) = self.run_browser_use(&args).await?;
        Ok(())
    }

    /// Switch to a different tab by index.
    pub async fn switch_tab(&self, tab_idx: u32) -> Result<(), NotionError> {
        debug!("Switching to tab {}", tab_idx);

        let args = vec!["switch".to_string(), tab_idx.to_string()];
        let (success, output) = self.run_browser_use(&args).await?;

        if !success {
            return Err(NotionError::BrowserError(format!(
                "Failed to switch to tab {}: {}",
                tab_idx, output
            )));
        }

        Ok(())
    }

    /// Import cookies from a JSON file.
    pub async fn import_cookies(&self, path: &str) -> Result<(), NotionError> {
        debug!("Importing cookies from {}", path);

        let args = vec![
            "cookies".to_string(),
            "import".to_string(),
            path.to_string(),
        ];
        let (success, output) = self.run_browser_use(&args).await?;

        if !success {
            return Err(NotionError::BrowserError(format!(
                "Failed to import cookies: {}",
                output
            )));
        }

        Ok(())
    }

    /// Export cookies to a JSON file.
    pub async fn export_cookies(&self, path: &str) -> Result<(), NotionError> {
        debug!("Exporting cookies to {}", path);

        // Ensure parent directory exists
        if let Some(parent) = std::path::Path::new(path).parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                NotionError::BrowserError(format!("Failed to create cookie directory: {}", e))
            })?;
        }

        let args = vec![
            "cookies".to_string(),
            "export".to_string(),
            path.to_string(),
        ];
        let (success, output) = self.run_browser_use(&args).await?;

        if !success {
            return Err(NotionError::BrowserError(format!(
                "Failed to export cookies: {}",
                output
            )));
        }

        Ok(())
    }

    // =========================================================================
    // Private helper methods
    // =========================================================================

    /// Build arguments for the `open` command.
    fn build_open_args(&self, url: &str) -> Vec<String> {
        let mut args = vec![
            "--session".to_string(),
            self.config.session_name.clone(),
            "--browser".to_string(),
            self.config.browser_mode.clone(),
        ];

        if !self.config.headless {
            args.push("--headed".to_string());
        }

        if let Some(ref profile) = self.config.profile {
            args.push("--profile".to_string());
            args.push(profile.clone());
        }

        args.push("open".to_string());
        args.push(url.to_string());

        args
    }

    /// Run a browser-use CLI command.
    async fn run_browser_use(&self, args: &[String]) -> Result<(bool, String), NotionError> {
        // Check if we need to add session arg
        let full_args: Vec<String> = if args.first().map(|s| s.as_str()) != Some("--session") &&
                                        args.first().map(|s| s.as_str()) != Some("open") &&
                                        !args.iter().any(|s| s == "--session") {
            // Add session arg for non-open commands
            let mut full = vec![
                "--session".to_string(),
                self.config.session_name.clone(),
            ];
            full.extend(args.iter().cloned());
            full
        } else {
            args.to_vec()
        };

        debug!("Running: browser-use {}", full_args.join(" "));

        // Run command with IN_DOCKER=true for WSL compatibility
        let output = tokio::task::spawn_blocking(move || {
            Command::new(shellexpand::tilde("~/.local/bin/browser-use").to_string())
                .args(&full_args)
                .env("IN_DOCKER", "true")
                .output()
        })
        .await
        .map_err(|e| NotionError::BrowserError(format!("Task join error: {}", e)))?
        .map_err(|e| NotionError::BrowserError(format!("Command failed: {}", e)))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        let combined = format!("{}{}", stdout, stderr);

        Ok((output.status.success(), combined))
    }

    /// Parse browser state from raw output.
    fn parse_state(&self, raw: &str) -> BrowserState {
        let mut state = BrowserState {
            raw: raw.to_string(),
            ..Default::default()
        };

        // Parse URL
        if let Some(caps) = Regex::new(r"url:\s*(\S+)").ok().and_then(|r| r.captures(raw)) {
            state.url = caps.get(1).map(|m| m.as_str().to_string()).unwrap_or_default();
        }

        // Parse viewport
        if let Some(caps) = Regex::new(r"viewport:\s*(\d+)x(\d+)").ok().and_then(|r| r.captures(raw)) {
            let w = caps.get(1).and_then(|m| m.as_str().parse().ok()).unwrap_or(0);
            let h = caps.get(2).and_then(|m| m.as_str().parse().ok()).unwrap_or(0);
            state.viewport = (w, h);
        }

        // Parse elements with indices: [123]<tag>text (text on same line)
        if let Ok(re) = Regex::new(r"\[(\d+)\]<[^>]+>([^<\n\[]+)") {
            for caps in re.captures_iter(raw) {
                if let (Some(idx), Some(text)) = (caps.get(1), caps.get(2)) {
                    if let Ok(i) = idx.as_str().parse::<u32>() {
                        let t = text.as_str().trim();
                        if !t.is_empty() {
                            state.elements.insert(i, t.to_string());
                        }
                    }
                }
            }
        }

        // Parse elements with text on next line: [123]<tag />\n    text
        if let Ok(re) = Regex::new(r"\[(\d+)\]<[^>]+/>\s*\n\s*([^\n\[]+)") {
            for caps in re.captures_iter(raw) {
                if let (Some(idx), Some(text)) = (caps.get(1), caps.get(2)) {
                    if let Ok(i) = idx.as_str().parse::<u32>() {
                        let t = text.as_str().trim();
                        if !t.is_empty() && !state.elements.contains_key(&i) {
                            state.elements.insert(i, t.to_string());
                        }
                    }
                }
            }
        }

        // Parse non-self-closing elements with text on next line: [123]<tag>\n    text
        if let Ok(re) = Regex::new(r"\[(\d+)\]<[^/>]+>\s*\n\s*([^\n\[]+)") {
            for caps in re.captures_iter(raw) {
                if let (Some(idx), Some(text)) = (caps.get(1), caps.get(2)) {
                    if let Ok(i) = idx.as_str().parse::<u32>() {
                        let t = text.as_str().trim();
                        if !t.is_empty() && !state.elements.contains_key(&i) {
                            state.elements.insert(i, t.to_string());
                        }
                    }
                }
            }
        }

        state
    }

    /// Find an element index by matching text patterns.
    fn find_element_index(&self, raw: &str, patterns: &[&str]) -> Option<u32> {
        // Match [index]<tag>...text... (text on same line)
        let re = Regex::new(r"\[(\d+)\]<[^>]+>([^\n\[]+)").ok()?;

        for caps in re.captures_iter(raw) {
            let idx = caps.get(1)?.as_str();
            let text = caps.get(2)?.as_str();

            for pattern in patterns {
                if text.contains(pattern) {
                    return idx.parse().ok();
                }
            }
        }

        // Match [index]<tag /> followed by text on next line (common in browser-use output)
        // Pattern: [27]<div role=button />\n\t\tContinue
        let re2 = Regex::new(r"\[(\d+)\]<[^>]+/>\s*\n\s*([^\n\[]+)").ok()?;
        for caps in re2.captures_iter(raw) {
            let idx = caps.get(1)?.as_str();
            let text = caps.get(2)?.as_str().trim();

            for pattern in patterns {
                if text.contains(pattern) {
                    return idx.parse().ok();
                }
            }
        }

        // Match [index]<tag> (non-self-closing) followed by text on next line
        let re3 = Regex::new(r"\[(\d+)\]<[^/>]+>\s*\n\s*([^\n\[]+)").ok()?;
        for caps in re3.captures_iter(raw) {
            let idx = caps.get(1)?.as_str();
            let text = caps.get(2)?.as_str().trim();

            for pattern in patterns {
                if text.contains(pattern) {
                    return idx.parse().ok();
                }
            }
        }

        None
    }

    /// Find an input element index by type or name patterns.
    fn find_input_index(&self, raw: &str, patterns: &[&str]) -> Option<u32> {
        // Match input tags with various attribute formats (quoted and unquoted)
        // e.g., [index]<input type=email...> or |SHADOW(open)|[index]<input type=email...>
        let re_full = Regex::new(r#"(?:\|SHADOW\([^)]+\)\|)?\[(\d+)\]<input[^>]+>"#).ok()?;

        for caps in re_full.captures_iter(raw) {
            let idx = caps.get(1)?.as_str();
            let full_tag = caps.get(0)?.as_str().to_lowercase();

            for pattern in patterns {
                // Check if the pattern appears in the tag (type=email, name=identifier, id=email, etc.)
                if full_tag.contains(&pattern.to_lowercase()) {
                    return idx.parse().ok();
                }
            }
        }

        None
    }

    /// Find a contenteditable element index.
    fn find_contenteditable_index(&self, raw: &str) -> Option<u32> {
        let re = Regex::new(r"\[(\d+)\]<[^>]*contenteditable[^>]*>").ok()?;

        if let Some(caps) = re.captures(raw) {
            return caps.get(1)?.as_str().parse().ok();
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_state() {
        let browser = NotionBrowser::new(NotionBrowserConfig::default());

        let raw = r#"
viewport: 1920x1080
url: https://www.notion.so/notifications
[12]<a />
    Gmail
[14]<a />
    Sign in
[2]<form />
    [15]<input type=email />
[100]<div role=button />
    Google
"#;

        let state = browser.parse_state(raw);

        assert_eq!(state.url, "https://www.notion.so/notifications");
        assert_eq!(state.viewport, (1920, 1080));
        assert!(state.elements.contains_key(&12));
        assert!(state.elements.contains_key(&14));
    }

    #[test]
    fn test_find_element_index() {
        let browser = NotionBrowser::new(NotionBrowserConfig::default());

        let raw = r#"
[100]<div role=button />
    Google
[101]<div role=button />
    Apple
[102]<a />
    Sign in
"#;

        assert_eq!(browser.find_element_index(raw, &["Google"]), Some(100));
        assert_eq!(browser.find_element_index(raw, &["Apple"]), Some(101));
        assert_eq!(browser.find_element_index(raw, &["Sign in"]), Some(102));
        assert_eq!(browser.find_element_index(raw, &["NotFound"]), None);
    }

    #[test]
    fn test_find_input_index() {
        let browser = NotionBrowser::new(NotionBrowserConfig::default());

        let raw = r#"
[15]<input type=email placeholder="Enter email" />
[16]<input type=password name=Passwd />
[17]<input type=text id=username />
"#;

        assert_eq!(browser.find_input_index(raw, &["email"]), Some(15));
        assert_eq!(browser.find_input_index(raw, &["password", "Passwd"]), Some(16));
        assert_eq!(browser.find_input_index(raw, &["username"]), Some(17));
    }
}
