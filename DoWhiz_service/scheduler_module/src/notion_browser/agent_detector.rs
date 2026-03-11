//! Agent-driven inbox detection using LLM analysis.
//!
//! This module provides LLM-based analysis of Notion inbox screenshots
//! to detect @mentions without relying on fragile hardcoded regex patterns.
//!
//! The agent analyzes screenshots and browser state to:
//! - Detect notification items mentioning the employee
//! - Handle unexpected UI states (popups, modals)
//! - Extract structured mention data for processing

use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use serde::{Deserialize, Serialize};
use std::env;
use std::path::Path;
use tracing::{debug, info, warn};

use super::NotionError;

/// Default model for inbox analysis (Haiku for cost efficiency)
const DEFAULT_DETECTION_MODEL: &str = "claude-haiku-4-5-20251001";

/// Configuration for the agent detector.
#[derive(Debug, Clone)]
pub struct AgentDetectorConfig {
    /// Anthropic API key
    pub api_key: String,
    /// Model to use for detection (default: claude-haiku)
    pub model: String,
    /// Employee name to look for in mentions
    pub employee_name: String,
    /// Maximum tokens for response
    pub max_tokens: u32,
}

impl AgentDetectorConfig {
    /// Create configuration from environment variables.
    pub fn from_env(employee_name: &str) -> Result<Self, NotionError> {
        // Try multiple API key sources
        let api_key = env::var("ANTHROPIC_API_KEY")
            .or_else(|_| env::var("ANTHROPIC_FOUNDRY_API_KEY"))
            .or_else(|_| env::var("AZURE_OPENAI_API_KEY_BACKUP"))
            .map_err(|_| {
                NotionError::ConfigError(
                    "No Anthropic API key found. Set ANTHROPIC_API_KEY, ANTHROPIC_FOUNDRY_API_KEY, or AZURE_OPENAI_API_KEY_BACKUP".to_string()
                )
            })?;

        let model = env::var("NOTION_DETECTION_MODEL")
            .unwrap_or_else(|_| DEFAULT_DETECTION_MODEL.to_string());

        Ok(Self {
            api_key,
            model,
            employee_name: employee_name.to_string(),
            max_tokens: 2000,
        })
    }
}

/// A mention detected by the agent from inbox analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedMention {
    /// Workspace name (if identifiable)
    pub workspace_name: Option<String>,
    /// Page title where mention occurred
    pub page_title: String,
    /// URL to the page (if available)
    pub page_url: Option<String>,
    /// Name of person who mentioned the employee
    pub mentioner: String,
    /// Preview/snippet of the mention content
    pub snippet: String,
    /// Timestamp string (e.g., "2d", "Yesterday", "Mar 5")
    pub timestamp: Option<String>,
    /// Element index from browser state for clicking
    pub element_index: Option<u32>,
    /// Confidence score (0.0-1.0)
    pub confidence: f32,
}

/// Result of inbox analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboxAnalysisResult {
    /// Detected mentions
    pub mentions: Vec<DetectedMention>,
    /// Whether more mentions might be visible by scrolling
    pub scroll_needed: bool,
    /// Whether the inbox appears empty
    pub inbox_empty: bool,
    /// Error message if analysis failed
    pub error: Option<String>,
}

/// Action to take for handling unexpected UI.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UiAction {
    /// No action needed, UI is ready
    None,
    /// Click an element by index
    Click(u32),
    /// Press Escape key
    PressEscape,
    /// Refresh the page
    Refresh,
    /// Wait and retry
    Wait(u32),
}

/// Result of UI state analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiStateResult {
    /// Whether UI is blocked by overlay/popup
    pub blocked: bool,
    /// Type of blocker (if any)
    pub blocker_type: Option<String>,
    /// Action to dismiss the blocker
    pub dismiss_action: UiAction,
    /// Whether we're on the inbox page
    pub on_inbox_page: bool,
}

/// Agent-driven detector for Notion inbox mentions.
pub struct AgentDetector {
    config: AgentDetectorConfig,
    http_client: reqwest::Client,
}

impl AgentDetector {
    /// Create a new agent detector with the given configuration.
    pub fn new(config: AgentDetectorConfig) -> Self {
        Self {
            config,
            http_client: reqwest::Client::new(),
        }
    }

