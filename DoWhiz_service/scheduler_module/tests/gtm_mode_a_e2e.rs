use scheduler_module::gtm_agents::{
    AgentId, AgentTaskEnvelope, ChannelPolicy, ClaimRisk, GtmChannel, ManualDispatchStatus,
    MessageBundle, MessageVariant, ModeAAgentEngine, ModeAOutboundDispatchInput,
    ModeAWorkflowInput, OutboundSdrInput, Phase1AgentEngine, PolicyPack, SegmentContact,
    SequencePolicy, TaskPriority, TaskStatus,
};
use uuid::Uuid;

fn base_envelope() -> AgentTaskEnvelope {
    let mut envelope = AgentTaskEnvelope::new(AgentId::RachelOrchestrator);
    envelope.priority = TaskPriority::High;
    envelope.policy_pack = PolicyPack::default();
    envelope.policy_pack.allowed_channels =
        vec![GtmChannel::LinkedinDm, GtmChannel::HubspotWorkflow];
    envelope
}

fn outbound_input() -> OutboundSdrInput {
    OutboundSdrInput {
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
                template_id: "linkedin_dm_safe_v1".to_string(),
                subject: "Idea for faster GTM execution".to_string(),
                body: "Sharing a short playbook.".to_string(),
                claim_risk: ClaimRisk::Low,
            }],
        },
        sequence_policy: SequencePolicy {
            max_touches: 2,
            cadence_days: 2,
            stop_conditions: vec!["positive_reply".to_string()],
        },
        channel_policy: ChannelPolicy {
            email_enabled: false,
            linkedin_ads_enabled: false,
            linkedin_dm_enabled: true,
        },
    }
}

#[test]
fn mode_a_workflow_builds_approval_queue_and_hubspot_drafts_from_phase1_outbound() {
    let phase1 = Phase1AgentEngine;
    let mode_a = ModeAAgentEngine;
    let envelope = base_envelope();
    let outbound_in = outbound_input();

    let outbound_result = phase1
        .run_outbound_sdr(
            envelope.with_agent(AgentId::RachelOutboundSdr),
            outbound_in.clone(),
        )
        .expect("phase1 outbound should run");
    assert_eq!(outbound_result.status, TaskStatus::Succeeded);

    let result = mode_a
        .run_workflow(ModeAWorkflowInput {
            base_envelope: envelope,
            dispatch: ModeAOutboundDispatchInput {
                outbound_input: outbound_in,
                outbound_output: outbound_result.output_payload,
                assignee_team: "sdr_team".to_string(),
                reviewer_group: "gtm_ops".to_string(),
                approval_required: true,
            },
        })
        .expect("mode a workflow should run");

    assert_eq!(result.dispatch.status, TaskStatus::NeedsHuman);
    assert_eq!(result.dispatch.output_payload.approval_queue.len(), 1);
    assert_eq!(result.dispatch.output_payload.manual_send_tasks.len(), 2);
    assert_eq!(result.dispatch.output_payload.hubspot_task_drafts.len(), 2);
    assert_eq!(
        result
            .dispatch
            .output_payload
            .hubspot_communication_drafts
            .len(),
        2
    );
    assert!(result
        .dispatch
        .output_payload
        .manual_send_tasks
        .iter()
        .all(|task| task.status == ManualDispatchStatus::PendingApproval));
    assert!(result
        .events
        .iter()
        .any(|event| event.event_type == "mode_a.approval.queued"));
}

#[test]
fn mode_a_workflow_can_prepare_ready_for_rep_tasks() {
    let phase1 = Phase1AgentEngine;
    let mode_a = ModeAAgentEngine;
    let envelope = base_envelope();
    let outbound_in = outbound_input();

    let outbound_result = phase1
        .run_outbound_sdr(
            envelope.with_agent(AgentId::RachelOutboundSdr),
            outbound_in.clone(),
        )
        .expect("phase1 outbound should run");
    assert_eq!(outbound_result.status, TaskStatus::Succeeded);

    let result = mode_a
        .run_workflow(ModeAWorkflowInput {
            base_envelope: envelope,
            dispatch: ModeAOutboundDispatchInput {
                outbound_input: outbound_in,
                outbound_output: outbound_result.output_payload,
                assignee_team: "sdr_team".to_string(),
                reviewer_group: "gtm_ops".to_string(),
                approval_required: false,
            },
        })
        .expect("mode a workflow should run");

    assert_eq!(result.dispatch.status, TaskStatus::Succeeded);
    assert!(result.dispatch.output_payload.approval_queue.is_empty());
    assert_eq!(result.dispatch.output_payload.manual_send_tasks.len(), 2);
    assert!(result
        .dispatch
        .output_payload
        .manual_send_tasks
        .iter()
        .all(|task| task.status == ManualDispatchStatus::ReadyForRep));
}
