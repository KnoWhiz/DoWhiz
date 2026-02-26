use std::cmp::Reverse;
use std::collections::{BTreeMap, HashMap, HashSet};

use chrono::{Duration, Utc};
use serde::Serialize;
use uuid::Uuid;

use super::contracts::{
    AccountSignal, AgentId, AgentTaskEnvelope, AgentTaskResult, ApprovalRequest, ClaimRisk,
    DriftReport, EntityType, EventEnvelope, FeedbackPrdOutput, GtmChannel, IcpScore,
    IcpScoutOutput, IcpTier, InsightCluster, JobStory, OrchestratorOutput, OutboundSdrInput,
    OutboundSdrOutput, PrdDraft, PriorityScore, SegmentDefinition, SendRequest, SequenceDraft,
    SequenceTouch, SubjectType, TaskAssignment, TaskStatus, WorkflowState, GTM_SCHEMA_VERSION,
};
use super::contracts::{FeedbackPrdInput, IcpScoutInput, OrchestratorInput};

#[derive(Debug, thiserror::Error)]
pub enum GtmAgentError {
    #[error("event payload serialization failed: {0}")]
    EventSerialization(#[from] serde_json::Error),
}

#[derive(Debug, Clone)]
pub struct Phase1WorkflowInput {
    pub base_envelope: AgentTaskEnvelope,
    pub orchestrator: OrchestratorInput,
    pub icp_scout: IcpScoutInput,
    pub outbound_sdr: OutboundSdrInput,
    pub feedback_prd: FeedbackPrdInput,
}

#[derive(Debug, Clone)]
pub struct Phase1WorkflowResult {
    pub orchestrator: AgentTaskResult<OrchestratorOutput>,
    pub icp_scout: AgentTaskResult<IcpScoutOutput>,
    pub outbound_sdr: AgentTaskResult<OutboundSdrOutput>,
    pub feedback_prd: AgentTaskResult<FeedbackPrdOutput>,
    pub events: Vec<EventEnvelope>,
}

#[derive(Debug, Default, Clone)]
pub struct Phase1AgentEngine;

impl Phase1AgentEngine {
    pub fn run_workflow(
        &self,
        input: Phase1WorkflowInput,
    ) -> Result<Phase1WorkflowResult, GtmAgentError> {
        let orchestrator = self.run_orchestrator(
            input.base_envelope.with_agent(AgentId::RachelOrchestrator),
            input.orchestrator,
        )?;
        let icp_scout = self.run_icp_scout(
            input.base_envelope.with_agent(AgentId::RachelIcpScout),
            input.icp_scout,
        )?;
        let outbound_sdr = self.run_outbound_sdr(
            input.base_envelope.with_agent(AgentId::RachelOutboundSdr),
            input.outbound_sdr,
        )?;
        let feedback_prd = self.run_feedback_prd(
            input
                .base_envelope
                .with_agent(AgentId::RachelFeedbackPrdSynthesizer),
            input.feedback_prd,
        )?;

        let mut events = Vec::new();
        events.extend(orchestrator.emitted_events.clone());
        events.extend(icp_scout.emitted_events.clone());
        events.extend(outbound_sdr.emitted_events.clone());
        events.extend(feedback_prd.emitted_events.clone());

        Ok(Phase1WorkflowResult {
            orchestrator,
            icp_scout,
            outbound_sdr,
            feedback_prd,
            events,
        })
    }

