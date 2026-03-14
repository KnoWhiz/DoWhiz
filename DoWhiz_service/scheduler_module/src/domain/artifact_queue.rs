use serde::{Deserialize, Serialize};

use crate::domain::starter_tasks::{StarterTaskPlan, StarterTaskStatus};
use crate::domain::workspace_blueprint::StartupWorkspaceBlueprint;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactQueueStatus {
    Planned,
    PendingReview,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ArtifactPlaceholder {
    pub id: String,
    pub title: String,
    pub surface: String,
    pub owner_role: String,
    pub status: ArtifactQueueStatus,
    pub rationale: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ArtifactQueuePlan {
    pub artifacts: Vec<ArtifactPlaceholder>,
}

pub fn build_initial_artifact_queue(
    blueprint: &StartupWorkspaceBlueprint,
    starter_tasks: &StarterTaskPlan,
) -> ArtifactQueuePlan {
    let mut plan = ArtifactQueuePlan::default();
    let default_owner = blueprint
        .requested_agents
        .first()
        .map(|agent| agent.role.clone())
        .unwrap_or_else(|| "Generalist".to_string());

    push_unique(
        &mut plan.artifacts,
        ArtifactPlaceholder {
            id: "artifact_founder_summary".to_string(),
            title: "Founder intake summary".to_string(),
            surface: "workspace_home".to_string(),
            owner_role: default_owner.clone(),
            status: ArtifactQueueStatus::Planned,
            rationale: "Capture normalized intake input as the baseline for workspace execution."
                .to_string(),
        },
    );

    push_unique(
        &mut plan.artifacts,
        ArtifactPlaceholder {
            id: "artifact_startup_brief".to_string(),
            title: "Startup workspace brief".to_string(),
            surface: "formal_docs".to_string(),
            owner_role: default_owner.clone(),
            status: ArtifactQueueStatus::Planned,
            rationale: "Translate thesis + goals into a shared brief for all startup agents."
                .to_string(),
        },
    );

    push_unique(
        &mut plan.artifacts,
        ArtifactPlaceholder {
            id: "artifact_30_day_execution_plan".to_string(),
            title: "30-day execution plan".to_string(),
            surface: "task_board".to_string(),
            owner_role: "Chief of Staff".to_string(),
            status: ArtifactQueueStatus::Planned,
            rationale: "Define milestone cadence with owner accountability.".to_string(),
        },
    );

    for (index, goal) in blueprint.goals_30_90_days.iter().take(3).enumerate() {
        push_unique(
            &mut plan.artifacts,
            ArtifactPlaceholder {
                id: format!("artifact_goal_brief_{index}"),
                title: format!("Goal brief: {goal}"),
                surface: "task_board".to_string(),
                owner_role: default_owner.clone(),
                status: ArtifactQueueStatus::Planned,
                rationale: "Create a goal-level brief with measurable outcomes and dependencies."
                    .to_string(),
            },
        );
    }

    for task in starter_tasks
        .tasks
        .iter()
        .filter(|task| matches!(task.status, StarterTaskStatus::ManualStepRequired))
        .take(3)
    {
        push_unique(
            &mut plan.artifacts,
            ArtifactPlaceholder {
                id: format!("artifact_manual_checklist_{}", task.id),
                title: format!("Manual setup checklist: {}", task.title),
                surface: "approval_queue".to_string(),
                owner_role: task.owner_role.clone(),
                status: ArtifactQueueStatus::PendingReview,
                rationale:
                    "Manual setup is required before this workflow can execute automatically."
                        .to_string(),
            },
        );
    }

    plan
}

fn push_unique(artifacts: &mut Vec<ArtifactPlaceholder>, candidate: ArtifactPlaceholder) {
    if artifacts
        .iter()
        .any(|existing| existing.id.eq_ignore_ascii_case(&candidate.id))
    {
        return;
    }

    artifacts.push(candidate);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::starter_tasks::StarterTask;

    #[test]
    fn includes_goal_briefs_from_blueprint() {
        let blueprint = StartupWorkspaceBlueprint {
            goals_30_90_days: vec!["Launch alpha".to_string(), "Ship onboarding".to_string()],
            ..StartupWorkspaceBlueprint::default()
        };

        let plan = build_initial_artifact_queue(&blueprint, &StarterTaskPlan::default());

        assert!(plan
            .artifacts
            .iter()
            .any(|artifact| artifact.title == "Goal brief: Launch alpha"));
        assert!(plan
            .artifacts
            .iter()
            .any(|artifact| artifact.title == "Goal brief: Ship onboarding"));
    }

    #[test]
    fn includes_manual_checklist_when_manual_tasks_exist() {
        let starter_tasks = StarterTaskPlan {
            tasks: vec![StarterTask {
                id: "task_repo_connect".to_string(),
                title: "Connect repository".to_string(),
                owner_role: "Founder".to_string(),
                status: StarterTaskStatus::ManualStepRequired,
                rationale: "Required for build workflows".to_string(),
                depends_on: vec![],
            }],
        };

        let plan =
            build_initial_artifact_queue(&StartupWorkspaceBlueprint::default(), &starter_tasks);

        assert!(plan.artifacts.iter().any(|artifact| {
            artifact.id == "artifact_manual_checklist_task_repo_connect"
                && artifact.status == ArtifactQueueStatus::PendingReview
        }));
    }
}
