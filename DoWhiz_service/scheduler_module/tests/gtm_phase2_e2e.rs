use chrono::Utc;
use scheduler_module::gtm_agents::{
    AgentId, AgentTaskEnvelope, AssetChannel, ContentInput, FunnelStage, InsightCluster,
    MessageMap, Phase2AgentEngine, Phase2WorkflowInput, PolicyPack, PositioningBundle,
    PositioningInput, PrdDraft, SegmentDefinition, TaskPriority, TaskStatus,
};

fn base_envelope() -> AgentTaskEnvelope {
    let mut envelope = AgentTaskEnvelope::new(AgentId::RachelOrchestrator);
    envelope.priority = TaskPriority::High;
    envelope.policy_pack = PolicyPack::default();
    envelope.input_refs = vec!["warehouse://stable_contracts_v1".to_string()];
    envelope
}

fn positioning_input() -> PositioningInput {
    PositioningInput {
        segment_definitions: vec![
            SegmentDefinition {
                segment_id: "tier_a_high_fit".to_string(),
                rule_dsl: "icp_tier == 'A'".to_string(),
                expected_lift: 1.35,
                confidence: 0.84,
            },
            SegmentDefinition {
                segment_id: "industry_saas_high_fit".to_string(),
                rule_dsl: "industry == 'saas'".to_string(),
                expected_lift: 1.22,
                confidence: 0.78,
            },
        ],
        insight_clusters: vec![
            InsightCluster {
                cluster_id: "cluster_integration_gap".to_string(),
                theme: "integration gap".to_string(),
                frequency: 6,
                affected_segments: vec!["tier_a_high_fit".to_string()],
                evidence_refs: vec!["ticket:991".to_string()],
            },
            InsightCluster {
                cluster_id: "cluster_onboarding_friction".to_string(),
                theme: "onboarding friction".to_string(),
                frequency: 4,
                affected_segments: vec!["tier_a_high_fit".to_string()],
                evidence_refs: vec!["onboarding:122".to_string()],
            },
        ],
        prd_drafts: vec![PrdDraft {
            prd_id: "prd_integration_gap".to_string(),
            problem: "integration reliability impacts activation".to_string(),
            users: vec!["tier_a_high_fit".to_string()],
            success_metrics: vec!["activation_rate_day_14".to_string()],
            scope: vec!["stabilize hubspot sync pipeline".to_string()],
            risks: vec!["cross-team dependency".to_string()],
        }],
        strategic_themes: vec!["activation".to_string(), "expansion".to_string()],
        data_contract_version: "1.0".to_string(),
    }
}

#[test]
fn phase2_workflow_generates_positioning_and_publishable_assets() {
    let engine = Phase2AgentEngine;
    let workflow = Phase2WorkflowInput {
        base_envelope: base_envelope(),
        data_contracts_stable: true,
        positioning: positioning_input(),
        content: ContentInput {
            positioning_bundle: PositioningBundle {
                bundle_id: "placeholder".to_string(),
                message_maps: vec![MessageMap {
                    segment_id: "placeholder".to_string(),
                    value_proposition: "placeholder".to_string(),
                    pains: vec!["placeholder".to_string()],
                    proof_points: vec!["placeholder".to_string()],
                    objection_handling: vec!["placeholder".to_string()],
                    funnel_stage: FunnelStage::Awareness,
                }],
                claim_safe_list: vec!["placeholder".to_string()],
                generated_at: Utc::now(),
            },
            channels: vec![
                AssetChannel::Email,
                AssetChannel::LandingPage,
                AssetChannel::SalesOnePager,
            ],
            max_assets_per_channel: 2,
            requires_human_review: false,
        },
    };

    let result = engine.run_workflow(workflow).expect("phase2 should run");
    assert_eq!(result.positioning.status, TaskStatus::Succeeded);
    assert_eq!(result.content.status, TaskStatus::Succeeded);
    assert!(result.content.output_payload.publish_ready);
    assert!(result.content.output_payload.assets.len() >= 3);
    assert!(result
        .events
        .iter()
        .any(|event| event.event_type == "positioning.bundle.published"));
    assert!(result
        .events
        .iter()
        .any(|event| event.event_type == "content.asset.published"));
}

#[test]
fn phase2_content_requests_review_when_explicitly_required() {
    let engine = Phase2AgentEngine;
    let workflow = Phase2WorkflowInput {
        base_envelope: base_envelope(),
        data_contracts_stable: true,
        positioning: positioning_input(),
        content: ContentInput {
            positioning_bundle: PositioningBundle {
                bundle_id: "placeholder".to_string(),
                message_maps: Vec::new(),
                claim_safe_list: Vec::new(),
                generated_at: Utc::now(),
            },
            channels: vec![AssetChannel::LinkedinAd],
            max_assets_per_channel: 1,
            requires_human_review: true,
        },
    };

    let result = engine.run_workflow(workflow).expect("phase2 should run");
    assert_eq!(result.content.status, TaskStatus::NeedsHuman);
    assert!(result
        .content
        .emitted_events
        .iter()
        .any(|event| event.event_type == "approval.requested"));
}
