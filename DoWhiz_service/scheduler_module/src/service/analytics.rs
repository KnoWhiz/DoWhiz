use axum::extract::{Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use chrono::{DateTime, Datelike, Duration, NaiveDate, Utc, Weekday};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::Arc;
use tokio::task;
use tracing::{error, info, warn};
use uuid::Uuid;

use crate::account_store::{
    Account, AccountStore, AnalyticsEventInsert, AnalyticsEventRecord, Payment,
};

use super::auth::{extract_bearer_token, validate_supabase_token};

const DEFAULT_RANGE_DAYS: i64 = 30;
const MAX_RANGE_DAYS: i64 = 365;

#[derive(Clone)]
pub struct AnalyticsState {
    pub account_store: Arc<AccountStore>,
    pub supabase_url: String,
    pub environment: String,
    pub admin_emails: Arc<HashSet<String>>,
}

impl AnalyticsState {
    pub fn from_env(account_store: Arc<AccountStore>) -> Self {
        let supabase_url = std::env::var("SUPABASE_PROJECT_URL")
            .unwrap_or_else(|_| "https://resmseutzmwumflevfqw.supabase.co".to_string());
        let environment = std::env::var("DEPLOY_TARGET")
            .ok()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty())
            .unwrap_or_else(|| "production".to_string());
        let admin_emails = std::env::var("ANALYTICS_ADMIN_EMAILS")
            .unwrap_or_else(|_| "admin@dowhiz.com,oliver@dowhiz.com".to_string());

        Self {
            account_store,
            supabase_url,
            environment,
            admin_emails: Arc::new(parse_admin_emails(&admin_emails)),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct TrackEventRequest {
    pub event_name: String,
    pub event_timestamp: Option<String>,
    pub source: Option<String>,
    pub anonymous_id: Option<String>,
    pub session_id: Option<String>,
    pub workspace_id: Option<String>,
    pub org_id: Option<String>,
    pub plan_type: Option<String>,
    pub environment: Option<String>,
    pub app_version: Option<String>,
    pub page_path: Option<String>,
    pub route_path: Option<String>,
    pub referrer: Option<String>,
    pub utm_source: Option<String>,
    pub utm_medium: Option<String>,
    pub utm_campaign: Option<String>,
    pub utm_term: Option<String>,
    pub utm_content: Option<String>,
    pub device_type: Option<String>,
    pub browser: Option<String>,
    pub os: Option<String>,
    pub event_key: Option<String>,
    pub properties: Option<Value>,
}

#[derive(Debug, Serialize)]
pub struct TrackEventResponse {
    pub accepted: bool,
    pub deduped: bool,
}

#[derive(Debug, Deserialize)]
pub struct DashboardQuery {
    pub start: Option<String>,
    pub end: Option<String>,
    pub range: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DateRangeSummary {
    pub start: String,
    pub end: String,
    pub days: i64,
}

#[derive(Debug, Serialize)]
pub struct ExecutiveKpis {
    pub unique_visitors: i64,
    pub signup_conversion_rate: f64,
    pub signup_to_activation_rate: f64,
    pub activation_to_paid_rate: f64,
    pub visitor_to_paid_rate: f64,
    pub activation_rate: f64,
    pub repeat_value_rate: f64,
    pub median_time_to_first_value_hours: Option<f64>,
    pub d7_retention_rate: f64,
    pub active_workspaces: i64,
    pub active_paid_accounts_30d: i64,
    pub revenue_usd: f64,
}

#[derive(Debug, Serialize)]
pub struct FunnelStepSummary {
    pub event_name: String,
    pub label: String,
    pub identities: i64,
    pub step_conversion_rate: f64,
    pub overall_conversion_rate: f64,
}

#[derive(Debug, Serialize)]
pub struct FunnelSummary {
    pub steps: Vec<FunnelStepSummary>,
}

#[derive(Debug, Serialize)]
pub struct AcquisitionRow {
    pub key: String,
    pub visitors: i64,
    pub signups: i64,
    pub activated: i64,
    pub paid: i64,
    pub signup_conversion_rate: f64,
    pub activation_rate: f64,
    pub paid_conversion_rate: f64,
}

#[derive(Debug, Serialize)]
pub struct AcquisitionSummary {
    pub by_source_campaign: Vec<AcquisitionRow>,
    pub by_referrer: Vec<AcquisitionRow>,
    pub by_device_type: Vec<AcquisitionRow>,
    pub by_landing_variant: Vec<AcquisitionRow>,
}

#[derive(Debug, Serialize)]
pub struct BreakdownRow {
    pub key: String,
    pub count: i64,
    pub rate: f64,
}

#[derive(Debug, Serialize)]
pub struct ActivationSummary {
    pub by_auth_method: Vec<BreakdownRow>,
    pub by_workspace_type: Vec<BreakdownRow>,
    pub by_connected_channel_type: Vec<BreakdownRow>,
    pub by_first_task_type: Vec<BreakdownRow>,
    pub agent_or_workflow_creation_rate: f64,
    pub multi_channel_connection_rate: f64,
}

#[derive(Debug, Serialize)]
pub struct MonetizationSummary {
    pub pricing_page_views: i64,
    pub upgrade_clicks: i64,
    pub paywall_views: i64,
    pub checkout_starts: i64,
    pub checkout_abandon_rate: f64,
    pub payment_succeeded: i64,
    pub subscription_activated: i64,
    pub subscription_renewed: i64,
    pub subscription_canceled: i64,
    pub trial_to_paid_rate: Option<f64>,
    pub plan_mix: Vec<BreakdownRow>,
}

#[derive(Debug, Serialize)]
pub struct TrendPoint {
    pub day: String,
    pub count: i64,
}

#[derive(Debug, Serialize)]
pub struct StickinessSummary {
    pub dau: i64,
    pub wau: i64,
    pub mau: i64,
    pub dau_wau_ratio: f64,
    pub dau_mau_ratio: f64,
}

#[derive(Debug, Serialize)]
pub struct CohortRow {
    pub cohort_week: String,
    pub users: i64,
    pub d1_retention_rate: f64,
    pub d7_retention_rate: f64,
    pub d30_retention_rate: f64,
}

#[derive(Debug, Serialize)]
pub struct RetentionSummary {
    pub d1_retention_rate: f64,
    pub d7_retention_rate: f64,
    pub d30_retention_rate: f64,
    pub repeat_successful_task_rate: f64,
    pub active_users_trend: Vec<TrendPoint>,
    pub active_workspaces_trend: Vec<TrendPoint>,
    pub stickiness: StickinessSummary,
    pub cohorts: Vec<CohortRow>,
}

#[derive(Debug, Serialize)]
pub struct LatencyRow {
    pub key: String,
    pub avg_latency_ms: f64,
    pub p95_latency_ms: f64,
}

#[derive(Debug, Serialize)]
pub struct ReasonRow {
    pub reason: String,
    pub count: i64,
}

#[derive(Debug, Serialize)]
pub struct ReliabilitySummary {
    pub task_success_rate: f64,
    pub api_error_rate: Option<f64>,
    pub integration_failure_rate: Option<f64>,
    pub checkout_failure_rate: Option<f64>,
    pub slowest_endpoints_or_workflows: Vec<LatencyRow>,
    pub top_failure_reasons: Vec<ReasonRow>,
}

#[derive(Debug, Serialize)]
pub struct MetricDefinition {
    pub metric: String,
    pub formula: String,
}

#[derive(Debug, Serialize)]
pub struct EventTaxonomyRow {
    pub category: String,
    pub event_name: String,
    pub trigger: String,
    pub required_properties: Vec<String>,
    pub optional_properties: Vec<String>,
    pub emitted_from: String,
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct DashboardResponse {
    pub generated_at: String,
    pub range: DateRangeSummary,
    pub kpis: ExecutiveKpis,
    pub funnel: FunnelSummary,
    pub acquisition: AcquisitionSummary,
    pub activation: ActivationSummary,
    pub monetization: MonetizationSummary,
    pub retention: RetentionSummary,
    pub reliability: ReliabilitySummary,
    pub metric_definitions: Vec<MetricDefinition>,
    pub taxonomy: Vec<EventTaxonomyRow>,
    pub implemented_events: Vec<String>,
    pub deferred_events: Vec<String>,
}

#[derive(Debug, Clone)]
struct IdentityTouch {
    utm_source: Option<String>,
    utm_medium: Option<String>,
    utm_campaign: Option<String>,
    referrer_domain: Option<String>,
    device_type: Option<String>,
    landing_variant: Option<String>,
}

#[derive(Debug, Clone)]
struct EventWithIdentity<'a> {
    event: &'a AnalyticsEventRecord,
    identity: String,
}

pub async fn track_event(
    State(state): State<AnalyticsState>,
    headers: HeaderMap,
    Json(payload): Json<TrackEventRequest>,
) -> impl axum::response::IntoResponse {
    let event_name = payload.event_name.trim();
    if event_name.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "event_name is required" })),
        )
            .into_response();
    }
    if event_name.len() > 120 {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "event_name too long" })),
        )
            .into_response();
    }

    let mut auth_user_id: Option<Uuid> = None;
    if let Some(token) = extract_bearer_token(&headers) {
        match validate_supabase_token(&state.supabase_url, &token).await {
            Ok(auth_user) => {
                auth_user_id = Some(auth_user.id);
            }
            Err((status, msg)) if status == StatusCode::UNAUTHORIZED => {
                warn!("analytics.track: invalid bearer token, accepting anonymous event");
                warn!("analytics.track token validation detail: {}", msg);
            }
            Err((_, msg)) => {
                warn!("analytics.track token validation failed: {}", msg);
            }
        }
    }

    let account_id = if let Some(auth_user_id) = auth_user_id {
        let store = state.account_store.clone();
        match task::spawn_blocking(move || store.get_account_by_auth_user(auth_user_id)).await {
            Ok(Ok(Some(account))) => Some(account.id),
            Ok(Ok(None)) => None,
            Ok(Err(err)) => {
                warn!(
                    "analytics.track: failed to resolve account for auth_user_id={}: {}",
                    auth_user_id, err
                );
                None
            }
            Err(err) => {
                warn!("analytics.track: spawn_blocking join error: {}", err);
                None
            }
        }
    } else {
        None
    };

    let event_timestamp =
        parse_rfc3339_utc(payload.event_timestamp.as_deref()).unwrap_or_else(Utc::now);
    let source = normalize_opt(payload.source).unwrap_or_else(|| "client".to_string());
    let event = AnalyticsEventInsert {
        event_name: event_name.to_string(),
        source,
        event_timestamp,
        account_id,
        auth_user_id,
        anonymous_id: normalize_opt(payload.anonymous_id),
        session_id: normalize_opt(payload.session_id),
        workspace_id: normalize_opt(payload.workspace_id),
        org_id: normalize_opt(payload.org_id),
        plan_type: normalize_opt(payload.plan_type),
        environment: normalize_opt(payload.environment).or_else(|| Some(state.environment.clone())),
        app_version: normalize_opt(payload.app_version),
        page_path: normalize_opt(payload.page_path),
        route_path: normalize_opt(payload.route_path),
        referrer: normalize_opt(payload.referrer),
        utm_source: normalize_opt(payload.utm_source),
        utm_medium: normalize_opt(payload.utm_medium),
        utm_campaign: normalize_opt(payload.utm_campaign),
        utm_term: normalize_opt(payload.utm_term),
        utm_content: normalize_opt(payload.utm_content),
        device_type: normalize_opt(payload.device_type),
        browser: normalize_opt(payload.browser),
        os: normalize_opt(payload.os),
        event_key: normalize_opt(payload.event_key),
        properties: payload.properties.unwrap_or_else(|| json!({})),
    };

    let store = state.account_store.clone();
    let inserted = match task::spawn_blocking(move || store.record_analytics_event(&event)).await {
        Ok(Ok(inserted)) => inserted,
        Ok(Err(err)) => {
            error!("analytics.track: failed to persist event: {}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Failed to persist analytics event" })),
            )
                .into_response();
        }
        Err(err) => {
            error!("analytics.track: spawn_blocking join error: {}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Failed to persist analytics event" })),
            )
                .into_response();
        }
    };

    (
        StatusCode::ACCEPTED,
        Json(TrackEventResponse {
            accepted: true,
            deduped: !inserted,
        }),
    )
        .into_response()
}