    pub fn run_orchestrator(
        &self,
        envelope: AgentTaskEnvelope,
        input: OrchestratorInput,
    ) -> Result<AgentTaskResult<OrchestratorOutput>, GtmAgentError> {
        let fallback_deadline = envelope.requested_at + Duration::hours(24);
        let deadline = envelope.deadline_at.unwrap_or(fallback_deadline);
        let stage = if !input.current_state.blockers.is_empty() {
            "blocked"
        } else if input.current_state.open_tasks > 0 {
            "execution"
        } else {
            "planning"
        };

        let execution_plan = vec![
            super::contracts::ExecutionStep {
                step_id: "phase1_icp".to_string(),
                description: "Score accounts and promote ICP segments".to_string(),
                depends_on: Vec::new(),
            },
            super::contracts::ExecutionStep {
                step_id: "phase1_outbound".to_string(),
                description: "Draft outbound sequence for promoted segments".to_string(),
                depends_on: vec!["phase1_icp".to_string()],
            },
            super::contracts::ExecutionStep {
                step_id: "phase1_feedback_prd".to_string(),
                description: "Synthesize feedback into PRD drafts".to_string(),
                depends_on: vec!["phase1_outbound".to_string()],
            },
        ];

        let task_assignments = vec![
            TaskAssignment {
                task_type: "icp.score.refresh".to_string(),
                agent_id: AgentId::RachelIcpScout,
                deadline_at: Some(deadline - Duration::hours(16)),
                input_refs: envelope.input_refs.clone(),
            },
            TaskAssignment {
                task_type: "outbound.sequence.prepare".to_string(),
                agent_id: AgentId::RachelOutboundSdr,
                deadline_at: Some(deadline - Duration::hours(8)),
                input_refs: envelope.input_refs.clone(),
            },
            TaskAssignment {
                task_type: "feedback.prd.synthesize".to_string(),
                agent_id: AgentId::RachelFeedbackPrdSynthesizer,
                deadline_at: Some(deadline - Duration::hours(2)),
                input_refs: envelope.input_refs.clone(),
            },
        ];

        let mut approval_requests = Vec::new();
        if envelope.policy_pack.human_approval_required {
            approval_requests.push(ApprovalRequest {
                reason: "Policy pack enforces human gate before external sends".to_string(),
                risk_level: "medium".to_string(),
                reviewer_group: "gtm_ops".to_string(),
            });
        }
        if input.resource_limits.human_review_capacity == 0 {
            approval_requests.push(ApprovalRequest {
                reason: "No human review capacity configured".to_string(),
                risk_level: "high".to_string(),
                reviewer_group: "revops".to_string(),
            });
        }

        let workflow_state = WorkflowState {
            stage: stage.to_string(),
            progress_pct: match stage {
                "blocked" => 35,
                "execution" => 55,
                _ => 20,
            },
            eta_minutes: 90
                + input.current_state.open_tasks * 15
                + input.current_state.active_campaigns * 10,
        };
        let output = OrchestratorOutput {
            execution_plan,
            task_assignments,
            approval_requests,
            workflow_state,
        };

        let mut events = Vec::new();
        events.push(self.make_event(
            &envelope,
            AgentId::RachelOrchestrator,
            "orchestrator.workflow.updated",
            SubjectType::Objective,
            envelope.objective_id,
            &output.workflow_state,
        )?);
        for assignment in &output.task_assignments {
            events.push(self.make_event(
                &envelope,
                AgentId::RachelOrchestrator,
                "orchestrator.task.assigned",
                SubjectType::Objective,
                envelope.objective_id,
                assignment,
            )?);
        }
        for approval in &output.approval_requests {
            events.push(self.make_event(
                &envelope,
                AgentId::RachelOrchestrator,
                "approval.requested",
                SubjectType::Objective,
                envelope.objective_id,
                approval,
            )?);
        }

        let confidence = if input.current_state.blockers.is_empty() {
            0.84
        } else {
            0.68
        };

        Ok(AgentTaskResult {
            task_id: envelope.task_id,
            status: TaskStatus::Succeeded,
            schema_version: GTM_SCHEMA_VERSION.to_string(),
            output_payload: output,
            emitted_events: events,
            confidence,
            evidence_refs: envelope.input_refs.clone(),
            next_action: "Run ICP Scout assignments and publish promoted segments".to_string(),
            errors: Vec::new(),
        })
    }

