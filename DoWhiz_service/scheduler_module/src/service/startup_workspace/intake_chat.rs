use std::collections::HashSet;
use std::env;
use std::time::Duration;

use reqwest::Client;
use serde::{Deserialize, Serialize};

const DEFAULT_OPENAI_URL: &str = "https://api.openai.com/v1";
const DEFAULT_MODEL: &str = "gpt-5.4";
const LLM_TIMEOUT: Duration = Duration::from_secs(30);

const DEFAULT_BUILD_SYSTEM: &str = "github";
const DEFAULT_FORMAL_DOCS: &str = "google_docs";
const DEFAULT_COORDINATION: &str = "slack";
const DEFAULT_EXTERNAL_EXECUTION: &str = "email";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StartupIntakeChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct StartupResourceToolDraft {
    pub build_system: Option<String>,
    pub formal_docs: Option<String>,
    pub coordination: Option<String>,
    pub external_execution: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct StartupRequestedAgentDraft {
    pub role: String,
    pub owner: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct StartupIntakeDraft {
    pub founder_name: Option<String>,
    pub founder_email: Option<String>,
    pub venture_name: Option<String>,
    pub venture_thesis: Option<String>,
    pub venture_stage: Option<String>,
    pub plan_horizon_days: Option<u16>,
    #[serde(default)]
    pub goals_30_90_days: Vec<String>,
    #[serde(default)]
    pub current_assets: Vec<String>,
    #[serde(default)]
    pub requested_agents: Vec<StartupRequestedAgentDraft>,
    pub resource_launch_mode: Option<String>,
    #[serde(default)]
    pub resource_tools: StartupResourceToolDraft,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartupIntakeChatRequest {
    #[serde(default)]
    pub messages: Vec<StartupIntakeChatMessage>,
    #[serde(default)]
    pub current_draft: Option<StartupIntakeDraft>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartupIntakeChatResponse {
    pub assistant_message: String,
    pub intake_draft: StartupIntakeDraft,
    pub missing_fields: Vec<String>,
    pub ready_for_blueprint: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct LlmIntakeOutput {
    #[serde(default, alias = "assistant_reply", alias = "message")]
    assistant_message: String,
    #[serde(default, alias = "draft")]
    intake_draft: StartupIntakeDraft,
}

#[derive(Debug, Clone)]
struct StartupIntakeLlmConfig {
    api_key: String,
    api_url: String,
    model: String,
    use_azure_auth: bool,
}

impl StartupIntakeLlmConfig {
    fn from_env() -> Result<Self, String> {
        let azure_api_key = env::var("AZURE_OPENAI_API_KEY_BACKUP")
            .ok()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());
        let azure_endpoint = env::var("AZURE_OPENAI_ENDPOINT_BACKUP")
            .ok()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());

        if let (Some(api_key), Some(endpoint)) = (azure_api_key, azure_endpoint) {
            let api_url = normalize_azure_endpoint(&endpoint);
            let model = env::var("STARTUP_INTAKE_MODEL")
                .ok()
                .filter(|value| !value.trim().is_empty())
                .unwrap_or_else(|| DEFAULT_MODEL.to_string());

            return Ok(Self {
                api_key,
                api_url,
                model,
                use_azure_auth: true,
            });
        }

        let api_key = env::var("OPENAI_API_KEY")
            .ok()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty())
            .ok_or_else(|| {
                "Startup intake LLM is not configured (set AZURE_OPENAI_API_KEY_BACKUP + AZURE_OPENAI_ENDPOINT_BACKUP, or OPENAI_API_KEY).".to_string()
            })?;

        let api_url = env::var("OPENAI_API_URL")
            .ok()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| DEFAULT_OPENAI_URL.to_string());
        let model = env::var("STARTUP_INTAKE_MODEL")
            .ok()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| DEFAULT_MODEL.to_string());

        Ok(Self {
            api_key,
            api_url: api_url.trim_end_matches('/').to_string(),
            model,
            use_azure_auth: false,
        })
    }
}

#[derive(Debug, Clone, Serialize)]
struct ChatRequestBody {
    model: String,
    messages: Vec<ChatMessage>,
    max_completion_tokens: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Debug, Clone, Deserialize)]
