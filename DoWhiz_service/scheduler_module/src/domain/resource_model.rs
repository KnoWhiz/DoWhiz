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

impl ResourceCategory {
    pub fn object_name(&self) -> &'static str {
        match self {
            ResourceCategory::WorkspaceHome => "Workspace Home",
            ResourceCategory::KnowledgeHubStructured => "Knowledge Hub (Structured)",
            ResourceCategory::FormalDocs => "Formal Docs",
            ResourceCategory::BuildSystem => "Build System",
            ResourceCategory::ExternalExecution => "External Execution",
            ResourceCategory::CoordinationLayer => "Coordination Layer",
            ResourceCategory::PublishPresence => "Publish Presence",
            ResourceCategory::AgentRoster => "Agent Roster",
            ResourceCategory::TaskBoard => "Task Board",
            ResourceCategory::ArtifactQueue => "Artifact Queue",
            ResourceCategory::ApprovalPolicy => "Approval Policy",
        }
    }

    pub fn object_purpose(&self) -> &'static str {
        match self {
            ResourceCategory::WorkspaceHome => {
                "Primary startup operating surface for context, tasks, artifacts, and approvals."
            }
            ResourceCategory::KnowledgeHubStructured => {
                "Structured operating hub for captured knowledge and decision records."
            }
            ResourceCategory::FormalDocs => {
                "Formal document artifact layer for specs, plans, and stakeholder-ready outputs."
            }
            ResourceCategory::BuildSystem => {
                "Code execution and delivery workflows through repository-connected tooling."
            }
            ResourceCategory::ExternalExecution => {
                "Outbound execution surface for external stakeholders and operating communication."
            }
            ResourceCategory::CoordinationLayer => {
                "Internal coordination loop for status updates, approvals, and handoffs."
            }
            ResourceCategory::PublishPresence => {
                "Publishing and distribution surfaces for launch and ongoing presence."
            }
            ResourceCategory::AgentRoster => {
                "Ownership map for digital founding-team roles and responsibilities."
            }
            ResourceCategory::TaskBoard => {
                "Execution board for startup milestones and active work."
            }
            ResourceCategory::ArtifactQueue => {
                "Queue of generated artifacts for reviewable, auditable delivery."
            }
            ResourceCategory::ApprovalPolicy => {
                "Human-review policy layer for sensitive or external actions."
            }
        }
    }
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
pub struct ProviderMetadata {
    pub key: String,
    pub display_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorkspaceResource {
    pub category: ResourceCategory,
    pub object_name: String,
    pub object_purpose: String,
    pub provider: ProviderMetadata,
    pub state: ProvisioningState,
    pub note: Option<String>,
    pub manual_next_step: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorkspaceResourcePlan {
    pub resources: Vec<WorkspaceResource>,
}

impl WorkspaceResourcePlan {
    pub fn add(
        &mut self,
        category: ResourceCategory,
        provider_key: impl Into<String>,
        provider_display_name: impl Into<String>,
        state: ProvisioningState,
        note: Option<String>,
        manual_next_step: Option<String>,
    ) {
        let provider_key = provider_key.into();
        let provider_display_name = provider_display_name.into();

        self.push_unique(WorkspaceResource {
            object_name: category.object_name().to_string(),
            object_purpose: category.object_purpose().to_string(),
            category,
            provider: ProviderMetadata {
                key: provider_key,
                display_name: provider_display_name,
            },
            state,
            note,
            manual_next_step,
        });
    }

    pub fn push_unique(&mut self, resource: WorkspaceResource) {
        let duplicate = self.resources.iter().any(|existing| {
            existing.category == resource.category && existing.provider.key == resource.provider.key
        });

        if !duplicate {
            self.resources.push(resource);
        }
    }
}