pub async fn get_dashboard(
    State(state): State<AnalyticsState>,
    headers: HeaderMap,
    Query(query): Query<DashboardQuery>,
) -> impl axum::response::IntoResponse {
    let token = match extract_bearer_token(&headers) {
        Some(token) => token,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": "Missing Authorization header" })),
            )
                .into_response();
        }
    };

    let auth_user = match validate_supabase_token(&state.supabase_url, &token).await {
        Ok(user) => user,
        Err((status, msg)) => {
            return (status, Json(json!({ "error": msg }))).into_response();
        }
    };

    let email = match auth_user.email {
        Some(email) => email.trim().to_ascii_lowercase(),
        None => {
            return (
                StatusCode::FORBIDDEN,
                Json(json!({ "error": "Admin email claim required" })),
            )
                .into_response();
        }
    };

    if !state.admin_emails.contains(&email) {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({ "error": "Dashboard is admin-only" })),
        )
            .into_response();
    }

    let (start, end) = match resolve_window(&query) {
        Ok(window) => window,
        Err(msg) => {
            return (StatusCode::BAD_REQUEST, Json(json!({ "error": msg }))).into_response();
        }
    };

    let store = state.account_store.clone();
    let env = state.environment.clone();
    let fetched = task::spawn_blocking(move || {
        let events = store.list_analytics_events_between(start, end)?;
        let accounts = store.list_accounts_created_between(start, end)?;
        let payments = store.list_payments_between(start, end)?;
        Ok::<_, crate::account_store::AccountStoreError>((events, accounts, payments))
    })
    .await;

    let (mut events, accounts, payments) = match fetched {
        Ok(Ok(values)) => values,
        Ok(Err(err)) => {
            error!("analytics.dashboard query error: {}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Failed to query analytics data" })),
            )
                .into_response();
        }
        Err(err) => {
            error!("analytics.dashboard join error: {}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Failed to query analytics data" })),
            )
                .into_response();
        }
    };

    augment_with_backfilled_events(&mut events, &accounts, &payments, &env);
    let response = build_dashboard_response(start, end, &events, &payments);
    info!(
        "analytics.dashboard generated for admin={} range={}..{} events={}",
        email,
        start.to_rfc3339(),
        end.to_rfc3339(),
        events.len()
    );
    (StatusCode::OK, Json(response)).into_response()
}

pub fn analytics_router(state: AnalyticsState) -> Router {
    Router::new()
        .route("/analytics/track", post(track_event))
        .route("/analytics/dashboard", get(get_dashboard))
        .with_state(state)
}

fn parse_admin_emails(value: &str) -> HashSet<String> {
    value
        .split(',')
        .map(|part| part.trim().to_ascii_lowercase())
        .filter(|part| !part.is_empty())
        .collect()
}

fn parse_rfc3339_utc(value: Option<&str>) -> Option<DateTime<Utc>> {
    value
        .and_then(|raw| DateTime::parse_from_rfc3339(raw).ok())
        .map(|dt| dt.with_timezone(&Utc))
}

fn normalize_opt(value: Option<String>) -> Option<String> {
    value
        .map(|raw| raw.trim().to_string())
        .filter(|raw| !raw.is_empty())
}

fn resolve_window(query: &DashboardQuery) -> Result<(DateTime<Utc>, DateTime<Utc>), String> {
    let end = parse_rfc3339_utc(query.end.as_deref()).unwrap_or_else(Utc::now);
    let start = if let Some(start) = parse_rfc3339_utc(query.start.as_deref()) {
        start
    } else {
        let mut days = DEFAULT_RANGE_DAYS;
        if let Some(range) = query.range.as_ref() {
            let parsed = range
                .trim()
                .strip_suffix('d')
                .and_then(|value| value.parse::<i64>().ok())
                .ok_or_else(|| "range must look like 7d, 30d, or 90d".to_string())?;
            days = parsed.clamp(1, MAX_RANGE_DAYS);
        }
        end - Duration::days(days)
    };

    if start >= end {
        return Err("start must be before end".to_string());
    }

    let span_days = (end - start).num_days();
    if span_days > MAX_RANGE_DAYS {
        return Err(format!("range cannot exceed {} days", MAX_RANGE_DAYS));
    }

    Ok((start, end))
}