struct ChatResponseBody {
    choices: Vec<ChatChoice>,
}

#[derive(Debug, Clone, Deserialize)]
struct ChatChoice {
    message: ChatMessage,
}

pub async fn generate_startup_intake_chat_response(
    request: StartupIntakeChatRequest,
) -> Result<StartupIntakeChatResponse, String> {
    let messages = normalize_messages(request.messages);
    if messages.is_empty() {
        return Err("No conversation messages were provided.".to_string());
    }

    let config = StartupIntakeLlmConfig::from_env()?;
    let system_prompt = startup_intake_system_prompt();
    let user_prompt = startup_intake_user_prompt(&messages, request.current_draft.as_ref())?;
    let raw = call_chat_completion(&config, &system_prompt, &user_prompt).await?;
    let parsed = parse_llm_output(&raw)?;

    let mut draft = normalize_intake_draft(parsed.intake_draft);
    if draft.resource_launch_mode.as_deref() == Some("default") {
        draft = apply_default_tool_selection(draft);
    }

    let missing_fields = derive_missing_fields(&draft);
    let ready_for_blueprint = missing_fields.is_empty();
    let assistant_message = normalize_assistant_message(
        parsed.assistant_message,
        &missing_fields,
        ready_for_blueprint,
    );

    Ok(StartupIntakeChatResponse {
        assistant_message,
        intake_draft: draft,
        missing_fields,
        ready_for_blueprint,
    })
}

fn normalize_messages(messages: Vec<StartupIntakeChatMessage>) -> Vec<StartupIntakeChatMessage> {
    messages
        .into_iter()
        .filter_map(|message| {
            let role = message.role.trim().to_lowercase();
            let normalized_role = if role == "assistant" {
                "assistant"
            } else if role == "system" {
                "system"
            } else {
                "user"
            };
            let content = message.content.trim();
            if content.is_empty() {
                return None;
            }
            Some(StartupIntakeChatMessage {
                role: normalized_role.to_string(),
                content: content.to_string(),
            })
        })
        .collect()
}

fn startup_intake_system_prompt() -> String {
    r#"You are DoWhiz intake assistant.

Your job is to collect enough information to create a startup workspace blueprint.
You must drive the conversation naturally and ask only the next most important question.

Required fields before ready_for_blueprint can be true:
- founder_name
- venture_thesis
- at least one goals_30_90_days item
- resource_launch_mode chosen: "default" or "custom"
- if mode is "custom", exactly one tool chosen for each category:
  - build_system: github | gitlab | bitbucket
  - formal_docs: google_docs | notion
  - coordination: slack | discord | email
  - external_execution: email | slack | discord

Output requirements:
- Return ONLY a valid JSON object (no markdown, no code fences, no extra text).
- JSON shape:
{
  "assistant_message": "string",
  "intake_draft": {
    "founder_name": "string|null",
    "founder_email": "string|null",
    "venture_name": "string|null",
    "venture_thesis": "string|null",
    "venture_stage": "idea|prototype|mvp|post_mvp|growth|null",
    "plan_horizon_days": 30|60|90|null,
    "goals_30_90_days": ["..."],
    "current_assets": ["..."],
    "requested_agents": [{"role":"string","owner":"string|null"}],
    "resource_launch_mode": "default|custom|null",
    "resource_tools": {
      "build_system": "github|gitlab|bitbucket|null",
      "formal_docs": "google_docs|notion|null",
      "coordination": "slack|discord|email|null",
      "external_execution": "email|slack|discord|null"
    }
  }
}

Behavior rules:
- Never hallucinate user details. Only fill values grounded in conversation.
- Keep assistant_message concise and actionable.
- If missing key data, ask one focused follow-up question.
- If enough data is present, ask user to click "Create blueprint now".
"#
    .to_string()
}

