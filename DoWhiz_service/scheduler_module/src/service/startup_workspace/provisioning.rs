use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::resource_model::{ProvisioningState, WorkspaceResourcePlan};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProvisioningStepStatus {
    Connected,
    AvailableNotConfigured,
    PlannedManual,
    Blocked,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StartupProvisioningSnapshot {
    pub generated_at: DateTime<Utc>,
    pub connected_resources: usize,
    pub manual_or_pending_resources: usize,
    pub blocked_resources: usize,
    pub status: ProvisioningStepStatus,
}

pub fn derive_provisioning_snapshot(plan: &WorkspaceResourcePlan) -> StartupProvisioningSnapshot {
    let connected_resources = plan
        .resources
        .iter()
        .filter(|resource| matches!(resource.state, ProvisioningState::Connected))
        .count();

    let manual_or_pending_resources = plan
        .resources
        .iter()
        .filter(|resource| {
            matches!(
                resource.state,
                ProvisioningState::AvailableButNotConfigured | ProvisioningState::PlannedManual
            )
        })
        .count();

    let blocked_resources = plan
        .resources
        .iter()
        .filter(|resource| matches!(resource.state, ProvisioningState::Blocked))
        .count();

    let status = if blocked_resources > 0 {
        ProvisioningStepStatus::Blocked
    } else if manual_or_pending_resources > 0 {
        ProvisioningStepStatus::AvailableNotConfigured
    } else {
        ProvisioningStepStatus::Connected
    };

    StartupProvisioningSnapshot {
        generated_at: Utc::now(),
        connected_resources,
        manual_or_pending_resources,
        blocked_resources,
        status,
    }
}
