use crate::domain::resource_model::{ProvisioningState, ResourceCategory, WorkspaceResourcePlan};
use crate::domain::workspace_blueprint::StartupWorkspaceBlueprint;

pub fn build_starter_resource_plan(blueprint: &StartupWorkspaceBlueprint) -> WorkspaceResourcePlan {
    let mut plan = WorkspaceResourcePlan::default();

    plan.add(
        ResourceCategory::WorkspaceHome,
        "dowhiz_workspace",
        "DoWhiz Workspace",
        ProvisioningState::Connected,
        Some("Workspace shell becomes the primary operating surface.".into()),
        None,
    );
    plan.add(
        ResourceCategory::AgentRoster,
        "dowhiz_agents",
        "DoWhiz Agents",
        ProvisioningState::Connected,
        Some("Founding-team ownership map is ready.".into()),
        None,
    );
    plan.add(
        ResourceCategory::TaskBoard,
        "dowhiz_task_board",
        "DoWhiz Task Board",
        ProvisioningState::AvailableButNotConfigured,
        Some("Starter task graph lands here once bootstrap is wired to runtime.".into()),
        Some("Confirm default board lanes and SLA expectations.".into()),
    );
    plan.add(
        ResourceCategory::ArtifactQueue,
        "dowhiz_artifacts",
        "DoWhiz Artifacts",
        ProvisioningState::AvailableButNotConfigured,
        Some("Artifact queue is reviewable before broad automation.".into()),
        Some("Choose artifact retention and review policy.".into()),
    );

    if blueprint.stack.has_existing_repo || channel_requested(blueprint, "github") {
        let provider_key = blueprint
            .stack
            .primary_repo_provider
            .clone()
            .unwrap_or_else(|| "github".into());
        let provider_display = repo_provider_display_name(&provider_key);

        plan.add(
            ResourceCategory::BuildSystem,
            provider_key,
            provider_display,
            ProvisioningState::Connected,
            Some("Repository execution workflows can run immediately.".into()),
            None,
        );
    } else {
        plan.add(
            ResourceCategory::BuildSystem,
            "github",
            "GitHub",
            ProvisioningState::AvailableButNotConfigured,
            Some("Connect a repository to unlock build-system execution.".into()),
            Some("Connect GitHub (or another repo provider) and approve access scope.".into()),
        );
    }

    if blueprint.stack.has_docs_workspace
        || channel_requested(blueprint, "google docs")
        || channel_requested(blueprint, "google sheets")
        || channel_requested(blueprint, "google slides")
    {
        plan.add(
            ResourceCategory::FormalDocs,
            "google_docs",
            "Google Docs",
            ProvisioningState::Connected,
            Some("Formal document artifact layer is connected.".into()),
            None,
        );
    } else {
        plan.add(
            ResourceCategory::FormalDocs,
            "google_docs",
            "Google Docs",
            ProvisioningState::AvailableButNotConfigured,
            Some("Formal document artifacts are available once Google Docs is connected.".into()),
            Some("Connect Google Docs for specification and execution artifacts.".into()),
        );
    }

    if channel_requested(blueprint, "email") {
        plan.add(
            ResourceCategory::ExternalExecution,
            "email",
            "Email",
            ProvisioningState::Connected,
            Some("External execution channel is active through email.".into()),
            None,
        );
    } else {
        plan.add(
            ResourceCategory::ExternalExecution,
            "email",
            "Email",
            ProvisioningState::AvailableButNotConfigured,
            Some("Email is the default external execution surface.".into()),
            Some("Connect or approve outbound email routing.".into()),
        );
    }

    let wants_coordination =
        channel_requested(blueprint, "slack") || channel_requested(blueprint, "discord");
    let coordination_provider_key = if channel_requested(blueprint, "slack") {
        "slack"
    } else {
        "discord"
    };
    let coordination_provider_display = if coordination_provider_key == "slack" {
        "Slack"
    } else {
        "Discord"
    };

    if wants_coordination {
        plan.add(
            ResourceCategory::CoordinationLayer,
            coordination_provider_key,
            coordination_provider_display,
            ProvisioningState::Connected,
            Some("Coordination channel is active for status updates and approvals.".into()),
            None,
        );
        plan.add(
            ResourceCategory::ApprovalPolicy,
            coordination_provider_key,
            coordination_provider_display,
            ProvisioningState::Connected,
            Some("Approval policy is enforced in the active coordination channel.".into()),
            None,
        );
    } else {
        plan.add(
            ResourceCategory::CoordinationLayer,
            "slack",
            "Slack",
            ProvisioningState::PlannedManual,
            Some("Slack/Discord can be connected later for team coordination.".into()),
            Some("Connect Slack or Discord and assign the coordination channel.".into()),
        );
        plan.add(
            ResourceCategory::ApprovalPolicy,
            "slack",
            "Slack",
            ProvisioningState::PlannedManual,
            Some("Approval routing depends on coordination channel setup.".into()),
            Some("Configure approval path after Slack/Discord connection.".into()),
        );
    }

    plan.add(
        ResourceCategory::KnowledgeHubStructured,
        "notion",
        "Notion",
        ProvisioningState::PlannedManual,
        Some("Structured operating hub can be modeled in Notion.".into()),
        Some("Create Notion workspace + templates for recurring operating reviews.".into()),
    );

    let has_publish_identity = !blueprint.venture.name.trim().is_empty();
    if has_publish_identity {
        plan.add(
            ResourceCategory::PublishPresence,
            "publish_channels",
            "Publishing Channels",
            ProvisioningState::PlannedManual,
            Some("Publishing/distribution presence can be connected incrementally.".into()),
            Some("Select launch channels (for example LinkedIn/X/Product Hunt).".into()),
        );
    } else {
        plan.add(
            ResourceCategory::PublishPresence,
            "publish_channels",
            "Publishing Channels",
            ProvisioningState::Blocked,
            Some("Publishing presence is blocked until startup identity is defined.".into()),
            Some("Set a startup/project name in intake to unblock publish presence setup.".into()),
        );
    }

    plan
}

