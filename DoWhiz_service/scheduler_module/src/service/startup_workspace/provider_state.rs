use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProviderCapabilitySnapshot {
    pub github: bool,
    pub google_docs: bool,
    pub email: bool,
    pub slack: bool,
    pub discord: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProviderConnectionSnapshot {
    pub github: bool,
    pub google_docs: bool,
    pub email: bool,
    pub slack: bool,
    pub discord: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorkspaceProviderRuntimeState {
    pub has_account: bool,
    pub capabilities: ProviderCapabilitySnapshot,
    pub connected: ProviderConnectionSnapshot,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LinkedIdentifierSnapshot {
    pub identifier_type: String,
    pub identifier: String,
    pub verified: bool,
}

impl From<(String, String, bool)> for LinkedIdentifierSnapshot {
    fn from(value: (String, String, bool)) -> Self {
        Self {
            identifier_type: value.0,
            identifier: value.1,
            verified: value.2,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProviderCapabilityInputs {
    pub github_oauth_ready: bool,
    pub google_docs_runtime_ready: bool,
    pub email_outbound_ready: bool,
    pub slack_oauth_ready: bool,
    pub slack_bot_ready: bool,
    pub discord_oauth_ready: bool,
    pub discord_bot_ready: bool,
}

pub fn derive_provider_capabilities(
    inputs: &ProviderCapabilityInputs,
) -> ProviderCapabilitySnapshot {
    ProviderCapabilitySnapshot {
        github: inputs.github_oauth_ready,
        google_docs: inputs.google_docs_runtime_ready,
        email: inputs.email_outbound_ready,
        slack: inputs.slack_oauth_ready || inputs.slack_bot_ready,
        discord: inputs.discord_oauth_ready || inputs.discord_bot_ready,
    }
}

pub fn derive_provider_connections(
    identifiers: &[LinkedIdentifierSnapshot],
) -> ProviderConnectionSnapshot {
    let is_connected = |provider: &str| -> bool {
        identifiers.iter().any(|identifier| {
            identifier.verified
                && identifier.identifier_type.eq_ignore_ascii_case(provider)
                && !identifier.identifier.trim().is_empty()
        })
    };

    ProviderConnectionSnapshot {
        github: is_connected("github"),
        google_docs: is_connected("google_docs")
            || is_connected("google")
            || is_connected("google_workspace"),
        email: is_connected("email"),
        slack: is_connected("slack"),
        discord: is_connected("discord"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capabilities_mark_slack_and_discord_available_with_bot_tokens() {
        let capabilities = derive_provider_capabilities(&ProviderCapabilityInputs {
            slack_bot_ready: true,
            discord_bot_ready: true,
            ..ProviderCapabilityInputs::default()
        });

        assert!(capabilities.slack);
        assert!(capabilities.discord);
        assert!(!capabilities.github);
    }

    #[test]
    fn connections_only_count_verified_identifiers() {
        let identifiers = vec![
            LinkedIdentifierSnapshot {
                identifier_type: "github".to_string(),
                identifier: "octocat".to_string(),
                verified: true,
            },
            LinkedIdentifierSnapshot {
                identifier_type: "email".to_string(),
                identifier: "founder@example.com".to_string(),
                verified: false,
            },
            LinkedIdentifierSnapshot {
                identifier_type: "google_workspace".to_string(),
                identifier: "workspace-id".to_string(),
                verified: true,
            },
        ];

        let connections = derive_provider_connections(&identifiers);

        assert!(connections.github);
        assert!(connections.google_docs);
        assert!(!connections.email);
    }
}