    /// Create a new agent detector from environment variables.
    pub fn from_env(employee_name: &str) -> Result<Self, NotionError> {
        let config = AgentDetectorConfig::from_env(employee_name)?;
        Ok(Self::new(config))
    }

    /// Analyze an inbox screenshot to detect mentions.
    pub async fn analyze_inbox(
        &self,
        screenshot_path: &Path,
        browser_state: &str,
    ) -> Result<InboxAnalysisResult, NotionError> {
        info!("Analyzing inbox screenshot with LLM...");

        // Read and encode screenshot
        let image_data = std::fs::read(screenshot_path).map_err(|e| {
            NotionError::IoError(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Failed to read screenshot: {}", e),
            ))
        })?;
        let image_base64 = BASE64.encode(&image_data);

        // Determine media type
        let media_type = if screenshot_path
            .extension()
            .map_or(false, |ext| ext == "png")
        {
            "image/png"
        } else {
            "image/jpeg"
        };

        // Build the analysis prompt
        let prompt = self.build_inbox_analysis_prompt(browser_state);

        // Call the API
        let response = self.call_vision_api(&image_base64, media_type, &prompt).await?;

        // Parse the response
        self.parse_inbox_analysis_response(&response)
    }

    /// Check for unexpected UI state (popups, modals) and suggest action.
    pub async fn check_ui_state(
        &self,
        screenshot_path: &Path,
        browser_state: &str,
    ) -> Result<UiStateResult, NotionError> {
        info!("Checking UI state for blockers...");

        let image_data = std::fs::read(screenshot_path).map_err(|e| {
            NotionError::IoError(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Failed to read screenshot: {}", e),
            ))
        })?;
        let image_base64 = BASE64.encode(&image_data);

        let media_type = if screenshot_path
            .extension()
            .map_or(false, |ext| ext == "png")
        {
            "image/png"
        } else {
            "image/jpeg"
        };

        let prompt = self.build_ui_state_prompt(browser_state);
        let response = self.call_vision_api(&image_base64, media_type, &prompt).await?;

        self.parse_ui_state_response(&response)
    }

    /// Build the prompt for inbox analysis.
    fn build_inbox_analysis_prompt(&self, browser_state: &str) -> String {
        // Truncate browser state if too long
        let state_preview = if browser_state.len() > 4000 {
            format!("{}...(truncated)", &browser_state[..4000])
        } else {
            browser_state.to_string()
        };

        format!(
            r#"You are analyzing a Notion inbox screenshot to detect @mentions for the employee "{employee_name}".

The browser automation state shows interactive elements with indices:
```
{state}
```

Your task:
1. Look at the screenshot to identify notification items in the inbox
2. Find notifications where someone mentioned "{employee_name}" or similar variations (Oliver, oliver, etc.)
3. For each mention, extract the key information

For each mention you find, provide:
- workspace_name: The workspace name if visible (optional)
- page_title: The page/document where the mention occurred
- mentioner: Who mentioned the employee
- snippet: The text content of the mention (what they said)
- timestamp: When it occurred (e.g., "2d", "Yesterday", "Mar 5")
- element_index: The clickable element index from browser state that would navigate to this notification
- confidence: How confident you are this is a valid mention (0.0-1.0)

Respond ONLY with valid JSON in this exact format:
```json
{{
  "mentions": [
    {{
      "workspace_name": "Company Name",
      "page_title": "Page Title",
      "mentioner": "John Doe",
      "snippet": "Hey @{employee_name}, can you help with...",
      "timestamp": "2d",
      "element_index": 42,
      "confidence": 0.95
    }}
  ],
  "scroll_needed": false,
  "inbox_empty": false,
  "error": null
}}
```

Important:
- Only include mentions that specifically tag or reference "{employee_name}"
- If the inbox is empty or shows "No notifications", set inbox_empty: true
- If you see indicators that more items exist below the fold, set scroll_needed: true
- If you cannot analyze the image, set error to describe the problem
- Return empty mentions array if no mentions are found"#,
            employee_name = self.config.employee_name,
            state = state_preview,
        )
    }

    /// Build the prompt for UI state checking.
    fn build_ui_state_prompt(&self, browser_state: &str) -> String {
        let state_preview = if browser_state.len() > 2000 {
            format!("{}...(truncated)", &browser_state[..2000])
        } else {
            browser_state.to_string()
        };

        format!(
            r#"Analyze this Notion screenshot to check for UI blockers.

Browser state with element indices:
```
{state}
```

Check if there are any popups, modals, overlays, or onboarding dialogs blocking the main content.

Look for:
1. Onboarding/welcome popups ("What's new", "Get started", "Tips")
2. Modal dialogs covering the page
3. Tooltips or guided tours
4. Cookie consent banners
5. Login prompts or session expiry warnings

If found, identify how to dismiss:
- "X" close buttons, "Skip", "Maybe later", "Got it", "Close" buttons
- The element index to click to dismiss

Also determine if we appear to be on the Notion inbox/notifications page.

Respond ONLY with valid JSON:
```json
{{
  "blocked": true,
  "blocker_type": "onboarding_popup",
  "dismiss_action": {{"type": "click", "element_index": 42}},
  "on_inbox_page": true
}}
```

Or if no blockers:
```json
{{
  "blocked": false,
  "blocker_type": null,
  "dismiss_action": {{"type": "none"}},
  "on_inbox_page": true
}}
```

dismiss_action types: "none", "click", "press_escape", "refresh", "wait""#,
            state = state_preview,
        )
    }

    /// Call the Vision API (supports Azure OpenAI and Anthropic).
    async fn call_vision_api(
        &self,
        image_base64: &str,
        media_type: &str,
        prompt: &str,
    ) -> Result<String, NotionError> {
        // Check if using Azure OpenAI or Anthropic
        let (api_url, headers, use_openai_format) = self.build_api_request_config()?;

        let request_body = if use_openai_format {
            // Azure OpenAI format (GPT-4 Vision)
            serde_json::json!({
                "messages": [
                    {
                        "role": "user",
                        "content": [
                            {
                                "type": "image_url",
                                "image_url": {
                                    "url": format!("data:{};base64,{}", media_type, image_base64)
                                }
                            },
                            {
                                "type": "text",
                                "text": prompt
                            }
                        ]
                    }
                ],
                "max_tokens": self.config.max_tokens
            })
        } else {
            // Anthropic format
            serde_json::json!({
                "model": self.config.model,
                "max_tokens": self.config.max_tokens,
                "messages": [
                    {
                        "role": "user",
                        "content": [
                            {
                                "type": "image",
                                "source": {
                                    "type": "base64",
                                    "media_type": media_type,
                                    "data": image_base64
                                }
                            },
                            {
                                "type": "text",
                                "text": prompt
                            }
                        ]
                    }
                ]
            })
        };

        debug!("Calling Vision API at {}", api_url);

        let response = self
            .http_client
            .post(&api_url)
            .headers(headers)
            .json(&request_body)
            .send()
            .await
            .map_err(|e| NotionError::BrowserError(format!("API request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(NotionError::BrowserError(format!(
                "API error {}: {}",
                status, error_text
            )));
        }

        let response_json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| NotionError::ParseError(format!("Failed to parse API response: {}", e)))?;

        // Extract text from response (different paths for OpenAI vs Anthropic)
        let text = if use_openai_format {
            // OpenAI format: choices[0].message.content
            response_json
                .get("choices")
                .and_then(|c| c.as_array())
                .and_then(|arr| arr.first())
                .and_then(|choice| choice.get("message"))
                .and_then(|msg| msg.get("content"))
                .and_then(|t| t.as_str())
        } else {
            // Anthropic format: content[0].text
            response_json
                .get("content")
                .and_then(|c| c.as_array())
                .and_then(|arr| arr.first())
                .and_then(|item| item.get("text"))
                .and_then(|t| t.as_str())
        }
        .ok_or_else(|| NotionError::ParseError("No text in API response".to_string()))?;

        Ok(text.to_string())
    }

    /// Build API request configuration based on available credentials.
    /// Returns (api_url, headers, use_openai_format)
    fn build_api_request_config(
        &self,
    ) -> Result<(String, reqwest::header::HeaderMap, bool), NotionError> {
        let mut headers = reqwest::header::HeaderMap::new();

        // Priority 1: Azure OpenAI (if endpoint is set)
        if let Ok(azure_endpoint) = env::var("AZURE_OPENAI_ENDPOINT") {
            if !azure_endpoint.is_empty() {
                // Get deployment name (default to gpt-4o for vision)
                let deployment = env::var("AZURE_OPENAI_DEPLOYMENT")
                    .unwrap_or_else(|_| "gpt-4o".to_string());

                // Build Azure OpenAI endpoint
                let base = azure_endpoint.trim_end_matches('/');
                let api_url = format!(
                    "{}/openai/deployments/{}/chat/completions?api-version=2024-02-15-preview",
                    base, deployment
                );
                info!("Using Azure OpenAI endpoint: {}", api_url);

                // Get API key (try AZURE_OPENAI_API_KEY first, then AZURE_OPENAI_API_KEY_BACKUP)
                let api_key = env::var("AZURE_OPENAI_API_KEY")
                    .or_else(|_| env::var("AZURE_OPENAI_API_KEY_BACKUP"))
                    .map_err(|_| NotionError::ConfigError("No Azure OpenAI API key found".to_string()))?;

                headers.insert(
                    "api-key",
                    api_key
                        .parse()
                        .map_err(|_| NotionError::ConfigError("Invalid API key".to_string()))?,
                );
                headers.insert(
                    reqwest::header::CONTENT_TYPE,
                    "application/json".parse().unwrap(),
                );
                return Ok((api_url, headers, true)); // use_openai_format = true
            }
        }

        // Priority 2: Custom Anthropic endpoint
        if let Ok(custom_endpoint) = env::var("ANTHROPIC_API_ENDPOINT") {
            if !custom_endpoint.is_empty() {
                info!("Using custom Anthropic endpoint: {}", custom_endpoint);
                headers.insert(
                    "api-key",
                    self.config
                        .api_key
                        .parse()
                        .map_err(|_| NotionError::ConfigError("Invalid API key".to_string()))?,
                );
                headers.insert(
                    reqwest::header::CONTENT_TYPE,
                    "application/json".parse().unwrap(),
                );
                headers.insert(
                    "anthropic-version",
                    "2023-06-01".parse().unwrap(),
                );
                return Ok((custom_endpoint, headers, false));
            }
        }

        // Priority 3: Direct Anthropic API
        let api_url = "https://api.anthropic.com/v1/messages".to_string();
        info!("Using direct Anthropic API");
        headers.insert(
            "x-api-key",
            self.config
                .api_key
                .parse()
                .map_err(|_| NotionError::ConfigError("Invalid API key".to_string()))?,
        );
        headers.insert(
            "anthropic-version",
            "2023-06-01".parse().unwrap(),
        );
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            "application/json".parse().unwrap(),
        );

        Ok((api_url, headers, false))
    }

    /// Parse the inbox analysis response from LLM.
    fn parse_inbox_analysis_response(&self, response: &str) -> Result<InboxAnalysisResult, NotionError> {
        // Log the raw response for debugging
        debug!("Raw LLM response:\n{}", response);

        // Try to extract JSON from markdown code blocks
        let json_str = extract_json_from_response(response);
        debug!("Extracted JSON: {}", json_str);

        match serde_json::from_str::<InboxAnalysisResult>(&json_str) {
            Ok(result) => {
                info!(
                    "Parsed {} mentions from LLM response (scroll_needed: {}, inbox_empty: {})",
                    result.mentions.len(),
                    result.scroll_needed,
                    result.inbox_empty
                );
                Ok(result)
            }
            Err(e) => {
                warn!("Failed to parse LLM response as JSON: {}", e);
                debug!("Raw response: {}", response);

                // Return empty result rather than error
                Ok(InboxAnalysisResult {
                    mentions: vec![],
                    scroll_needed: false,
                    inbox_empty: false,
                    error: Some(format!("Failed to parse LLM response: {}", e)),
                })
            }
        }
    }

    /// Parse the UI state response from LLM.
    fn parse_ui_state_response(&self, response: &str) -> Result<UiStateResult, NotionError> {
        let json_str = extract_json_from_response(response);

        // Parse the raw JSON first
        let raw: serde_json::Value = serde_json::from_str(&json_str).map_err(|e| {
            warn!("Failed to parse UI state response: {}", e);
            NotionError::ParseError(format!("Invalid JSON: {}", e))
        })?;

        let blocked = raw.get("blocked").and_then(|v| v.as_bool()).unwrap_or(false);
        let blocker_type = raw
            .get("blocker_type")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let on_inbox_page = raw
            .get("on_inbox_page")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        // Parse dismiss_action
        let dismiss_action = if let Some(action) = raw.get("dismiss_action") {
            let action_type = action.get("type").and_then(|v| v.as_str()).unwrap_or("none");
            match action_type {
                "click" => {
                    let idx = action
                        .get("element_index")
                        .and_then(|v| v.as_u64())
                        .unwrap_or(0) as u32;
                    UiAction::Click(idx)
                }
                "press_escape" => UiAction::PressEscape,
                "refresh" => UiAction::Refresh,
                "wait" => {
                    let secs = action
                        .get("seconds")
                        .and_then(|v| v.as_u64())
                        .unwrap_or(2) as u32;
                    UiAction::Wait(secs)
                }
                _ => UiAction::None,
            }
        } else {
            UiAction::None
        };

        Ok(UiStateResult {
            blocked,
            blocker_type,
            dismiss_action,
            on_inbox_page,
        })
    }
}

