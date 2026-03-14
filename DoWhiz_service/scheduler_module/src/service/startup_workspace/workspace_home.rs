use serde::{Deserialize, Serialize};

use crate::domain::agent_roster::AgentRosterPlan;
use crate::domain::artifact_queue::{ArtifactQueuePlan, ArtifactQueueStatus};
use crate::domain::resource_model::WorkspaceResourcePlan;
use crate::domain::starter_tasks::{StarterTaskPlan, StarterTaskStatus};
use crate::domain::workspace_blueprint::{BlueprintValidationError, StartupWorkspaceBlueprint};

use super::bootstrap::{bootstrap_workspace_plan, StartupWorkspaceBootstrapPlan};
use super::provisioning::StartupProvisioningSnapshot;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceQueueStatus {
    PendingReview,
    Planned,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorkspaceArtifactSummary {
    pub id: String,
    pub title: String,
    pub surface: String,
    pub status: WorkspaceQueueStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorkspaceApprovalItem {
    pub id: String,
    pub title: String,
    pub owner: String,
    pub status: WorkspaceQueueStatus,
    pub rationale: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorkspaceHomeSnapshot {
    pub workspace_title: String,
    pub startup_brief: String,
    pub founder_name: String,
    pub resources: WorkspaceResourcePlan,
    pub agent_roster: AgentRosterPlan,
    pub starter_tasks: StarterTaskPlan,
    pub artifact_queue: ArtifactQueuePlan,
    pub provisioning: StartupProvisioningSnapshot,
    pub recent_artifacts: Vec<WorkspaceArtifactSummary>,
    pub approval_queue: Vec<WorkspaceApprovalItem>,
    pub next_recommended_actions: Vec<String>,
}

pub fn build_workspace_home_snapshot(
    plan: &StartupWorkspaceBootstrapPlan,
) -> WorkspaceHomeSnapshot {
    let workspace_title = if plan.blueprint.venture.name.trim().is_empty() {
        "Founder Workspace".to_string()
    } else {
        plan.blueprint.venture.name.trim().to_string()
    };

    let startup_brief = plan.blueprint.venture.thesis.trim().to_string();
    let founder_name = if plan.blueprint.founder.name.trim().is_empty() {
        "Founder".to_string()
    } else {
        plan.blueprint.founder.name.trim().to_string()
    };

    let recent_artifacts = plan
        .artifact_queue
        .artifacts
        .iter()
        .take(3)
        .map(|artifact| WorkspaceArtifactSummary {
            id: artifact.id.clone(),
            title: artifact.title.clone(),
            surface: artifact.surface.clone(),
            status: match artifact.status {
                ArtifactQueueStatus::Planned => WorkspaceQueueStatus::Planned,
                ArtifactQueueStatus::PendingReview => WorkspaceQueueStatus::PendingReview,
            },
        })
        .collect::<Vec<_>>();

    let mut approval_queue = Vec::new();

    if !plan.blueprint.stack.has_existing_repo {
        approval_queue.push(WorkspaceApprovalItem {
            id: "approval_repo_connection".to_string(),
            title: "Approve repository connection scope".to_string(),
            owner: founder_name.clone(),
            status: WorkspaceQueueStatus::PendingReview,
            rationale: "Repository access is required before build-system execution.".to_string(),
        });
    }

    let has_coordination = plan.blueprint.preferred_channels.iter().any(|channel| {
        channel.eq_ignore_ascii_case("slack") || channel.eq_ignore_ascii_case("discord")
    });

    if !has_coordination {
        approval_queue.push(WorkspaceApprovalItem {
            id: "approval_coordination_channel".to_string(),
            title: "Select Slack or Discord for coordination".to_string(),
            owner: founder_name.clone(),
            status: WorkspaceQueueStatus::PendingReview,
            rationale: "Coordination channel selection keeps approvals and status updates visible."
                .to_string(),
        });
    }

    approval_queue.push(WorkspaceApprovalItem {
        id: "approval_delivery_policy".to_string(),
        title: "Review outbound delivery approval policy".to_string(),
        owner: founder_name.clone(),
        status: WorkspaceQueueStatus::PendingReview,
        rationale: "Human review remains explicit before sensitive external execution.".to_string(),
    });

    let mut next_recommended_actions = Vec::new();

    for resource in plan
        .resources
        .resources
        .iter()
        .filter(|resource| {
            !matches!(
                resource.state,
                crate::domain::resource_model::ProvisioningState::Connected
            )
        })
        .take(2)
    {
        if let Some(step) = resource
            .manual_next_step
            .as_deref()
            .map(str::trim)
            .filter(|step| !step.is_empty())
        {
            next_recommended_actions.push(step.to_string());
        } else {
            next_recommended_actions.push(format!("Configure {}", resource.object_name));
        }
    }

    for task in plan.starter_tasks.tasks.iter() {
        if matches!(task.status, StarterTaskStatus::ManualStepRequired) {
            next_recommended_actions.push(format!("Manual step: {}", task.title));
        }
    }

    for approval in approval_queue.iter().take(2) {
        next_recommended_actions.push(approval.title.clone());
    }

    if next_recommended_actions.is_empty() {
        next_recommended_actions
            .push("Start the first planned task from the starter task board.".to_string());
    }

    WorkspaceHomeSnapshot {
        workspace_title,
        startup_brief,
        founder_name,
        resources: plan.resources.clone(),
        agent_roster: plan.agent_roster.clone(),
        starter_tasks: plan.starter_tasks.clone(),
        artifact_queue: plan.artifact_queue.clone(),
        provisioning: plan.provisioning.clone(),
        recent_artifacts,
        approval_queue,
        next_recommended_actions,
    }
}

pub fn bootstrap_workspace_home_snapshot(
    blueprint: StartupWorkspaceBlueprint,
) -> Result<WorkspaceHomeSnapshot, BlueprintValidationError> {
    let plan = bootstrap_workspace_plan(blueprint)?;
    Ok(build_workspace_home_snapshot(&plan))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot_includes_manual_approvals_when_repo_and_coordination_are_missing() {
        let mut blueprint = StartupWorkspaceBlueprint::default();
        blueprint.founder.name = "Founder".to_string();
        blueprint.venture.thesis = "Build workspace-first execution".to_string();
        blueprint.goals_30_90_days = vec!["Launch alpha".to_string()];

        let snapshot = bootstrap_workspace_home_snapshot(blueprint)
            .expect("workspace home snapshot bootstrap should succeed");

        assert!(snapshot
            .approval_queue
            .iter()
            .any(|item| item.id == "approval_repo_connection"));
        assert!(!snapshot.agent_roster.assignments.is_empty());
        assert!(!snapshot.artifact_queue.artifacts.is_empty());
        assert!(snapshot
            .approval_queue
            .iter()
            .any(|item| item.id == "approval_coordination_channel"));
        assert!(!snapshot.recent_artifacts.is_empty());
    }

    #[test]
    fn snapshot_uses_fallback_workspace_title() {
        let mut blueprint = StartupWorkspaceBlueprint::default();
        blueprint.founder.name = "Founder".to_string();
        blueprint.venture.thesis = "Build workspace-first execution".to_string();
        blueprint.goals_30_90_days = vec!["Launch alpha".to_string()];

        let snapshot = bootstrap_workspace_home_snapshot(blueprint)
            .expect("workspace home snapshot bootstrap should succeed");

        assert_eq!(snapshot.workspace_title, "Founder Workspace");
    }
}
