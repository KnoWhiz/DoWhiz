use crate::domain::resource_model::{
    ProvisioningState, ResourceCategory, WorkspaceResource, WorkspaceResourcePlan,
};
use crate::domain::workspace_blueprint::StartupWorkspaceBlueprint;

pub fn build_starter_resource_plan(blueprint: &StartupWorkspaceBlueprint) -> WorkspaceResourcePlan {
    let mut plan = WorkspaceResourcePlan::default();

    plan.push_unique(WorkspaceResource {
        category: ResourceCategory::WorkspaceHome,
        provider: "dowhiz_workspace".into(),
        state: ProvisioningState::Connected,
        note: Some("Workspace shell becomes the primary operating surface.".into()),
    });
    plan.push_unique(WorkspaceResource {
        category: ResourceCategory::AgentRoster,
        provider: "dowhiz_agents".into(),
        state: ProvisioningState::Connected,
        note: None,
    });
    plan.push_unique(WorkspaceResource {
        category: ResourceCategory::TaskBoard,
        provider: "dowhiz_task_board".into(),
        state: ProvisioningState::AvailableButNotConfigured,
        note: Some("Starter task graph lands here once bootstrap is wired to runtime.".into()),
    });
    plan.push_unique(WorkspaceResource {
        category: ResourceCategory::ArtifactQueue,
        provider: "dowhiz_artifacts".into(),
        state: ProvisioningState::AvailableButNotConfigured,
        note: Some("Artifact queue is reviewable before broad automation.".into()),
    });
    plan.push_unique(WorkspaceResource {
        category: ResourceCategory::ApprovalPolicy,
        provider: "dowhiz_approvals".into(),
        state: ProvisioningState::AvailableButNotConfigured,
        note: Some("Explicit human review remains visible and configurable.".into()),
    });

    if blueprint.stack.has_existing_repo || channel_requested(blueprint, "github") {
        plan.push_unique(WorkspaceResource {
            category: ResourceCategory::BuildSystem,
            provider: blueprint
                .stack
                .primary_repo_provider
                .clone()
                .unwrap_or_else(|| "github".into()),
            state: ProvisioningState::Connected,
            note: None,
        });
    } else {
        plan.push_unique(WorkspaceResource {
            category: ResourceCategory::BuildSystem,
            provider: "github".into(),
            state: ProvisioningState::AvailableButNotConfigured,
            note: Some("Connect a repo to unlock build-system execution.".into()),
        });
    }

    if channel_requested(blueprint, "google docs")
        || channel_requested(blueprint, "google sheets")
        || channel_requested(blueprint, "google slides")
    {
        plan.push_unique(WorkspaceResource {
            category: ResourceCategory::FormalDocs,
            provider: "google_workspace".into(),
            state: ProvisioningState::Connected,
            note: None,
        });
    } else {
        plan.push_unique(WorkspaceResource {
            category: ResourceCategory::FormalDocs,
            provider: "google_workspace".into(),
            state: ProvisioningState::AvailableButNotConfigured,
            note: Some("Connect docs/sheets/slides when formal artifacts are needed.".into()),
        });
    }

    if channel_requested(blueprint, "email") {
        plan.push_unique(WorkspaceResource {
            category: ResourceCategory::ExternalExecution,
            provider: "email".into(),
            state: ProvisioningState::Connected,
            note: None,
        });
    }

    let wants_coordination =
        channel_requested(blueprint, "slack") || channel_requested(blueprint, "discord");
    if wants_coordination {
        let provider = if channel_requested(blueprint, "slack") {
            "slack"
        } else {
            "discord"
        };

        plan.push_unique(WorkspaceResource {
            category: ResourceCategory::CoordinationLayer,
            provider: provider.into(),
            state: ProvisioningState::Connected,
            note: Some(
                "Coordination channels are used for routing approvals and status updates.".into(),
            ),
        });
    } else {
        plan.push_unique(WorkspaceResource {
            category: ResourceCategory::CoordinationLayer,
            provider: "slack".into(),
            state: ProvisioningState::PlannedManual,
            note: Some("Slack/Discord can be connected later for team coordination.".into()),
        });
    }

    plan.push_unique(WorkspaceResource {
        category: ResourceCategory::KnowledgeHubStructured,
        provider: "notion".into(),
        state: ProvisioningState::PlannedManual,
        note: Some("Modeled as a resource even when provisioning is manual.".into()),
    });
    plan.push_unique(WorkspaceResource {
        category: ResourceCategory::PublishPresence,
        provider: "distribution_channels".into(),
        state: ProvisioningState::PlannedManual,
        note: Some("Publishing/presence setup may require manual onboarding.".into()),
    });

    if !plan
        .resources
        .iter()
        .any(|resource| resource.category == ResourceCategory::ExternalExecution)
    {
        plan.push_unique(WorkspaceResource {
            category: ResourceCategory::ExternalExecution,
            provider: "email".into(),
            state: ProvisioningState::AvailableButNotConfigured,
            note: Some("Email is the default external execution surface.".into()),
        });
    }

    plan
}

fn channel_requested(blueprint: &StartupWorkspaceBlueprint, needle: &str) -> bool {
    let needle_lower = needle.to_ascii_lowercase();

    blueprint
        .preferred_channels
        .iter()
        .any(|channel| channel.to_ascii_lowercase().contains(&needle_lower))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_existing_repo_and_channel_preferences_to_connected_resources() {
        let blueprint = StartupWorkspaceBlueprint {
            preferred_channels: vec!["Email".into(), "Slack".into(), "Google Docs".into()],
            stack: crate::domain::workspace_blueprint::StackSnapshot {
                has_existing_repo: true,
                primary_repo_provider: Some("github".into()),
                has_docs_workspace: true,
            },
            ..StartupWorkspaceBlueprint::default()
        };

        let plan = build_starter_resource_plan(&blueprint);

        assert!(plan.resources.iter().any(|resource| resource.category
            == ResourceCategory::BuildSystem
            && resource.state == ProvisioningState::Connected));
        assert!(plan.resources.iter().any(|resource| resource.category
            == ResourceCategory::CoordinationLayer
            && resource.provider == "slack"
            && resource.state == ProvisioningState::Connected));
        assert!(plan
            .resources
            .iter()
            .any(|resource| resource.category == ResourceCategory::FormalDocs
                && resource.state == ProvisioningState::Connected));
    }

    #[test]
    fn falls_back_to_truthful_available_states_when_not_connected() {
        let blueprint = StartupWorkspaceBlueprint::default();
        let plan = build_starter_resource_plan(&blueprint);

        assert!(plan.resources.iter().any(|resource| resource.category
            == ResourceCategory::BuildSystem
            && resource.state == ProvisioningState::AvailableButNotConfigured));
        assert!(plan.resources.iter().any(|resource| resource.category
            == ResourceCategory::CoordinationLayer
            && resource.state == ProvisioningState::PlannedManual));
        assert!(plan.resources.iter().any(|resource| resource.category
            == ResourceCategory::KnowledgeHubStructured
            && resource.state == ProvisioningState::PlannedManual));
    }
}