fn startup_intake_user_prompt(
    messages: &[StartupIntakeChatMessage],
    current_draft: Option<&StartupIntakeDraft>,
) -> Result<String, String> {
    let transcript = messages
        .iter()
        .map(|message| {
            let role = match message.role.as_str() {
                "assistant" => "Assistant",
                "system" => "System",
                _ => "User",
            };
            format!("{}: {}", role, message.content)
        })
        .collect::<Vec<_>>()
        .join("\n");

    let draft_json = serde_json::to_string_pretty(&current_draft.cloned().unwrap_or_default())
        .map_err(|err| format!("failed to serialize current draft: {}", err))?;

    Ok(format!(
        "Current intake draft (may be partial):\n{}\n\nConversation transcript:\n{}",
        draft_json, transcript
    ))
}

async fn call_chat_completion(
    config: &StartupIntakeLlmConfig,
    system_prompt: &str,
    user_prompt: &str,
) -> Result<String, String> {
    let client = Client::builder()
        .timeout(LLM_TIMEOUT)
        .build()
        .map_err(|err| format!("failed to build HTTP client: {}", err))?;

    let url = format!("{}/chat/completions", config.api_url.trim_end_matches('/'));
    let payload = ChatRequestBody {
        model: config.model.clone(),
        messages: vec![
            ChatMessage {
                role: "system".to_string(),
                content: system_prompt.to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: user_prompt.to_string(),
            },
        ],
        max_completion_tokens: 1600,
    };

    let mut request_builder = client.post(url).header("Content-Type", "application/json");

    if config.use_azure_auth {
        request_builder = request_builder.header("api-key", &config.api_key);
    } else {
        request_builder =
            request_builder.header("Authorization", format!("Bearer {}", config.api_key));
    }

    let response = request_builder
        .json(&payload)
        .send()
        .await
        .map_err(|err| format!("startup intake LLM request failed: {}", err))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("startup intake LLM returned {}: {}", status, body));
    }

    let parsed: ChatResponseBody = response
        .json()
        .await
        .map_err(|err| format!("failed to parse startup intake LLM response: {}", err))?;

    let content = parsed
        .choices
        .first()
        .map(|choice| choice.message.content.clone())
        .unwrap_or_default();

    if content.trim().is_empty() {
        return Err("startup intake LLM returned an empty response".to_string());
    }

    Ok(content)
}

fn parse_llm_output(raw: &str) -> Result<LlmIntakeOutput, String> {
    if let Ok(parsed) = serde_json::from_str::<LlmIntakeOutput>(raw) {
        return Ok(parsed);
    }

    if let Some(extracted) = extract_json_object(raw) {
        if let Ok(parsed) = serde_json::from_str::<LlmIntakeOutput>(&extracted) {
            return Ok(parsed);
        }
    }

    Err(format!(
        "startup intake LLM response was not valid JSON. raw={}",
        raw
    ))
}

fn extract_json_object(input: &str) -> Option<String> {
    let mut start_index = None;
    let mut brace_depth: i32 = 0;
    let mut in_string = false;
    let mut escaped = false;

    for (index, ch) in input.char_indices() {
        if in_string {
            if escaped {
                escaped = false;
                continue;
            }
            if ch == '\\' {
                escaped = true;
                continue;
            }
            if ch == '"' {
                in_string = false;
            }
            continue;
        }

        if ch == '"' {
            in_string = true;
            continue;
        }

        if ch == '{' {
            if start_index.is_none() {
                start_index = Some(index);
            }
            brace_depth += 1;
            continue;
        }

        if ch == '}' && brace_depth > 0 {
            brace_depth -= 1;
            if brace_depth == 0 {
                if let Some(start) = start_index {
                    return Some(input[start..=index].to_string());
                }
            }
        }
    }

    None
}

fn normalize_intake_draft(mut draft: StartupIntakeDraft) -> StartupIntakeDraft {
    draft.founder_name = normalize_optional_string(draft.founder_name);
    draft.founder_email = normalize_optional_string(draft.founder_email);
    draft.venture_name = normalize_optional_string(draft.venture_name);
    draft.venture_thesis = normalize_optional_string(draft.venture_thesis);
    draft.venture_stage = normalize_stage(draft.venture_stage);
    draft.plan_horizon_days = draft.plan_horizon_days.map(normalize_horizon);
    draft.goals_30_90_days = normalize_string_list(draft.goals_30_90_days);
    draft.current_assets = normalize_string_list(draft.current_assets);
    draft.requested_agents = normalize_agents(draft.requested_agents);
    draft.resource_launch_mode = normalize_launch_mode(draft.resource_launch_mode);
    draft.resource_tools = normalize_resource_tools(draft.resource_tools);
    draft
}

