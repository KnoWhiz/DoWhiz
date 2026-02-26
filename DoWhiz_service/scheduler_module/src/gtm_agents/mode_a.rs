use std::collections::HashMap;

use chrono::{Duration, Utc};
use serde::Serialize;
use uuid::Uuid;

use super::contracts::{
    AgentId, AgentTaskEnvelope, AgentTaskResult, ApprovalRequest, ClaimRisk, EventEnvelope,
    GtmChannel, HubspotCommunicationDraft, HubspotTaskDraft, LinkedinManualSendTask,
    ManualDispatchStatus, ModeAOutboundDispatchInput, ModeAOutboundDispatchOutput,
    OutboundSdrInput, SendRequest, SubjectType, TaskStatus, GTM_SCHEMA_VERSION,
};

#[derive(Debug, thiserror::Error)]
pub enum ModeAAgentError {
    #[error("event payload serialization failed: {0}")]
    EventSerialization(#[from] serde_json::Error),
    #[error("mode A dispatch requires an outbound sequence draft")]
    MissingSequenceDraft,
    #[error("mode A dispatch requires at least one outbound message variant")]
    MissingMessageVariant,
    #[error("mode A dispatch supports only linkedin_dm channel, got {0}")]
    UnsupportedChannel(String),
}

#[derive(Debug, Clone)]
pub struct ModeAWorkflowInput {
    pub base_envelope: AgentTaskEnvelope,
    pub dispatch: ModeAOutboundDispatchInput,
}

#[derive(Debug, Clone)]
pub struct ModeAWorkflowResult {
    pub dispatch: AgentTaskResult<ModeAOutboundDispatchOutput>,
    pub events: Vec<EventEnvelope>,
}

#[derive(Debug, Default, Clone)]
pub struct ModeAAgentEngine;

impl ModeAAgentEngine {
    pub fn run_workflow(
        &self,
        input: ModeAWorkflowInput,
    ) -> Result<ModeAWorkflowResult, ModeAAgentError> {
        let dispatch = self.run_linkedin_dispatch(
            input.base_envelope.with_agent(AgentId::RachelOutboundSdr),
            input.dispatch,
        )?;

        Ok(ModeAWorkflowResult {
            events: dispatch.emitted_events.clone(),
            dispatch,
        })
    }

