use chrono::Utc;
use serde::Serialize;
use uuid::Uuid;

use super::contracts::{
    AgentId, AgentTaskEnvelope, AgentTaskResult, ApprovalRequest, EventEnvelope, ExperimentDesign,
    ExperimentInput, ExperimentOutput, ExperimentRecommendation, ExperimentResultSummary,
    FeedbackItem, FeedbackSource, OnboardingInput, OnboardingMilestone, OnboardingOutput,
    OnboardingRiskFlag, RiskSeverity, SubjectType, TaskStatus, GTM_SCHEMA_VERSION,
};

#[derive(Debug, thiserror::Error)]
pub enum Phase3AgentError {
    #[error("event payload serialization failed: {0}")]
    EventSerialization(#[from] serde_json::Error),
    #[error("reliable metrics are required before phase 3 rollout")]
    MetricsNotReliable,
}

#[derive(Debug, Clone)]
pub struct Phase3WorkflowInput {
    pub base_envelope: AgentTaskEnvelope,
    pub metrics_reliable: bool,
    pub onboarding: OnboardingInput,
    pub experiment: ExperimentInput,
}

#[derive(Debug, Clone)]
pub struct Phase3WorkflowResult {
    pub onboarding: AgentTaskResult<OnboardingOutput>,
    pub experiment: AgentTaskResult<ExperimentOutput>,
    pub events: Vec<EventEnvelope>,
}

#[derive(Debug, Default, Clone)]
pub struct Phase3AgentEngine;

impl Phase3AgentEngine {
    pub fn run_workflow(
        &self,
        input: Phase3WorkflowInput,
    ) -> Result<Phase3WorkflowResult, Phase3AgentError> {
        if !input.metrics_reliable {
            return Err(Phase3AgentError::MetricsNotReliable);
        }

        let onboarding = self.run_onboarding_csm(
            input.base_envelope.with_agent(AgentId::RachelOnboardingCsm),
            input.onboarding,
        )?;
        let experiment = self.run_experiment_analyst(
            input
                .base_envelope
                .with_agent(AgentId::RachelExperimentAnalyst),
            input.experiment,
        )?;

        let mut events = Vec::new();
        events.extend(onboarding.emitted_events.clone());
        events.extend(experiment.emitted_events.clone());

        Ok(Phase3WorkflowResult {
            onboarding,
            experiment,
            events,
        })
    }