fn apply_default_tool_selection(mut draft: StartupIntakeDraft) -> StartupIntakeDraft {
    draft.resource_tools.build_system = Some(DEFAULT_BUILD_SYSTEM.to_string());
    draft.resource_tools.formal_docs = Some(DEFAULT_FORMAL_DOCS.to_string());
    draft.resource_tools.coordination = Some(DEFAULT_COORDINATION.to_string());
    draft.resource_tools.external_execution = Some(DEFAULT_EXTERNAL_EXECUTION.to_string());
    draft
}

fn normalize_optional_string(value: Option<String>) -> Option<String> {
    value
        .map(|item| item.trim().to_string())
        .filter(|item| !item.is_empty())
}

fn normalize_horizon(value: u16) -> u16 {
    if value <= 30 {
        30
    } else if value <= 60 {
        60
    } else {
        90
    }
}

fn normalize_stage(value: Option<String>) -> Option<String> {
    let normalized = normalize_optional_string(value)?;
    let lower = normalized.to_lowercase();
    match lower.as_str() {
        "idea" => Some("idea".to_string()),
        "prototype" => Some("prototype".to_string()),
        "mvp" => Some("mvp".to_string()),
        "post_mvp" | "post-mvp" | "post mvp" => Some("post_mvp".to_string()),
        "growth" => Some("growth".to_string()),
        _ => None,
    }
}

fn normalize_launch_mode(value: Option<String>) -> Option<String> {
    let normalized = normalize_optional_string(value)?;
    let lower = normalized.to_lowercase();

    if lower == "default" || lower.contains("default") || lower == "auto" {
        return Some("default".to_string());
    }
    if lower == "custom" || lower.contains("custom") || lower == "manual" {
        return Some("custom".to_string());
    }

    None
}

fn normalize_string_list(values: Vec<String>) -> Vec<String> {
    let mut output = Vec::new();
    let mut seen = HashSet::new();

    for value in values {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            continue;
        }

        let key = trimmed.to_lowercase();
        if seen.insert(key) {
            output.push(trimmed.to_string());
        }
    }

    output
}

fn normalize_agents(values: Vec<StartupRequestedAgentDraft>) -> Vec<StartupRequestedAgentDraft> {
    let mut output = Vec::new();
    let mut seen = HashSet::new();

    for mut agent in values {
        agent.role = agent.role.trim().to_string();
        agent.owner = normalize_optional_string(agent.owner);
        if agent.role.is_empty() {
            continue;
        }

        let key = agent.role.to_lowercase();
        if seen.insert(key) {
            output.push(agent);
        }
    }

    output
}

fn normalize_resource_tools(mut tools: StartupResourceToolDraft) -> StartupResourceToolDraft {
    tools.build_system =
        normalize_allowed_value(tools.build_system, &["github", "gitlab", "bitbucket"]);
    tools.formal_docs = normalize_allowed_value(tools.formal_docs, &["google_docs", "notion"]);
    tools.coordination =
        normalize_allowed_value(tools.coordination, &["slack", "discord", "email"]);
    tools.external_execution =
        normalize_allowed_value(tools.external_execution, &["email", "slack", "discord"]);
    tools
}

fn normalize_allowed_value(value: Option<String>, allowed: &[&str]) -> Option<String> {
    let normalized = normalize_optional_string(value)?.to_lowercase();
    allowed
        .iter()
        .copied()
        .find(|candidate| *candidate == normalized)
        .map(|candidate| candidate.to_string())
}

fn derive_missing_fields(draft: &StartupIntakeDraft) -> Vec<String> {
    let mut missing = Vec::new();

    if draft.founder_name.is_none() {
        missing.push("founder_name".to_string());
    }
    if draft.venture_thesis.is_none() {
        missing.push("venture_thesis".to_string());
    }
    if draft.goals_30_90_days.is_empty() {
        missing.push("goals_30_90_days".to_string());
    }

    match draft.resource_launch_mode.as_deref() {
        Some("default") => {}
        Some("custom") => {
            if draft.resource_tools.build_system.is_none() {
                missing.push("resource_tools.build_system".to_string());
            }
            if draft.resource_tools.formal_docs.is_none() {
                missing.push("resource_tools.formal_docs".to_string());
            }
            if draft.resource_tools.coordination.is_none() {
                missing.push("resource_tools.coordination".to_string());
            }
            if draft.resource_tools.external_execution.is_none() {
                missing.push("resource_tools.external_execution".to_string());
            }
        }
        _ => {
            missing.push("resource_launch_mode".to_string());
        }
    }

    missing
}