fn augment_with_backfilled_events(
    events: &mut Vec<AnalyticsEventRecord>,
    accounts: &[Account],
    payments: &[Payment],
    environment: &str,
) {
    let signup_seen: HashSet<Uuid> = events
        .iter()
        .filter(|event| event.event_name == "signup_completed")
        .filter_map(|event| event.account_id)
        .collect();
    let workspace_seen: HashSet<Uuid> = events
        .iter()
        .filter(|event| event.event_name == "workspace_created")
        .filter_map(|event| event.account_id)
        .collect();
    let payment_seen: HashSet<String> = events
        .iter()
        .filter(|event| event.event_name == "payment_succeeded")
        .filter_map(|event| event.event_key.clone())
        .collect();

    for account in accounts {
        if !signup_seen.contains(&account.id) {
            events.push(backfill_event(
                "signup_completed",
                account.created_at,
                Some(account.id),
                Some(account.auth_user_id),
                Some(format!("signup:{}", account.id)),
                Some("credits".to_string()),
                environment,
                json!({
                    "auth_method": "unknown",
                    "backfilled_from": "accounts"
                }),
            ));
        }
        if !workspace_seen.contains(&account.id) {
            events.push(backfill_event(
                "workspace_created",
                account.created_at,
                Some(account.id),
                Some(account.auth_user_id),
                Some(format!("workspace:{}", account.id)),
                Some("credits".to_string()),
                environment,
                json!({
                    "workspace_type": "account_workspace",
                    "backfilled_from": "accounts"
                }),
            ));
        }
    }

    for payment in payments {
        let payment_key = format!("payment:{}", payment.stripe_session_id);
        if payment_seen.contains(&payment_key) {
            continue;
        }
        events.push(backfill_event(
            "payment_succeeded",
            payment.created_at,
            Some(payment.account_id),
            None,
            Some(payment_key.clone()),
            Some("hourly_credits".to_string()),
            environment,
            json!({
                "amount_cents": payment.amount_cents,
                "amount_usd": (payment.amount_cents as f64) / 100.0,
                "currency": "usd",
                "hours_purchased": payment.hours_purchased,
                "billing_interval": "one_time"
            }),
        ));
        events.push(backfill_event(
            "subscription_activated",
            payment.created_at,
            Some(payment.account_id),
            None,
            Some(format!("subscription:{}", payment.stripe_session_id)),
            Some("hourly_credits".to_string()),
            environment,
            json!({
                "status": "active_credits",
                "billing_interval": "one_time",
                "backfilled_from": "payments"
            }),
        ));
    }
}

fn backfill_event(
    event_name: &str,
    event_timestamp: DateTime<Utc>,
    account_id: Option<Uuid>,
    auth_user_id: Option<Uuid>,
    event_key: Option<String>,
    plan_type: Option<String>,
    environment: &str,
    properties: Value,
) -> AnalyticsEventRecord {
    AnalyticsEventRecord {
        event_name: event_name.to_string(),
        source: "server_backfill".to_string(),
        event_timestamp,
        account_id,
        auth_user_id,
        anonymous_id: None,
        session_id: None,
        workspace_id: account_id.map(|id| id.to_string()),
        org_id: None,
        plan_type,
        environment: Some(environment.to_string()),
        app_version: None,
        page_path: None,
        route_path: None,
        referrer: None,
        utm_source: None,
        utm_medium: None,
        utm_campaign: None,
        utm_term: None,
        utm_content: None,
        device_type: None,
        browser: None,
        os: None,
        event_key,
        properties,
    }
}

fn build_dashboard_response(
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    events: &[AnalyticsEventRecord],
    payments: &[Payment],
) -> DashboardResponse {
    let mut sorted_events: Vec<&AnalyticsEventRecord> = events.iter().collect();
    sorted_events.sort_by(|a, b| a.event_timestamp.cmp(&b.event_timestamp));

    let anon_to_known = build_anon_identity_map(&sorted_events);
    let events_with_identity = sorted_events
        .iter()
        .filter_map(|event| {
            canonical_identity(event, &anon_to_known)
                .map(|identity| EventWithIdentity { event, identity })
        })
        .collect::<Vec<_>>();

    let mut identity_events: HashMap<String, HashMap<String, DateTime<Utc>>> = HashMap::new();
    let mut identity_event_log: HashMap<String, Vec<(&str, DateTime<Utc>)>> = HashMap::new();
    let mut identity_touch: HashMap<String, IdentityTouch> = HashMap::new();
    let mut identity_accounts: HashMap<String, Uuid> = HashMap::new();
    let mut identity_channels: HashMap<String, HashSet<String>> = HashMap::new();
    let mut auth_method_by_identity: HashMap<String, String> = HashMap::new();
    let mut workspace_type_by_identity: HashMap<String, String> = HashMap::new();
    let mut first_task_type_by_identity: HashMap<String, String> = HashMap::new();
    let mut failure_reasons: HashMap<String, i64> = HashMap::new();
    let mut endpoint_latencies: HashMap<String, Vec<f64>> = HashMap::new();
    let mut daily_active_users: HashMap<String, HashSet<String>> = HashMap::new();
    let mut daily_active_workspaces: HashMap<String, HashSet<Uuid>> = HashMap::new();

    for wrapped in &events_with_identity {
        let event = wrapped.event;
        let identity = &wrapped.identity;

        if let Some(account_id) = event.account_id {
            identity_accounts
                .entry(identity.clone())
                .or_insert(account_id);
        }

        let event_map = identity_events.entry(identity.clone()).or_default();
        event_map
            .entry(event.event_name.clone())
            .and_modify(|ts| {
                if event.event_timestamp < *ts {
                    *ts = event.event_timestamp;
                }
            })
            .or_insert(event.event_timestamp);
        identity_event_log
            .entry(identity.clone())
            .or_default()
            .push((&event.event_name, event.event_timestamp));

        let touch = identity_touch
            .entry(identity.clone())
            .or_insert(IdentityTouch {
                utm_source: None,
                utm_medium: None,
                utm_campaign: None,
                referrer_domain: None,
                device_type: None,
                landing_variant: None,
            });
        if touch.utm_source.is_none() {
            touch.utm_source = event.utm_source.clone();
        }
        if touch.utm_medium.is_none() {
            touch.utm_medium = event.utm_medium.clone();
        }
        if touch.utm_campaign.is_none() {
            touch.utm_campaign = event.utm_campaign.clone();
        }
        if touch.referrer_domain.is_none() {
            touch.referrer_domain = extract_referrer_domain(event.referrer.as_deref());
        }
        if touch.device_type.is_none() {
            touch.device_type = event.device_type.clone();
        }
        if touch.landing_variant.is_none() {
            touch.landing_variant = property_str(event, "landing_page_variant")
                .or_else(|| property_str(event, "page_variant"));
        }

        if (event.event_name == "first_channel_or_tool_connected"
            || event.event_name == "channel_connect_succeeded"
            || event.event_name == "tool_connect_succeeded")
            && property_str(event, "identifier_type").is_some()
        {
            let channel =
                property_str(event, "identifier_type").unwrap_or_else(|| "unknown".to_string());
            identity_channels
                .entry(identity.clone())
                .or_default()
                .insert(channel);
        }
        if event.event_name == "first_channel_or_tool_connected"
            || event.event_name == "channel_connect_succeeded"
            || event.event_name == "tool_connect_succeeded"
        {
            let channel = property_str(event, "channel_type")
                .or_else(|| property_str(event, "tool_type"))
                .or_else(|| property_str(event, "identifier_type"))
                .unwrap_or_else(|| "unknown".to_string());
            identity_channels
                .entry(identity.clone())
                .or_default()
                .insert(channel);
        }

        if event.event_name == "signup_completed" {
            let auth_method =
                property_str(event, "auth_method").unwrap_or_else(|| "unknown".to_string());
            auth_method_by_identity
                .entry(identity.clone())
                .or_insert(auth_method);
        }

        if event.event_name == "workspace_created" {
            let workspace_type = property_str(event, "workspace_type")
                .unwrap_or_else(|| "account_workspace".to_string());
            workspace_type_by_identity
                .entry(identity.clone())
                .or_insert(workspace_type);
        }

        if event.event_name == "first_task_started" || event.event_name == "task_started" {
            let task_type = property_str(event, "task_type")
                .or_else(|| property_str(event, "channel"))
                .unwrap_or_else(|| "unknown".to_string());
            first_task_type_by_identity
                .entry(identity.clone())
                .or_insert(task_type);
        }

        if is_failure_event(&event.event_name) {
            let reason = property_str(event, "error_reason")
                .or_else(|| property_str(event, "error_type"))
                .or_else(|| property_str(event, "error"))
                .or_else(|| property_str(event, "reason"))
                .unwrap_or_else(|| "unknown".to_string());
            *failure_reasons.entry(reason).or_insert(0) += 1;
        }

        if event.event_name == "latency_metric_logged" {
            let key = property_str(event, "endpoint")
                .or_else(|| property_str(event, "workflow"))
                .unwrap_or_else(|| "unknown".to_string());
            if let Some(latency_ms) = property_f64(event, "latency_ms") {
                endpoint_latencies.entry(key).or_default().push(latency_ms);
            }
        }

        if is_usage_event(&event.event_name) {
            let day_key = event.event_timestamp.format("%Y-%m-%d").to_string();
            daily_active_users
                .entry(day_key.clone())
                .or_default()
                .insert(identity.clone());
            if let Some(account_id) = event.account_id {
                daily_active_workspaces
                    .entry(day_key)
                    .or_default()
                    .insert(account_id);
            }
        }
    }

    let funnel_steps = funnel_step_definitions();
    let identities = identity_events.keys().cloned().collect::<Vec<_>>();

    let unique_visitors =
        count_identities_reaching(&identities, &identity_events, &["landing_page_view"]);
    let signups = count_identities_reaching(&identities, &identity_events, &["signup_completed"]);
    let activated = count_identities_reaching(
        &identities,
        &identity_events,
        &["first_task_succeeded", "task_succeeded"],
    );
    let paid = count_identities_reaching(
        &identities,
        &identity_events,
        &["payment_succeeded", "subscription_activated"],
    );

    let mut funnel_rows = Vec::new();
    let mut previous = 0_i64;
    let mut first_step = 0_i64;
    for (idx, (event_name, label, aliases)) in funnel_steps.iter().enumerate() {
        let count = count_identities_reaching(&identities, &identity_events, aliases);
        if idx == 0 {
            first_step = count;
        }
        let step_rate = if idx == 0 {
            1.0
        } else {
            ratio(count, previous)
        };
        let overall_rate = ratio(count, first_step);
        funnel_rows.push(FunnelStepSummary {
            event_name: (*event_name).to_string(),
            label: (*label).to_string(),
            identities: count,
            step_conversion_rate: step_rate,
            overall_conversion_rate: overall_rate,
        });
        previous = count;
    }

    let time_to_first_value_hours = median_time_to_first_value_hours(
        &identities,
        &identity_events,
        &["first_task_succeeded", "task_succeeded"],
    );
    let _d1_retention =
        retention_rate_days(1, end, &identities, &identity_events, &identity_event_log);
    let d7_retention =
        retention_rate_days(7, end, &identities, &identity_events, &identity_event_log);
    let _d30_retention =
        retention_rate_days(30, end, &identities, &identity_events, &identity_event_log);

    let repeat_value_rate = repeat_value_rate(&identities, &identity_events);
    let active_workspaces: i64 = identity_accounts.values().collect::<HashSet<_>>().len() as i64;
    let revenue_usd: f64 = payments
        .iter()
        .map(|payment| payment.amount_cents as f64 / 100.0)
        .sum();
    let active_paid_accounts_30d: i64 = payments
        .iter()
        .filter(|payment| payment.created_at >= end - Duration::days(30))
        .map(|payment| payment.account_id)
        .collect::<HashSet<_>>()
        .len() as i64;

    let kpis = ExecutiveKpis {
        unique_visitors,
        signup_conversion_rate: ratio(signups, unique_visitors),
        signup_to_activation_rate: ratio(activated, signups),
        activation_to_paid_rate: ratio(paid, activated),
        visitor_to_paid_rate: ratio(paid, unique_visitors),
        activation_rate: ratio(activated, signups),
        repeat_value_rate,
        median_time_to_first_value_hours: time_to_first_value_hours,
        d7_retention_rate: d7_retention,
        active_workspaces,
        active_paid_accounts_30d,
        revenue_usd,
    };

    let acquisition = build_acquisition_summary(
        &identities,
        &identity_events,
        &identity_touch,
        &["landing_page_view"],
        &["signup_completed"],
        &["first_task_succeeded", "task_succeeded"],
        &["payment_succeeded", "subscription_activated"],
    );

    let activation = build_activation_summary(
        &identities,
        &identity_events,
        &auth_method_by_identity,
        &workspace_type_by_identity,
        &identity_channels,
        &first_task_type_by_identity,
    );

    let monetization = build_monetization_summary(events, &identities, &identity_events, payments);
    let retention = build_retention_summary(
        start,
        end,
        &identities,
        &identity_events,
        &identity_event_log,
        &daily_active_users,
        &daily_active_workspaces,
    );
    let reliability = build_reliability_summary(events, failure_reasons, endpoint_latencies);

    DashboardResponse {
        generated_at: Utc::now().to_rfc3339(),
        range: DateRangeSummary {
            start: start.to_rfc3339(),
            end: end.to_rfc3339(),
            days: (end - start).num_days(),
        },
        kpis,
        funnel: FunnelSummary { steps: funnel_rows },
        acquisition,
        activation,
        monetization,
        retention,
        reliability,
        metric_definitions: metric_definitions(),
        taxonomy: taxonomy_rows(),
        implemented_events: implemented_events(),
        deferred_events: deferred_events(),
    }
}

