use chrono::Utc;
use scheduler_module::gtm_agents::{
    AccountSignal, AgentId, AgentTaskEnvelope, BusinessContext, ChannelPolicy, ClaimRisk,
    ClusterPolicy, CurrentState, FeedbackItem, FeedbackPrdInput, FeedbackSource, IcpScoutInput,
    MessageBundle, MessageVariant, Objective, OrchestratorInput, OutboundSdrInput,
    Phase1AgentEngine, Phase1WorkflowInput, PolicyPack, ProductContext, ResourceLimits,
    SegmentContact, SequencePolicy, TaskPriority, TaskStatus,
};
use uuid::Uuid;

fn base_envelope() -> AgentTaskEnvelope {
    let mut envelope = AgentTaskEnvelope::new(AgentId::RachelOrchestrator);
    envelope.priority = TaskPriority::High;
    envelope.policy_pack = PolicyPack::default();
    envelope.input_refs = vec!["warehouse://daily_snapshot".to_string()];
    envelope
}

#[test]
fn phase1_workflow_end_to_end_emits_cross_agent_events() {
    let engine = Phase1AgentEngine;
    let workflow = Phase1WorkflowInput {
        base_envelope: base_envelope(),
        orchestrator: OrchestratorInput {
            objective: Objective {
                name: "Increase high-fit meetings".to_string(),
                target_metric: "meeting_rate".to_string(),
                target_value: "0.18".to_string(),
                due_date: None,
                owner: "gtm_owner".to_string(),
            },
            current_state: CurrentState::default(),
            resource_limits: ResourceLimits {
                daily_email_cap: 300,
                budget_cap_usd: 15_000,
                human_review_capacity: 3,
            },
        },
        icp_scout: IcpScoutInput {
            accounts: vec![
                AccountSignal {
                    entity_id: Uuid::new_v4(),
                    company_size: 90,
                    industry: "SaaS".to_string(),
                    region: "US".to_string(),
                    product_events_14d: 10,
                    support_tickets_30d: 1,
                    won_deals_12m: 4,
                    lost_deals_12m: 1,
                    churned: false,
                    activation_days: 5,
                    ltv_usd: 8_000.0,
                },
                AccountSignal {
                    entity_id: Uuid::new_v4(),
                    company_size: 120,
                    industry: "SaaS".to_string(),
                    region: "US".to_string(),
                    product_events_14d: 8,
                    support_tickets_30d: 2,
                    won_deals_12m: 3,
                    lost_deals_12m: 2,
                    churned: false,
                    activation_days: 7,
                    ltv_usd: 6_000.0,
                },
            ],
            current_segment_ids: Vec::new(),
            min_sample_size: 2,
        },
        outbound_sdr: OutboundSdrInput {
            segment_manifest: vec![
                SegmentContact {
                    recipient_id: Uuid::new_v4(),
                    account_id: Uuid::new_v4(),
                    email: "alpha@example.com".to_string(),
                    first_name: Some("Avery".to_string()),
                    job_title: Some("Head of Growth".to_string()),
                    company_name: Some("Alpha".to_string()),
                    timezone: Some("America/Los_Angeles".to_string()),
                },
                SegmentContact {
                    recipient_id: Uuid::new_v4(),
                    account_id: Uuid::new_v4(),
                    email: "bravo@example.com".to_string(),
                    first_name: Some("Bailey".to_string()),
                    job_title: Some("VP Marketing".to_string()),
                    company_name: Some("Bravo".to_string()),
                    timezone: Some("America/New_York".to_string()),
                },
            ],
            message_bundle: MessageBundle {
                segment_id: "tier_a_high_fit".to_string(),
                variants: vec![MessageVariant {
                    template_id: "safe_email_v1".to_string(),
                    subject: "Could this shorten your onboarding time?".to_string(),
                    body: "Low-risk variant focused on activation metrics.".to_string(),
                    claim_risk: ClaimRisk::Low,
                }],
            },
            sequence_policy: SequencePolicy {
                max_touches: 3,
                cadence_days: 2,
                stop_conditions: vec!["positive_reply".to_string()],
            },
            channel_policy: ChannelPolicy {
                email_enabled: true,
                linkedin_ads_enabled: false,
                linkedin_dm_enabled: false,
            },
        },
        feedback_prd: FeedbackPrdInput {
            feedback_items: vec![
                FeedbackItem {
                    feedback_id: Uuid::new_v4(),
                    source: FeedbackSource::SupportTicket,
                    segment_id: Some("tier_a_high_fit".to_string()),
                    text: "HubSpot integration misses custom properties".to_string(),
                    created_at: Utc::now(),
                    evidence_ref: Some("ticket:1001".to_string()),
                },
                FeedbackItem {
                    feedback_id: Uuid::new_v4(),
                    source: FeedbackSource::OutboundReply,
                    segment_id: Some("tier_a_high_fit".to_string()),
                    text: "Need better integration for existing CRM workflows".to_string(),
                    created_at: Utc::now(),
                    evidence_ref: Some("reply:2002".to_string()),
                },
            ],
            product_context: ProductContext {
                roadmap_refs: vec!["roadmap://q3".to_string()],
                constraints: vec!["single platform squad".to_string()],
                architecture_notes: vec!["legacy ETL path".to_string()],
            },
            business_context: BusinessContext {
                revenue_goal: "increase enterprise pipeline".to_string(),
                strategic_themes: vec!["activation".to_string(), "expansion".to_string()],
            },
            cluster_policy: ClusterPolicy {
                min_cluster_size: 1,
                recency_weight: 0.6,
            },
        },
    };

    let result = engine
        .run_workflow(workflow)
        .expect("phase1 workflow should run");
    assert_eq!(result.orchestrator.status, TaskStatus::Succeeded);
    assert_eq!(result.icp_scout.status, TaskStatus::Succeeded);
    assert_eq!(result.outbound_sdr.status, TaskStatus::Succeeded);
    assert_eq!(result.feedback_prd.status, TaskStatus::Succeeded);
    assert!(result
        .events
        .iter()
        .any(|event| event.event_type == "orchestrator.task.assigned"));
    assert!(result
        .events
        .iter()
        .any(|event| event.event_type == "icp.segment.promoted"));
    assert!(result
        .events
        .iter()
        .any(|event| event.event_type == "outbound.send.requested"));
    assert!(result
        .events
        .iter()
        .any(|event| event.event_type == "prd.draft.generated"));
}