fn normalize_assistant_message(
    assistant_message: String,
    missing_fields: &[String],
    ready_for_blueprint: bool,
) -> String {
    let trimmed = assistant_message.trim();
    if !trimmed.is_empty() {
        return trimmed.to_string();
    }

    if ready_for_blueprint {
        return "I have enough information. Click \"Create blueprint now\" to continue."
            .to_string();
    }

    match missing_fields.first().map(|value| value.as_str()) {
        Some("founder_name") => "What should I call you as the founder?".to_string(),
        Some("venture_thesis") => {
            "Can you describe the project you want to start in one or two sentences?".to_string()
        }
        Some("goals_30_90_days") => "What are your top goals for the next 30-90 days?".to_string(),
        Some("resource_launch_mode") => {
            "Do you want default resource launch, or custom tool selection by category?".to_string()
        }
        Some("resource_tools.build_system") => {
            "Choose one Build System tool: GitHub, GitLab, or Bitbucket.".to_string()
        }
        Some("resource_tools.formal_docs") => {
            "Choose one Formal Docs tool: Google Docs or Notion.".to_string()
        }
        Some("resource_tools.coordination") => {
            "Choose one Coordination tool: Slack, Discord, or Email.".to_string()
        }
        Some("resource_tools.external_execution") => {
            "Choose one External Execution tool: Email, Slack, or Discord.".to_string()
        }
        _ => "Tell me a bit more about your startup goals and preferred setup.".to_string(),
    }
}

fn normalize_azure_endpoint(raw: &str) -> String {
    let trimmed = raw.trim().trim_end_matches('/');
    if trimmed.ends_with("/openai/v1") {
        trimmed.to_string()
    } else {
        format!("{}/openai/v1", trimmed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_json_object_finds_first_valid_object() {
        let raw = "noise {\"assistant_message\":\"hi\",\"intake_draft\":{}} trailing";
        let extracted = extract_json_object(raw).expect("json object should be extracted");
        assert!(extracted.contains("\"assistant_message\":\"hi\""));
    }

    #[test]
    fn normalize_draft_clamps_horizon_and_mode() {
        let draft = StartupIntakeDraft {
            founder_name: Some("  Found  ".to_string()),
            venture_thesis: Some("  Build it ".to_string()),
            goals_30_90_days: vec![" Goal ".to_string()],
            plan_horizon_days: Some(42),
            resource_launch_mode: Some("Manual".to_string()),
            ..StartupIntakeDraft::default()
        };

        let normalized = normalize_intake_draft(draft);
        assert_eq!(normalized.founder_name.as_deref(), Some("Found"));
        assert_eq!(normalized.venture_thesis.as_deref(), Some("Build it"));
        assert_eq!(normalized.plan_horizon_days, Some(60));
        assert_eq!(normalized.resource_launch_mode.as_deref(), Some("custom"));
    }

    #[test]
    fn derive_missing_fields_requires_custom_tools() {
        let draft = StartupIntakeDraft {
            founder_name: Some("Founder".to_string()),
            venture_thesis: Some("Build".to_string()),
            goals_30_90_days: vec!["Ship MVP".to_string()],
            resource_launch_mode: Some("custom".to_string()),
            resource_tools: StartupResourceToolDraft {
                build_system: Some("github".to_string()),
                ..StartupResourceToolDraft::default()
            },
            ..StartupIntakeDraft::default()
        };

        let missing = derive_missing_fields(&draft);
        assert!(missing.contains(&"resource_tools.formal_docs".to_string()));
        assert!(missing.contains(&"resource_tools.coordination".to_string()));
        assert!(missing.contains(&"resource_tools.external_execution".to_string()));
    }
}
