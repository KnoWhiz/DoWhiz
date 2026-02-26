use std::cmp::Reverse;

use chrono::Utc;
use serde::Serialize;
use uuid::Uuid;

use super::contracts::{
    AgentId, AgentTaskEnvelope, AgentTaskResult, ApprovalRequest, AssetChannel, ContentAsset,
    ContentInput, ContentOutput, EventEnvelope, FunnelStage, MessageMap, PositioningBundle,
    PositioningInput, PositioningOutput, SubjectType, TaskStatus, GTM_SCHEMA_VERSION,
};

#[derive(Debug, thiserror::Error)]
pub enum Phase2AgentError {
    #[error("event payload serialization failed: {0}")]
    EventSerialization(#[from] serde_json::Error),
    #[error("data contracts are not stable; phase 2 rollout remains gated")]
    DataContractsNotStable,
}

#[derive(Debug, Clone)]
pub struct Phase2WorkflowInput {
    pub base_envelope: AgentTaskEnvelope,
    pub data_contracts_stable: bool,
    pub positioning: PositioningInput,
    pub content: ContentInput,
}

#[derive(Debug, Clone)]
pub struct Phase2WorkflowResult {
    pub positioning: AgentTaskResult<PositioningOutput>,
    pub content: AgentTaskResult<ContentOutput>,
    pub events: Vec<EventEnvelope>,
}

#[derive(Debug, Default, Clone)]
pub struct Phase2AgentEngine;

impl Phase2AgentEngine {
    pub fn run_workflow(
        &self,
        input: Phase2WorkflowInput,
    ) -> Result<Phase2WorkflowResult, Phase2AgentError> {
        if !input.data_contracts_stable {
            return Err(Phase2AgentError::DataContractsNotStable);
        }

        let positioning = self.run_positioning_pmm(
            input
                .base_envelope
                .with_agent(AgentId::RachelPositioningPmm),
            input.positioning,
        )?;

        let mut content_input = input.content;
        content_input.positioning_bundle = positioning.output_payload.positioning_bundle.clone();
        let content = self.run_content_studio(
            input.base_envelope.with_agent(AgentId::RachelContentStudio),
            content_input,
        )?;

        let mut events = Vec::new();
        events.extend(positioning.emitted_events.clone());
        events.extend(content.emitted_events.clone());

        Ok(Phase2WorkflowResult {
            positioning,
            content,
            events,
        })
    }

