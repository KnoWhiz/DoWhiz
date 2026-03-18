use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::workspace_blueprint::StartupWorkspaceBlueprint;
use crate::TaskStatusSummary;

use super::provider_state::WorkspaceProviderRuntimeState;

const DISMISS_COOLDOWN_DAYS: i64 = 7;
const IDLE_WORKSPACE_DAYS: i64 = 7;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProactivityLevel {
    Off,
    Minimal,
    Helpful,
    HandsOn,
}

impl Default for ProactivityLevel {
    fn default() -> Self {
        Self::Minimal
    }
}

impl ProactivityLevel {
    pub fn from_storage_value(value: &str) -> Self {
        match value.trim().to_ascii_lowercase().as_str() {
            "off" => Self::Off,
            "helpful" => Self::Helpful,
            "hands_on" | "hands-on" | "handson" => Self::HandsOn,
            _ => Self::Minimal,
        }
    }

    pub fn as_storage_value(&self) -> &'static str {
        match self {
            Self::Off => "off",
            Self::Minimal => "minimal",
            Self::Helpful => "helpful",
            Self::HandsOn => "hands_on",
        }
    }

    pub fn allows_dashboard_recommendations(&self) -> bool {
        !matches!(self, Self::Off)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RecommendationFeedbackKind {
    Shown,
    Accepted,
    Dismissed,
    Deferred,
}

impl RecommendationFeedbackKind {
    pub fn from_storage_value(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "shown" => Some(Self::Shown),
            "accepted" => Some(Self::Accepted),
            "dismissed" => Some(Self::Dismissed),
            "deferred" => Some(Self::Deferred),
            _ => None,
        }
    }

    pub fn as_storage_value(&self) -> &'static str {
        match self {
            Self::Shown => "shown",
            Self::Accepted => "accepted",
            Self::Dismissed => "dismissed",
            Self::Deferred => "deferred",
        }
    }

    fn suppresses_same_state(&self) -> bool {
        matches!(self, Self::Dismissed | Self::Deferred)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorkspaceRecommendationRequest {
    pub blueprint: StartupWorkspaceBlueprint,
    #[serde(default)]
    pub client_timezone: Option<String>,
    #[serde(default)]
    pub current_surface: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorkspaceRecommendationResponse {
    pub recommendation: Option<WorkspaceRecommendation>,
    #[serde(default)]
    pub alternatives: Vec<WorkspaceRecommendation>,
    pub preferences: WorkspaceRecommendationPreferences,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorkspaceRecommendationPreferences {
    pub proactivity_level: ProactivityLevel,
    pub effective_proactivity_level: ProactivityLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorkspaceRecommendation {
    pub recommendation_key: String,
    pub trigger_type: String,
    pub title: String,
    pub why_now: String,
    pub outcome: String,
    pub state_signature: String,
    pub action: WorkspaceRecommendationAction,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum WorkspaceRecommendationAction {
    ConnectProvider { provider: String },
    OpenDashboardSection { section: String },
    TriggerCreateBrief,
    UpdateTeamBrief,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorkspaceRecommendationFeedbackRequest {
    pub recommendation_key: String,
    pub state_signature: String,
    pub feedback: RecommendationFeedbackKind,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorkspaceRecommendationPreferencesUpdateRequest {
    pub proactivity_level: ProactivityLevel,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecommendationFeedbackSnapshot {
    pub recommendation_key: String,
    pub state_signature: String,
    pub feedback: RecommendationFeedbackKind,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct WorkspaceRecommendationContext<'a> {
    pub account_created_at: DateTime<Utc>,
    pub blueprint: &'a StartupWorkspaceBlueprint,
    pub provider_runtime: &'a WorkspaceProviderRuntimeState,
    pub recent_tasks: &'a [TaskStatusSummary],
    pub proactivity_level: ProactivityLevel,
    pub recent_feedback: &'a [RecommendationFeedbackSnapshot],
    pub now: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CandidateRecommendation {
    recommendation: WorkspaceRecommendation,
    priority: u8,
}

pub fn evaluate_workspace_recommendations(
    context: WorkspaceRecommendationContext<'_>,
) -> WorkspaceRecommendationResponse {
    let preferences = WorkspaceRecommendationPreferences {
        proactivity_level: context.proactivity_level,
        effective_proactivity_level: context.proactivity_level,
    };

    if !preferences
        .effective_proactivity_level
        .allows_dashboard_recommendations()
    {
        return WorkspaceRecommendationResponse {
            recommendation: None,
            alternatives: Vec::new(),
            preferences,
        };
    }

    let mut candidates = build_candidates(&context);
    candidates.retain(|candidate| !is_suppressed(candidate, &context));
    candidates.sort_by(|left, right| right.priority.cmp(&left.priority));

    let recommendation = candidates
        .first()
        .map(|candidate| candidate.recommendation.clone());
    let alternatives = candidates
        .iter()
        .skip(1)
        .take(2)
        .map(|candidate| candidate.recommendation.clone())
        .collect();

    WorkspaceRecommendationResponse {
        recommendation,
        alternatives,
        preferences,
    }
}

fn build_candidates(context: &WorkspaceRecommendationContext<'_>) -> Vec<CandidateRecommendation> {
    let mut candidates = Vec::new();

    if let Some(candidate) = build_failed_task_candidate(context) {
        candidates.push(candidate);
    }

    if let Some(candidate) = build_github_setup_candidate(context) {
        candidates.push(candidate);
    }

    if let Some(candidate) = build_coordination_setup_candidate(context) {
        candidates.push(candidate);
    }

    if let Some(candidate) = build_create_brief_candidate(context) {
        candidates.push(candidate);
    }

    if let Some(candidate) = build_idle_workspace_candidate(context) {
        candidates.push(candidate);
    }

    candidates
}

fn build_failed_task_candidate(
    context: &WorkspaceRecommendationContext<'_>,
) -> Option<CandidateRecommendation> {
    let failed_task = latest_task_matching(context.recent_tasks, |task| {
        task.execution_status.as_deref() == Some("failed")
    })?;

    let summary = task_display_summary(failed_task);
    Some(CandidateRecommendation {
        recommendation: WorkspaceRecommendation {
            recommendation_key: "active_blocker:failed_task".to_string(),
            trigger_type: "active_blocker".to_string(),
            title: format!("Review failed task: {}", summary),
            why_now: format!(
                "\"{}\" failed the last time it ran, so the workspace has an active blocker.",
                summary
            ),
            outcome:
                "You can inspect the error, adjust the request, or retry from the task center."
                    .to_string(),
            state_signature: format!("active_blocker:failed_task:{}", failed_task.id),
            action: WorkspaceRecommendationAction::OpenDashboardSection {
                section: "section-tasks".to_string(),
            },
        },
        priority: 100,
    })
}

fn build_github_setup_candidate(
    context: &WorkspaceRecommendationContext<'_>,
) -> Option<CandidateRecommendation> {
    let blueprint = context.blueprint;
    let runtime = context.provider_runtime;
    let needs_build_workflows = blueprint.stack.has_existing_repo
        || requested_channel(blueprint, "github")
        || requested_channel(blueprint, "gitlab")
        || requested_channel(blueprint, "bitbucket");

    if !needs_build_workflows || runtime.connected.github || !runtime.capabilities.github {
        return None;
    }

    Some(CandidateRecommendation {
        recommendation: WorkspaceRecommendation {
            recommendation_key: "setup_blocker:github".to_string(),
            trigger_type: "setup_blocker".to_string(),
            title: "Connect GitHub".to_string(),
            why_now: "Your workspace expects repository-based delivery, but GitHub is not linked yet."
                .to_string(),
            outcome:
                "This unlocks implementation, code review, and repository-driven execution workflows."
                    .to_string(),
            state_signature: "setup_blocker:github:not_connected".to_string(),
            action: WorkspaceRecommendationAction::ConnectProvider {
                provider: "github".to_string(),
            },
        },
        priority: 80,
    })
}

fn build_coordination_setup_candidate(
    context: &WorkspaceRecommendationContext<'_>,
) -> Option<CandidateRecommendation> {
    let blueprint = context.blueprint;
    let runtime = context.provider_runtime;
    let wants_slack = requested_channel(blueprint, "slack");
    let wants_discord = requested_channel(blueprint, "discord");

    if wants_slack && !runtime.connected.slack && runtime.capabilities.slack {
        return Some(CandidateRecommendation {
            recommendation: WorkspaceRecommendation {
                recommendation_key: "setup_blocker:slack".to_string(),
                trigger_type: "setup_blocker".to_string(),
                title: "Connect Slack".to_string(),
                why_now:
                    "Slack is part of your requested coordination flow, but the workspace is not linked yet."
                        .to_string(),
                outcome:
                    "This gives your team a shared place for status updates, approvals, and quick handoffs."
                        .to_string(),
                state_signature: "setup_blocker:slack:not_connected".to_string(),
                action: WorkspaceRecommendationAction::ConnectProvider {
                    provider: "slack".to_string(),
                },
            },
            priority: 70,
        });
    }

    if wants_discord && !runtime.connected.discord && runtime.capabilities.discord {
        return Some(CandidateRecommendation {
            recommendation: WorkspaceRecommendation {
                recommendation_key: "setup_blocker:discord".to_string(),
                trigger_type: "setup_blocker".to_string(),
                title: "Connect Discord".to_string(),
                why_now:
                    "Discord is part of your requested coordination flow, but the workspace is not linked yet."
                        .to_string(),
                outcome:
                    "This gives your team a shared place for status updates, approvals, and quick handoffs."
                        .to_string(),
                state_signature: "setup_blocker:discord:not_connected".to_string(),
                action: WorkspaceRecommendationAction::ConnectProvider {
                    provider: "discord".to_string(),
                },
            },
            priority: 69,
        });
    }

    None
}

fn build_create_brief_candidate(
    context: &WorkspaceRecommendationContext<'_>,
) -> Option<CandidateRecommendation> {
    if has_successful_task(context.recent_tasks) || has_workspace_brief_task(context.recent_tasks) {
        return None;
    }

    Some(CandidateRecommendation {
        recommendation: WorkspaceRecommendation {
            recommendation_key: "first_value_gap:create_brief".to_string(),
            trigger_type: "first_value_gap".to_string(),
            title: "Create startup brief".to_string(),
            why_now:
                "You already have enough founder context saved, but no shared kickoff artifact has been created yet."
                    .to_string(),
            outcome:
                "This gives you a first deliverable and a concrete base for follow-on execution."
                    .to_string(),
            state_signature: "first_value_gap:create_brief:no_success_task".to_string(),
            action: WorkspaceRecommendationAction::TriggerCreateBrief,
        },
        priority: 60,
    })
}

fn build_idle_workspace_candidate(
    context: &WorkspaceRecommendationContext<'_>,
) -> Option<CandidateRecommendation> {
    if !has_successful_task(context.recent_tasks) {
        return None;
    }

    let last_activity =
        latest_activity_at(context.recent_tasks).unwrap_or(context.account_created_at);
    if context.now - last_activity < Duration::days(IDLE_WORKSPACE_DAYS) {
        return None;
    }

    Some(CandidateRecommendation {
        recommendation: WorkspaceRecommendation {
            recommendation_key: "idle_workspace:review_tasks".to_string(),
            trigger_type: "idle_workspace".to_string(),
            title: "Re-open your task loop".to_string(),
            why_now:
                "This workspace has been quiet for a while after earlier activity, so momentum may be stalled."
                    .to_string(),
            outcome:
                "You can restart work from existing context instead of re-explaining everything from scratch."
                    .to_string(),
            state_signature: "idle_workspace:review_tasks".to_string(),
            action: WorkspaceRecommendationAction::OpenDashboardSection {
                section: "section-tasks".to_string(),
            },
        },
        priority: 20,
    })
}

fn is_suppressed(
    candidate: &CandidateRecommendation,
    context: &WorkspaceRecommendationContext<'_>,
) -> bool {
    let cooldown_window = Duration::days(DISMISS_COOLDOWN_DAYS);
    context.recent_feedback.iter().any(|feedback| {
        feedback.recommendation_key == candidate.recommendation.recommendation_key
            && feedback.state_signature == candidate.recommendation.state_signature
            && feedback.feedback.suppresses_same_state()
            && context.now - feedback.created_at < cooldown_window
    })
}

fn requested_channel(blueprint: &StartupWorkspaceBlueprint, needle: &str) -> bool {
    let needle_lower = needle.to_ascii_lowercase();
    blueprint
        .preferred_channels
        .iter()
        .any(|channel| channel.to_ascii_lowercase().contains(&needle_lower))
}

fn has_successful_task(tasks: &[TaskStatusSummary]) -> bool {
    tasks
        .iter()
        .any(|task| task.execution_status.as_deref() == Some("success"))
}

fn has_workspace_brief_task(tasks: &[TaskStatusSummary]) -> bool {
    tasks.iter().any(|task| {
        task.request_summary
            .as_deref()
            .map(|summary| {
                summary
                    .trim()
                    .to_ascii_lowercase()
                    .starts_with("create workspace brief")
            })
            .unwrap_or(false)
    })
}

fn latest_activity_at(tasks: &[TaskStatusSummary]) -> Option<DateTime<Utc>> {
    tasks.iter().filter_map(task_last_activity_at).max()
}

fn latest_task_matching<'a, F>(
    tasks: &'a [TaskStatusSummary],
    predicate: F,
) -> Option<&'a TaskStatusSummary>
where
    F: Fn(&TaskStatusSummary) -> bool,
{
    tasks
        .iter()
        .filter(|task| predicate(task))
        .max_by_key(|task| task_last_activity_at(task))
}

fn task_last_activity_at(task: &TaskStatusSummary) -> Option<DateTime<Utc>> {
    task.execution_started_at
        .as_deref()
        .and_then(parse_rfc3339)
        .or_else(|| task.last_run.as_deref().and_then(parse_rfc3339))
        .or_else(|| parse_rfc3339(&task.created_at))
}

fn parse_rfc3339(raw: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(raw)
        .ok()
        .map(|value| value.with_timezone(&Utc))
}

fn task_display_summary(task: &TaskStatusSummary) -> String {
    let summary = task
        .request_summary
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("Untitled task");

    truncate(summary, 72)
}

fn truncate(value: &str, max_chars: usize) -> String {
    let mut chars = value.chars();
    let truncated: String = chars.by_ref().take(max_chars).collect();
    if chars.next().is_some() {
        format!("{}...", truncated.trim_end())
    } else {
        truncated
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    fn make_context<'a>(
        blueprint: &'a StartupWorkspaceBlueprint,
        provider_runtime: &'a WorkspaceProviderRuntimeState,
        recent_tasks: &'a [TaskStatusSummary],
        recent_feedback: &'a [RecommendationFeedbackSnapshot],
    ) -> WorkspaceRecommendationContext<'a> {
        WorkspaceRecommendationContext {
            account_created_at: Utc::now() - Duration::days(10),
            blueprint,
            provider_runtime,
            recent_tasks,
            proactivity_level: ProactivityLevel::Minimal,
            recent_feedback,
            now: Utc::now(),
        }
    }

    fn base_blueprint() -> StartupWorkspaceBlueprint {
        StartupWorkspaceBlueprint {
            founder: crate::domain::workspace_blueprint::FounderProfile {
                name: "Founder".to_string(),
                email: "founder@example.com".to_string(),
            },
            venture: crate::domain::workspace_blueprint::VentureProfile {
                name: "Acme".to_string(),
                thesis: "Build AI coordination workflows".to_string(),
                stage: Some("mvp".to_string()),
            },
            goals_30_90_days: vec!["Launch MVP".to_string()],
            ..StartupWorkspaceBlueprint::default()
        }
    }

    fn runtime() -> WorkspaceProviderRuntimeState {
        WorkspaceProviderRuntimeState {
            has_account: true,
            capabilities: super::super::provider_state::ProviderCapabilitySnapshot {
                github: true,
                google_docs: true,
                email: true,
                slack: true,
                discord: true,
            },
            connected: super::super::provider_state::ProviderConnectionSnapshot::default(),
        }
    }

    fn task(
        id: &str,
        execution_status: Option<&str>,
        request_summary: Option<&str>,
    ) -> TaskStatusSummary {
        TaskStatusSummary {
            id: id.to_string(),
            kind: "run_task".to_string(),
            channel: "email".to_string(),
            request_summary: request_summary.map(|value| value.to_string()),
            enabled: true,
            created_at: Utc::now().to_rfc3339(),
            last_run: Some(Utc::now().to_rfc3339()),
            schedule_type: "one_shot".to_string(),
            next_run: None,
            run_at: None,
            execution_status: execution_status.map(|value| value.to_string()),
            error_message: None,
            execution_started_at: Some(Utc::now().to_rfc3339()),
        }
    }

    #[test]
    fn recommends_github_setup_when_build_workflow_is_requested() {
        let mut blueprint = base_blueprint();
        blueprint.stack.has_existing_repo = true;
        blueprint.preferred_channels = vec!["GitHub".to_string()];
        let runtime = runtime();

        let response =
            evaluate_workspace_recommendations(make_context(&blueprint, &runtime, &[], &[]));

        let recommendation = response.recommendation.expect("recommendation");
        assert_eq!(recommendation.recommendation_key, "setup_blocker:github");
    }

    #[test]
    fn failed_task_outranks_setup_recommendation() {
        let mut blueprint = base_blueprint();
        blueprint.stack.has_existing_repo = true;
        blueprint.preferred_channels = vec!["GitHub".to_string()];
        let runtime = runtime();
        let tasks = vec![task("task-1", Some("failed"), Some("Fix customer reply"))];

        let response =
            evaluate_workspace_recommendations(make_context(&blueprint, &runtime, &tasks, &[]));

        let recommendation = response.recommendation.expect("recommendation");
        assert_eq!(recommendation.trigger_type, "active_blocker");
        assert!(recommendation.title.contains("Review failed task"));
        assert!(!response.alternatives.is_empty());
        assert!(response
            .alternatives
            .iter()
            .any(|alternative| alternative.recommendation_key == "setup_blocker:github"));
    }

    #[test]
    fn dismissal_suppresses_same_recommendation_state() {
        let mut blueprint = base_blueprint();
        blueprint.stack.has_existing_repo = true;
        blueprint.preferred_channels = vec!["GitHub".to_string()];
        let runtime = runtime();
        let feedback = vec![RecommendationFeedbackSnapshot {
            recommendation_key: "setup_blocker:github".to_string(),
            state_signature: "setup_blocker:github:not_connected".to_string(),
            feedback: RecommendationFeedbackKind::Dismissed,
            created_at: Utc::now(),
        }];

        let response =
            evaluate_workspace_recommendations(make_context(&blueprint, &runtime, &[], &feedback));

        assert_eq!(
            response
                .recommendation
                .as_ref()
                .map(|value| value.recommendation_key.as_str()),
            Some("first_value_gap:create_brief")
        );
    }

    #[test]
    fn create_brief_is_not_recommended_after_successful_work() {
        let blueprint = base_blueprint();
        let runtime = runtime();
        let tasks = vec![task(
            "task-1",
            Some("success"),
            Some("Create Workspace Brief for Acme"),
        )];

        let response =
            evaluate_workspace_recommendations(make_context(&blueprint, &runtime, &tasks, &[]));

        assert_ne!(
            response
                .recommendation
                .as_ref()
                .map(|value| value.recommendation_key.as_str()),
            Some("first_value_gap:create_brief")
        );
    }

    #[test]
    fn off_preference_returns_no_recommendation() {
        let blueprint = base_blueprint();
        let runtime = runtime();
        let mut context = make_context(&blueprint, &runtime, &[], &[]);
        context.proactivity_level = ProactivityLevel::Off;

        let response = evaluate_workspace_recommendations(context);

        assert!(response.recommendation.is_none());
        assert!(response.alternatives.is_empty());
    }
}