fn repo_provider_display_name(provider_key: &str) -> &'static str {
    if provider_key.eq_ignore_ascii_case("github") {
        "GitHub"
    } else if provider_key.eq_ignore_ascii_case("gitlab") {
        "GitLab"
    } else if provider_key.eq_ignore_ascii_case("bitbucket") {
        "Bitbucket"
    } else {
        "Repository Provider"
    }
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
            venture: crate::domain::workspace_blueprint::VentureProfile {
                name: "Acme".into(),
                thesis: "Build AI workflow ops".into(),
                stage: Some("mvp".into()),
            },
            preferred_channels: vec!["Email".into(), "Slack".into(), "Google Docs".into()],
            stack: crate::domain::workspace_blueprint::StackSnapshot {
                has_existing_repo: true,
                primary_repo_provider: Some("github".into()),
                has_docs_workspace: true,
            },
            ..StartupWorkspaceBlueprint::default()
        };

        let plan = build_starter_resource_plan(&blueprint);

        assert!(plan.resources.iter().any(|resource| {
            resource.category == ResourceCategory::BuildSystem
                && resource.state == ProvisioningState::Connected
                && resource.provider.key == "github"
        }));
        assert!(plan.resources.iter().any(|resource| {
            resource.category == ResourceCategory::CoordinationLayer
                && resource.provider.key == "slack"
                && resource.state == ProvisioningState::Connected
        }));
        assert!(plan.resources.iter().any(|resource| {
            resource.category == ResourceCategory::FormalDocs
                && resource.provider.key == "google_docs"
                && resource.state == ProvisioningState::Connected
        }));
    }

    #[test]
    fn falls_back_to_truthful_states_when_not_connected() {
        let blueprint = StartupWorkspaceBlueprint::default();
        let plan = build_starter_resource_plan(&blueprint);

        assert!(plan.resources.iter().any(|resource| {
            resource.category == ResourceCategory::BuildSystem
                && resource.state == ProvisioningState::AvailableButNotConfigured
        }));
        assert!(plan.resources.iter().any(|resource| {
            resource.category == ResourceCategory::CoordinationLayer
                && resource.state == ProvisioningState::PlannedManual
        }));
        assert!(plan.resources.iter().any(|resource| {
            resource.category == ResourceCategory::KnowledgeHubStructured
                && resource.provider.key == "notion"
                && resource.state == ProvisioningState::PlannedManual
        }));
    }

    #[test]
    fn marks_publish_presence_blocked_without_startup_identity() {
        let blueprint = StartupWorkspaceBlueprint::default();
        let plan = build_starter_resource_plan(&blueprint);

        assert!(plan.resources.iter().any(|resource| {
            resource.category == ResourceCategory::PublishPresence
                && resource.state == ProvisioningState::Blocked
        }));
    }

    #[test]
    fn includes_object_name_and_purpose_for_each_resource() {
        let blueprint = StartupWorkspaceBlueprint::default();
        let plan = build_starter_resource_plan(&blueprint);

        assert!(plan
            .resources
            .iter()
            .all(|resource| !resource.object_name.trim().is_empty()));
        assert!(plan
            .resources
            .iter()
            .all(|resource| !resource.object_purpose.trim().is_empty()));
    }
}
