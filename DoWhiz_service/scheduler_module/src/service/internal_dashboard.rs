use std::sync::Arc;

use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use chrono::{DateTime, Utc};
use serde::Serialize;
use tokio::task;
use tracing::error;

use crate::account_store::{AccountMetricsSnapshot, AccountStore};
use crate::index_store::{IndexStore, TaskIndexMetricsSnapshot};
use crate::ingestion_queue::{ingestion_queue_metrics_from_env, IngestionQueueMetricsSnapshot};

#[derive(Clone)]
pub struct InternalDashboardState {
    pub account_store: Arc<AccountStore>,
    pub index_store: Arc<IndexStore>,
    pub api_key: Option<String>,
}

impl InternalDashboardState {
    pub fn from_env(account_store: Arc<AccountStore>, index_store: Arc<IndexStore>) -> Self {
        Self {
            account_store,
            index_store,
            api_key: std::env::var("INTERNAL_DASHBOARD_API_KEY")
                .ok()
                .map(|value| value.trim().to_string())
                .filter(|value| !value.is_empty()),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct MetricsSection<T> {
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> MetricsSection<T> {
    fn ok(data: T) -> Self {
        Self {
            data: Some(data),
            error: None,
        }
    }

    fn err(message: impl Into<String>) -> Self {
        Self {
            data: None,
            error: Some(message.into()),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct InternalMetricsResponse {
    pub generated_at: DateTime<Utc>,
    pub accounts: MetricsSection<AccountMetricsSnapshot>,
    pub tasks: MetricsSection<TaskIndexMetricsSnapshot>,
    pub ingestion_queue: MetricsSection<IngestionQueueMetricsSnapshot>,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

pub fn internal_dashboard_router(state: InternalDashboardState) -> Router {
    Router::new()
        .route("/api/internal/metrics", get(get_internal_metrics))
        .with_state(state)
}

async fn get_internal_metrics(
    State(state): State<InternalDashboardState>,
    headers: HeaderMap,
) -> impl IntoResponse {
    if let Err(status) = validate_internal_api_key(&headers, state.api_key.as_deref()) {
        return (
            status,
            Json(ErrorResponse {
                error: authorization_error(status).to_string(),
            }),
        )
            .into_response();
    }

    let account_store = state.account_store.clone();
    let index_store = state.index_store.clone();
    let generated_at = Utc::now();
    let snapshot = task::spawn_blocking(move || {
        build_metrics_snapshot(account_store, index_store, generated_at)
    })
    .await;

    match snapshot {
        Ok(payload) => (StatusCode::OK, Json(payload)).into_response(),
        Err(err) => {
            error!("internal dashboard snapshot task join error: {}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Failed to build internal metrics snapshot".to_string(),
                }),
            )
                .into_response()
        }
    }
}

fn build_metrics_snapshot(
    account_store: Arc<AccountStore>,
    index_store: Arc<IndexStore>,
    generated_at: DateTime<Utc>,
) -> InternalMetricsResponse {
    let accounts = match account_store.metrics_snapshot() {
        Ok(snapshot) => MetricsSection::ok(snapshot),
        Err(err) => MetricsSection::err(err.to_string()),
    };

    let tasks = match index_store.metrics_snapshot(generated_at) {
        Ok(snapshot) => MetricsSection::ok(snapshot),
        Err(err) => MetricsSection::err(err.to_string()),
    };

    let ingestion_queue = match ingestion_queue_metrics_from_env() {
        Ok(snapshot) => MetricsSection::ok(snapshot),
        Err(err) => MetricsSection::err(err.to_string()),
    };

    InternalMetricsResponse {
        generated_at,
        accounts,
        tasks,
        ingestion_queue,
    }
}

fn validate_internal_api_key(headers: &HeaderMap, expected_key: Option<&str>) -> Result<(), StatusCode> {
    let expected_key = expected_key
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or(StatusCode::SERVICE_UNAVAILABLE)?;

    let provided_key = extract_internal_dashboard_key(headers).ok_or(StatusCode::UNAUTHORIZED)?;
    if provided_key == expected_key {
        Ok(())
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

fn extract_internal_dashboard_key(headers: &HeaderMap) -> Option<String> {
    let direct = headers
        .get("x-internal-dashboard-key")
        .and_then(|value| value.to_str().ok())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|value| value.to_string());
    if direct.is_some() {
        return direct;
    }

    headers
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|value| value.to_string())
}

fn authorization_error(status: StatusCode) -> &'static str {
    match status {
        StatusCode::SERVICE_UNAVAILABLE => {
            "Internal dashboard is disabled. Set INTERNAL_DASHBOARD_API_KEY to enable it."
        }
        StatusCode::UNAUTHORIZED => {
            "Missing or invalid credentials. Provide x-internal-dashboard-key or Bearer token."
        }
        _ => "Unauthorized",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderValue;

    #[test]
    fn validate_internal_api_key_requires_configured_key() {
        let headers = HeaderMap::new();
        let status = validate_internal_api_key(&headers, None)
            .expect_err("expected service unavailable when key is not configured");
        assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
    }

    #[test]
    fn validate_internal_api_key_accepts_custom_header() {
        let mut headers = HeaderMap::new();
        headers.insert("x-internal-dashboard-key", HeaderValue::from_static("secret"));
        assert!(validate_internal_api_key(&headers, Some("secret")).is_ok());
    }

    #[test]
    fn validate_internal_api_key_accepts_bearer_header() {
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", HeaderValue::from_static("Bearer secret"));
        assert!(validate_internal_api_key(&headers, Some("secret")).is_ok());
    }

    #[test]
    fn validate_internal_api_key_rejects_wrong_key() {
        let mut headers = HeaderMap::new();
        headers.insert("x-internal-dashboard-key", HeaderValue::from_static("wrong"));
        let status = validate_internal_api_key(&headers, Some("secret"))
            .expect_err("expected unauthorized for incorrect key");
        assert_eq!(status, StatusCode::UNAUTHORIZED);
    }
}