    pub fn run_linkedin_dispatch(
        &self,
        envelope: AgentTaskEnvelope,
        input: ModeAOutboundDispatchInput,
    ) -> Result<AgentTaskResult<ModeAOutboundDispatchOutput>, ModeAAgentError> {
        let sequence = input
            .outbound_output
            .sequence_draft
            .as_ref()
            .ok_or(ModeAAgentError::MissingSequenceDraft)?;
        if sequence.channel != GtmChannel::LinkedinDm {
            return Err(ModeAAgentError::UnsupportedChannel(
                channel_label(sequence.channel).to_string(),
            ));
        }

        let selected_variant = input
            .outbound_input
            .message_bundle
            .variants
            .iter()
            .min_by_key(|variant| claim_risk_rank(variant.claim_risk))
            .cloned()
            .ok_or(ModeAAgentError::MissingMessageVariant)?;

        let reviewer_group = normalize_non_empty(input.reviewer_group, "gtm_ops");
        let assignee_team = normalize_non_empty(input.assignee_team, "sdr_team");

        let approval_required = input.approval_required
            || envelope.policy_pack.human_approval_required
            || selected_variant.claim_risk != ClaimRisk::Low;

        let mut send_requests = input.outbound_output.send_requests.clone();
        if send_requests.is_empty() {
            send_requests =
                synthesize_send_requests(&input.outbound_input, &selected_variant.template_id);
        }

        let contacts_by_id = input
            .outbound_input
            .segment_manifest
            .iter()
            .map(|contact| (contact.recipient_id, contact))
            .collect::<HashMap<_, _>>();

        let mut manual_send_tasks = Vec::new();
        let mut hubspot_task_drafts = Vec::new();
        let mut hubspot_communication_drafts = Vec::new();
        let mut errors = Vec::new();
        let mut events = Vec::new();

        for (index, send_request) in send_requests.iter().enumerate() {
            let Some(contact) = contacts_by_id.get(&send_request.recipient_id) else {
                errors.push(format!(
                    "recipient {} not found in segment manifest",
                    send_request.recipient_id
                ));
                continue;
            };

            let manual_task_id = format!("mode_a:{}:{}", envelope.task_id.simple(), index + 1);
            let dispatch_status = if approval_required {
                ManualDispatchStatus::PendingApproval
            } else {
                ManualDispatchStatus::ReadyForRep
            };
            let recipient_name = contact.first_name.clone();
            let instructions = build_linkedin_instructions(contact, &selected_variant.template_id);
            let manual_task = LinkedinManualSendTask {
                manual_task_id: manual_task_id.clone(),
                recipient_id: contact.recipient_id,
                account_id: contact.account_id,
                recipient_email: contact.email.clone(),
                recipient_name,
                company_name: contact.company_name.clone(),
                channel: GtmChannel::LinkedinDm,
                template_id: selected_variant.template_id.clone(),
                send_at: send_request.send_at,
                assignee_team: assignee_team.clone(),
                status: dispatch_status,
                instructions,
            };

            let task_subject = format!(
                "[LinkedIn Manual Send] {}",
                contact
                    .company_name
                    .as_deref()
                    .unwrap_or(contact.email.as_str())
            );
            let task_body = format!(
                "Use template `{}` and send a LinkedIn DM manually.\nRecipient: {}\nSegment: {}\nMessage preview subject: {}",
                selected_variant.template_id,
                contact.email,
                input.outbound_input.message_bundle.segment_id,
                selected_variant.subject
            );
            let hubspot_task = HubspotTaskDraft {
                external_id: format!("{}:hubspot_task", manual_task_id),
                subject: task_subject,
                body: task_body,
                due_at: send_request.send_at,
                contact_email: contact.email.clone(),
                owner_team: assignee_team.clone(),
            };
            let hubspot_communication = HubspotCommunicationDraft {
                external_id: format!("{}:hubspot_comm", manual_task_id),
                channel: GtmChannel::LinkedinDm,
                contact_email: contact.email.clone(),
                scheduled_at: send_request.send_at,
                summary: format!(
                    "Planned manual LinkedIn outreach using template {}",
                    selected_variant.template_id
                ),
            };

            events.push(self.make_event(
                &envelope,
                AgentId::RachelOutboundSdr,
                "mode_a.manual_send.task_created",
                SubjectType::Contact,
                contact.recipient_id,
                &manual_task,
            )?);
            events.push(self.make_event(
                &envelope,
                AgentId::RachelOutboundSdr,
                "mode_a.hubspot.task.drafted",
                SubjectType::Contact,
                contact.recipient_id,
                &hubspot_task,
            )?);
            events.push(self.make_event(
                &envelope,
                AgentId::RachelOutboundSdr,
                "mode_a.hubspot.communication.drafted",
                SubjectType::Contact,
                contact.recipient_id,
                &hubspot_communication,
            )?);

            manual_send_tasks.push(manual_task);
            hubspot_task_drafts.push(hubspot_task);
            hubspot_communication_drafts.push(hubspot_communication);
        }

        let mut approval_queue = Vec::new();
        if approval_required && !manual_send_tasks.is_empty() {
            let approval = ApprovalRequest {
                reason:
                    "Manual LinkedIn send tasks are queued and require manager approval before reps execute"
                        .to_string(),
                risk_level: if selected_variant.claim_risk == ClaimRisk::High {
                    "high".to_string()
                } else {
                    "medium".to_string()
                },
                reviewer_group,
            };
            events.push(self.make_event(
                &envelope,
                AgentId::RachelOutboundSdr,
                "mode_a.approval.queued",
                SubjectType::Campaign,
                envelope.objective_id,
                &approval,
            )?);
            approval_queue.push(approval);
        }

        if manual_send_tasks.is_empty() && errors.is_empty() {
            errors.push("no manual LinkedIn tasks could be generated".to_string());
        }

        let output = ModeAOutboundDispatchOutput {
            approval_queue,
            manual_send_tasks,
            hubspot_task_drafts,
            hubspot_communication_drafts,
        };

        let status = if output.manual_send_tasks.is_empty() {
            TaskStatus::Failed
        } else if !output.approval_queue.is_empty() {
            TaskStatus::NeedsHuman
        } else {
            TaskStatus::Succeeded
        };

        Ok(AgentTaskResult {
            task_id: envelope.task_id,
            status,
            schema_version: GTM_SCHEMA_VERSION.to_string(),
            output_payload: output,
            emitted_events: events,
            confidence: match status {
                TaskStatus::Failed => 0.3,
                TaskStatus::NeedsHuman => 0.68,
                _ => 0.84,
            },
            evidence_refs: vec![format!(
                "segment:{}",
                input.outbound_input.message_bundle.segment_id
            )],
            next_action: match status {
                TaskStatus::Failed => {
                    "Fix segment manifest and outbound sequence before creating manual tasks"
                        .to_string()
                }
                TaskStatus::NeedsHuman => {
                    "Complete manager approval, then reps execute LinkedIn sends and mark completion"
                        .to_string()
                }
                _ => "Create HubSpot tasks and start manual LinkedIn execution".to_string(),
            },
            errors,
        })
    }