    pub fn run_icp_scout(
        &self,
        envelope: AgentTaskEnvelope,
        input: IcpScoutInput,
    ) -> Result<AgentTaskResult<IcpScoutOutput>, GtmAgentError> {
        let mut icp_scores = Vec::with_capacity(input.accounts.len());
        let mut industry_hits: HashMap<String, u32> = HashMap::new();

        for account in &input.accounts {
            let (score, top_drivers) = score_account(account);
            let tier = tier_for_score(score);
            if matches!(tier, IcpTier::A | IcpTier::B) {
                *industry_hits
                    .entry(account.industry.to_ascii_lowercase())
                    .or_insert(0) += 1;
            }
            icp_scores.push(IcpScore {
                entity_id: account.entity_id,
                entity_type: EntityType::Account,
                score_0_100: score,
                tier,
                top_drivers,
            });
        }

        icp_scores.sort_by_key(|score| Reverse(score.score_0_100));
        let sample_size = input.accounts.len();
        let sample_ratio = if input.min_sample_size == 0 {
            1.0
        } else {
            (sample_size as f32 / input.min_sample_size as f32).min(1.2)
        };
        let base_confidence = (0.45 + sample_ratio * 0.35).min(0.95);

        let mut segment_definitions = vec![SegmentDefinition {
            segment_id: "tier_a_high_fit".to_string(),
            rule_dsl: "icp_tier == 'A'".to_string(),
            expected_lift: 1.35,
            confidence: (base_confidence + 0.05).min(0.95),
        }];
        if let Some((industry, _)) = industry_hits.into_iter().max_by_key(|(_, count)| *count) {
            segment_definitions.push(SegmentDefinition {
                segment_id: format!("industry_{}_high_fit", slugify(&industry)),
                rule_dsl: format!("industry == '{}' AND icp_tier IN ('A','B')", industry),
                expected_lift: 1.22,
                confidence: base_confidence,
            });
        }
        segment_definitions.push(SegmentDefinition {
            segment_id: "fast_activation".to_string(),
            rule_dsl: "activation_days <= 7 AND support_tickets_30d <= 2".to_string(),
            expected_lift: 1.18,
            confidence: (base_confidence - 0.05).max(0.3),
        });

        let anti_icp_rules = vec![
            "churned == true".to_string(),
            "support_tickets_30d > 8".to_string(),
            "activation_days > 30".to_string(),
        ];

        let promoted_segment_ids: HashSet<String> = segment_definitions
            .iter()
            .map(|segment| segment.segment_id.clone())
            .collect();
        let drift_detected = !input.current_segment_ids.is_empty()
            && input
                .current_segment_ids
                .iter()
                .all(|segment| !promoted_segment_ids.contains(segment));
        let drift_report = DriftReport {
            drift_detected,
            drift_dimensions: if drift_detected {
                vec![
                    "segment_membership".to_string(),
                    "conversion_signal_weight".to_string(),
                ]
            } else {
                Vec::new()
            },
            recommended_retrain_date: Some(Utc::now() + Duration::days(30)),
        };

        let output = IcpScoutOutput {
            icp_scores,
            segment_definitions,
            anti_icp_rules,
            drift_report,
        };
        let needs_human = sample_size < input.min_sample_size;

        let mut events = Vec::new();
        events.push(self.make_event(
            &envelope,
            AgentId::RachelIcpScout,
            "icp.score.updated",
            SubjectType::Objective,
            envelope.objective_id,
            &serde_json::json!({
                "score_count": output.icp_scores.len(),
                "top_score": output.icp_scores.first().map(|score| score.score_0_100),
            }),
        )?);
        for segment in &output.segment_definitions {
            events.push(self.make_event(
                &envelope,
                AgentId::RachelIcpScout,
                "icp.segment.promoted",
                SubjectType::Objective,
                envelope.objective_id,
                segment,
            )?);
        }
        events.push(self.make_event(
            &envelope,
            AgentId::RachelIcpScout,
            "icp.anti_segment.updated",
            SubjectType::Objective,
            envelope.objective_id,
            &serde_json::json!({ "rules": output.anti_icp_rules.clone() }),
        )?);
        if output.drift_report.drift_detected {
            events.push(self.make_event(
                &envelope,
                AgentId::RachelIcpScout,
                "icp.drift.detected",
                SubjectType::Objective,
                envelope.objective_id,
                &output.drift_report,
            )?);
        }

        Ok(AgentTaskResult {
            task_id: envelope.task_id,
            status: if needs_human {
                TaskStatus::NeedsHuman
            } else {
                TaskStatus::Succeeded
            },
            schema_version: GTM_SCHEMA_VERSION.to_string(),
            output_payload: output,
            emitted_events: events,
            confidence: if needs_human {
                base_confidence.min(0.6)
            } else {
                base_confidence
            },
            evidence_refs: envelope.input_refs.clone(),
            next_action: if needs_human {
                "Collect more won/lost and activation samples before promoting ICP tiers"
                    .to_string()
            } else {
                "Publish promoted segments to outbound sequencing".to_string()
            },
            errors: if needs_human {
                vec![format!(
                    "insufficient sample size: {} < {}",
                    sample_size, input.min_sample_size
                )]
            } else {
                Vec::new()
            },
        })
    }

