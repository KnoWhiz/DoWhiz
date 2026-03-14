use crate::domain::resource_model::WorkspaceResourcePlan;
use crate::domain::workspace_blueprint::{BlueprintValidationError, StartupWorkspaceBlueprint};

use super::intake::normalize_and_validate_blueprint;
use super::provisioning::{derive_provisioning_snapshot, StartupProvisioningSnapshot};
use super::resource_mapping::build_starter_resource_plan;

#[derive(Debug, Clone)]
pub struct StartupWorkspaceBootstrapPlan {
    pub blueprint: StartupWorkspaceBlueprint,
    pub resources: WorkspaceResourcePlan,
    pub provisioning: StartupProvisioningSnapshot,
}

pub fn bootstrap_workspace_plan(
    blueprint: StartupWorkspaceBlueprint,
) -> Result<StartupWorkspaceBootstrapPlan, BlueprintValidationError> {
    let validated_blueprint = normalize_and_validate_blueprint(blueprint)?;
    let resources = build_starter_resource_plan(&validated_blueprint);
    let provisioning = derive_provisioning_snapshot(&resources);

    Ok(StartupWorkspaceBootstrapPlan {
        blueprint: validated_blueprint,
        resources,
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
        assert!(plan.provisioning.generated_at.timestamp() > 0);
        assert_eq!(plan.blueprint.plan_horizon_days, 30);
    }
}
