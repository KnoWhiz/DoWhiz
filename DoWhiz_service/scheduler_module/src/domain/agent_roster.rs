use serde::{Deserialize, Serialize};

use crate::domain::resource_model::{ProvisioningState, ResourceCategory, WorkspaceResourcePlan};
use crate::domain::starter_tasks::StarterTaskPlan;
use crate::domain::workspace_blueprint::StartupWorkspaceBlueprint;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AgentAssignmentStatus {
    Active,
    Planned,
    ManualSetupRequired,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AgentAssignment {
    pub role: String,
    pub owner: String,
    pub status: AgentAssignmentStatus,
    pub focus: String,
    pub owned_resources: Vec<ResourceCategory>,
    pub starter_task_ids: Vec<String>,
    pub manual_next_steps: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct AgentRosterPlan {
    pub assignments: Vec<AgentAssignment>,
}

pub fn build_starter_agent_roster(
    blueprint: &StartupWorkspaceBlueprint,
    resources: &WorkspaceResourcePlan,
    starter_tasks: &StarterTaskPlan,
) -> AgentRosterPlan {
    let mut assignments = Vec::new();
    let role_requests = seed_role_requests(blueprint);

    for (index, (role, owner_override)) in role_requests.into_iter().enumerate() {
        let role = role.trim().to_string();
        if role.is_empty() {
            continue;
        }

        let owner = owner_override.unwrap_or_else(|| default_owner_name(&role));
        let owned_resources = resolve_owned_resources(&role, resources);
        let starter_task_ids = resolve_starter_task_ids(&role, starter_tasks);
        let manual_next_steps = resolve_manual_next_steps(&owned_resources, resources);

        let status = if !manual_next_steps.is_empty() {
            AgentAssignmentStatus::ManualSetupRequired
        } else if index == 0 {
            AgentAssignmentStatus::Active
        } else {
            AgentAssignmentStatus::Planned
        };

        assignments.push(AgentAssignment {
            role: role.clone(),
            owner,
            status,
            focus: focus_for_role(&role),
            owned_resources,
            starter_task_ids,
            manual_next_steps,
        });
    }

    AgentRosterPlan { assignments }
}

fn seed_role_requests(blueprint: &StartupWorkspaceBlueprint) -> Vec<(String, Option<String>)> {
    if !blueprint.requested_agents.is_empty() {
        return blueprint
            .requested_agents
            .iter()
            .filter(|agent| !agent.role.trim().is_empty())
            .map(|agent| (agent.role.trim().to_string(), agent.owner.clone()))
            .collect();
    }

    vec![
        ("Generalist".to_string(), None),
        ("Builder".to_string(), None),
        ("Chief of Staff".to_string(), None),
        ("GTM Strategist".to_string(), None),
    ]
}

fn default_owner_name(role: &str) -> String {
    let trimmed = role.trim();
    if trimmed.is_empty() {
        return "Agent Owner".to_string();
    }
    format!("{trimmed} Agent")
}

fn resolve_owned_resources(role: &str, resources: &WorkspaceResourcePlan) -> Vec<ResourceCategory> {
    let preferred_categories = preferred_categories_for_role(role);
    let mut owned_resources = Vec::new();

    for resource in resources.resources.iter() {
        if preferred_categories
            .iter()
            .any(|category| category == &resource.category)
            && !owned_resources
                .iter()
                .any(|existing| existing == &resource.category)
        {
            owned_resources.push(resource.category.clone());
        }
    }

    owned_resources
}

fn preferred_categories_for_role(role: &str) -> Vec<ResourceCategory> {
    let role_lower = role.to_ascii_lowercase();

    if role_lower.contains("builder")
        || role_lower.contains("engineer")
        || role_lower.contains("coder")
    {
        return vec![
            ResourceCategory::BuildSystem,
            ResourceCategory::TaskBoard,
            ResourceCategory::ArtifactQueue,
        ];
    }

    if role_lower.contains("gtm")
        || role_lower.contains("growth")
        || role_lower.contains("marketing")
    {
        return vec![
            ResourceCategory::PublishPresence,
            ResourceCategory::ExternalExecution,
            ResourceCategory::FormalDocs,
        ];
    }

    if role_lower.contains("chief") || role_lower.contains("ops") || role_lower.contains("staff") {
        return vec![
            ResourceCategory::CoordinationLayer,
            ResourceCategory::ApprovalPolicy,
            ResourceCategory::TaskBoard,
        ];
    }

    vec![
        ResourceCategory::WorkspaceHome,
        ResourceCategory::AgentRoster,
        ResourceCategory::KnowledgeHubStructured,
    ]
}

fn resolve_starter_task_ids(role: &str, starter_tasks: &StarterTaskPlan) -> Vec<String> {
    starter_tasks
        .tasks
        .iter()
        .filter(|task| role_matches_owner(role, &task.owner_role))
        .map(|task| task.id.clone())
        .collect()
}

fn role_matches_owner(role: &str, owner_role: &str) -> bool {
    if role.eq_ignore_ascii_case(owner_role) {
        return true;
    }

    let role_lower = role.to_ascii_lowercase();
    let owner_lower = owner_role.to_ascii_lowercase();

    role_lower.contains(&owner_lower) || owner_lower.contains(&role_lower)
}

fn resolve_manual_next_steps(
    owned_resources: &[ResourceCategory],
    resources: &WorkspaceResourcePlan,
) -> Vec<String> {
    let mut manual_steps: Vec<String> = Vec::new();

    for resource in resources.resources.iter() {
        if !owned_resources
            .iter()
            .any(|category| category == &resource.category)
        {
            continue;
        }

        if matches!(resource.state, ProvisioningState::Connected) {
            continue;
        }

        if let Some(step) = resource
            .manual_next_step
            .as_deref()
            .map(str::trim)
            .filter(|step| !step.is_empty())
        {
            if !manual_steps
                .iter()
                .any(|existing| existing.eq_ignore_ascii_case(step))
            {
                manual_steps.push(step.to_string());
            }
            continue;
        }

        let fallback = format!("Configure {}", resource.object_name);
        if !manual_steps
            .iter()
            .any(|existing| existing.eq_ignore_ascii_case(&fallback))
        {
            manual_steps.push(fallback);
        }
    }

    manual_steps
}

fn focus_for_role(role: &str) -> String {
    let role_lower = role.to_ascii_lowercase();

    if role_lower.contains("builder")
        || role_lower.contains("engineer")
        || role_lower.contains("coder")
    {
        return "Ship implementation milestones with test-ready outputs.".to_string();
    }

    if role_lower.contains("gtm")
        || role_lower.contains("growth")
        || role_lower.contains("marketing")
    {
        return "Drive distribution loops from early positioning to launch traction.".to_string();
    }

    if role_lower.contains("chief") || role_lower.contains("ops") || role_lower.contains("staff") {
        return "Coordinate approvals, execution cadence, and cross-agent handoffs.".to_string();
    }

    "Coordinate end-to-end startup execution across channels with shared memory.".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::resource_model::ProvisioningState;
    use crate::domain::starter_tasks::{StarterTask, StarterTaskStatus};
    use crate::domain::workspace_blueprint::AgentRoleRequest;

    #[test]
    fn defaults_to_founding_team_roster_when_no_roles_requested() {
        let blueprint = StartupWorkspaceBlueprint::default();
        let resources = WorkspaceResourcePlan::default();
        let starter_tasks = StarterTaskPlan::default();

        let roster = build_starter_agent_roster(&blueprint, &resources, &starter_tasks);

        assert_eq!(roster.assignments.len(), 4);
        assert_eq!(roster.assignments[0].role, "Generalist");
    }

    #[test]
    fn uses_requested_agent_owner_and_manual_steps() {
        let blueprint = StartupWorkspaceBlueprint {
            requested_agents: vec![AgentRoleRequest {
                role: "Builder".to_string(),
                owner: Some("Founder".to_string()),
            }],
            ..StartupWorkspaceBlueprint::default()
        };

        let mut resources = WorkspaceResourcePlan::default();
        resources.add(
            ResourceCategory::BuildSystem,
            "github",
            "GitHub",
            ProvisioningState::AvailableButNotConfigured,
            Some("Repository execution workflows need connection.".to_string()),
            Some("Connect GitHub and approve repository scope.".to_string()),
        );

        let starter_tasks = StarterTaskPlan {
            tasks: vec![StarterTask {
                id: "task_repo_connect".to_string(),
                title: "Connect repository".to_string(),
                owner_role: "Builder".to_string(),
                status: StarterTaskStatus::ManualStepRequired,
                rationale: "Required before run-task execution.".to_string(),
                depends_on: vec![],
            }],
        };

        let roster = build_starter_agent_roster(&blueprint, &resources, &starter_tasks);
        let builder = roster
            .assignments
            .iter()
            .find(|assignment| assignment.role == "Builder")
            .expect("builder assignment should exist");

        assert_eq!(builder.owner, "Founder");
        assert_eq!(builder.status, AgentAssignmentStatus::ManualSetupRequired);
        assert!(builder
            .manual_next_steps
            .iter()
            .any(|step| step.contains("Connect GitHub")));
        assert!(builder
            .starter_task_ids
            .iter()
            .any(|task_id| task_id == "task_repo_connect"));
    }
}