fn build_anon_identity_map(events: &[&AnalyticsEventRecord]) -> HashMap<String, String> {
    let mut mapping = HashMap::new();
    for event in events {
        let known_identity = if let Some(account_id) = event.account_id {
            Some(format!("account:{}", account_id))
        } else {
            event
                .auth_user_id
                .map(|auth_user_id| format!("user:{}", auth_user_id))
        };
        if let (Some(anonymous_id), Some(known_identity)) =
            (event.anonymous_id.as_ref(), known_identity)
        {
            mapping
                .entry(anonymous_id.clone())
                .or_insert(known_identity);
        }
    }
    mapping
}

fn canonical_identity(
    event: &AnalyticsEventRecord,
    anon_to_known: &HashMap<String, String>,
) -> Option<String> {
    if let Some(account_id) = event.account_id {
        return Some(format!("account:{}", account_id));
    }
    if let Some(auth_user_id) = event.auth_user_id {
        return Some(format!("user:{}", auth_user_id));
    }
    if let Some(anonymous_id) = event.anonymous_id.as_ref() {
        if let Some(known) = anon_to_known.get(anonymous_id) {
            return Some(known.clone());
        }
        return Some(format!("anon:{}", anonymous_id));
    }
    None
}

fn count_identities_reaching(
    identities: &[String],
    identity_events: &HashMap<String, HashMap<String, DateTime<Utc>>>,
    aliases: &[&str],
) -> i64 {
    identities
        .iter()
        .filter(|identity| {
            identity_events
                .get(*identity)
                .map(|events| aliases.iter().any(|alias| events.contains_key(*alias)))
                .unwrap_or(false)
        })
        .count() as i64
}

fn first_event_time(
    identity_events: &HashMap<String, HashMap<String, DateTime<Utc>>>,
    identity: &str,
    aliases: &[&str],
) -> Option<DateTime<Utc>> {
    identity_events.get(identity).and_then(|events| {
        aliases
            .iter()
            .filter_map(|alias| events.get(*alias).cloned())
            .min()
    })
}

fn median_time_to_first_value_hours(
    identities: &[String],
    identity_events: &HashMap<String, HashMap<String, DateTime<Utc>>>,
    success_aliases: &[&str],
) -> Option<f64> {
    let mut durations = identities
        .iter()
        .filter_map(|identity| {
            let signup = first_event_time(identity_events, identity, &["signup_completed"])?;
            let success = first_event_time(identity_events, identity, success_aliases)?;
            if success < signup {
                return None;
            }
            Some((success - signup).num_seconds() as f64 / 3600.0)
        })
        .collect::<Vec<_>>();

    if durations.is_empty() {
        return None;
    }
    durations.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
    let mid = durations.len() / 2;
    if durations.len() % 2 == 0 {
        Some((durations[mid - 1] + durations[mid]) / 2.0)
    } else {
        Some(durations[mid])
    }
}

fn repeat_value_rate(
    identities: &[String],
    identity_events: &HashMap<String, HashMap<String, DateTime<Utc>>>,
) -> f64 {
    let mut eligible = 0_i64;
    let mut repeated = 0_i64;
    for identity in identities {
        let Some(first_success) = first_event_time(
            identity_events,
            identity,
            &["first_task_succeeded", "task_succeeded"],
        ) else {
            continue;
        };
        eligible += 1;
        let second = first_event_time(identity_events, identity, &["second_successful_task"]);
        if let Some(second) = second {
            if second <= first_success + Duration::days(7) {
                repeated += 1;
            }
        }
    }
    ratio(repeated, eligible)
}

fn retention_rate_days(
    days: i64,
    end: DateTime<Utc>,
    identities: &[String],
    identity_events: &HashMap<String, HashMap<String, DateTime<Utc>>>,
    identity_event_log: &HashMap<String, Vec<(&str, DateTime<Utc>)>>,
) -> f64 {
    let mut eligible = 0_i64;
    let mut retained = 0_i64;

    for identity in identities {
        let Some(signup) = first_event_time(identity_events, identity, &["signup_completed"])
        else {
            continue;
        };
        let window_start = signup + Duration::days(days);
        let window_end = window_start + Duration::days(1);
        if window_start > end {
            continue;
        }
        eligible += 1;
        if identity_event_log
            .get(identity)
            .map(|events| {
                events.iter().any(|(name, ts)| {
                    is_usage_event(name) && *ts >= window_start && *ts < window_end
                })
            })
            .unwrap_or(false)
        {
            retained += 1;
        }
    }

    ratio(retained, eligible)
}