/// Extract JSON from a response that might be wrapped in markdown code blocks.
fn extract_json_from_response(response: &str) -> String {
    // Try to find JSON in code blocks first
    if let Some(start) = response.find("```json") {
        if let Some(end) = response[start + 7..].find("```") {
            return response[start + 7..start + 7 + end].trim().to_string();
        }
    }

    // Try generic code blocks
    if let Some(start) = response.find("```") {
        let after_start = start + 3;
        // Skip language identifier if present
        let content_start = response[after_start..]
            .find('\n')
            .map(|i| after_start + i + 1)
            .unwrap_or(after_start);
        if let Some(end) = response[content_start..].find("```") {
            return response[content_start..content_start + end]
                .trim()
                .to_string();
        }
    }

    // Try to find raw JSON object
    if let Some(start) = response.find('{') {
        // Find matching closing brace
        let mut depth = 0;
        let mut end = start;
        for (i, c) in response[start..].char_indices() {
            match c {
                '{' => depth += 1,
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        end = start + i + 1;
                        break;
                    }
                }
                _ => {}
            }
        }
        if end > start {
            return response[start..end].to_string();
        }
    }

    // Return as-is
    response.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_json_from_response() {
        // Test with markdown code block
        let response = r#"Here's the analysis:
```json
{"mentions": [], "scroll_needed": false, "inbox_empty": true}
```
That's all."#;
        let json = extract_json_from_response(response);
        assert!(json.starts_with('{'));
        assert!(json.contains("inbox_empty"));

        // Test with raw JSON
        let response = r#"{"mentions": [], "scroll_needed": false}"#;
        let json = extract_json_from_response(response);
        assert_eq!(json, response);

        // Test with nested braces
        let response = r#"{"mentions": [{"name": "test"}], "nested": {"a": 1}}"#;
        let json = extract_json_from_response(response);
        assert_eq!(json, response);
    }

    #[test]
    fn test_parse_inbox_analysis_response() {
        let config = AgentDetectorConfig {
            api_key: "test".to_string(),
            model: "test".to_string(),
            employee_name: "Oliver".to_string(),
            max_tokens: 1000,
        };
        let detector = AgentDetector::new(config);

        let response = r#"```json
{
  "mentions": [
    {
      "workspace_name": "DoWhiz",
      "page_title": "Test Page",
      "mentioner": "John",
      "snippet": "@Oliver please review",
      "timestamp": "2d",
      "element_index": 42,
      "confidence": 0.9
    }
  ],
  "scroll_needed": false,
  "inbox_empty": false,
  "error": null
}
```"#;

        let result = detector.parse_inbox_analysis_response(response).unwrap();
        assert_eq!(result.mentions.len(), 1);
        assert_eq!(result.mentions[0].mentioner, "John");
        assert_eq!(result.mentions[0].element_index, Some(42));
    }

    #[test]
    fn test_parse_ui_state_response() {
        let config = AgentDetectorConfig {
            api_key: "test".to_string(),
            model: "test".to_string(),
            employee_name: "Oliver".to_string(),
            max_tokens: 1000,
        };
        let detector = AgentDetector::new(config);

        let response = r#"{
  "blocked": true,
  "blocker_type": "onboarding_popup",
  "dismiss_action": {"type": "click", "element_index": 15},
  "on_inbox_page": true
}"#;

        let result = detector.parse_ui_state_response(response).unwrap();
        assert!(result.blocked);
        assert_eq!(result.blocker_type, Some("onboarding_popup".to_string()));
        assert!(matches!(result.dismiss_action, UiAction::Click(15)));
    }
}
