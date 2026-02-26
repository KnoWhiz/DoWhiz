use scheduler_module::gtm_agents::{
    AgentId, AgentTaskEnvelope, CampaignPerformance, ExperimentInput, FeatureAdoptionSignal,
    OnboardingInput, Phase3AgentEngine, Phase3WorkflowInput, PolicyPack, TaskPriority, TaskStatus,
};
use uuid::Uuid;

fn base_envelope() -> AgentTaskEnvelope {
    let mut envelope = AgentTaskEnvelope::new(AgentId::RachelOrchestrator);
    envelope.priority = TaskPriority::High;
    envelope.policy_pack = PolicyPack::default();
    envelope.input_refs = vec!["warehouse://reliable_metrics_weekly".to_string()];
    envelope
}

fn onboarding_input() -> OnboardingInput {
    OnboardingInput {
        customer_id: Uuid::new_v4(),
        account_name: "Northstar AI".to_string(),
        segment_id: "tier_a_high_fit".to_string(),
        customer_goals: vec![
            "shorten onboarding cycle".to_string(),
            "ship first campaign this month".to_string(),
        ],
        known_blockers: vec![
            "email domain warmup incomplete".to_string(),
            "lead source mapping unclear".to_string(),
        ],
        current_activation_rate: 0.48,
        target_activation_rate: 0.72,
        handoff_summary: Some("handoff from outbound SDR with high purchase intent".to_string()),
    }
}

fn experiment_input() -> ExperimentInput {
    ExperimentInput {
        experiment_name: "guided onboarding playbook".to_string(),
        primary_metric: "activation_rate_day_14".to_string(),
        baseline_value: 0.41,
        observed_value: 0.50,
        sample_size: 410,
        min_sample_size: 250,
        confidence_estimate: 0.82,
        segment_ids: vec!["tier_a_high_fit".to_string(), "fast_activation".to_string()],
        campaign_results: vec![
            CampaignPerformance {
                campaign_id: "cmp_100".to_string(),
                segment_id: "tier_a_high_fit".to_string(),
                spend_usd: 5200.0,
                impressions: 138_000,
                clicks: 3520,
                meetings: 220,
                sqls: 72,
            },
            CampaignPerformance {
                campaign_id: "cmp_101".to_string(),
                segment_id: "fast_activation".to_string(),
                spend_usd: 2100.0,
                impressions: 62_000,
                clicks: 1510,
                meetings: 88,
                sqls: 25,
            },
        ],
        adoption_signals: vec![
            FeatureAdoptionSignal {
                feature_name: "workflow_builder".to_string(),
                before_rate: 0.34,
                after_rate: 0.52,
            },
            FeatureAdoptionSignal {
                feature_name: "sequence_analytics".to_string(),
                before_rate: 0.21,
                after_rate: 0.33,
            },
        ],
    }
}

#[test]
fn phase3_workflow_produces_onboarding_and_experiment_outputs() {
    let engine = Phase3AgentEngine;
    let workflow = Phase3WorkflowInput {
        base_envelope: base_envelope(),
        metrics_reliable: true,
        onboarding: onboarding_input(),
        experiment: experiment_input(),
    };

    let result = engine.run_workflow(workflow).expect("phase3 should run");
    assert_eq!(result.onboarding.status, TaskStatus::NeedsHuman);
    assert_eq!(result.experiment.status, TaskStatus::Succeeded);
    assert!(!result.onboarding.output_payload.onboarding_plan.is_empty());
    assert!(!result.experiment.output_payload.recommendations.is_empty());
    assert!(result
        .events
        .iter()
        .any(|event| event.event_type == "onboarding.plan.started"));
    assert!(result
        .events
        .iter()
        .any(|event| event.event_type == "experiment.result.published"));
}

#[test]
fn phase3_experiment_requests_human_review_when_signal_is_weak() {
    let engine = Phase3AgentEngine;
    let mut weak = experiment_input();
    weak.sample_size = 80;
    weak.min_sample_size = 250;
    weak.confidence_estimate = 0.55;

    let workflow = Phase3WorkflowInput {
        base_envelope: base_envelope(),
        metrics_reliable: true,
        onboarding: onboarding_input(),
        experiment: weak,
    };

    let result = engine.run_workflow(workflow).expect("phase3 should run");
    assert_eq!(result.experiment.status, TaskStatus::NeedsHuman);
    assert!(result
        .experiment
        .emitted_events
        .iter()
        .any(|event| event.event_type == "approval.requested"));
}