    pub fn run_outbound_sdr(
        &self,
        envelope: AgentTaskEnvelope,
        input: OutboundSdrInput,
    ) -> Result<AgentTaskResult<OutboundSdrOutput>, GtmAgentError> {
        let selected_channel = select_channel(&envelope, &input);
        let selected_variant = input
            .message_bundle
            .variants
            .iter()
            .min_by_key(|variant| claim_risk_rank(variant.claim_risk))
            .cloned();
        let risk_blocked = selected_variant
            .as_ref()
            .map(|variant| variant.claim_risk == ClaimRisk::High)
            .unwrap_or(false);

        let mut errors = Vec::new();
        if selected_channel.is_none() {
            errors.push(
                "no outbound channel available after applying policy and channel settings"
                    .to_string(),
            );
        }
        if selected_variant.is_none() {
            errors.push("message bundle has no variants".to_string());
        }

        let mut events = Vec::new();
        let no_channel_or_variant = selected_channel.is_none() || selected_variant.is_none();
        let output = if no_channel_or_variant {
            OutboundSdrOutput {
                sequence_draft: None,
                personalization_fields_used: Vec::new(),
                send_requests: Vec::new(),
                reply_classifications: Vec::new(),
                handoffs: Vec::new(),
            }
        } else {
            let channel = selected_channel.expect("checked above");
            let variant = selected_variant.clone().expect("checked above");
            let touch_count = input.sequence_policy.max_touches.max(1).min(6);
            let touches = (0..touch_count)
                .map(|idx| SequenceTouch {
                    touch_number: idx + 1,
                    offset_days: idx as u16 * input.sequence_policy.cadence_days,
                    channel,
                    template_id: variant.template_id.clone(),
                })
                .collect::<Vec<_>>();
            let sequence_draft = SequenceDraft {
                sequence_id: format!("seq-{}", envelope.task_id.simple()),
                touches,
                channel,
            };

            let risk_requires_approval = envelope.policy_pack.human_approval_required
                || variant.claim_risk == ClaimRisk::High;
            let send_requests = if risk_requires_approval {
                Vec::new()
            } else {
                input
                    .segment_manifest
                    .iter()
                    .enumerate()
                    .map(|(idx, contact)| SendRequest {
                        recipient_id: contact.recipient_id,
                        template_id: variant.template_id.clone(),
                        send_at: Utc::now() + Duration::minutes((idx as i64) * 5),
                    })
                    .collect()
            };

            events.push(self.make_event(
                &envelope,
                AgentId::RachelOutboundSdr,
                "outbound.sequence.drafted",
                SubjectType::Campaign,
                envelope.objective_id,
                &sequence_draft,
            )?);
            if risk_requires_approval {
                let approval = ApprovalRequest {
                    reason: if variant.claim_risk == ClaimRisk::High {
                        "Selected copy variant has high claim risk".to_string()
                    } else {
                        "Policy pack requires human approval before send".to_string()
                    },
                    risk_level: "high".to_string(),
                    reviewer_group: "gtm_ops".to_string(),
                };
                events.push(self.make_event(
                    &envelope,
                    AgentId::RachelOutboundSdr,
                    "approval.requested",
                    SubjectType::Campaign,
                    envelope.objective_id,
                    &approval,
                )?);
            } else {
                events.push(self.make_event(
                    &envelope,
                    AgentId::RachelOutboundSdr,
                    "outbound.send.requested",
                    SubjectType::Campaign,
                    envelope.objective_id,
                    &serde_json::json!({
                        "segment_size": input.segment_manifest.len(),
                        "send_request_count": send_requests.len(),
                        "channel": channel,
                    }),
                )?);
            }

            OutboundSdrOutput {
                sequence_draft: Some(sequence_draft),
                personalization_fields_used: vec![
                    "first_name".to_string(),
                    "company_name".to_string(),
                    "job_title".to_string(),
                ],
                send_requests,
                reply_classifications: Vec::new(),
                handoffs: Vec::new(),
            }
        };

        let status = if no_channel_or_variant {
            TaskStatus::Failed
        } else if envelope.policy_pack.human_approval_required || risk_blocked {
            TaskStatus::NeedsHuman
        } else {
            TaskStatus::Succeeded
        };

        let confidence = match status {
            TaskStatus::Failed => 0.25,
            TaskStatus::NeedsHuman => 0.62,
            _ => (0.6 + (input.segment_manifest.len().min(50) as f32 / 250.0)).min(0.9),
        };
        let next_action = match status {
            TaskStatus::Failed => {
                "Fix channel policy and message variant configuration".to_string()
            }
            TaskStatus::NeedsHuman => {
                "Wait for human approval, then enqueue outbound send requests".to_string()
            }
            _ => "Dispatch send requests and start reply classification loop".to_string(),
        };

        Ok(AgentTaskResult {
            task_id: envelope.task_id,
            status,
            schema_version: GTM_SCHEMA_VERSION.to_string(),
            output_payload: output,
            emitted_events: events,
            confidence,
            evidence_refs: vec![format!("segment:{}", input.message_bundle.segment_id)],
            next_action,
            errors,
        })
    }

