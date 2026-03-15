use crate::domain::workspace_blueprint::{BlueprintValidationError, StartupWorkspaceBlueprint};

pub fn normalize_and_validate_blueprint(
    blueprint: StartupWorkspaceBlueprint,
) -> Result<StartupWorkspaceBlueprint, BlueprintValidationError> {
    let normalized = blueprint.normalize();
    normalized.validate()?;
    Ok(normalized)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_and_validate_returns_error_for_missing_thesis() {
        let mut blueprint = StartupWorkspaceBlueprint::default();
        blueprint.founder.name = "Founder".into();
        blueprint.goals_30_90_days = vec!["Ship MVP".into()];

        assert!(normalize_and_validate_blueprint(blueprint).is_err());
    }
}