    pub fn run_positioning_pmm(
        &self,
        envelope: AgentTaskEnvelope,
        input: PositioningInput,
    ) -> Result<AgentTaskResult<PositioningOutput>, Phase2AgentError> {
        let top_themes = input
            .insight_clusters
            .iter()
            .map(|cluster| (cluster.frequency, cluster.theme.clone()))
            .collect::<Vec<_>>();
        let mut top_themes = top_themes;
        top_themes.sort_by_key(|(frequency, _)| Reverse(*frequency));
        let top_theme_labels = top_themes
            .into_iter()
            .take(3)
            .map(|(_, theme)| theme)
            .collect::<Vec<_>>();
        let proof_points = input
            .prd_drafts
            .iter()
            .take(3)
            .map(|prd| format!("PRD {} targets {}", prd.prd_id, prd.problem))
            .collect::<Vec<_>>();

        let message_maps = input
            .segment_definitions
            .iter()
            .map(|segment| MessageMap {
                segment_id: segment.segment_id.clone(),
                value_proposition: format!(
                    "Help {} segment convert faster with lower GTM friction",
                    segment.segment_id
                ),
                pains: if top_theme_labels.is_empty() {
                    vec!["feature discoverability".to_string()]
                } else {
                    top_theme_labels.clone()
                },
                proof_points: if proof_points.is_empty() {
                    vec!["No PRD draft linked yet; use baseline claim-safe messaging".to_string()]
                } else {
                    proof_points.clone()
                },
                objection_handling: vec![
                    "Start with one workflow and expand after KPI verification".to_string(),
                    "Use explicit approval gates for risky claims".to_string(),
                    format!("Validated against contract {}", input.data_contract_version),
                ],
                funnel_stage: if segment.segment_id.contains("fast_activation") {
                    FunnelStage::Awareness
                } else if segment.segment_id.contains("tier_a") {
                    FunnelStage::Decision
                } else {
                    FunnelStage::Consideration
                },
            })
            .collect::<Vec<_>>();

        let claim_safe_list = {
            let mut safe_claims = vec![
                "improve activation visibility".to_string(),
                "reduce manual handoff overhead".to_string(),
                "faster campaign iteration loop".to_string(),
            ];
            for theme in &input.strategic_themes {
                safe_claims.push(format!("aligned with strategic theme: {}", theme));
            }
            safe_claims
        };

        let bundle = PositioningBundle {
            bundle_id: format!("positioning-{}", envelope.task_id.simple()),
            message_maps: message_maps.clone(),
            claim_safe_list,
            generated_at: Utc::now(),
        };
        let output = PositioningOutput {
            positioning_bundle: bundle,
        };

        let mut events = Vec::new();
        for map in &message_maps {
            events.push(self.make_event(
                &envelope,
                AgentId::RachelPositioningPmm,
                "positioning.message_map.generated",
                SubjectType::Feature,
                envelope.objective_id,
                map,
            )?);
        }
        events.push(self.make_event(
            &envelope,
            AgentId::RachelPositioningPmm,
            "positioning.bundle.published",
            SubjectType::Feature,
            envelope.objective_id,
            &output.positioning_bundle,
        )?);

        let partial = output.positioning_bundle.message_maps.is_empty();
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
                0.35
            } else {
                (0.58 + (input.segment_definitions.len().min(5) as f32 * 0.06)).min(0.9)
            },
            evidence_refs: envelope.input_refs.clone(),
            next_action: if partial {
                "Need at least one promoted segment to generate message maps".to_string()
            } else {
                "Publish positioning bundle for content generation".to_string()
            },
            errors: if partial {
                vec!["no segment definitions available for positioning".to_string()]
            } else {
                Vec::new()
            },
        })
    }

    pub fn run_content_studio(
        &self,
        envelope: AgentTaskEnvelope,
        input: ContentInput,
    ) -> Result<AgentTaskResult<ContentOutput>, Phase2AgentError> {
        let no_channels = input.channels.is_empty();
        let no_message_maps = input.positioning_bundle.message_maps.is_empty();
        let asset_limit = input.max_assets_per_channel.clamp(1, 4) as usize;

        let mut errors = Vec::new();
        if no_channels {
            errors.push("content channels list is empty".to_string());
        }
        if no_message_maps {
            errors.push("positioning bundle has no message maps".to_string());
        }

        let mut assets = Vec::new();
        if !no_channels && !no_message_maps {
            for channel in &input.channels {
                for (idx, message_map) in input
                    .positioning_bundle
                    .message_maps
                    .iter()
                    .take(asset_limit)
                    .enumerate()
                {
                    assets.push(build_asset(
                        &input.positioning_bundle.bundle_id,
                        message_map,
                        *channel,
                        idx + 1,
                    ));
                }
            }
        }

        let publish_ready =
            !input.requires_human_review && !envelope.policy_pack.human_approval_required;
        let output = ContentOutput {
            assets,
            publish_ready,
        };

        let mut events = Vec::new();
        for asset in &output.assets {
            events.push(self.make_event(
                &envelope,
                AgentId::RachelContentStudio,
                "content.asset.drafted",
                SubjectType::Campaign,
                envelope.objective_id,
                asset,
            )?);
        }
        if output.assets.is_empty() {
            // no additional events
        } else if output.publish_ready {
            events.push(self.make_event(
                &envelope,
                AgentId::RachelContentStudio,
                "content.asset.published",
                SubjectType::Campaign,
                envelope.objective_id,
                &serde_json::json!({
                    "asset_count": output.assets.len(),
                    "bundle_id": input.positioning_bundle.bundle_id,
                }),
            )?);
        } else {
            let approval = ApprovalRequest {
                reason: "Content assets require human review before publishing".to_string(),
                risk_level: "medium".to_string(),
                reviewer_group: "content_ops".to_string(),
            };
            events.push(self.make_event(
                &envelope,
                AgentId::RachelContentStudio,
                "approval.requested",
                SubjectType::Campaign,
                envelope.objective_id,
                &approval,
            )?);
        }

        let status = if no_channels || no_message_maps {
            TaskStatus::Failed
        } else if output.publish_ready {
            TaskStatus::Succeeded
        } else {
            TaskStatus::NeedsHuman
        };

        Ok(AgentTaskResult {
            task_id: envelope.task_id,
            status,
            schema_version: GTM_SCHEMA_VERSION.to_string(),
            output_payload: output,
            emitted_events: events,
            confidence: match status {
                TaskStatus::Failed => 0.3,
                TaskStatus::NeedsHuman => 0.65,
                _ => 0.82,
            },
            evidence_refs: envelope.input_refs.clone(),
            next_action: match status {
                TaskStatus::Failed => {
                    "Provide valid channels and positioning message maps for content generation"
                        .to_string()
                }
                TaskStatus::NeedsHuman => {
                    "Collect human review approval then publish drafted assets".to_string()
                }
                _ => "Distribute published content assets by channel".to_string(),
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
    ) -> Result<EventEnvelope, Phase2AgentError> {
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

fn build_asset(
    bundle_id: &str,
    message_map: &MessageMap,
    channel: AssetChannel,
    ordinal: usize,
) -> ContentAsset {
    let channel_name = match channel {
        AssetChannel::Email => "email",
        AssetChannel::LandingPage => "landing",
        AssetChannel::LinkedinAd => "linkedin-ad",
        AssetChannel::SalesOnePager => "sales-one-pager",
    };

    let title = match channel {
        AssetChannel::Email => format!("{}: concise intro", message_map.segment_id),
        AssetChannel::LandingPage => format!("{}: value narrative", message_map.segment_id),
        AssetChannel::LinkedinAd => format!("{}: social proof angle", message_map.segment_id),
        AssetChannel::SalesOnePager => format!("{}: enablement brief", message_map.segment_id),
    };
    let body = format!(
        "Value: {}.\nPain: {}.\nProof: {}.",
        message_map.value_proposition,
        message_map
            .pains
            .first()
            .cloned()
            .unwrap_or_else(|| "unspecified".to_string()),
        message_map
            .proof_points
            .first()
            .cloned()
            .unwrap_or_else(|| "pending proof point".to_string())
    );

    ContentAsset {
        asset_id: format!(
            "{}-{}-{}-{}",
            bundle_id, channel_name, message_map.segment_id, ordinal
        ),
        channel,
        segment_id: message_map.segment_id.clone(),
        title,
        body,
        cta: "Book a 20-minute workflow fit session".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use super::super::contracts::{
        AgentId, AgentTaskEnvelope, AssetChannel, ContentInput, FunnelStage, InsightCluster,
        MessageMap, PolicyPack, PositioningBundle, PositioningInput, PrdDraft, SegmentDefinition,
        TaskPriority, TaskStatus,
    };
    use super::*;

    fn base_envelope(agent_id: AgentId) -> AgentTaskEnvelope {
        let mut envelope = AgentTaskEnvelope::new(agent_id);
        envelope.priority = TaskPriority::High;
        envelope.policy_pack = PolicyPack::default();
        envelope.input_refs = vec!["warehouse://contract_snapshot".to_string()];
        envelope
    }

    fn positioning_input() -> PositioningInput {
        PositioningInput {
            segment_definitions: vec![
                SegmentDefinition {
                    segment_id: "tier_a_high_fit".to_string(),
                    rule_dsl: "icp_tier == 'A'".to_string(),
                    expected_lift: 1.32,
                    confidence: 0.81,
                },
                SegmentDefinition {
                    segment_id: "fast_activation".to_string(),
                    rule_dsl: "activation_days <= 7".to_string(),
                    expected_lift: 1.2,
                    confidence: 0.75,
                },
            ],
            insight_clusters: vec![InsightCluster {
                cluster_id: "cluster_integration_gap".to_string(),
                theme: "integration gap".to_string(),
                frequency: 4,
                affected_segments: vec!["tier_a_high_fit".to_string()],
                evidence_refs: vec!["ticket:123".to_string()],
            }],
            prd_drafts: vec![PrdDraft {
                prd_id: "prd_cluster_integration_gap".to_string(),
                problem: "integration gap occurred often".to_string(),
                users: vec!["tier_a_high_fit".to_string()],
                success_metrics: vec!["activation_rate_day_14".to_string()],
                scope: vec!["hubspot sync reliability".to_string()],
                risks: vec!["dependency risk".to_string()],
            }],
            strategic_themes: vec!["activation".to_string()],
            data_contract_version: "1.0".to_string(),
        }
    }

    #[test]
    fn positioning_generates_bundle_and_events() {
        let engine = Phase2AgentEngine;
        let envelope = base_envelope(AgentId::RachelPositioningPmm);
        let result = engine
            .run_positioning_pmm(envelope, positioning_input())
            .unwrap();
        assert_eq!(result.status, TaskStatus::Succeeded);
        assert_eq!(
            result.output_payload.positioning_bundle.message_maps.len(),
            2
        );
        assert!(result
            .emitted_events
            .iter()
            .any(|event| event.event_type == "positioning.bundle.published"));
    }

    #[test]
    fn content_requires_human_when_policy_gate_enabled() {
        let engine = Phase2AgentEngine;
        let mut envelope = base_envelope(AgentId::RachelContentStudio);
        envelope.policy_pack.human_approval_required = true;
        let input = ContentInput {
            positioning_bundle: PositioningBundle {
                bundle_id: "positioning-x".to_string(),
                message_maps: vec![MessageMap {
                    segment_id: "tier_a_high_fit".to_string(),
                    value_proposition: "Speed up GTM execution".to_string(),
                    pains: vec!["integration gap".to_string()],
                    proof_points: vec!["PRD linked".to_string()],
                    objection_handling: vec!["Start small".to_string()],
                    funnel_stage: FunnelStage::Decision,
                }],
                claim_safe_list: vec!["activation visibility".to_string()],
                generated_at: Utc::now(),
            },
            channels: vec![AssetChannel::Email],
            max_assets_per_channel: 2,
            requires_human_review: false,
        };
        let result = engine.run_content_studio(envelope, input).unwrap();
        assert_eq!(result.status, TaskStatus::NeedsHuman);
        assert!(result
            .emitted_events
            .iter()
            .any(|event| event.event_type == "approval.requested"));
    }

    #[test]
    fn phase2_workflow_enforces_contract_stability_gate() {
        let engine = Phase2AgentEngine;
        let workflow = Phase2WorkflowInput {
            base_envelope: base_envelope(AgentId::RachelOrchestrator),
            data_contracts_stable: false,
            positioning: positioning_input(),
            content: ContentInput {
                positioning_bundle: PositioningBundle {
                    bundle_id: "placeholder".to_string(),
                    message_maps: Vec::new(),
                    claim_safe_list: Vec::new(),
                    generated_at: Utc::now(),
                },
                channels: vec![AssetChannel::Email],
                max_assets_per_channel: 1,
                requires_human_review: true,
            },
        };
        let error = engine.run_workflow(workflow).unwrap_err();
        assert!(matches!(error, Phase2AgentError::DataContractsNotStable));
    }

    #[test]
    fn phase2_workflow_runs_positioning_then_content() {
        let engine = Phase2AgentEngine;
        let workflow = Phase2WorkflowInput {
            base_envelope: base_envelope(AgentId::RachelOrchestrator),
            data_contracts_stable: true,
            positioning: positioning_input(),
            content: ContentInput {
                positioning_bundle: PositioningBundle {
                    bundle_id: "placeholder".to_string(),
                    message_maps: Vec::new(),
                    claim_safe_list: Vec::new(),
                    generated_at: Utc::now(),
                },
                channels: vec![AssetChannel::Email, AssetChannel::LinkedinAd],
                max_assets_per_channel: 2,
                requires_human_review: false,
            },
        };

        let result = engine.run_workflow(workflow).unwrap();
        assert_eq!(result.positioning.status, TaskStatus::Succeeded);
        assert_eq!(result.content.status, TaskStatus::Succeeded);
        assert!(!result.content.output_payload.assets.is_empty());
        assert!(result
            .events
            .iter()
            .any(|event| event.event_type == "content.asset.published"));
    }
}
