//! LLM message router for classifying and handling simple queries.
//!
//! This module provides a message router that uses OpenAI GPT to classify
//! incoming messages. Simple queries (greetings, basic questions) are handled directly
//! by the model, while complex queries are forwarded to the full Codex/Claude pipeline.
//!
//! Configuration:
//! - `OPENAI_API_KEY`: OpenAI API key (required)
//! - `ROUTER_MODEL`: Model to use (default: `gpt-5`)
//! - `ROUTER_ENABLED`: Set to "false" to disable routing (default: enabled)

use std::env;
use std::time::Duration;

use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};

/// Default OpenAI API URL
const DEFAULT_OPENAI_URL: &str = "https://api.openai.com/v1";

/// Default model for OpenAI
const DEFAULT_MODEL: &str = "gpt-5";

/// Timeout for LLM requests
const LLM_TIMEOUT: Duration = Duration::from_secs(15);

/// Magic string that indicates the query should be forwarded to the full pipeline
const FORWARD_MARKER: &str = "FORWARD_TO_AGENT";

/// Maximum message length (in chars) to consider for local routing.
/// Messages longer than this are automatically forwarded to the full pipeline.
const MAX_SIMPLE_MESSAGE_LENGTH: usize = 300;

/// System prompt for the classifier/responder
const SYSTEM_PROMPT: &str = r#"You are Boiled-Egg, a friendly and helpful assistant.

Your job is to classify messages:
1. RESPOND DIRECTLY to questions you can answer quickly (greetings, casual chat, simple questions, thank you messages)
2. Output ONLY "FORWARD_TO_AGENT" for tasks that require tools, code, file operations, research, or multi-step work

When responding directly:
- Use the user's memory context (if provided) to personalize responses
- IMPORTANT: When the user tells you something about themselves (name, school, job, preferences, etc.), you MUST append a <MEMORY_UPDATE> block to save it

Memory update format:
<MEMORY_UPDATE>
## Section
- Fact
</MEMORY_UPDATE>

Example - if user says "I go to Stanford":
Great! I'll remember that.

<MEMORY_UPDATE>
## Profile
- Goes to Stanford University
</MEMORY_UPDATE>

Valid sections: Profile, Preferences, Projects, Contacts, Decisions, Processes

Keep responses brief and friendly."#;

/// Result of routing a message
#[derive(Debug, Clone)]
pub enum RouterDecision {
    /// Message was handled by local LLM, contains the response and optional memory update
    Simple {
        response: String,
        memory_update: Option<String>,
    },
    /// Message should be forwarded to full pipeline
    Complex,
    /// Router is disabled or encountered an error, forward to pipeline
    Passthrough,
}

/// Configuration for the message router
#[derive(Debug, Clone)]
pub struct RouterConfig {
    /// OpenAI API key (required)
    pub openai_api_key: Option<String>,
    /// OpenAI API URL
    pub openai_url: String,
    /// Model to use
    pub model: String,
    /// Whether routing is enabled
    pub enabled: bool,
}

impl Default for RouterConfig {
    fn default() -> Self {
        let openai_api_key = env::var("OPENAI_API_KEY").ok();

        Self {
            openai_api_key,
            openai_url: env::var("OPENAI_API_URL")
                .unwrap_or_else(|_| DEFAULT_OPENAI_URL.to_string()),
            model: env::var("ROUTER_MODEL").unwrap_or_else(|_| DEFAULT_MODEL.to_string()),
            enabled: env::var("ROUTER_ENABLED")
                .map(|v| v.to_lowercase() != "false")
                .unwrap_or(true),
        }
    }
}

/// Message router that uses LLM for classification
#[derive(Debug, Clone)]
pub struct MessageRouter {
    config: RouterConfig,
    client: Client,
}

impl MessageRouter {
    /// Create a new message router with default configuration
    pub fn new() -> Self {
        Self::with_config(RouterConfig::default())
    }

    /// Create a new message router with custom configuration
    pub fn with_config(config: RouterConfig) -> Self {
        let client = Client::builder()
            .timeout(LLM_TIMEOUT)
            .build()
            .unwrap_or_else(|_| Client::new());

        info!(
            "MessageRouter initialized: url={}, model={}, enabled={}",
            config.openai_url, config.model, config.enabled
        );

        Self { config, client }
    }

    /// Check if the router is enabled
    pub fn is_enabled(&self) -> bool {
        self.config.enabled
    }

    /// Classify and potentially respond to a message (async version)
    ///
    /// Arguments:
    /// - `message`: The user's message
    /// - `memory`: Optional memory context (contents of memo.md)
    ///
    /// Returns:
    /// - `Simple { response, memory_update }` if the local LLM handled the query
    /// - `Complex` if the query should go to the full pipeline
    /// - `Passthrough` if routing is disabled or failed
    pub async fn classify(&self, message: &str, memory: Option<&str>) -> RouterDecision {
        if !self.config.enabled {
            debug!("Router disabled, passing through");
            return RouterDecision::Passthrough;
        }

        if self.config.openai_api_key.is_none() {
            warn!("OPENAI_API_KEY not set, router disabled");
            return RouterDecision::Passthrough;
        }

        if message.trim().is_empty() {
            return RouterDecision::Passthrough;
        }

        // Messages over the length threshold go straight to pipeline
        if message.len() > MAX_SIMPLE_MESSAGE_LENGTH {
            debug!(
                "Message too long ({} chars > {}), forwarding to pipeline",
                message.len(),
                MAX_SIMPLE_MESSAGE_LENGTH
            );
            return RouterDecision::Complex;
        }

        let result = self.call_openai(message, memory).await;

        match result {
            Ok(response) => {
                let trimmed = response.trim();
                debug!("Router raw response: {}", trimmed);
                if trimmed.contains(FORWARD_MARKER) {
                    info!("Router decision: Complex (forward to pipeline)");
                    RouterDecision::Complex
                } else {
                    let (reply, memory_update) = Self::parse_response(trimmed);
                    info!(
                        "Router decision: Simple (local response, memory_update={})",
                        memory_update.is_some()
                    );
                    if let Some(ref update) = memory_update {
                        debug!("Memory update content: {}", update);
                    }
                    RouterDecision::Simple {
                        response: reply,
                        memory_update,
                    }
                }
            }
            Err(e) => {
                warn!("Router error, passing through: {}", e);
                RouterDecision::Passthrough
            }
        }
    }