    pub fn run_feedback_prd(
        &self,
        envelope: AgentTaskEnvelope,
        input: FeedbackPrdInput,
    ) -> Result<AgentTaskResult<FeedbackPrdOutput>, GtmAgentError> {
        #[derive(Default)]
        struct ClusterAccumulator {
            frequency: u32,
            segments: HashSet<String>,
            evidence_refs: Vec<String>,
        }

        let mut clusters_by_theme: BTreeMap<String, ClusterAccumulator> = BTreeMap::new();
        for item in &input.feedback_items {
            let theme = classify_feedback_theme(&item.text).to_string();
            let entry = clusters_by_theme.entry(theme).or_default();
            entry.frequency += 1;
            if let Some(segment) = item.segment_id.as_deref() {
                entry.segments.insert(segment.to_string());
            }
            entry.evidence_refs.push(
                item.evidence_ref
                    .clone()
                    .unwrap_or_else(|| format!("feedback:{}", item.feedback_id)),
            );
        }

        let mut insight_clusters = clusters_by_theme
            .into_iter()
            .filter(|(_, acc)| acc.frequency as usize >= input.cluster_policy.min_cluster_size)
            .map(|(theme, acc)| InsightCluster {
                cluster_id: format!("cluster_{}", slugify(&theme)),
                theme,
                frequency: acc.frequency,
                affected_segments: {
                    let mut segments = acc.segments.into_iter().collect::<Vec<_>>();
                    segments.sort();
                    segments
                },
                evidence_refs: acc.evidence_refs,
            })
            .collect::<Vec<_>>();
        insight_clusters.sort_by_key(|cluster| Reverse(cluster.frequency));

        let mut job_stories = Vec::new();
        let mut prd_drafts = Vec::new();
        let mut priority_scores = Vec::new();

        for cluster in &insight_clusters {
            let persona = persona_for_theme(&cluster.theme);
            let prd_id = format!("prd_{}", cluster.cluster_id);
            let users = if cluster.affected_segments.is_empty() {
                vec!["core_gtm_segment".to_string()]
            } else {
                cluster.affected_segments.clone()
            };
            let effort = effort_for_theme(&cluster.theme);
            let impact = ((cluster.frequency as f32 / 8.0) + 0.2).min(1.0);
            let reach = ((users.len().max(1) as f32) / 4.0).min(1.0);
            let confidence =
                (0.5 + input.cluster_policy.recency_weight.clamp(0.0, 1.0) * 0.3).min(0.95);
            let overall = (impact * reach * confidence) / (effort + 0.1);

            job_stories.push(JobStory {
                as_persona: persona.to_string(),
                when_context: format!("when {} keeps appearing in customer signals", cluster.theme),
                i_want: format!("a clear fix plan for {}", cluster.theme),
                so_i_can: "improve activation and pipeline quality".to_string(),
            });
            prd_drafts.push(PrdDraft {
                prd_id: prd_id.clone(),
                problem: format!(
                    "{} occurred {} times across active segments",
                    cluster.theme, cluster.frequency
                ),
                users,
                success_metrics: vec![
                    "activation_rate_day_14".to_string(),
                    "meeting_to_sql_conversion".to_string(),
                    "support_ticket_volume".to_string(),
                ],
                scope: vec![
                    format!("deliver solution for {}", cluster.theme),
                    "instrument KPI tracking".to_string(),
                    "publish rollout plan".to_string(),
                ],
                risks: derive_risks(&input, &cluster.theme),
            });
            priority_scores.push(PriorityScore {
                prd_id,
                impact,
                reach,
                confidence,
                effort,
                overall,
            });
        }

        let output = FeedbackPrdOutput {
            insight_clusters,
            job_stories,
            prd_drafts,
            priority_scores,
        };

        let mut events = Vec::new();
        events.push(self.make_event(
            &envelope,
            AgentId::RachelFeedbackPrdSynthesizer,
            "feedback.cluster.updated",
            SubjectType::Feature,
            envelope.objective_id,
            &serde_json::json!({
                "cluster_count": output.insight_clusters.len(),
                "feedback_count": input.feedback_items.len(),
            }),
        )?);
        for cluster in &output.insight_clusters {
            events.push(self.make_event(
                &envelope,
                AgentId::RachelFeedbackPrdSynthesizer,
                "feedback.insight.published",
                SubjectType::Feature,
                envelope.objective_id,
                cluster,
            )?);
        }
        for prd in &output.prd_drafts {
            events.push(self.make_event(
                &envelope,
                AgentId::RachelFeedbackPrdSynthesizer,
                "prd.draft.generated",
                SubjectType::Feature,
                envelope.objective_id,
                prd,
            )?);
        }

        let partial = output.prd_drafts.is_empty();
        Ok(AgentTaskResult {
            task_id: envelope.task_id,
            status: if partial {
                TaskStatus::Partial
            } else {
                TaskStatus::Succeeded
            },
            schema_version: GTM_SCHEMA_VERSION.to_string(),
            output_payload: output,
            emitted_events: events,
            confidence: if partial {
                0.4
            } else {
                (0.55 + (input.feedback_items.len().min(12) as f32 * 0.03)).min(0.9)
            },
            evidence_refs: envelope.input_refs.clone(),
            next_action: if partial {
                "Collect more feedback data or lower min_cluster_size to generate PRDs".to_string()
            } else {
                "Prioritize generated PRDs and hand off to product planning".to_string()
            },
            errors: if partial {
                vec!["no clusters met the minimum cluster size".to_string()]
            } else {
                Vec::new()
            },
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
    ) -> Result<EventEnvelope, GtmAgentError> {
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

fn select_channel(envelope: &AgentTaskEnvelope, input: &OutboundSdrInput) -> Option<GtmChannel> {
    let mut candidates = Vec::new();
    if input.channel_policy.email_enabled {
        candidates.push(GtmChannel::Email);
    }
    if input.channel_policy.linkedin_ads_enabled {
        candidates.push(GtmChannel::LinkedinAds);
    }
    if input.channel_policy.linkedin_dm_enabled {
        candidates.push(GtmChannel::LinkedinDm);
    }
    candidates.push(GtmChannel::HubspotWorkflow);

    candidates
        .into_iter()
        .find(|channel| envelope.policy_pack.allowed_channels.contains(channel))
}

fn score_account(account: &AccountSignal) -> (u8, Vec<String>) {
    let mut score = 0_i32;
    let mut drivers = Vec::new();

    let usage_points = (account.product_events_14d.min(12) * 3) as i32;
    score += usage_points;
    if usage_points > 0 {
        drivers.push("strong_product_activity".to_string());
    }
    if account.won_deals_12m >= account.lost_deals_12m && account.won_deals_12m > 0 {
        score += 15;
        drivers.push("win_rate_positive".to_string());
    } else if account.lost_deals_12m > account.won_deals_12m {
        score -= 6;
    }
    if account.churned {
        score -= 25;
        drivers.push("recent_churn_signal".to_string());
    } else {
        score += 12;
        drivers.push("retention_signal".to_string());
    }
    if account.activation_days <= 7 {
        score += 14;
        drivers.push("fast_activation".to_string());
    } else if account.activation_days <= 14 {
        score += 8;
        drivers.push("acceptable_activation".to_string());
    }
    if account.support_tickets_30d <= 2 {
        score += 10;
        drivers.push("low_support_load".to_string());
    } else if account.support_tickets_30d >= 8 {
        score -= 12;
        drivers.push("high_support_load".to_string());
    }
    if account.ltv_usd >= 5_000.0 {
        score += 10;
        drivers.push("high_ltv".to_string());
    }
    if account.company_size >= 50 && account.company_size <= 500 {
        score += 5;
        drivers.push("target_company_size".to_string());
    }

    let clamped = score.clamp(0, 100) as u8;
    (clamped, drivers)
}

fn tier_for_score(score: u8) -> IcpTier {
    match score {
        80..=100 => IcpTier::A,
        65..=79 => IcpTier::B,
        45..=64 => IcpTier::C,
        _ => IcpTier::D,
    }
}

fn classify_feedback_theme(text: &str) -> &'static str {
    let normalized = text.to_ascii_lowercase();
    if normalized.contains("onboard")
        || normalized.contains("activation")
        || normalized.contains("setup")
    {
        "onboarding friction"
    } else if normalized.contains("integrat")
        || normalized.contains("api")
        || normalized.contains("webhook")
        || normalized.contains("hubspot")
    {
        "integration gap"
    } else if normalized.contains("price")
        || normalized.contains("pricing")
        || normalized.contains("budget")
    {
        "pricing objection"
    } else if normalized.contains("deliverability")
        || normalized.contains("spam")
        || normalized.contains("unsubscribe")
    {
        "outbound quality"
    } else {
        "feature discoverability"
    }
}

fn persona_for_theme(theme: &str) -> &'static str {
    match theme {
        "integration gap" => "operations lead",
        "pricing objection" => "budget owner",
        "onboarding friction" => "new champion",
        "outbound quality" => "demand generation manager",
        _ => "growth manager",
    }
}

fn effort_for_theme(theme: &str) -> f32 {
    match theme {
        "integration gap" => 0.8,
        "onboarding friction" => 0.55,
        "pricing objection" => 0.45,
        "outbound quality" => 0.5,
        _ => 0.6,
    }
}

fn derive_risks(input: &FeedbackPrdInput, theme: &str) -> Vec<String> {
    let mut risks = Vec::new();
    if !input.product_context.constraints.is_empty() {
        for constraint in input.product_context.constraints.iter().take(2) {
            risks.push(format!("constraint: {}", constraint));
        }
    }
    risks.push(format!("insufficient telemetry for {}", theme));
    risks.push("cross-team dependency misalignment".to_string());
    risks
}

fn slugify(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
        } else if ch.is_ascii_whitespace() || ch == '-' {
            out.push('_');
        }
    }
    while out.contains("__") {
        out = out.replace("__", "_");
    }
    out.trim_matches('_').to_string()
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use super::super::contracts::{
        AgentId, AgentTaskEnvelope, BusinessContext, ChannelPolicy, ClaimRisk, ClusterPolicy,
        CurrentState, FeedbackItem, FeedbackPrdInput, FeedbackSource, MessageBundle,
        MessageVariant, Objective, OrchestratorInput, OutboundSdrInput, PolicyPack, ProductContext,
        ResourceLimits, SegmentContact, SequencePolicy, TaskPriority, TaskStatus,
    };
    use super::*;

