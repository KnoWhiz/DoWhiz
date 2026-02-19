pub(super) const CODEX_CONFIG_MARKER: &str = "# IMPORTANT: Use your Azure *deployment name* here";
pub(super) const CODEX_CONFIG_BLOCK_TEMPLATE: &str = r#"# IMPORTANT: Use your Azure *deployment name* here (e.g., "gpt-5.2-codex")
model = "{model_name}"
model_provider = "azure"
model_reasoning_effort = "xhigh"
web_search = "live"
ask_for_approval = "never"
sandbox = "{sandbox_mode}"

[sandbox_workspace_write]
network_access = true

[model_providers.azure]
name = "Azure OpenAI"
base_url = "{azure_endpoint}"
env_key = "AZURE_OPENAI_API_KEY_BACKUP"
wire_api = "responses"
"#;
pub(super) const DEFAULT_CLAUDE_MODEL: &str = "claude-opus-4-5";
pub(super) const CLAUDE_FOUNDRY_RESOURCE_DEFAULT: &str = "knowhiz-service-openai-backup-2";
pub(super) const DOCKER_WORKSPACE_DIR: &str = "/workspace";
pub(super) const DOCKER_CODEX_HOME_DIR: &str = ".codex";
pub(super) const SCHEDULED_TASKS_BEGIN: &str = "SCHEDULED_TASKS_JSON_BEGIN";
pub(super) const SCHEDULED_TASKS_END: &str = "SCHEDULED_TASKS_JSON_END";
pub(super) const SCHEDULER_ACTIONS_BEGIN: &str = "SCHEDULER_ACTIONS_JSON_BEGIN";
pub(super) const SCHEDULER_ACTIONS_END: &str = "SCHEDULER_ACTIONS_JSON_END";
pub(super) const GIT_ASKPASS_SCRIPT: &str = r#"#!/bin/sh
case "$1" in
  *Username*)
    if [ -n "$GITHUB_USERNAME" ]; then
      printf "%s" "$GITHUB_USERNAME"
    elif [ -n "$USER" ]; then
      printf "%s" "$USER"
    else
      printf "%s" "x-access-token"
    fi
    ;;
  *Password*)
    if [ -n "$GH_TOKEN" ]; then
      printf "%s" "$GH_TOKEN"
    elif [ -n "$GITHUB_TOKEN" ]; then
      printf "%s" "$GITHUB_TOKEN"
    elif [ -n "$GITHUB_PERSONAL_ACCESS_TOKEN" ]; then
      printf "%s" "$GITHUB_PERSONAL_ACCESS_TOKEN"
    fi
    ;;
  *)
    ;;
esac
exit 0
"#;
