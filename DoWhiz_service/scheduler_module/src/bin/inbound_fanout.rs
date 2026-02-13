use axum::body::Bytes;
use axum::extract::State;
use axum::http::{header, HeaderMap, Method, StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use axum::routing::any;
use axum::Router;
use reqwest::Client;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tracing::{error, info, warn};

#[derive(Clone)]
struct AppState {
    client: Client,
    targets: Vec<String>,
    timeout: Duration,
}

#[derive(Debug, Clone)]
struct ForwardResponse {
    status: StatusCode,
    content_type: Option<String>,
    body: Bytes,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_target(false).init();

    let host = env::var("FANOUT_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("FANOUT_PORT")
        .ok()
        .and_then(|value| value.parse::<u16>().ok())
        .unwrap_or(9100);
    let timeout = env::var("FANOUT_TIMEOUT_SECS")
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .filter(|value| *value > 0)
        .map(Duration::from_secs)
        .unwrap_or_else(|| Duration::from_secs(15));

    let targets = parse_targets()?;
    if targets.is_empty() {
        return Err("FANOUT_TARGETS must include at least one target".into());
    }

    info!("fanout targets: {:?}", targets);

    let state = Arc::new(AppState {
        client: Client::new(),
        targets,
        timeout,
    });

    let app = Router::new()
        .route("/health", any(health))
        .fallback(any(forward_request))
        .with_state(state);

    let addr: SocketAddr = format!("{}:{}", host, port).parse()?;
    info!("fanout listening on {}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;
    Ok(())
}

async fn health() -> impl IntoResponse {
    (StatusCode::OK, "ok")
}

async fn forward_request(
    State(state): State<Arc<AppState>>,
    method: Method,
    uri: Uri,
    headers: HeaderMap,
    body: Bytes,
) -> Response {
    let path = uri
        .path_and_query()
        .map(|value| value.as_str())
        .unwrap_or(uri.path());

    let (tx, mut rx) = mpsc::channel::<ForwardResponse>(1);

    for target in state.targets.iter().cloned() {
        let client = state.client.clone();
        let timeout = state.timeout;
        let method = method.clone();
        let headers = headers.clone();
        let body = body.clone();
        let tx = tx.clone();
        let url = format!("{}{}", target, path);

        tokio::spawn(async move {
            let req_method = match reqwest::Method::from_bytes(method.as_str().as_bytes()) {
                Ok(method) => method,
                Err(_) => {
                    warn!("fanout skipping unsupported method {}", method);
                    return;
                }
            };

            let mut request = client.request(req_method, &url).body(body);
            for (name, value) in headers.iter() {
                if should_skip_header(name) {
                    continue;
                }
                let header_name = match reqwest::header::HeaderName::from_bytes(name.as_str().as_bytes()) {
                    Ok(name) => name,
                    Err(_) => continue,
                };
                let header_value = match reqwest::header::HeaderValue::from_bytes(value.as_bytes()) {
                    Ok(value) => value,
                    Err(_) => continue,
                };
                request = request.header(header_name, header_value);
            }

            let response = match tokio::time::timeout(timeout, request.send()).await {
                Ok(Ok(response)) => response,
                Ok(Err(err)) => {
                    warn!("fanout forward failed to {}: {}", url, err);
                    return;
                }
                Err(_) => {
                    warn!("fanout forward timed out to {}", url);
                    return;
                }
            };

            let status = match StatusCode::from_u16(response.status().as_u16()) {
                Ok(status) => status,
                Err(_) => StatusCode::BAD_GATEWAY,
            };
            let content_type = response
                .headers()
                .get(reqwest::header::CONTENT_TYPE)
                .and_then(|value| value.to_str().ok())
                .map(|value| value.to_string());
            let body = match response.bytes().await {
                Ok(bytes) => bytes,
                Err(err) => {
                    warn!("fanout response read failed from {}: {}", url, err);
                    return;
                }
            };

            if status.is_success() {
                let payload = ForwardResponse {
                    status,
                    content_type,
                    body,
                };
                let _ = tx.send(payload).await;
            } else {
                warn!("fanout target {} returned status {}", url, status);
            }
        });
    }

    drop(tx);

    match tokio::time::timeout(state.timeout, rx.recv()).await {
        Ok(Some(payload)) => {
            let mut response = Response::builder().status(payload.status);
            if let Some(content_type) = payload.content_type {
                if let Ok(value) = header::HeaderValue::from_str(&content_type) {
                    response = response.header(header::CONTENT_TYPE, value);
                }
            }
            match response.body(axum::body::Body::from(payload.body)) {
                Ok(resp) => resp,
                Err(err) => {
                    error!("failed to build fanout response: {}", err);
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            }
        }
        Ok(None) => {
            warn!("fanout response channel closed without success");
            StatusCode::BAD_GATEWAY.into_response()
        }
        Err(_) => {
            warn!("fanout timed out waiting for downstream response");
            StatusCode::BAD_GATEWAY.into_response()
        }
    }
}

fn parse_targets() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let raw = env::var("FANOUT_TARGETS").unwrap_or_default();
    let mut targets = Vec::new();
    for target in raw.split(',') {
        let trimmed = target.trim();
        if trimmed.is_empty() {
            continue;
        }
        let normalized = trimmed.trim_end_matches('/').to_string();
        targets.push(normalized);
    }
    Ok(targets)
}

fn should_skip_header(name: &header::HeaderName) -> bool {
    matches!(
        name.as_str().to_ascii_lowercase().as_str(),
        "connection" | "host" | "content-length"
    )
}