    fn base_envelope(agent_id: AgentId) -> AgentTaskEnvelope {
        let mut envelope = AgentTaskEnvelope::new(agent_id);
        envelope.priority = TaskPriority::High;
        envelope.policy_pack = PolicyPack::default();
        envelope.input_refs = vec!["warehouse://daily_snapshot".to_string()];
        envelope
    }

    #[test]
    fn orchestrator_assigns_three_phase1_agents() {
        let engine = Phase1AgentEngine;
        let envelope = base_envelope(AgentId::RachelOrchestrator);
        let input = OrchestratorInput {
            objective: Objective {
                name: "Increase SQL quality".to_string(),
                target_metric: "sql_conversion_rate".to_string(),
                target_value: "0.24".to_string(),
                due_date: None,
                owner: "gtm_lead".to_string(),
            },
            current_state: CurrentState::default(),
            resource_limits: ResourceLimits {
                daily_email_cap: 1000,
                budget_cap_usd: 20_000,
                human_review_capacity: 4,
            },
        };

        let result = engine.run_orchestrator(envelope, input).unwrap();
        assert_eq!(result.status, TaskStatus::Succeeded);
        assert_eq!(result.output_payload.task_assignments.len(), 3);
        assert!(result
            .output_payload
            .task_assignments
            .iter()
            .any(|assignment| assignment.agent_id == AgentId::RachelIcpScout));
        assert!(result
            .output_payload
            .task_assignments
            .iter()
            .any(|assignment| assignment.agent_id == AgentId::RachelOutboundSdr));
        assert!(result
            .output_payload
            .task_assignments
            .iter()
            .any(|assignment| assignment.agent_id == AgentId::RachelFeedbackPrdSynthesizer));
        assert!(result
            .emitted_events
            .iter()
            .any(|event| event.event_type == "orchestrator.task.assigned"));
    }

