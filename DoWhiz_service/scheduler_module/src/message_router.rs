//! Local LLM message router for classifying and handling simple queries.
//!
//! This module provides a message router that uses a local LLM (via Ollama) to classify
//! incoming messages. Simple queries (greetings, basic questions) are handled directly
//! by the local model, while complex queries are forwarded to the full Codex/Claude pipeline.
//!
//! Configuration:
//! - `OLLAMA_URL`: Ollama server URL (default: `http://localhost:11434`)
//! - `OLLAMA_MODEL`: Model to use (default: `phi3:mini`)
//! - `OLLAMA_ENABLED`: Set to "false" to disable routing (default: enabled)

use std::env;
use std::time::Duration;

use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};

/// Default Ollama server URL
const DEFAULT_OLLAMA_URL: &str = "http://localhost:11434";

/// Default model for classification/response
const DEFAULT_OLLAMA_MODEL: &str = "phi3:mini";

/// Timeout for Ollama requests
const OLLAMA_TIMEOUT: Duration = Duration::from_secs(30);

/// Magic string that indicates the query should be forwarded to the full pipeline
const FORWARD_MARKER: &str = "FORWARD_TO_AGENT";

/// Maximum message length (in chars) to consider for local routing.
/// Messages longer than this are automatically forwarded to the full pipeline.
const MAX_SIMPLE_MESSAGE_LENGTH: usize = 300;

/// System prompt for the classifier/responder
const SYSTEM_PROMPT: &str = r#"You are a friendly AI assistant. Your job is to:
1. RESPOND DIRECTLY to greetings and casual conversation
2. Output ONLY "FORWARD_TO_AGENT" for technical/complex requests

ALWAYS respond directly to:
- Greetings: "hi", "hello", "hey", "how are you", "what's up"
- Casual chat: "how's it going", "what are you up to", "nice to meet you"
- Simple questions about yourself: "what's your name", "what can you do"
- Thank you messages

ONLY output "FORWARD_TO_AGENT" for:
- Code or programming requests
- File/document operations
- Research tasks requiring search
- Multi-step technical tasks

Keep responses brief and friendly. Output ONLY your response, nothing else."#;

/// Result of routing a message
#[derive(Debug, Clone)]
pub enum RouterDecision {
    /// Message was handled by local LLM, contains the response
    Simple(String),
    /// Message should be forwarded to full pipeline
    Complex,
    /// Router is disabled or encountered an error, forward to pipeline
    Passthrough,
}

/// Configuration for the message router
#[derive(Debug, Clone)]
pub struct RouterConfig {
    /// Ollama server URL
    pub ollama_url: String,
    /// Model to use
    pub model: String,
    /// Whether routing is enabled
    pub enabled: bool,
}

impl Default for RouterConfig {
    fn default() -> Self {
        Self {
            ollama_url: env::var("OLLAMA_URL").unwrap_or_else(|_| DEFAULT_OLLAMA_URL.to_string()),
            model: env::var("OLLAMA_MODEL").unwrap_or_else(|_| DEFAULT_OLLAMA_MODEL.to_string()),
            enabled: env::var("OLLAMA_ENABLED")
                .map(|v| v.to_lowercase() != "false")
                .unwrap_or(true),
        }
    }
}

/// Message router that uses local LLM for classification
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
            .timeout(OLLAMA_TIMEOUT)
            .build()
            .unwrap_or_else(|_| Client::new());

        info!(
            "MessageRouter initialized: url={}, model={}, enabled={}",
            config.ollama_url, config.model, config.enabled
        );

        Self { config, client }
    }

    /// Check if the router is enabled
    pub fn is_enabled(&self) -> bool {
        self.config.enabled
    }

    /// Classify and potentially respond to a message (async version)
    ///
    /// Returns:
    /// - `Simple(response)` if the local LLM handled the query
    /// - `Complex` if the query should go to the full pipeline
    /// - `Passthrough` if routing is disabled or failed
    pub async fn classify(&self, message: &str) -> RouterDecision {
        if !self.config.enabled {
            debug!("Router disabled, passing through");
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

        match self.call_ollama(message).await {
            Ok(response) => {
                let trimmed = response.trim();
                if trimmed.contains(FORWARD_MARKER) {
                    info!("Router decision: Complex (forward to pipeline)");
                    RouterDecision::Complex
                } else {
                    info!("Router decision: Simple (local response)");
                    RouterDecision::Simple(trimmed.to_string())
                }
            }
            Err(e) => {
                warn!("Router error, passing through: {}", e);
                RouterDecision::Passthrough
            }
        }
    }

    /// Make a request to the Ollama API (async)
    async fn call_ollama(&self, message: &str) -> Result<String, String> {
        let url = format!("{}/api/generate", self.config.ollama_url);

        let request = OllamaGenerateRequest {
            model: self.config.model.clone(),
            prompt: message.to_string(),
            system: Some(SYSTEM_PROMPT.to_string()),
            stream: false,
            temperature: 0.3, // Low temp for consistent classification, some variety in responses
        };

        debug!("Calling Ollama: {} with model {}", url, self.config.model);

        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("HTTP request failed: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(format!("Ollama returned {}: {}", status, body));
        }

        let ollama_response: OllamaGenerateResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        debug!(
            "Ollama response received in {:?}",
            Duration::from_nanos(ollama_response.total_duration.unwrap_or(0))
        );

        Ok(ollama_response.response)
    }
}

impl Default for MessageRouter {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Ollama API types
// ============================================================================

/// Request body for Ollama generate endpoint
#[derive(Debug, Clone, Serialize)]
struct OllamaGenerateRequest {
    model: String,
    prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,
    stream: bool,
    /// Temperature for sampling (0.0 = deterministic, 1.0 = max randomness)
    temperature: f32,
}

/// Response from Ollama generate endpoint
#[derive(Debug, Clone, Deserialize)]
struct OllamaGenerateResponse {
    response: String,
    #[serde(default)]
    total_duration: Option<u64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn router_config_defaults() {
        // Clear env vars for test
        env::remove_var("OLLAMA_URL");
        env::remove_var("OLLAMA_MODEL");
        env::remove_var("OLLAMA_ENABLED");

        let config = RouterConfig::default();
        assert_eq!(config.ollama_url, DEFAULT_OLLAMA_URL);
        assert_eq!(config.model, DEFAULT_OLLAMA_MODEL);
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
}
