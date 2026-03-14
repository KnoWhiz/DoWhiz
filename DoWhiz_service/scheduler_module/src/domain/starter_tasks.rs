use serde::{Deserialize, Serialize};

use crate::domain::workspace_blueprint::StartupWorkspaceBlueprint;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum StarterTaskStatus {
    Planned,
    ManualStepRequired,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StarterTask {
    pub id: String,
    pub title: String,
    pub owner_role: String,
    pub status: StarterTaskStatus,
    pub rationale: String,
    pub depends_on: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct StarterTaskPlan {
    pub tasks: Vec<StarterTask>,
}

pub fn build_starter_task_plan(blueprint: &StartupWorkspaceBlueprint) -> StarterTaskPlan {
    let default_owner = blueprint
        .requested_agents
        .first()
        .map(|agent| agent.role.clone())
        .unwrap_or_else(|| "Generalist".to_string());

    let mut tasks = Vec::new();

    tasks.push(StarterTask {
        id: "task_workspace_brief".to_string(),
        title: "Finalize startup workspace brief".to_string(),
        owner_role: default_owner.clone(),
        status: StarterTaskStatus::Planned,
        rationale: "Turn founder thesis and goals into a shared execution brief.".to_string(),
        depends_on: Vec::new(),
    });

    tasks.push(StarterTask {
        id: "task_30_day_plan".to_string(),
        title: "Generate 30-day execution plan".to_string(),
        owner_role: "Chief of Staff".to_string(),
        status: StarterTaskStatus::Planned,
        rationale: "Translate the blueprint into milestones, owners, and review checkpoints."
            .to_string(),
        depends_on: vec!["task_workspace_brief".to_string()],
    });

    tasks.push(StarterTask {
        id: "task_channel_routing".to_string(),
        title: "Set default channel routing + approvals".to_string(),
        owner_role: "Operations".to_string(),
        status: StarterTaskStatus::Planned,
        rationale: "Map where work starts and where approvals are required for external delivery."
            .to_string(),
        depends_on: vec!["task_workspace_brief".to_string()],
    });

    if blueprint.stack.has_existing_repo {
        tasks.push(StarterTask {
            id: "task_repo_bootstrap".to_string(),
            title: "Bootstrap delivery workflow in repository".to_string(),
            owner_role: "Builder".to_string(),
            status: StarterTaskStatus::Planned,
            rationale:
                "Use existing repo to create implementation, testing, and release workflow handoffs."
                    .to_string(),
            depends_on: vec!["task_30_day_plan".to_string()],
        });
    } else {
        tasks.push(StarterTask {
            id: "task_repo_connect".to_string(),
            title: "Connect repository for build system workflows".to_string(),
            owner_role: "Founder".to_string(),
            status: StarterTaskStatus::ManualStepRequired,
            rationale: "Repo connection is required before code execution workflows can run."
                .to_string(),
            depends_on: vec!["task_30_day_plan".to_string()],
        });
    }

    if blueprint.preferred_channels.iter().any(|channel| {
        channel.eq_ignore_ascii_case("slack") || channel.eq_ignore_ascii_case("discord")
    }) {
        tasks.push(StarterTask {
            id: "task_coordination_channel".to_string(),
            title: "Activate coordination channel status loop".to_string(),
            owner_role: "Chief of Staff".to_string(),
            status: StarterTaskStatus::Planned,
            rationale: "Keep task status and approvals visible in team coordination channels."
                .to_string(),
            depends_on: vec!["task_channel_routing".to_string()],
        });
    } else {
        tasks.push(StarterTask {
            id: "task_coordination_channel".to_string(),
            title: "Connect Slack or Discord for team coordination".to_string(),
            owner_role: "Founder".to_string(),
            status: StarterTaskStatus::ManualStepRequired,
            rationale: "Coordination channel setup is optional but recommended for team loops."
                .to_string(),
            depends_on: vec!["task_channel_routing".to_string()],
        });
    }

    StarterTaskPlan { tasks }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::workspace_blueprint::{AgentRoleRequest, StackSnapshot};

    #[test]
    fn includes_manual_repo_task_when_repo_not_connected() {
        let blueprint = StartupWorkspaceBlueprint::default();
        let plan = build_starter_task_plan(&blueprint);

        assert!(plan.tasks.iter().any(|task| {
            task.id == "task_repo_connect" && task.status == StarterTaskStatus::ManualStepRequired
        }));
    }

    #[test]
    fn uses_requested_agent_as_default_owner() {
        let blueprint = StartupWorkspaceBlueprint {
            requested_agents: vec![AgentRoleRequest {
                role: "Builder".to_string(),
                owner: Some("Founder".to_string()),
            }],
            stack: StackSnapshot {
                has_existing_repo: true,
                primary_repo_provider: Some("github".to_string()),
                has_docs_workspace: false,
            },
            preferred_channels: vec!["Slack".to_string()],
            ..StartupWorkspaceBlueprint::default()
        };

        let plan = build_starter_task_plan(&blueprint);

        let workspace_brief_task = plan
            .tasks
            .iter()
            .find(|task| task.id == "task_workspace_brief")
            .expect("workspace brief task should exist");

        assert_eq!(workspace_brief_task.owner_role, "Builder");
        assert!(plan
            .tasks
            .iter()
            .any(|task| task.id == "task_repo_bootstrap"));
    }
}
