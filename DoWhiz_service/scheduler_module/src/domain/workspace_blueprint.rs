use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const WORKSPACE_BLUEPRINT_SCHEMA_VERSION: &str = "2026-03-13";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StartupWorkspaceBlueprint {
    pub schema_version: String,
    pub founder: FounderProfile,
    pub venture: VentureProfile,
    pub plan_horizon_days: u16,
    pub goals_30_90_days: Vec<String>,
    pub current_assets: Vec<String>,
    pub preferred_channels: Vec<String>,
    pub stack: StackSnapshot,
    pub requested_agents: Vec<AgentRoleRequest>,
}

impl Default for StartupWorkspaceBlueprint {
    fn default() -> Self {
        Self {
            schema_version: WORKSPACE_BLUEPRINT_SCHEMA_VERSION.to_string(),
            founder: FounderProfile::default(),
            venture: VentureProfile::default(),
            plan_horizon_days: 30,
            goals_30_90_days: Vec::new(),
            current_assets: Vec::new(),
            preferred_channels: Vec::new(),
            stack: StackSnapshot::default(),
            requested_agents: Vec::new(),
        }
    }
}

impl StartupWorkspaceBlueprint {
    pub fn normalize(mut self) -> Self {
        self.schema_version = WORKSPACE_BLUEPRINT_SCHEMA_VERSION.to_string();
        self.founder.name = self.founder.name.trim().to_string();
        self.founder.email = self.founder.email.trim().to_string();
        self.venture.name = self.venture.name.trim().to_string();
        self.venture.thesis = self.venture.thesis.trim().to_string();

        self.goals_30_90_days = normalize_string_list(self.goals_30_90_days);
        self.current_assets = normalize_string_list(self.current_assets);
        self.preferred_channels = normalize_string_list(self.preferred_channels);
        self.requested_agents = self
            .requested_agents
            .into_iter()
            .map(AgentRoleRequest::normalize)
            .collect();

        if self.plan_horizon_days < 30 {
            self.plan_horizon_days = 30;
        }
        if self.plan_horizon_days > 90 {
            self.plan_horizon_days = 90;
        }

        self
    }

    pub fn validate(&self) -> Result<(), BlueprintValidationError> {
        if self.founder.name.trim().is_empty() {
            return Err(BlueprintValidationError::MissingFounderName);
        }

        if self.venture.thesis.trim().is_empty() {
            return Err(BlueprintValidationError::MissingVentureThesis);
        }

        if !(30..=90).contains(&self.plan_horizon_days) {
            return Err(BlueprintValidationError::InvalidPlanHorizon {
                days: self.plan_horizon_days,
            });
        }

        if self.goals_30_90_days.is_empty() {
            return Err(BlueprintValidationError::MissingGoals);
        }

        Ok(())
    }
}

fn normalize_string_list(values: Vec<String>) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();

    for value in values {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            continue;
        }
        if !output
            .iter()
            .any(|existing| existing.eq_ignore_ascii_case(trimmed))
        {
            output.push(trimmed.to_string());
        }
    }

    output
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct FounderProfile {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct VentureProfile {
    pub name: String,
    pub thesis: String,
    pub stage: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct StackSnapshot {
    pub has_existing_repo: bool,
    pub primary_repo_provider: Option<String>,
    pub has_docs_workspace: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct AgentRoleRequest {
    pub role: String,
    pub owner: Option<String>,
}

impl AgentRoleRequest {
    fn normalize(mut self) -> Self {
        self.role = self.role.trim().to_string();
        self.owner = self.owner.map(|owner| owner.trim().to_string());
        self
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum BlueprintValidationError {
    #[error("founder name is required")]
    MissingFounderName,
    #[error("venture thesis is required")]
    MissingVentureThesis,
    #[error("at least one 30-90 day goal is required")]
    MissingGoals,
    #[error("plan horizon must be between 30 and 90 days, received {days}")]
    InvalidPlanHorizon { days: u16 },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_trims_and_deduplicates_lists() {
        let normalized = StartupWorkspaceBlueprint {
            goals_30_90_days: vec![" Launch beta ".into(), "launch beta".into(), "".into()],
            current_assets: vec!["Deck".into(), " deck ".into()],
            preferred_channels: vec![" Slack ".into(), "slack".into()],
            requested_agents: vec![AgentRoleRequest {
                role: " CTO ".into(),
                owner: Some(" founder ".into()),
            }],
            ..StartupWorkspaceBlueprint::default()
        }
        .normalize();

        assert_eq!(normalized.goals_30_90_days, vec!["Launch beta"]);
        assert_eq!(normalized.current_assets, vec!["Deck"]);
        assert_eq!(normalized.preferred_channels, vec!["Slack"]);
        assert_eq!(normalized.requested_agents[0].role, "CTO");
        assert_eq!(
            normalized.requested_agents[0].owner.as_deref(),
            Some("founder")
        );
    }

    #[test]
    fn validate_requires_core_fields() {
        let mut blueprint = StartupWorkspaceBlueprint::default();
        blueprint.founder.name = "Founder".into();
        blueprint.venture.thesis = "Build an AI operations partner".into();
        blueprint.goals_30_90_days = vec!["Get 3 design partners".into()];

        assert!(blueprint.validate().is_ok());

        blueprint.goals_30_90_days.clear();
        assert_eq!(
            blueprint.validate(),
            Err(BlueprintValidationError::MissingGoals)
        );
    }
}