    pub fn run_onboarding_csm(
        &self,
        envelope: AgentTaskEnvelope,
        input: OnboardingInput,
    ) -> Result<AgentTaskResult<OnboardingOutput>, Phase3AgentError> {
        let onboarding_plan = vec![
            OnboardingMilestone {
                milestone_id: format!("{}-kickoff", input.customer_id.simple()),
                name: "Kickoff and success criteria alignment".to_string(),
                due_in_days: 2,
                owner_role: "csm".to_string(),
                success_criteria: "Mutual action plan approved with target KPI and stakeholders"
                    .to_string(),
            },
            OnboardingMilestone {
                milestone_id: format!("{}-activation", input.customer_id.simple()),
                name: "Activation workflow completion".to_string(),
                due_in_days: 7,
                owner_role: "implementation_specialist".to_string(),
                success_criteria: "Primary workflow active and first business outcome observed"
                    .to_string(),
            },
            OnboardingMilestone {
                milestone_id: format!("{}-qbr", input.customer_id.simple()),
                name: "First value review".to_string(),
                due_in_days: 21,
                owner_role: "csm".to_string(),
                success_criteria: "QBR completed with next-quarter expansion hypothesis"
                    .to_string(),
            },
        ];

        let mut activation_risk_flags = Vec::new();
        let activation_gap = input.target_activation_rate - input.current_activation_rate;
        if activation_gap > 0.2 {
            activation_risk_flags.push(OnboardingRiskFlag {
                code: "activation_gap_high".to_string(),
                severity: RiskSeverity::High,
                summary: format!(
                    "Activation rate ({:.2}) is far below target ({:.2})",
                    input.current_activation_rate, input.target_activation_rate
                ),
                mitigation:
                    "Run weekly unblock review and fast-track implementation specialist support"
                        .to_string(),
            });
        } else if activation_gap > 0.1 {
            activation_risk_flags.push(OnboardingRiskFlag {
                code: "activation_gap_moderate".to_string(),
                severity: RiskSeverity::Medium,
                summary: format!(
                    "Activation rate ({:.2}) is below target ({:.2})",
                    input.current_activation_rate, input.target_activation_rate
                ),
                mitigation: "Prioritize setup checklist and tighten onboarding cadence".to_string(),
            });
        }
        for blocker in &input.known_blockers {
            activation_risk_flags.push(OnboardingRiskFlag {
                code: format!("blocker_{}", slugify(blocker)),
                severity: RiskSeverity::Medium,
                summary: blocker.clone(),
                mitigation: "Assign owner and due date in weekly onboarding stand-up".to_string(),
            });
        }

        let mut captured_feedback = Vec::new();
        for blocker in input.known_blockers.iter().take(5) {
            captured_feedback.push(FeedbackItem {
                feedback_id: Uuid::new_v4(),
                source: FeedbackSource::Onboarding,
                segment_id: Some(input.segment_id.clone()),
                text: blocker.clone(),
                created_at: Utc::now(),
                evidence_ref: Some(format!("onboarding://{}", slugify(blocker))),
            });
        }
        if let Some(handoff) = input.handoff_summary.as_deref() {
            captured_feedback.push(FeedbackItem {
                feedback_id: Uuid::new_v4(),
                source: FeedbackSource::Onboarding,
                segment_id: Some(input.segment_id.clone()),
                text: format!("handoff context: {}", handoff),
                created_at: Utc::now(),
                evidence_ref: Some("handoff://outbound_or_sales".to_string()),
            });
        }

        let qbr_summary = format!(
            "Account {} onboarding focus: {}. Goals: {}.",
            input.account_name,
            if activation_risk_flags.is_empty() {
                "stabilize and expand usage"
            } else {
                "close activation gap and remove blockers"
            },
            if input.customer_goals.is_empty() {
                "no explicit goals captured".to_string()
            } else {
                input.customer_goals.join("; ")
            }
        );

        let output = OnboardingOutput {
            onboarding_plan,
            activation_risk_flags,
            captured_feedback,
            qbr_summary,
        };

        let mut events = Vec::new();
        events.push(self.make_event(
            &envelope,
            AgentId::RachelOnboardingCsm,
            "onboarding.plan.started",
            SubjectType::Account,
            input.customer_id,
            &serde_json::json!({
                "milestone_count": output.onboarding_plan.len(),
                "segment_id": input.segment_id,
            }),
        )?);
        for risk in &output.activation_risk_flags {
            events.push(self.make_event(
                &envelope,
                AgentId::RachelOnboardingCsm,
                "onboarding.risk.flagged",
                SubjectType::Account,
                input.customer_id,
                risk,
            )?);
        }
        for feedback in &output.captured_feedback {
            events.push(self.make_event(
                &envelope,
                AgentId::RachelOnboardingCsm,
                "onboarding.feedback.captured",
                SubjectType::Account,
                input.customer_id,
                feedback,
            )?);
        }

        let status = if output.onboarding_plan.is_empty() {
            TaskStatus::Failed
        } else if !output.activation_risk_flags.is_empty()
            && output
                .activation_risk_flags
                .iter()
                .any(|risk| risk.severity == RiskSeverity::High)
        {
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
                TaskStatus::NeedsHuman => 0.64,
                _ => 0.83,
            },
            evidence_refs: envelope.input_refs.clone(),
            next_action: match status {
                TaskStatus::Failed => {
                    "Provide onboarding inputs and retry plan generation".to_string()
                }
                TaskStatus::NeedsHuman => {
                    "Escalate high-risk onboarding account for manual intervention".to_string()
                }
                _ => "Execute onboarding milestones and monitor activation weekly".to_string(),
            },
            errors: Vec::new(),
        })
    }

    pub fn run_experiment_analyst(
        &self,
        envelope: AgentTaskEnvelope,
        input: ExperimentInput,
    ) -> Result<AgentTaskResult<ExperimentOutput>, Phase3AgentError> {
        let safe_baseline = input.baseline_value.max(0.0001);
        let uplift_ratio = (input.observed_value - safe_baseline) / safe_baseline;
        let sample_ready = input.sample_size >= input.min_sample_size;
        let confidence = input.confidence_estimate.clamp(0.0, 1.0);
        let reliable = sample_ready && confidence >= 0.7;

        let experiment_design = ExperimentDesign {
            experiment_id: format!("exp-{}", envelope.task_id.simple()),
            hypothesis: format!(
                "Improving {} should increase {}",
                input.experiment_name, input.primary_metric
            ),
            success_metric: input.primary_metric.clone(),
            guardrails: vec![
                "do not increase unsubscribe_rate".to_string(),
                "hold support_ticket_volume within baseline tolerance".to_string(),
                "keep CAC within approved budget band".to_string(),
            ],
            segments: input.segment_ids.clone(),
        };

        let result_summary = ExperimentResultSummary {
            experiment_id: experiment_design.experiment_id.clone(),
            uplift_ratio,
            statistically_reliable: reliable,
            confidence_estimate: confidence,
            sample_size: input.sample_size,
        };

        let mut recommendations = Vec::new();
        if reliable && uplift_ratio > 0.05 {
            recommendations.push(ExperimentRecommendation {
                action: "scale_winning_variant".to_string(),
                owner: "growth_ops".to_string(),
                rationale: "Reliable positive uplift observed with sufficient sample".to_string(),
                expected_impact: format!("{:.1}% primary metric uplift", uplift_ratio * 100.0),
            });
        } else if reliable && uplift_ratio < -0.03 {
            recommendations.push(ExperimentRecommendation {
                action: "rollback_and_retest".to_string(),
                owner: "pmm".to_string(),
                rationale: "Reliable negative movement against baseline".to_string(),
                expected_impact: "Recover baseline conversion performance".to_string(),
            });
        } else {
            recommendations.push(ExperimentRecommendation {
                action: "continue_data_collection".to_string(),
                owner: "experiment_analyst".to_string(),
                rationale: "Need stronger statistical signal or larger sample".to_string(),
                expected_impact: "Increase confidence for next decision cycle".to_string(),
            });
        }
        if !input.adoption_signals.is_empty() {
            let avg_delta = input
                .adoption_signals
                .iter()
                .map(|signal| signal.after_rate - signal.before_rate)
                .sum::<f32>()
                / input.adoption_signals.len() as f32;
            recommendations.push(ExperimentRecommendation {
                action: "feature_adoption_followup".to_string(),
                owner: "product_growth".to_string(),
                rationale: "Adoption deltas provide additional context for GTM impact".to_string(),
                expected_impact: format!("{:.1}% mean adoption delta", avg_delta * 100.0),
            });
        }

        let output = ExperimentOutput {
            experiment_design,
            result_summary,
            recommendations,
        };

        let mut events = Vec::new();
        events.push(self.make_event(
            &envelope,
            AgentId::RachelExperimentAnalyst,
            "experiment.started",
            SubjectType::Campaign,
            envelope.objective_id,
            &output.experiment_design,
        )?);
        events.push(self.make_event(
            &envelope,
            AgentId::RachelExperimentAnalyst,
            "experiment.result.published",
            SubjectType::Campaign,
            envelope.objective_id,
            &output.result_summary,
        )?);
        for recommendation in &output.recommendations {
            events.push(self.make_event(
                &envelope,
                AgentId::RachelExperimentAnalyst,
                "experiment.recommendation.issued",
                SubjectType::Campaign,
                envelope.objective_id,
                recommendation,
            )?);
        }

        let needs_human = !output.result_summary.statistically_reliable;
        if needs_human {
            let approval = ApprovalRequest {
                reason: "Metrics reliability below threshold for autonomous scaling".to_string(),
                risk_level: "medium".to_string(),
                reviewer_group: "revops".to_string(),
            };
            events.push(self.make_event(
                &envelope,
                AgentId::RachelExperimentAnalyst,
                "approval.requested",
                SubjectType::Campaign,
                envelope.objective_id,
                &approval,
            )?);
        }

        let status = if needs_human {
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
            confidence: if needs_human {
                0.62
            } else {
                confidence.max(0.75)
            },
            evidence_refs: envelope.input_refs.clone(),
            next_action: if needs_human {
                "Review experiment reliability and decide whether to extend sample window"
                    .to_string()
            } else {
                "Apply experiment recommendation and schedule next validation cycle".to_string()
            },
            errors: Vec::new(),
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
    ) -> Result<EventEnvelope, Phase3AgentError> {
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
    use super::super::contracts::{
        AgentId, AgentTaskEnvelope, CampaignPerformance, ExperimentInput, FeatureAdoptionSignal,
        OnboardingInput, PolicyPack, RiskSeverity, TaskPriority, TaskStatus,
    };
    use super::*;

    fn base_envelope(agent_id: AgentId) -> AgentTaskEnvelope {
        let mut envelope = AgentTaskEnvelope::new(agent_id);
        envelope.priority = TaskPriority::High;
        envelope.policy_pack = PolicyPack::default();
        envelope.input_refs = vec!["warehouse://metrics_snapshot".to_string()];
        envelope
    }

    fn onboarding_input() -> OnboardingInput {
        OnboardingInput {
            customer_id: Uuid::new_v4(),
            account_name: "Acme Corp".to_string(),
            segment_id: "tier_a_high_fit".to_string(),
            customer_goals: vec![
                "launch first outbound workflow".to_string(),
                "reach activation milestone in two weeks".to_string(),
            ],
            known_blockers: vec!["CRM field mapping incomplete".to_string()],
            current_activation_rate: 0.52,
            target_activation_rate: 0.72,
            handoff_summary: Some("High intent from outbound SDR handoff".to_string()),
        }
    }

    fn experiment_input() -> ExperimentInput {
        ExperimentInput {
            experiment_name: "new onboarding sequence".to_string(),
            primary_metric: "activation_rate_day_14".to_string(),
            baseline_value: 0.42,
            observed_value: 0.50,
            sample_size: 320,
            min_sample_size: 200,
            confidence_estimate: 0.83,
            segment_ids: vec!["tier_a_high_fit".to_string()],
            campaign_results: vec![CampaignPerformance {
                campaign_id: "cmp-001".to_string(),
                segment_id: "tier_a_high_fit".to_string(),
                spend_usd: 4500.0,
                impressions: 120_000,
                clicks: 3_200,
                meetings: 210,
                sqls: 62,
            }],
            adoption_signals: vec![FeatureAdoptionSignal {
                feature_name: "workflow_builder".to_string(),
                before_rate: 0.33,
                after_rate: 0.49,
            }],
        }
    }

    #[test]
    fn onboarding_generates_plan_and_risk_flags() {
        let engine = Phase3AgentEngine;
        let result = engine
            .run_onboarding_csm(
                base_envelope(AgentId::RachelOnboardingCsm),
                onboarding_input(),
            )
            .unwrap();
        assert_eq!(result.status, TaskStatus::NeedsHuman);
        assert!(!result.output_payload.onboarding_plan.is_empty());
        assert!(result
            .output_payload
            .activation_risk_flags
            .iter()
            .any(|risk| risk.severity == RiskSeverity::Medium));
        assert!(result
            .emitted_events
            .iter()
            .any(|event| event.event_type == "onboarding.plan.started"));
    }

    #[test]
    fn experiment_publishes_recommendations() {
        let engine = Phase3AgentEngine;
        let result = engine
            .run_experiment_analyst(
                base_envelope(AgentId::RachelExperimentAnalyst),
                experiment_input(),
            )
            .unwrap();
        assert_eq!(result.status, TaskStatus::Succeeded);
        assert!(result.output_payload.result_summary.statistically_reliable);
        assert!(result
            .emitted_events
            .iter()
            .any(|event| event.event_type == "experiment.result.published"));
    }

    #[test]
    fn phase3_workflow_requires_reliable_metrics_gate() {
        let engine = Phase3AgentEngine;
        let workflow = Phase3WorkflowInput {
            base_envelope: base_envelope(AgentId::RachelOrchestrator),
            metrics_reliable: false,
            onboarding: onboarding_input(),
            experiment: experiment_input(),
        };
        let error = engine.run_workflow(workflow).unwrap_err();
        assert!(matches!(error, Phase3AgentError::MetricsNotReliable));
    }

    #[test]
    fn phase3_workflow_runs_onboarding_and_experiment() {
        let engine = Phase3AgentEngine;
        let workflow = Phase3WorkflowInput {
            base_envelope: base_envelope(AgentId::RachelOrchestrator),
            metrics_reliable: true,
            onboarding: onboarding_input(),
            experiment: experiment_input(),
        };

        let result = engine.run_workflow(workflow).unwrap();
        assert!(!result.events.is_empty());
        assert!(result
            .events
            .iter()
            .any(|event| event.event_type == "onboarding.feedback.captured"));
        assert!(result
            .events
            .iter()
            .any(|event| event.event_type == "experiment.recommendation.issued"));
    }
}
