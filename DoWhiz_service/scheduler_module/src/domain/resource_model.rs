use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ResourceCategory {
    WorkspaceHome,
    KnowledgeHubStructured,
    FormalDocs,
    BuildSystem,
    ExternalExecution,
    CoordinationLayer,
    PublishPresence,
    AgentRoster,
    TaskBoard,
    ArtifactQueue,
    ApprovalPolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProvisioningState {
    Connected,
    AvailableButNotConfigured,
    PlannedManual,
    Blocked,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorkspaceResource {
    pub category: ResourceCategory,
    pub provider: String,
    pub state: ProvisioningState,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorkspaceResourcePlan {
    pub resources: Vec<WorkspaceResource>,
}

impl WorkspaceResourcePlan {
    pub fn push_unique(&mut self, resource: WorkspaceResource) {
        let duplicate = self.resources.iter().any(|existing| {
            existing.category == resource.category && existing.provider == resource.provider
        });

        if !duplicate {
            self.resources.push(resource);
        }
    }
}