fn build_acquisition_summary(
    identities: &[String],
    identity_events: &HashMap<String, HashMap<String, DateTime<Utc>>>,
    identity_touch: &HashMap<String, IdentityTouch>,
    visitor_aliases: &[&str],
    signup_aliases: &[&str],
    activation_aliases: &[&str],
    paid_aliases: &[&str],
) -> AcquisitionSummary {
    let mut by_source: HashMap<String, (i64, i64, i64, i64)> = HashMap::new();
    let mut by_referrer: HashMap<String, (i64, i64, i64, i64)> = HashMap::new();
    let mut by_device: HashMap<String, (i64, i64, i64, i64)> = HashMap::new();
    let mut by_variant: HashMap<String, (i64, i64, i64, i64)> = HashMap::new();

    for identity in identities {
        let Some(events) = identity_events.get(identity) else {
            continue;
        };
        if !visitor_aliases
            .iter()
            .any(|alias| events.contains_key(*alias))
        {
            continue;
        }
        let signups = signup_aliases
            .iter()
            .any(|alias| events.contains_key(*alias));
        let activated = activation_aliases
            .iter()
            .any(|alias| events.contains_key(*alias));
        let paid = paid_aliases.iter().any(|alias| events.contains_key(*alias));
        let touch = identity_touch.get(identity);

        let source_key = touch
            .map(|touch| {
                let source = touch
                    .utm_source
                    .clone()
                    .unwrap_or_else(|| "direct".to_string());
                let medium = touch
                    .utm_medium
                    .clone()
                    .unwrap_or_else(|| "(none)".to_string());
                let campaign = touch
                    .utm_campaign
                    .clone()
                    .unwrap_or_else(|| "(none)".to_string());
                format!("{} / {} / {}", source, medium, campaign)
            })
            .unwrap_or_else(|| "direct / (none) / (none)".to_string());
        let referrer_key = touch
            .and_then(|touch| touch.referrer_domain.clone())
            .unwrap_or_else(|| "direct".to_string());
        let device_key = touch
            .and_then(|touch| touch.device_type.clone())
            .unwrap_or_else(|| "unknown".to_string());
        let variant_key = touch
            .and_then(|touch| touch.landing_variant.clone())
            .unwrap_or_else(|| "default".to_string());

        apply_conversion_tuple(
            by_source.entry(source_key).or_insert((0, 0, 0, 0)),
            signups,
            activated,
            paid,
        );
        apply_conversion_tuple(
            by_referrer.entry(referrer_key).or_insert((0, 0, 0, 0)),
            signups,
            activated,
            paid,
        );
        apply_conversion_tuple(
            by_device.entry(device_key).or_insert((0, 0, 0, 0)),
            signups,
            activated,
            paid,
        );
        apply_conversion_tuple(
            by_variant.entry(variant_key).or_insert((0, 0, 0, 0)),
            signups,
            activated,
            paid,
        );
    }

    AcquisitionSummary {
        by_source_campaign: to_acquisition_rows(by_source),
        by_referrer: to_acquisition_rows(by_referrer),
        by_device_type: to_acquisition_rows(by_device),
        by_landing_variant: to_acquisition_rows(by_variant),
    }
}

fn apply_conversion_tuple(
    value: &mut (i64, i64, i64, i64),
    signup: bool,
    activated: bool,
    paid: bool,
) {
    value.0 += 1;
    if signup {
        value.1 += 1;
    }
    if activated {
        value.2 += 1;
    }
    if paid {
        value.3 += 1;
    }
}

fn to_acquisition_rows(map: HashMap<String, (i64, i64, i64, i64)>) -> Vec<AcquisitionRow> {
    let mut rows = map
        .into_iter()
        .map(
            |(key, (visitors, signups, activated, paid))| AcquisitionRow {
                key,
                visitors,
                signups,
                activated,
                paid,
                signup_conversion_rate: ratio(signups, visitors),
                activation_rate: ratio(activated, signups),
                paid_conversion_rate: ratio(paid, signups),
            },
        )
        .collect::<Vec<_>>();
    rows.sort_by(|a, b| b.visitors.cmp(&a.visitors));
    rows.truncate(20);
    rows
}

fn build_activation_summary(
    identities: &[String],
    identity_events: &HashMap<String, HashMap<String, DateTime<Utc>>>,
    auth_method_by_identity: &HashMap<String, String>,
    workspace_type_by_identity: &HashMap<String, String>,
    identity_channels: &HashMap<String, HashSet<String>>,
    first_task_type_by_identity: &HashMap<String, String>,
) -> ActivationSummary {
    let signups = count_identities_reaching(identities, identity_events, &["signup_completed"]);
    let mut auth_methods: HashMap<String, i64> = HashMap::new();
    let mut workspace_types: HashMap<String, i64> = HashMap::new();
    let mut connected_channels: HashMap<String, i64> = HashMap::new();
    let mut first_task_types: HashMap<String, i64> = HashMap::new();
    let mut created_agent_or_workflow = 0_i64;
    let mut multi_channel = 0_i64;

    for identity in identities {
        let Some(events) = identity_events.get(identity) else {
            continue;
        };
        if !events.contains_key("signup_completed") {
            continue;
        }

        let auth_method = auth_method_by_identity
            .get(identity)
            .cloned()
            .unwrap_or_else(|| "unknown".to_string());
        *auth_methods.entry(auth_method).or_insert(0) += 1;

        let workspace_type = workspace_type_by_identity
            .get(identity)
            .cloned()
            .unwrap_or_else(|| "account_workspace".to_string());
        *workspace_types.entry(workspace_type).or_insert(0) += 1;

        if let Some(channels) = identity_channels.get(identity) {
            if channels.len() > 1 {
                multi_channel += 1;
            }
            for channel in channels {
                *connected_channels.entry(channel.clone()).or_insert(0) += 1;
            }
        }

        if let Some(task_type) = first_task_type_by_identity.get(identity) {
            *first_task_types.entry(task_type.clone()).or_insert(0) += 1;
        }

        if events.contains_key("first_agent_or_workflow_created") {
            created_agent_or_workflow += 1;
        }
    }

    ActivationSummary {
        by_auth_method: to_breakdown_rows(auth_methods, signups),
        by_workspace_type: to_breakdown_rows(workspace_types, signups),
        by_connected_channel_type: to_breakdown_rows(connected_channels, signups),
        by_first_task_type: to_breakdown_rows(first_task_types, signups),
        agent_or_workflow_creation_rate: ratio(created_agent_or_workflow, signups),
        multi_channel_connection_rate: ratio(multi_channel, signups),
    }
}

fn build_monetization_summary(
    events: &[AnalyticsEventRecord],
    identities: &[String],
    identity_events: &HashMap<String, HashMap<String, DateTime<Utc>>>,
    payments: &[Payment],
) -> MonetizationSummary {
    let pricing_page_views = count_raw_events(events, &["pricing_page_view"]);
    let upgrade_clicks = count_raw_events(events, &["upgrade_clicked"]);
    let paywall_views =
        count_raw_events(events, &["upgrade_viewed_or_paywall_seen", "paywall_seen"]);
    let checkout_starts = count_raw_events(events, &["checkout_started"]);
    let checkout_abandons = count_raw_events(events, &["checkout_abandoned"]);
    let payment_succeeded = count_raw_events(events, &["payment_succeeded"]);
    let subscription_activated = count_raw_events(events, &["subscription_activated"]);
    let subscription_renewed = count_raw_events(events, &["subscription_renewed"]);
    let subscription_canceled = count_raw_events(events, &["subscription_canceled"]);
    let checkout_failures = count_raw_events(events, &["checkout_error"]);

    let trial_starts = count_raw_events(events, &["trial_started"]);
    let trial_to_paid_rate = if trial_starts > 0 {
        Some(ratio(payment_succeeded, trial_starts))
    } else {
        None
    };

    let mut plan_mix: HashMap<String, i64> = HashMap::new();
    for event in events {
        if event.event_name == "payment_succeeded" || event.event_name == "subscription_activated" {
            let plan = event
                .plan_type
                .clone()
                .or_else(|| property_str(event, "plan"))
                .unwrap_or_else(|| "hourly_credits".to_string());
            *plan_mix.entry(plan).or_insert(0) += 1;
        }
    }
    if plan_mix.is_empty() && !payments.is_empty() {
        plan_mix.insert("hourly_credits".to_string(), payments.len() as i64);
    }

    // Abandon = checkout started but no payment success in range.
    // This is an event-level approximation suitable for a pragmatic internal view.
    let checkout_abandon_rate = if checkout_starts > 0 {
        if checkout_abandons > 0 {
            ratio(checkout_abandons, checkout_starts)
        } else {
            ratio(
                (checkout_starts - payment_succeeded).max(0),
                checkout_starts,
            )
        }
    } else {
        0.0
    };

    // Make sure funnel "paid" phase remains identity-based even with no direct event rows.
    let _paid_identities = count_identities_reaching(
        identities,
        identity_events,
        &["payment_succeeded", "subscription_activated"],
    );
    let _checkout_failure_rate = if checkout_starts > 0 {
        ratio(checkout_failures, checkout_starts)
    } else {
        0.0
    };

    MonetizationSummary {
        pricing_page_views,
        upgrade_clicks,
        paywall_views,
        checkout_starts,
        checkout_abandon_rate,
        payment_succeeded,
        subscription_activated,
        subscription_renewed,
        subscription_canceled,
        trial_to_paid_rate,
        plan_mix: to_breakdown_rows(plan_mix, payment_succeeded.max(subscription_activated)),
    }
}

