use serde::{Deserialize, Serialize};

use crate::domain::agent_roster::{build_starter_agent_roster, AgentRosterPlan};
use crate::domain::artifact_queue::{build_initial_artifact_queue, ArtifactQueuePlan};
use crate::domain::resource_model::WorkspaceResourcePlan;
use crate::domain::starter_tasks::{build_starter_task_plan, StarterTaskPlan};
use crate::domain::workspace_blueprint::{BlueprintValidationError, StartupWorkspaceBlueprint};

use super::intake::normalize_and_validate_blueprint;
use super::provisioning::{derive_provisioning_snapshot, StartupProvisioningSnapshot};
use super::resource_mapping::build_starter_resource_plan;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartupWorkspaceBootstrapPlan {
    pub blueprint: StartupWorkspaceBlueprint,
    pub resources: WorkspaceResourcePlan,
    pub agent_roster: AgentRosterPlan,
    pub starter_tasks: StarterTaskPlan,
    pub artifact_queue: ArtifactQueuePlan,
    pub provisioning: StartupProvisioningSnapshot,
}

pub fn bootstrap_workspace_plan(
    blueprint: StartupWorkspaceBlueprint,
) -> Result<StartupWorkspaceBootstrapPlan, BlueprintValidationError> {
    let validated_blueprint = normalize_and_validate_blueprint(blueprint)?;
    let resources = build_starter_resource_plan(&validated_blueprint);
    let starter_tasks = build_starter_task_plan(&validated_blueprint);
    let agent_roster = build_starter_agent_roster(&validated_blueprint, &resources, &starter_tasks);
    let artifact_queue = build_initial_artifact_queue(&validated_blueprint, &starter_tasks);
    let provisioning = derive_provisioning_snapshot(&resources);

    Ok(StartupWorkspaceBootstrapPlan {
        blueprint: validated_blueprint,
        resources,
        agent_roster,
        starter_tasks,
        artifact_queue,
        provisioning,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bootstrap_returns_validated_plan() {
        let mut blueprint = StartupWorkspaceBlueprint::default();
        blueprint.founder.name = "Founder".into();
        blueprint.venture.thesis = "Build with agent-native workflows".into();
        blueprint.goals_30_90_days = vec!["Launch alpha".into()];

        let plan = bootstrap_workspace_plan(blueprint).expect("bootstrap should succeed");

        assert!(!plan.resources.resources.is_empty());
        assert!(!plan.agent_roster.assignments.is_empty());
        assert!(!plan.starter_tasks.tasks.is_empty());
        assert!(!plan.artifact_queue.artifacts.is_empty());
        assert!(plan.provisioning.generated_at.timestamp() > 0);
        assert_eq!(plan.blueprint.plan_horizon_days, 30);
    }
}