    #[test]
    fn icp_scout_scores_accounts_and_promotes_segments() {
        let engine = Phase1AgentEngine;
        let envelope = base_envelope(AgentId::RachelIcpScout);
        let input = IcpScoutInput {
            accounts: vec![
                AccountSignal {
                    entity_id: Uuid::new_v4(),
                    company_size: 120,
                    industry: "SaaS".to_string(),
                    region: "US".to_string(),
                    product_events_14d: 14,
                    support_tickets_30d: 1,
                    won_deals_12m: 5,
                    lost_deals_12m: 1,
                    churned: false,
                    activation_days: 4,
                    ltv_usd: 9000.0,
                },
                AccountSignal {
                    entity_id: Uuid::new_v4(),
                    company_size: 20,
                    industry: "SaaS".to_string(),
                    region: "US".to_string(),
                    product_events_14d: 6,
                    support_tickets_30d: 3,
                    won_deals_12m: 2,
                    lost_deals_12m: 2,
                    churned: false,
                    activation_days: 10,
                    ltv_usd: 3000.0,
                },
                AccountSignal {
                    entity_id: Uuid::new_v4(),
                    company_size: 8,
                    industry: "Agency".to_string(),
                    region: "US".to_string(),
                    product_events_14d: 1,
                    support_tickets_30d: 10,
                    won_deals_12m: 0,
                    lost_deals_12m: 4,
                    churned: true,
                    activation_days: 45,
                    ltv_usd: 500.0,
                },
            ],
            current_segment_ids: vec!["tier_a_high_fit".to_string()],
            min_sample_size: 2,
        };

        let result = engine.run_icp_scout(envelope, input).unwrap();
        assert_eq!(result.status, TaskStatus::Succeeded);
        assert_eq!(result.output_payload.icp_scores.len(), 3);
        assert!(result
            .output_payload
            .icp_scores
            .iter()
            .any(|score| score.tier == IcpTier::A));
        assert!(result.output_payload.segment_definitions.len() >= 2);
        assert!(result
            .emitted_events
            .iter()
            .any(|event| event.event_type == "icp.segment.promoted"));
    }

    #[test]
    fn outbound_requires_human_review_for_high_risk_copy() {
        let engine = Phase1AgentEngine;
        let envelope = base_envelope(AgentId::RachelOutboundSdr);
        let input = OutboundSdrInput {
            segment_manifest: vec![SegmentContact {
                recipient_id: Uuid::new_v4(),
                account_id: Uuid::new_v4(),
                email: "prospect@example.com".to_string(),
                first_name: Some("Taylor".to_string()),
                job_title: Some("VP Revenue".to_string()),
                company_name: Some("Acme".to_string()),
                timezone: Some("America/Los_Angeles".to_string()),
            }],
            message_bundle: MessageBundle {
                segment_id: "tier_a_high_fit".to_string(),
                variants: vec![MessageVariant {
                    template_id: "v1".to_string(),
                    subject: "Guaranteed 4x pipeline in 2 weeks".to_string(),
                    body: "High-risk claim copy".to_string(),
                    claim_risk: ClaimRisk::High,
                }],
            },
            sequence_policy: SequencePolicy {
                max_touches: 3,
                cadence_days: 3,
                stop_conditions: vec!["positive_reply".to_string()],
            },
            channel_policy: ChannelPolicy {
                email_enabled: true,
                linkedin_ads_enabled: false,
                linkedin_dm_enabled: false,
            },
        };

        let result = engine.run_outbound_sdr(envelope, input).unwrap();
        assert_eq!(result.status, TaskStatus::NeedsHuman);
        assert!(result.output_payload.send_requests.is_empty());
        assert!(result
            .emitted_events
            .iter()
            .any(|event| event.event_type == "approval.requested"));
    }