fn build_retention_summary(
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    identities: &[String],
    identity_events: &HashMap<String, HashMap<String, DateTime<Utc>>>,
    identity_event_log: &HashMap<String, Vec<(&str, DateTime<Utc>)>>,
    daily_active_users: &HashMap<String, HashSet<String>>,
    daily_active_workspaces: &HashMap<String, HashSet<Uuid>>,
) -> RetentionSummary {
    let d1_retention = retention_rate_days(1, end, identities, identity_events, identity_event_log);
    let d7_retention = retention_rate_days(7, end, identities, identity_events, identity_event_log);
    let d30_retention =
        retention_rate_days(30, end, identities, identity_events, identity_event_log);
    let repeat_rate = repeat_value_rate(identities, identity_events);

    let active_users_trend = build_daily_trend(start, end, daily_active_users);
    let active_workspaces_trend = build_daily_workspace_trend(start, end, daily_active_workspaces);

    let dau =
        distinct_usage_identities(identities, identity_event_log, end - Duration::days(1), end);
    let wau =
        distinct_usage_identities(identities, identity_event_log, end - Duration::days(7), end);
    let mau = distinct_usage_identities(
        identities,
        identity_event_log,
        end - Duration::days(30),
        end,
    );

    let cohorts = build_cohorts(end, identities, identity_events, identity_event_log);

    RetentionSummary {
        d1_retention_rate: d1_retention,
        d7_retention_rate: d7_retention,
        d30_retention_rate: d30_retention,
        repeat_successful_task_rate: repeat_rate,
        active_users_trend,
        active_workspaces_trend,
        stickiness: StickinessSummary {
            dau,
            wau,
            mau,
            dau_wau_ratio: ratio(dau, wau),
            dau_mau_ratio: ratio(dau, mau),
        },
        cohorts,
    }
}

fn build_reliability_summary(
    events: &[AnalyticsEventRecord],
    failure_reasons: HashMap<String, i64>,
    endpoint_latencies: HashMap<String, Vec<f64>>,
) -> ReliabilitySummary {
    let task_successes = count_raw_events(events, &["task_succeeded"]);
    let task_failures = count_raw_events(events, &["task_failed"]);
    let api_errors = count_raw_events(events, &["api_error"]);
    let api_requests = count_raw_events(events, &["api_request"]);
    let integration_failures = count_raw_events(
        events,
        &[
            "channel_connect_failed",
            "tool_connect_failed",
            "integration_error",
        ],
    );
    let integration_successes = count_raw_events(
        events,
        &[
            "channel_connect_succeeded",
            "tool_connect_succeeded",
            "first_channel_or_tool_connected",
        ],
    );
    let checkout_failures = count_raw_events(events, &["checkout_error"]);
    let checkout_starts = count_raw_events(events, &["checkout_started"]);

    let mut slowest = endpoint_latencies
        .into_iter()
        .filter_map(|(key, mut values)| {
            if values.is_empty() {
                return None;
            }
            values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
            let avg = values.iter().sum::<f64>() / values.len() as f64;
            let p95_idx = ((values.len() as f64) * 0.95).ceil() as usize;
            let p95 = values[p95_idx.saturating_sub(1).min(values.len() - 1)];
            Some(LatencyRow {
                key,
                avg_latency_ms: avg,
                p95_latency_ms: p95,
            })
        })
        .collect::<Vec<_>>();
    slowest.sort_by(|a, b| {
        b.p95_latency_ms
            .partial_cmp(&a.p95_latency_ms)
            .unwrap_or(Ordering::Equal)
    });
    slowest.truncate(10);

    let mut reasons = failure_reasons
        .into_iter()
        .map(|(reason, count)| ReasonRow { reason, count })
        .collect::<Vec<_>>();
    reasons.sort_by(|a, b| b.count.cmp(&a.count));
    reasons.truncate(10);

    ReliabilitySummary {
        task_success_rate: ratio(task_successes, task_successes + task_failures),
        api_error_rate: if api_requests > 0 {
            Some(ratio(api_errors, api_requests))
        } else {
            None
        },
        integration_failure_rate: if integration_successes + integration_failures > 0 {
            Some(ratio(
                integration_failures,
                integration_successes + integration_failures,
            ))
        } else {
            None
        },
        checkout_failure_rate: if checkout_starts > 0 {
            Some(ratio(checkout_failures, checkout_starts))
        } else {
            None
        },
        slowest_endpoints_or_workflows: slowest,
        top_failure_reasons: reasons,
    }
}

fn build_daily_trend(
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    daily_active_users: &HashMap<String, HashSet<String>>,
) -> Vec<TrendPoint> {
    let mut rows = Vec::new();
    let mut day = start.date_naive();
    while day <= end.date_naive() {
        let key = day.format("%Y-%m-%d").to_string();
        let count = daily_active_users
            .get(&key)
            .map(|users| users.len() as i64)
            .unwrap_or(0);
        rows.push(TrendPoint { day: key, count });
        if let Some(next_day) = day.succ_opt() {
            day = next_day;
        } else {
            break;
        }
    }
    rows
}

fn build_daily_workspace_trend(
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    daily_active_workspaces: &HashMap<String, HashSet<Uuid>>,
) -> Vec<TrendPoint> {
    let mut rows = Vec::new();
    let mut day = start.date_naive();
    while day <= end.date_naive() {
        let key = day.format("%Y-%m-%d").to_string();
        let count = daily_active_workspaces
            .get(&key)
            .map(|workspaces| workspaces.len() as i64)
            .unwrap_or(0);
        rows.push(TrendPoint { day: key, count });
        if let Some(next_day) = day.succ_opt() {
            day = next_day;
        } else {
            break;
        }
    }
    rows
}