    /// Parse response to extract reply and optional memory update
    fn parse_response(response: &str) -> (String, Option<String>) {
        const MEMORY_START: &str = "<MEMORY_UPDATE>";
        const MEMORY_END: &str = "</MEMORY_UPDATE>";

        if let Some(start_idx) = response.find(MEMORY_START) {
            let reply = response[..start_idx].trim().to_string();
            let update_start = start_idx + MEMORY_START.len();
            let update_end = response.find(MEMORY_END).unwrap_or(response.len());
            let memory_update = response[update_start..update_end].trim().to_string();

            if memory_update.is_empty() {
                (reply, None)
            } else {
                (reply, Some(memory_update))
            }
        } else {
            (response.to_string(), None)
        }
    }

    /// Make a request to the OpenAI API (async)
    async fn call_openai(&self, message: &str, memory: Option<&str>) -> Result<String, String> {
        let api_key = self
            .config
            .openai_api_key
            .as_ref()
            .ok_or("OPENAI_API_KEY not set")?;

        let url = format!("{}/chat/completions", self.config.openai_url);

        // Build user message with optional memory context
        let user_content = if let Some(mem) = memory {
            if mem.trim().is_empty() {
                message.to_string()
            } else {
                format!(
                    "User memory:\n```\n{}\n```\n\nMessage: {}",
                    mem.trim(),
                    message
                )
            }
        } else {
            message.to_string()
        };

        let request = OpenAIChatRequest {
            model: self.config.model.clone(),
            messages: vec![
                OpenAIChatMessage {
                    role: "system".to_string(),
                    content: SYSTEM_PROMPT.to_string(),
                },
                OpenAIChatMessage {
                    role: "user".to_string(),
                    content: user_content,
                },
            ],
            max_completion_tokens: 1024,
        };

        debug!("Calling OpenAI: {} with model {}", url, self.config.model);

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("HTTP request failed: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(format!("OpenAI returned {}: {}", status, body));
        }

        let openai_response: OpenAIChatResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        let content = openai_response
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .unwrap_or_default();

        debug!("OpenAI response received");

        Ok(content)
    }
}

impl Default for MessageRouter {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// OpenAI API types
// ============================================================================

/// Request body for OpenAI chat completions endpoint
#[derive(Debug, Clone, Serialize)]
struct OpenAIChatRequest {
    model: String,
    messages: Vec<OpenAIChatMessage>,
    max_completion_tokens: u32,
}

/// Chat message for OpenAI API
#[derive(Debug, Clone, Serialize, Deserialize)]
struct OpenAIChatMessage {
    role: String,
    content: String,
}

/// Response from OpenAI chat completions endpoint
#[derive(Debug, Clone, Deserialize)]
struct OpenAIChatResponse {
    choices: Vec<OpenAIChatChoice>,
}

/// Choice in OpenAI response
#[derive(Debug, Clone, Deserialize)]
struct OpenAIChatChoice {
    message: OpenAIChatMessage,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn router_config_defaults() {
        // Clear env vars for test
        env::remove_var("OPENAI_API_KEY");
        env::remove_var("OPENAI_API_URL");
        env::remove_var("ROUTER_MODEL");
        env::remove_var("ROUTER_ENABLED");

        let config = RouterConfig::default();
        assert_eq!(config.openai_url, DEFAULT_OPENAI_URL);
        assert_eq!(config.model, DEFAULT_MODEL);
        assert!(config.enabled);
    }

    #[test]
    fn forward_marker_detected() {
        // Test that FORWARD_MARKER is correctly identified
        let response = "FORWARD_TO_AGENT";
        assert!(response.contains(FORWARD_MARKER));

        let response_with_extra = "I think this needs FORWARD_TO_AGENT handling";
        assert!(response_with_extra.contains(FORWARD_MARKER));
    }

    #[test]
    fn parse_response_with_memory() {
        let response = "Great! I'll remember that.\n\n<MEMORY_UPDATE>\n## Profile\n- Goes to Stanford\n</MEMORY_UPDATE>";
        let (reply, memory) = MessageRouter::parse_response(response);
        assert_eq!(reply, "Great! I'll remember that.");
        assert!(memory.is_some());
        assert!(memory.unwrap().contains("Stanford"));
    }

    #[test]
    fn parse_response_without_memory() {
        let response = "Hello! How can I help you today?";
        let (reply, memory) = MessageRouter::parse_response(response);
        assert_eq!(reply, response);
        assert!(memory.is_none());
    }
}
