use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

pub const GTM_SCHEMA_VERSION: &str = "1.0";

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum AgentId {
    RachelOrchestrator,
    RachelIcpScout,
    RachelOutboundSdr,
    RachelFeedbackPrdSynthesizer,
    RachelPositioningPmm,
    RachelContentStudio,
    RachelOnboardingCsm,
    RachelExperimentAnalyst,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TaskPriority {
    Low,
    Normal,
    High,
    Urgent,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Succeeded,
    NeedsHuman,
    Failed,
    Partial,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum GtmChannel {
    Email,
    LinkedinAds,
    HubspotWorkflow,
    LinkedinDm,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SubjectType {
    Objective,
    Account,
    Contact,
    Campaign,
    Feature,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PolicyPack {
    pub human_approval_required: bool,
    pub allowed_channels: Vec<GtmChannel>,
    pub blocked_actions: Vec<String>,
    pub pii_policy: String,
    pub compliance_region: Vec<String>,
}

impl Default for PolicyPack {
    fn default() -> Self {
        Self {
            human_approval_required: false,
            allowed_channels: vec![
                GtmChannel::Email,
                GtmChannel::LinkedinAds,
                GtmChannel::HubspotWorkflow,
            ],
            blocked_actions: Vec::new(),
            pii_policy: "mask".to_string(),
            compliance_region: vec!["US".to_string()],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AgentTaskEnvelope {
    pub task_id: Uuid,
    pub tenant_id: Uuid,
    pub objective_id: Uuid,
    pub agent_id: AgentId,
    pub schema_version: String,
    pub requested_at: DateTime<Utc>,
    pub deadline_at: Option<DateTime<Utc>>,
    pub priority: TaskPriority,
    pub policy_pack: PolicyPack,
    pub input_refs: Vec<String>,
    pub trace_id: Uuid,
    pub idempotency_key: String,
}

impl AgentTaskEnvelope {
    pub fn new(agent_id: AgentId) -> Self {
        let now = Utc::now();
        let task_id = Uuid::new_v4();
        Self {
            task_id,
            tenant_id: Uuid::new_v4(),
            objective_id: Uuid::new_v4(),
            agent_id,
            schema_version: GTM_SCHEMA_VERSION.to_string(),
            requested_at: now,
            deadline_at: None,
            priority: TaskPriority::Normal,
            policy_pack: PolicyPack::default(),
            input_refs: Vec::new(),
            trace_id: Uuid::new_v4(),
            idempotency_key: format!("{}:{}", agent_id.as_str(), task_id),
        }
    }

    pub fn with_agent(&self, agent_id: AgentId) -> Self {
        let task_id = Uuid::new_v4();
        Self {
            task_id,
            tenant_id: self.tenant_id,
            objective_id: self.objective_id,
            agent_id,
            schema_version: self.schema_version.clone(),
            requested_at: self.requested_at,
            deadline_at: self.deadline_at,
            priority: self.priority,
            policy_pack: self.policy_pack.clone(),
            input_refs: self.input_refs.clone(),
            trace_id: self.trace_id,
            idempotency_key: format!("{}:{}", agent_id.as_str(), task_id),
        }
    }
}

impl AgentId {
    pub fn as_str(self) -> &'static str {
        match self {
            AgentId::RachelOrchestrator => "rachel_orchestrator",
            AgentId::RachelIcpScout => "rachel_icp_scout",
            AgentId::RachelOutboundSdr => "rachel_outbound_sdr",
            AgentId::RachelFeedbackPrdSynthesizer => "rachel_feedback_prd_synthesizer",
            AgentId::RachelPositioningPmm => "rachel_positioning_pmm",
            AgentId::RachelContentStudio => "rachel_content_studio",
            AgentId::RachelOnboardingCsm => "rachel_onboarding_csm",
            AgentId::RachelExperimentAnalyst => "rachel_experiment_analyst",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AgentTaskResult<T> {
    pub task_id: Uuid,
    pub status: TaskStatus,
    pub schema_version: String,
    pub output_payload: T,
    pub emitted_events: Vec<EventEnvelope>,
    pub confidence: f32,
    pub evidence_refs: Vec<String>,
    pub next_action: String,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EventEnvelope {
    pub event_id: Uuid,
    pub event_type: String,
    pub occurred_at: DateTime<Utc>,
    pub producer: AgentId,
    pub tenant_id: Uuid,
    pub subject_type: SubjectType,
    pub subject_id: Uuid,
    pub schema_version: String,
    pub trace_id: Uuid,
    pub idempotency_key: String,
    pub payload: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Objective {
    pub name: String,
    pub target_metric: String,
    pub target_value: String,
    pub due_date: Option<DateTime<Utc>>,
    pub owner: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct CurrentState {
    pub open_tasks: u32,
    pub blockers: Vec<String>,
    pub active_campaigns: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResourceLimits {
    pub daily_email_cap: u32,
    pub budget_cap_usd: u32,
    pub human_review_capacity: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OrchestratorInput {
    pub objective: Objective,
    pub current_state: CurrentState,
    pub resource_limits: ResourceLimits,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExecutionStep {
    pub step_id: String,
    pub description: String,
    pub depends_on: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TaskAssignment {
    pub task_type: String,
    pub agent_id: AgentId,
    pub deadline_at: Option<DateTime<Utc>>,
    pub input_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ApprovalRequest {
    pub reason: String,
    pub risk_level: String,
    pub reviewer_group: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorkflowState {
    pub stage: String,
    pub progress_pct: u8,
    pub eta_minutes: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OrchestratorOutput {
    pub execution_plan: Vec<ExecutionStep>,
    pub task_assignments: Vec<TaskAssignment>,
    pub approval_requests: Vec<ApprovalRequest>,
    pub workflow_state: WorkflowState,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AccountSignal {
    pub entity_id: Uuid,
    pub company_size: u32,
    pub industry: String,
    pub region: String,
    pub product_events_14d: u32,
    pub support_tickets_30d: u32,
    pub won_deals_12m: u32,
    pub lost_deals_12m: u32,
    pub churned: bool,
    pub activation_days: u32,
    pub ltv_usd: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IcpScoutInput {
    pub accounts: Vec<AccountSignal>,
    pub current_segment_ids: Vec<String>,
    pub min_sample_size: usize,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EntityType {
    Account,
    Contact,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum IcpTier {
    A,
    B,
    C,
    D,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IcpScore {
    pub entity_id: Uuid,
    pub entity_type: EntityType,
    pub score_0_100: u8,
    pub tier: IcpTier,
    pub top_drivers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SegmentDefinition {
    pub segment_id: String,
    pub rule_dsl: String,
    pub expected_lift: f32,
    pub confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DriftReport {
    pub drift_detected: bool,
    pub drift_dimensions: Vec<String>,
    pub recommended_retrain_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IcpScoutOutput {
    pub icp_scores: Vec<IcpScore>,
    pub segment_definitions: Vec<SegmentDefinition>,
    pub anti_icp_rules: Vec<String>,
    pub drift_report: DriftReport,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SegmentContact {
    pub recipient_id: Uuid,
    pub account_id: Uuid,
    pub email: String,
    pub first_name: Option<String>,
    pub job_title: Option<String>,
    pub company_name: Option<String>,
    pub timezone: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ClaimRisk {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MessageVariant {
    pub template_id: String,
    pub subject: String,
    pub body: String,
    pub claim_risk: ClaimRisk,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MessageBundle {
    pub segment_id: String,
    pub variants: Vec<MessageVariant>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SequencePolicy {
    pub max_touches: u8,
    pub cadence_days: u16,
    pub stop_conditions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChannelPolicy {
    pub email_enabled: bool,
    pub linkedin_ads_enabled: bool,
    pub linkedin_dm_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OutboundSdrInput {
    pub segment_manifest: Vec<SegmentContact>,
    pub message_bundle: MessageBundle,
    pub sequence_policy: SequencePolicy,
    pub channel_policy: ChannelPolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SequenceTouch {
    pub touch_number: u8,
    pub offset_days: u16,
    pub channel: GtmChannel,
    pub template_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SequenceDraft {
    pub sequence_id: String,
    pub touches: Vec<SequenceTouch>,
    pub channel: GtmChannel,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SendRequest {
    pub recipient_id: Uuid,
    pub template_id: String,
    pub send_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ReplyClass {
    Positive,
    Neutral,
    Negative,
    Unsubscribe,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReplyClassification {
    pub reply_id: String,
    pub class: ReplyClass,
    pub sentiment: String,
    pub intent: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Handoff {
    pub to_team: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OutboundSdrOutput {
    pub sequence_draft: Option<SequenceDraft>,
    pub personalization_fields_used: Vec<String>,
    pub send_requests: Vec<SendRequest>,
    pub reply_classifications: Vec<ReplyClassification>,
    pub handoffs: Vec<Handoff>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ManualDispatchStatus {
    PendingApproval,
    ReadyForRep,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LinkedinManualSendTask {
    pub manual_task_id: String,
    pub recipient_id: Uuid,
    pub account_id: Uuid,
    pub recipient_email: String,
    pub recipient_name: Option<String>,
    pub company_name: Option<String>,
    pub channel: GtmChannel,
    pub template_id: String,
    pub send_at: DateTime<Utc>,
    pub assignee_team: String,
    pub status: ManualDispatchStatus,
    pub instructions: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HubspotTaskDraft {
    pub external_id: String,
    pub subject: String,
    pub body: String,
    pub due_at: DateTime<Utc>,
    pub contact_email: String,
    pub owner_team: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HubspotCommunicationDraft {
    pub external_id: String,
    pub channel: GtmChannel,
    pub contact_email: String,
    pub scheduled_at: DateTime<Utc>,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ModeAOutboundDispatchInput {
    pub outbound_input: OutboundSdrInput,
    pub outbound_output: OutboundSdrOutput,
    pub assignee_team: String,
    pub reviewer_group: String,
    pub approval_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ModeAOutboundDispatchOutput {
    pub approval_queue: Vec<ApprovalRequest>,
    pub manual_send_tasks: Vec<LinkedinManualSendTask>,
    pub hubspot_task_drafts: Vec<HubspotTaskDraft>,
    pub hubspot_communication_drafts: Vec<HubspotCommunicationDraft>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct HubspotDispatchReport {
    pub tasks_attempted: usize,
    pub tasks_created: usize,
    pub notes_attempted: usize,
    pub notes_created: usize,
    pub associations_attempted: usize,
    pub associations_created: usize,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FeedbackSource {
    Onboarding,
    OutboundReply,
    SupportTicket,
    SalesCall,
    ProductUsage,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FeedbackItem {
    pub feedback_id: Uuid,
    pub source: FeedbackSource,
    pub segment_id: Option<String>,
    pub text: String,
    pub created_at: DateTime<Utc>,
    pub evidence_ref: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProductContext {
    pub roadmap_refs: Vec<String>,
    pub constraints: Vec<String>,
    pub architecture_notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BusinessContext {
    pub revenue_goal: String,
    pub strategic_themes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClusterPolicy {
    pub min_cluster_size: usize,
    pub recency_weight: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FeedbackPrdInput {
    pub feedback_items: Vec<FeedbackItem>,
    pub product_context: ProductContext,
    pub business_context: BusinessContext,
    pub cluster_policy: ClusterPolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InsightCluster {
    pub cluster_id: String,
    pub theme: String,
    pub frequency: u32,
    pub affected_segments: Vec<String>,
    pub evidence_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct JobStory {
    pub as_persona: String,
    pub when_context: String,
    pub i_want: String,
    pub so_i_can: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PrdDraft {
    pub prd_id: String,
    pub problem: String,
    pub users: Vec<String>,
    pub success_metrics: Vec<String>,
    pub scope: Vec<String>,
    pub risks: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PriorityScore {
    pub prd_id: String,
    pub impact: f32,
    pub reach: f32,
    pub confidence: f32,
    pub effort: f32,
    pub overall: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FeedbackPrdOutput {
    pub insight_clusters: Vec<InsightCluster>,
    pub job_stories: Vec<JobStory>,
    pub prd_drafts: Vec<PrdDraft>,
    pub priority_scores: Vec<PriorityScore>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FunnelStage {
    Awareness,
    Consideration,
    Decision,
    Expansion,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PositioningInput {
    pub segment_definitions: Vec<SegmentDefinition>,
    pub insight_clusters: Vec<InsightCluster>,
    pub prd_drafts: Vec<PrdDraft>,
    pub strategic_themes: Vec<String>,
    pub data_contract_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MessageMap {
    pub segment_id: String,
    pub value_proposition: String,
    pub pains: Vec<String>,
    pub proof_points: Vec<String>,
    pub objection_handling: Vec<String>,
    pub funnel_stage: FunnelStage,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PositioningBundle {
    pub bundle_id: String,
    pub message_maps: Vec<MessageMap>,
    pub claim_safe_list: Vec<String>,
    pub generated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PositioningOutput {
    pub positioning_bundle: PositioningBundle,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum AssetChannel {
    Email,
    LandingPage,
    LinkedinAd,
    SalesOnePager,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContentInput {
    pub positioning_bundle: PositioningBundle,
    pub channels: Vec<AssetChannel>,
    pub max_assets_per_channel: u8,
    pub requires_human_review: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContentAsset {
    pub asset_id: String,
    pub channel: AssetChannel,
    pub segment_id: String,
    pub title: String,
    pub body: String,
    pub cta: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContentOutput {
    pub assets: Vec<ContentAsset>,
    pub publish_ready: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RiskSeverity {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OnboardingInput {
    pub customer_id: Uuid,
    pub account_name: String,
    pub segment_id: String,
    pub customer_goals: Vec<String>,
    pub known_blockers: Vec<String>,
    pub current_activation_rate: f32,
    pub target_activation_rate: f32,
    pub handoff_summary: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OnboardingMilestone {
    pub milestone_id: String,
    pub name: String,
    pub due_in_days: u16,
    pub owner_role: String,
    pub success_criteria: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OnboardingRiskFlag {
    pub code: String,
    pub severity: RiskSeverity,
    pub summary: String,
    pub mitigation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OnboardingOutput {
    pub onboarding_plan: Vec<OnboardingMilestone>,
    pub activation_risk_flags: Vec<OnboardingRiskFlag>,
    pub captured_feedback: Vec<FeedbackItem>,
    pub qbr_summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CampaignPerformance {
    pub campaign_id: String,
    pub segment_id: String,
    pub spend_usd: f32,
    pub impressions: u32,
    pub clicks: u32,
    pub meetings: u32,
    pub sqls: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FeatureAdoptionSignal {
    pub feature_name: String,
    pub before_rate: f32,
    pub after_rate: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExperimentInput {
    pub experiment_name: String,
    pub primary_metric: String,
    pub baseline_value: f32,
    pub observed_value: f32,
    pub sample_size: usize,
    pub min_sample_size: usize,
    pub confidence_estimate: f32,
    pub segment_ids: Vec<String>,
    pub campaign_results: Vec<CampaignPerformance>,
    pub adoption_signals: Vec<FeatureAdoptionSignal>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExperimentDesign {
    pub experiment_id: String,
    pub hypothesis: String,
    pub success_metric: String,
    pub guardrails: Vec<String>,
    pub segments: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExperimentResultSummary {
    pub experiment_id: String,
    pub uplift_ratio: f32,
    pub statistically_reliable: bool,
    pub confidence_estimate: f32,
    pub sample_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExperimentRecommendation {
    pub action: String,
    pub owner: String,
    pub rationale: String,
    pub expected_impact: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExperimentOutput {
    pub experiment_design: ExperimentDesign,
    pub result_summary: ExperimentResultSummary,
    pub recommendations: Vec<ExperimentRecommendation>,
}