fn distinct_usage_identities(
    identities: &[String],
    identity_event_log: &HashMap<String, Vec<(&str, DateTime<Utc>)>>,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> i64 {
    identities
        .iter()
        .filter(|identity| {
            identity_event_log
                .get(*identity)
                .map(|events| {
                    events
                        .iter()
                        .any(|(name, ts)| is_usage_event(name) && *ts >= start && *ts < end)
                })
                .unwrap_or(false)
        })
        .count() as i64
}

fn build_cohorts(
    end: DateTime<Utc>,
    identities: &[String],
    identity_events: &HashMap<String, HashMap<String, DateTime<Utc>>>,
    identity_event_log: &HashMap<String, Vec<(&str, DateTime<Utc>)>>,
) -> Vec<CohortRow> {
    let mut cohorts: BTreeMap<NaiveDate, Vec<String>> = BTreeMap::new();

    for identity in identities {
        let Some(signup_time) = first_event_time(identity_events, identity, &["signup_completed"])
        else {
            continue;
        };
        let cohort_start = start_of_week(signup_time.date_naive());
        cohorts
            .entry(cohort_start)
            .or_default()
            .push(identity.clone());
    }

    cohorts
        .into_iter()
        .map(|(cohort_week, cohort_identities)| {
            let users = cohort_identities.len() as i64;
            let d1 = retention_for_cohort(
                &cohort_identities,
                identity_events,
                identity_event_log,
                1,
                end,
            );
            let d7 = retention_for_cohort(
                &cohort_identities,
                identity_events,
                identity_event_log,
                7,
                end,
            );
            let d30 = retention_for_cohort(
                &cohort_identities,
                identity_events,
                identity_event_log,
                30,
                end,
            );
            CohortRow {
                cohort_week: cohort_week.format("%Y-%m-%d").to_string(),
                users,
                d1_retention_rate: d1,
                d7_retention_rate: d7,
                d30_retention_rate: d30,
            }
        })
        .collect()
}

fn retention_for_cohort(
    cohort_identities: &[String],
    identity_events: &HashMap<String, HashMap<String, DateTime<Utc>>>,
    identity_event_log: &HashMap<String, Vec<(&str, DateTime<Utc>)>>,
    days: i64,
    end: DateTime<Utc>,
) -> f64 {
    let mut eligible = 0_i64;
    let mut retained = 0_i64;
    for identity in cohort_identities {
        let Some(signup_time) = first_event_time(identity_events, identity, &["signup_completed"])
        else {
            continue;
        };
        let window_start = signup_time + Duration::days(days);
        if window_start > end {
            continue;
        }
        let window_end = window_start + Duration::days(1);
        eligible += 1;
        if identity_event_log
            .get(identity)
            .map(|events| {
                events.iter().any(|(name, ts)| {
                    is_usage_event(name) && *ts >= window_start && *ts < window_end
                })
            })
            .unwrap_or(false)
        {
            retained += 1;
        }
    }
    ratio(retained, eligible)
}

fn start_of_week(day: NaiveDate) -> NaiveDate {
    let mut current = day;
    while current.weekday() != Weekday::Mon {
        if let Some(prev) = current.pred_opt() {
            current = prev;
        } else {
            break;
        }
    }
    current
}

fn ratio(numerator: i64, denominator: i64) -> f64 {
    if denominator <= 0 {
        return 0.0;
    }
    numerator as f64 / denominator as f64
}

fn to_breakdown_rows(map: HashMap<String, i64>, denominator: i64) -> Vec<BreakdownRow> {
    let mut rows = map
        .into_iter()
        .map(|(key, count)| BreakdownRow {
            key,
            count,
            rate: ratio(count, denominator),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|a, b| b.count.cmp(&a.count));
    rows
}

fn count_raw_events(events: &[AnalyticsEventRecord], aliases: &[&str]) -> i64 {
    events
        .iter()
        .filter(|event| aliases.iter().any(|alias| event.event_name == *alias))
        .count() as i64
}

fn property_str(event: &AnalyticsEventRecord, key: &str) -> Option<String> {
    event
        .properties
        .get(key)
        .and_then(|value| value.as_str())
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn property_f64(event: &AnalyticsEventRecord, key: &str) -> Option<f64> {
    event.properties.get(key).and_then(|value| {
        if let Some(number) = value.as_f64() {
            Some(number)
        } else {
            value.as_str().and_then(|raw| raw.parse::<f64>().ok())
        }
    })
}

fn extract_referrer_domain(referrer: Option<&str>) -> Option<String> {
    let Some(referrer) = referrer else {
        return None;
    };
    let trimmed = referrer.trim();
    if trimmed.is_empty() {
        return None;
    }
    let without_protocol = trimmed
        .strip_prefix("https://")
        .or_else(|| trimmed.strip_prefix("http://"))
        .unwrap_or(trimmed);
    let domain = without_protocol
        .split('/')
        .next()
        .unwrap_or_default()
        .trim();
    if domain.is_empty() {
        None
    } else {
        Some(domain.to_ascii_lowercase())
    }
}

fn is_usage_event(event_name: &str) -> bool {
    matches!(
        event_name,
        "first_authenticated_session"
            | "first_channel_or_tool_connected"
            | "first_agent_or_workflow_created"
            | "first_task_started"
            | "first_task_succeeded"
            | "second_successful_task"
            | "task_started"
            | "task_succeeded"
            | "task_failed"
            | "workflow_run_started"
            | "workflow_run_succeeded"
            | "workflow_run_failed"
            | "active_day"
            | "active_week"
    )
}

fn is_failure_event(event_name: &str) -> bool {
    matches!(
        event_name,
        "task_failed"
            | "api_error"
            | "integration_error"
            | "checkout_error"
            | "channel_connect_failed"
            | "tool_connect_failed"
            | "webhook_error"
            | "auth_error"
    )
}

fn funnel_step_definitions() -> Vec<(&'static str, &'static str, Vec<&'static str>)> {
    vec![
        (
            "landing_page_view",
            "Landing Page View",
            vec!["landing_page_view"],
        ),
        (
            "primary_cta_click",
            "Primary CTA Click",
            vec!["primary_cta_click"],
        ),
        ("signup_started", "Signup Started", vec!["signup_started"]),
        (
            "signup_completed",
            "Signup Completed",
            vec!["signup_completed"],
        ),
        (
            "first_authenticated_session",
            "First Authenticated Session",
            vec!["first_authenticated_session"],
        ),
        (
            "workspace_created",
            "Workspace Created",
            vec!["workspace_created"],
        ),
        (
            "first_channel_or_tool_connected",
            "First Channel or Tool Connected",
            vec!["first_channel_or_tool_connected"],
        ),
        (
            "first_agent_or_workflow_created",
            "First Agent or Workflow Created",
            vec!["first_agent_or_workflow_created"],
        ),
        (
            "first_task_started",
            "First Task Started",
            vec!["first_task_started", "task_started"],
        ),
        (
            "first_task_succeeded",
            "First Task Succeeded",
            vec!["first_task_succeeded", "task_succeeded"],
        ),
        (
            "second_successful_task",
            "Second Successful Task",
            vec!["second_successful_task"],
        ),
        (
            "upgrade_viewed_or_paywall_seen",
            "Upgrade Viewed or Paywall Seen",
            vec!["upgrade_viewed_or_paywall_seen", "paywall_seen"],
        ),
        (
            "checkout_started",
            "Checkout Started",
            vec!["checkout_started"],
        ),
        (
            "payment_succeeded",
            "Payment Succeeded",
            vec!["payment_succeeded"],
        ),
        (
            "subscription_activated",
            "Subscription Activated",
            vec!["subscription_activated"],
        ),
    ]
}

fn metric_definitions() -> Vec<MetricDefinition> {
    vec![
        MetricDefinition {
            metric: "Visit-to-signup conversion".to_string(),
            formula: "signup_completed identities / landing_page_view identities".to_string(),
        },
        MetricDefinition {
            metric: "Signup-to-activation conversion".to_string(),
            formula: "first_task_succeeded identities / signup_completed identities".to_string(),
        },
        MetricDefinition {
            metric: "Activation-to-paid conversion".to_string(),
            formula: "payment_succeeded identities / first_task_succeeded identities".to_string(),
        },
        MetricDefinition {
            metric: "Overall visitor-to-paid conversion".to_string(),
            formula: "payment_succeeded identities / landing_page_view identities".to_string(),
        },
        MetricDefinition {
            metric: "Activation rate".to_string(),
            formula: "first_task_succeeded identities / signup_completed identities".to_string(),
        },
        MetricDefinition {
            metric: "Repeat-value rate".to_string(),
            formula:
                "identities with second_successful_task within 7 days of first_task_succeeded / identities with first_task_succeeded"
                    .to_string(),
        },
        MetricDefinition {
            metric: "Time to first value".to_string(),
            formula:
                "median hours from signup_completed timestamp to first_task_succeeded timestamp".to_string(),
        },
        MetricDefinition {
            metric: "Checkout abandon rate".to_string(),
            formula: "(checkout_started - payment_succeeded) / checkout_started".to_string(),
        },
        MetricDefinition {
            metric: "Trial-to-paid rate".to_string(),
            formula: "payment_succeeded / trial_started (if trial_started > 0)".to_string(),
        },
        MetricDefinition {
            metric: "D1 / D7 / D30 retention".to_string(),
            formula:
                "cohort users with usage event on day N window (N to N+1 days after signup_completed) / eligible signup_completed cohort users"
                    .to_string(),
        },
        MetricDefinition {
            metric: "Task success rate".to_string(),
            formula: "task_succeeded / (task_succeeded + task_failed)".to_string(),
        },
        MetricDefinition {
            metric: "Workspace activation rate".to_string(),
            formula:
                "workspace_created identities reaching first_task_succeeded / workspace_created identities"
                    .to_string(),
        },
    ]
}

fn taxonomy_rows() -> Vec<EventTaxonomyRow> {
    vec![
        taxonomy(
            "Acquisition / Website",
            "landing_page_view",
            "Landing page rendered for a visitor session.",
            vec!["timestamp", "anonymous_id", "session_id", "page_path"],
            vec!["utm_*", "referrer", "device_type", "browser", "os"],
            "client",
            "must_have",
        ),
        taxonomy(
            "Acquisition / Website",
            "signup_page_view",
            "Auth/signup page rendered after site CTA or direct visit.",
            vec!["anonymous_id", "session_id", "page_path"],
            vec!["utm_*", "referrer", "entry_mode"],
            "client",
            "must_have",
        ),
        taxonomy(
            "Acquisition / Website",
            "primary_cta_click",
            "Visitor clicks the primary signup CTA on the landing page.",
            vec!["anonymous_id", "session_id", "cta_location"],
            vec!["utm_*", "referrer"],
            "client",
            "must_have",
        ),
        taxonomy(
            "Acquisition / Website",
            "secondary_cta_click",
            "Visitor clicks a secondary CTA (for example mailto demo/contact intent).",
            vec!["anonymous_id", "session_id", "cta_location"],
            vec!["cta_text", "utm_*", "referrer"],
            "client",
            "must_have",
        ),
        taxonomy(
            "Auth / Signup",
            "signup_started",
            "User submits signup form or begins OAuth signup intent.",
            vec!["anonymous_id", "session_id", "auth_method"],
            vec!["invite_flow"],
            "client",
            "must_have",
        ),
        taxonomy(
            "Auth / Signup",
            "signup_completed",
            "Backend account creation succeeds for authenticated user.",
            vec!["user_id", "workspace_id"],
            vec!["auth_method"],
            "server",
            "must_have",
        ),
        taxonomy(
            "Auth / Signup",
            "login_started",
            "User submits sign-in form or starts OAuth sign-in.",
            vec!["anonymous_id", "session_id", "auth_method"],
            vec!["flow"],
            "client",
            "must_have",
        ),
        taxonomy(
            "Auth / Signup",
            "login_completed",
            "Authenticated user successfully establishes app session.",
            vec!["user_id", "workspace_id", "auth_method"],
            vec!["flow"],
            "client",
            "must_have",
        ),
        taxonomy(
            "Auth / Signup",
            "first_authenticated_session",
            "First authenticated session observed for a workspace identity.",
            vec!["user_id", "workspace_id"],
            vec!["auth_method", "flow"],
            "server/client",
            "must_have",
        ),
        taxonomy(
            "Auth / Signup",
            "auth_error",
            "Auth/signup flow fails before account handoff.",
            vec!["auth_method", "error_type"],
            vec!["flow", "error"],
            "client/server",
            "must_have",
        ),
        taxonomy(
            "Onboarding / Activation",
            "workspace_created",
            "Initial account workspace is provisioned.",
            vec!["user_id", "workspace_id"],
            vec!["workspace_type"],
            "server",
            "must_have",
        ),
        taxonomy(
            "Onboarding / Activation",
            "first_channel_or_tool_connected",
            "First verified integration/channel link is added.",
            vec!["user_id", "workspace_id", "identifier_type"],
            vec!["provider"],
            "server",
            "must_have",
        ),
        taxonomy(
            "Onboarding / Activation",
            "first_agent_or_workflow_created",
            "First workflow-equivalent artifact appears (mapped to first task run for current model).",
            vec!["user_id", "workspace_id"],
            vec!["mapped_from"],
            "server",
            "must_have",
        ),
        taxonomy(
            "Core Usage",
            "task_started",
            "A run_task execution starts.",
            vec!["user_id", "workspace_id", "task_id"],
            vec!["task_type", "channel"],
            "server",
            "must_have",
        ),
        taxonomy(
            "Core Usage",
            "task_succeeded",
            "A run_task execution succeeds.",
            vec!["user_id", "workspace_id", "task_id"],
            vec!["duration_ms", "token_usage"],
            "server",
            "must_have",
        ),
        taxonomy(
            "Core Usage",
            "task_failed",
            "A run_task execution fails.",
            vec!["user_id", "workspace_id", "task_id"],
            vec!["error_reason", "error_type"],
            "server",
            "must_have",
        ),
        taxonomy(
            "Upgrade / Monetization",
            "upgrade_viewed_or_paywall_seen",
            "User views upgrade affordance or receives paywall/insufficient balance notice.",
            vec!["user_id", "workspace_id"],
            vec!["trigger_reason"],
            "client/server",
            "must_have",
        ),
        taxonomy(
            "Upgrade / Monetization",
            "upgrade_clicked",
            "User clicks explicit upgrade/buy action.",
            vec!["user_id", "workspace_id"],
            vec!["checkout_entry_point", "hours"],
            "client",
            "must_have",
        ),
        taxonomy(
            "Upgrade / Monetization",
            "checkout_started",
            "Backend creates Stripe checkout session.",
            vec!["user_id", "workspace_id", "amount_cents", "plan_type"],
            vec!["hours", "billing_interval"],
            "server",
            "must_have",
        ),
        taxonomy(
            "Upgrade / Monetization",
            "payment_succeeded",
            "Stripe webhook confirms completed payment.",
            vec!["workspace_id", "amount_cents", "currency"],
            vec!["hours_purchased", "plan_type"],
            "webhook",
            "must_have",
        ),
        taxonomy(
            "Upgrade / Monetization",
            "subscription_activated",
            "Paid state is activated after successful checkout fulfillment.",
            vec!["workspace_id", "plan_type"],
            vec!["billing_interval", "status"],
            "webhook",
            "must_have",
        ),
        taxonomy(
            "Upgrade / Monetization",
            "checkout_abandoned",
            "Checkout flow is cancelled before successful payment.",
            vec!["user_id", "workspace_id"],
            vec!["checkout_entry_point", "reason_for_paywall_trigger"],
            "client",
            "must_have",
        ),
        taxonomy(
            "Retention / Engagement",
            "active_day",
            "Daily activity heartbeat.",
            vec!["user_id", "workspace_id"],
            vec!["activity_type"],
            "deferred",
            "deferred",
        ),
        taxonomy(
            "Reliability / Performance",
            "api_error",
            "API request fails in a monitored path.",
            vec!["route_path", "error_type"],
            vec!["status_code", "error_reason"],
            "deferred",
            "deferred",
        ),
        taxonomy(
            "Reliability / Performance",
            "checkout_error",
            "Checkout create/redirect flow fails.",
            vec!["error_reason"],
            vec!["error", "checkout_entry_point", "hours"],
            "client/server",
            "must_have",
        ),
        taxonomy(
            "Reliability / Performance",
            "latency_metric_logged",
            "Explicit latency datapoint is recorded for endpoint/workflow.",
            vec!["route_path", "latency_ms"],
            vec!["workflow", "channel"],
            "deferred",
            "deferred",
        ),
    ]
}

fn taxonomy(
    category: &str,
    event_name: &str,
    trigger: &str,
    required_properties: Vec<&str>,
    optional_properties: Vec<&str>,
    emitted_from: &str,
    status: &str,
) -> EventTaxonomyRow {
    EventTaxonomyRow {
        category: category.to_string(),
        event_name: event_name.to_string(),
        trigger: trigger.to_string(),
        required_properties: required_properties
            .into_iter()
            .map(str::to_string)
            .collect(),
        optional_properties: optional_properties
            .into_iter()
            .map(str::to_string)
            .collect(),
        emitted_from: emitted_from.to_string(),
        status: status.to_string(),
    }
}

fn implemented_events() -> Vec<String> {
    vec![
        "landing_page_view",
        "signup_page_view",
        "primary_cta_click",
        "secondary_cta_click",
        "signup_started",
        "signup_completed",
        "login_started",
        "login_completed",
        "auth_error",
        "first_authenticated_session",
        "workspace_created",
        "first_channel_or_tool_connected",
        "first_agent_or_workflow_created",
        "task_started",
        "first_task_started",
        "task_succeeded",
        "first_task_succeeded",
        "second_successful_task",
        "task_failed",
        "upgrade_viewed_or_paywall_seen",
        "upgrade_clicked",
        "checkout_started",
        "checkout_abandoned",
        "payment_succeeded",
        "subscription_activated",
        "checkout_error",
        "channel_connect_succeeded",
        "channel_connect_failed",
    ]
    .into_iter()
    .map(str::to_string)
    .collect()
}

fn deferred_events() -> Vec<String> {
    vec![
        "pricing_page_view",
        "demo_or_waitlist_cta_click",
        "trial_started",
        "trial_converted",
        "subscription_renewed",
        "subscription_failed",
        "subscription_canceled",
        "active_day",
        "active_week",
        "session_started",
        "session_ended",
        "api_error",
        "integration_error",
        "webhook_error",
        "latency_metric_logged",
    ]
    .into_iter()
    .map(str::to_string)
    .collect()
}