    fn make_event<T: Serialize>(
        &self,
        envelope: &AgentTaskEnvelope,
        producer: AgentId,
        event_type: &str,
        subject_type: SubjectType,
        subject_id: Uuid,
        payload: &T,
    ) -> Result<EventEnvelope, ModeAAgentError> {
        Ok(EventEnvelope {
            event_id: Uuid::new_v4(),
            event_type: event_type.to_string(),
            occurred_at: Utc::now(),
            producer,
            tenant_id: envelope.tenant_id,
            subject_type,
            subject_id,
            schema_version: GTM_SCHEMA_VERSION.to_string(),
            trace_id: envelope.trace_id,
            idempotency_key: format!(
                "{}:{}:{}",
                envelope.idempotency_key,
                producer.as_str(),
                event_type
            ),
            payload: serde_json::to_value(payload)?,
        })
    }
}

fn claim_risk_rank(risk: ClaimRisk) -> u8 {
    match risk {
        ClaimRisk::Low => 0,
        ClaimRisk::Medium => 1,
        ClaimRisk::High => 2,
    }
}

fn synthesize_send_requests(input: &OutboundSdrInput, template_id: &str) -> Vec<SendRequest> {
    input
        .segment_manifest
        .iter()
        .enumerate()
        .map(|(index, contact)| SendRequest {
            recipient_id: contact.recipient_id,
            template_id: template_id.to_string(),
            send_at: Utc::now() + Duration::minutes((index as i64) * 5),
        })
        .collect()
}

fn normalize_non_empty(value: String, fallback: &str) -> String {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        fallback.to_string()
    } else {
        trimmed.to_string()
    }
}

fn build_linkedin_instructions(
    contact: &super::contracts::SegmentContact,
    template_id: &str,
) -> String {
    let recipient = contact
        .first_name
        .as_deref()
        .unwrap_or(contact.email.as_str());
    let company = contact.company_name.as_deref().unwrap_or("target account");
    format!(
        "Find {} at {} on LinkedIn, send DM using template `{}`, then log outcome in HubSpot.",
        recipient, company, template_id
    )
}

fn channel_label(channel: GtmChannel) -> &'static str {
    match channel {
        GtmChannel::Email => "email",
        GtmChannel::LinkedinAds => "linkedin_ads",
        GtmChannel::HubspotWorkflow => "hubspot_workflow",
        GtmChannel::LinkedinDm => "linkedin_dm",
    }
}

#[cfg(test)]
mod tests {
    use super::super::contracts::{
        AgentId, AgentTaskEnvelope, ChannelPolicy, MessageBundle, MessageVariant, OutboundSdrInput,
        OutboundSdrOutput, SegmentContact, SequenceDraft, SequencePolicy, SequenceTouch,
        TaskPriority,
    };
    use super::*;

    fn base_envelope() -> AgentTaskEnvelope {
        let mut envelope = AgentTaskEnvelope::new(AgentId::RachelOrchestrator);
        envelope.priority = TaskPriority::High;
        envelope.policy_pack.allowed_channels =
            vec![GtmChannel::LinkedinDm, GtmChannel::HubspotWorkflow];
        envelope
    }

