pub mod bootstrap;
pub mod intake;
pub mod intake_chat;
pub mod provider_state;
pub mod provisioning;
pub mod resource_mapping;
pub mod workspace_home;

pub use bootstrap::{bootstrap_workspace_plan, StartupWorkspaceBootstrapPlan};
pub use intake::normalize_and_validate_blueprint;
pub use intake_chat::{
    generate_startup_intake_chat_response, StartupIntakeChatRequest, StartupIntakeChatResponse,
};
pub use provider_state::{
    derive_provider_capabilities, derive_provider_connections, LinkedIdentifierSnapshot,
    ProviderCapabilityInputs, ProviderCapabilitySnapshot, ProviderConnectionSnapshot,
    WorkspaceProviderRuntimeState,
};
pub use provisioning::{
    derive_provisioning_snapshot, ProvisioningStepStatus, StartupProvisioningSnapshot,
};
pub use resource_mapping::build_starter_resource_plan;
pub use workspace_home::{
    bootstrap_workspace_home_snapshot, build_workspace_home_snapshot, WorkspaceHomeSnapshot,
};
