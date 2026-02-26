mod contracts;
mod hubspot;
mod mode_a;
mod phase1;
mod phase2;
mod phase3;

pub use contracts::{
    AccountSignal, AgentId, AgentTaskEnvelope, AgentTaskResult, ApprovalRequest, AssetChannel,
    BusinessContext, CampaignPerformance, ChannelPolicy, ClaimRisk, ClusterPolicy, ContentAsset,
    ContentInput, ContentOutput, CurrentState, EntityType, EventEnvelope, ExperimentDesign,
    ExperimentInput, ExperimentOutput, ExperimentRecommendation, ExperimentResultSummary,
    FeatureAdoptionSignal, FeedbackItem, FeedbackPrdInput, FeedbackPrdOutput, FeedbackSource,
    FunnelStage, GtmChannel, HubspotCommunicationDraft, HubspotDispatchReport, HubspotTaskDraft,
    IcpScore, IcpScoutInput, IcpScoutOutput, IcpTier, InsightCluster, JobStory,
    LinkedinManualSendTask, ManualDispatchStatus, MessageBundle, MessageMap, MessageVariant,
    ModeAOutboundDispatchInput, ModeAOutboundDispatchOutput, Objective, OnboardingInput,
    OnboardingMilestone, OnboardingOutput, OnboardingRiskFlag, OrchestratorInput,
    OrchestratorOutput, OutboundSdrInput, OutboundSdrOutput, PolicyPack, PositioningBundle,
    PositioningInput, PositioningOutput, PrdDraft, PriorityScore, ProductContext, ResourceLimits,
    RiskSeverity, SegmentContact, SegmentDefinition, SequencePolicy, TaskPriority, TaskStatus,
    GTM_SCHEMA_VERSION,
};
pub use hubspot::{HubspotDispatchError, HubspotModeAExecutor};
pub use mode_a::{ModeAAgentEngine, ModeAAgentError, ModeAWorkflowInput, ModeAWorkflowResult};
pub use phase1::{GtmAgentError, Phase1AgentEngine, Phase1WorkflowInput, Phase1WorkflowResult};
pub use phase2::{Phase2AgentEngine, Phase2AgentError, Phase2WorkflowInput, Phase2WorkflowResult};
pub use phase3::{Phase3AgentEngine, Phase3AgentError, Phase3WorkflowInput, Phase3WorkflowResult};