#[test]
fn phase1_workflow_requires_human_when_icp_sample_too_small() {
    let engine = Phase1AgentEngine;
    let workflow = Phase1WorkflowInput {
        base_envelope: base_envelope(),
        orchestrator: OrchestratorInput {
            objective: Objective {
                name: "Validate niche segment".to_string(),
                target_metric: "meeting_rate".to_string(),
                target_value: "0.2".to_string(),
                due_date: None,
                owner: "gtm_owner".to_string(),
            },
            current_state: CurrentState::default(),
            resource_limits: ResourceLimits {
                daily_email_cap: 100,
                budget_cap_usd: 5_000,
                human_review_capacity: 1,
            },
        },
        icp_scout: IcpScoutInput {
            accounts: vec![AccountSignal {
                entity_id: Uuid::new_v4(),
                company_size: 40,
                industry: "Fintech".to_string(),
                region: "US".to_string(),
                product_events_14d: 4,
                support_tickets_30d: 1,
                won_deals_12m: 1,
                lost_deals_12m: 0,
                churned: false,
                activation_days: 9,
                ltv_usd: 3_500.0,
            }],
            current_segment_ids: Vec::new(),
            min_sample_size: 3,
        },
        outbound_sdr: OutboundSdrInput {
            segment_manifest: Vec::new(),
            message_bundle: MessageBundle {
                segment_id: "tier_a_high_fit".to_string(),
                variants: vec![MessageVariant {
                    template_id: "safe_email_v1".to_string(),
                    subject: "Quick question".to_string(),
                    body: "Short low-risk copy.".to_string(),
                    claim_risk: ClaimRisk::Low,
                }],
            },
            sequence_policy: SequencePolicy {
                max_touches: 1,
                cadence_days: 2,
                stop_conditions: vec!["positive_reply".to_string()],
            },
            channel_policy: ChannelPolicy {
                email_enabled: true,
                linkedin_ads_enabled: false,
                linkedin_dm_enabled: false,
            },
        },
        feedback_prd: FeedbackPrdInput {
            feedback_items: vec![FeedbackItem {
                feedback_id: Uuid::new_v4(),
                source: FeedbackSource::Onboarding,
                segment_id: Some("tier_a_high_fit".to_string()),
                text: "Onboarding is confusing for admins".to_string(),
                created_at: Utc::now(),
                evidence_ref: Some("onboarding:88".to_string()),
            }],
            product_context: ProductContext {
                roadmap_refs: Vec::new(),
                constraints: vec!["limited eng bandwidth".to_string()],
                architecture_notes: Vec::new(),
            },
            business_context: BusinessContext {
                revenue_goal: "reduce churn".to_string(),
                strategic_themes: vec!["activation".to_string()],
            },
            cluster_policy: ClusterPolicy {
                min_cluster_size: 1,
                recency_weight: 0.5,
            },
        },
    };

    let result = engine
        .run_workflow(workflow)
        .expect("phase1 workflow should run");
    assert_eq!(result.icp_scout.status, TaskStatus::NeedsHuman);
    assert!(!result.icp_scout.errors.is_empty());
}