    fn outbound_input(risk: ClaimRisk) -> OutboundSdrInput {
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
                    claim_risk: risk,
                }],
            },
            sequence_policy: SequencePolicy {
                max_touches: 3,
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

    fn outbound_output(input: &OutboundSdrInput, channel: GtmChannel) -> OutboundSdrOutput {
        OutboundSdrOutput {
            sequence_draft: Some(SequenceDraft {
                sequence_id: format!("seq-{}", Uuid::new_v4().simple()),
                touches: vec![SequenceTouch {
                    touch_number: 1,
                    offset_days: 0,
                    channel,
                    template_id: "linkedin_dm_safe_v1".to_string(),
                }],
                channel,
            }),
            personalization_fields_used: vec!["first_name".to_string()],
            send_requests: input
                .segment_manifest
                .iter()
                .enumerate()
                .map(|(idx, contact)| SendRequest {
                    recipient_id: contact.recipient_id,
                    template_id: "linkedin_dm_safe_v1".to_string(),
                    send_at: Utc::now() + Duration::minutes((idx as i64) * 5),
                })
                .collect(),
            reply_classifications: Vec::new(),
            handoffs: Vec::new(),
        }
    }

    #[test]
    fn mode_a_dispatch_creates_approval_and_hubspot_drafts() {
        let engine = ModeAAgentEngine;
        let input = outbound_input(ClaimRisk::Low);
        let output = outbound_output(&input, GtmChannel::LinkedinDm);

        let result = engine
            .run_linkedin_dispatch(
                base_envelope(),
                ModeAOutboundDispatchInput {
                    outbound_input: input,
                    outbound_output: output,
                    assignee_team: "sdr_team".to_string(),
                    reviewer_group: "gtm_ops".to_string(),
                    approval_required: true,
                },
            )
            .unwrap();

        assert_eq!(result.status, TaskStatus::NeedsHuman);
        assert_eq!(result.output_payload.manual_send_tasks.len(), 2);
        assert_eq!(result.output_payload.hubspot_task_drafts.len(), 2);
        assert_eq!(result.output_payload.hubspot_communication_drafts.len(), 2);
        assert_eq!(result.output_payload.approval_queue.len(), 1);
        assert!(result
            .output_payload
            .manual_send_tasks
            .iter()
            .all(|task| task.status == ManualDispatchStatus::PendingApproval));
        assert!(result
            .emitted_events
            .iter()
            .any(|event| event.event_type == "mode_a.approval.queued"));
    }

    #[test]
    fn mode_a_dispatch_rejects_non_linkedin_channel() {
        let engine = ModeAAgentEngine;
        let input = outbound_input(ClaimRisk::Low);
        let output = outbound_output(&input, GtmChannel::Email);

        let error = engine
            .run_linkedin_dispatch(
                base_envelope(),
                ModeAOutboundDispatchInput {
                    outbound_input: input,
                    outbound_output: output,
                    assignee_team: "sdr_team".to_string(),
                    reviewer_group: "gtm_ops".to_string(),
                    approval_required: true,
                },
            )
            .unwrap_err();

        assert!(matches!(error, ModeAAgentError::UnsupportedChannel(_)));
    }

    #[test]
    fn mode_a_dispatch_can_prepare_ready_for_rep_tasks() {
        let engine = ModeAAgentEngine;
        let input = outbound_input(ClaimRisk::Low);
        let mut output = outbound_output(&input, GtmChannel::LinkedinDm);
        output.send_requests.clear();

        let result = engine
            .run_linkedin_dispatch(
                base_envelope(),
                ModeAOutboundDispatchInput {
                    outbound_input: input,
                    outbound_output: output,
                    assignee_team: "sdr_team".to_string(),
                    reviewer_group: "gtm_ops".to_string(),
                    approval_required: false,
                },
            )
            .unwrap();

        assert_eq!(result.status, TaskStatus::Succeeded);
        assert_eq!(result.output_payload.approval_queue.len(), 0);
        assert_eq!(result.output_payload.manual_send_tasks.len(), 2);
        assert!(result
            .output_payload
            .manual_send_tasks
            .iter()
            .all(|task| task.status == ManualDispatchStatus::ReadyForRep));
    }
}