    #[test]
    fn feedback_prd_generates_cluster_and_prd_draft() {
        let engine = Phase1AgentEngine;
        let envelope = base_envelope(AgentId::RachelFeedbackPrdSynthesizer);
        let now = Utc::now();
        let input = FeedbackPrdInput {
            feedback_items: vec![
                FeedbackItem {
                    feedback_id: Uuid::new_v4(),
                    source: FeedbackSource::SupportTicket,
                    segment_id: Some("tier_a_high_fit".to_string()),
                    text: "HubSpot integration keeps failing for custom fields".to_string(),
                    created_at: now,
                    evidence_ref: Some("ticket:123".to_string()),
                },
                FeedbackItem {
                    feedback_id: Uuid::new_v4(),
                    source: FeedbackSource::SalesCall,
                    segment_id: Some("tier_a_high_fit".to_string()),
                    text: "Need better API integration for CRM sync".to_string(),
                    created_at: now,
                    evidence_ref: Some("call:456".to_string()),
                },
                FeedbackItem {
                    feedback_id: Uuid::new_v4(),
                    source: FeedbackSource::OutboundReply,
                    segment_id: Some("fast_activation".to_string()),
                    text: "Your integration story is unclear".to_string(),
                    created_at: now,
                    evidence_ref: Some("reply:789".to_string()),
                },
            ],
            product_context: ProductContext {
                roadmap_refs: vec!["roadmap://q2".to_string()],
                constraints: vec!["single backend engineer".to_string()],
                architecture_notes: vec!["legacy sync worker".to_string()],
            },
            business_context: BusinessContext {
                revenue_goal: "expand enterprise ARR".to_string(),
                strategic_themes: vec!["expansion".to_string()],
            },
            cluster_policy: ClusterPolicy {
                min_cluster_size: 2,
                recency_weight: 0.7,
            },
        };

        let result = engine.run_feedback_prd(envelope, input).unwrap();
        assert_eq!(result.status, TaskStatus::Succeeded);
        assert!(!result.output_payload.insight_clusters.is_empty());
        assert!(!result.output_payload.prd_drafts.is_empty());
        assert!(result
            .emitted_events
            .iter()
            .any(|event| event.event_type == "prd.draft.generated"));
    }

    #[test]
    fn phase1_workflow_runs_all_four_agents() {
        let engine = Phase1AgentEngine;
        let base = base_envelope(AgentId::RachelOrchestrator);
        let workflow_input = Phase1WorkflowInput {
            base_envelope: base,
            orchestrator: OrchestratorInput {
                objective: Objective {
                    name: "Improve pipeline quality".to_string(),
                    target_metric: "meeting_to_sql".to_string(),
                    target_value: "0.3".to_string(),
                    due_date: None,
                    owner: "gtm_lead".to_string(),
                },
                current_state: CurrentState::default(),
                resource_limits: ResourceLimits {
                    daily_email_cap: 500,
                    budget_cap_usd: 10_000,
                    human_review_capacity: 2,
                },
            },
            icp_scout: IcpScoutInput {
                accounts: vec![AccountSignal {
                    entity_id: Uuid::new_v4(),
                    company_size: 75,
                    industry: "SaaS".to_string(),
                    region: "US".to_string(),
                    product_events_14d: 11,
                    support_tickets_30d: 1,
                    won_deals_12m: 3,
                    lost_deals_12m: 1,
                    churned: false,
                    activation_days: 6,
                    ltv_usd: 7000.0,
                }],
                current_segment_ids: Vec::new(),
                min_sample_size: 1,
            },
            outbound_sdr: OutboundSdrInput {
                segment_manifest: vec![SegmentContact {
                    recipient_id: Uuid::new_v4(),
                    account_id: Uuid::new_v4(),
                    email: "prospect@company.com".to_string(),
                    first_name: Some("Alex".to_string()),
                    job_title: Some("Head of Marketing".to_string()),
                    company_name: Some("Company".to_string()),
                    timezone: None,
                }],
                message_bundle: MessageBundle {
                    segment_id: "tier_a_high_fit".to_string(),
                    variants: vec![MessageVariant {
                        template_id: "safe-v1".to_string(),
                        subject: "Idea to reduce time-to-value".to_string(),
                        body: "Low risk copy".to_string(),
                        claim_risk: ClaimRisk::Low,
                    }],
                },
                sequence_policy: SequencePolicy {
                    max_touches: 2,
                    cadence_days: 3,
                    stop_conditions: vec!["meeting_booked".to_string()],
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
                    text: "Onboarding setup takes too long".to_string(),
                    created_at: Utc::now(),
                    evidence_ref: Some("onboarding:1".to_string()),
                }],
                product_context: ProductContext {
                    roadmap_refs: Vec::new(),
                    constraints: vec!["limited QA capacity".to_string()],
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

        let result = engine.run_workflow(workflow_input).unwrap();
        assert_eq!(result.orchestrator.status, TaskStatus::Succeeded);
        assert_eq!(result.icp_scout.status, TaskStatus::Succeeded);
        assert_eq!(result.outbound_sdr.status, TaskStatus::Succeeded);
        assert_eq!(result.feedback_prd.status, TaskStatus::Succeeded);
        assert!(!result.events.is_empty());
    }
}
