use chrono::{DateTime, Utc};
use reqwest::{Client, StatusCode};
use serde_json::json;
use std::sync::OnceLock;
use uuid::Uuid;

const DEFAULT_BUCKET: &str = "ingestion-raw";
const DEFAULT_PREFIX: &str = "ingestion_raw";

static BUCKET_READY: OnceLock<()> = OnceLock::new();

#[derive(Debug, thiserror::Error)]
pub enum RawPayloadStoreError {
    #[error("missing SUPABASE_PROJECT_URL")]
    MissingProjectUrl,
    #[error("missing SUPABASE_SECRET_KEY")]
    MissingServiceKey,
    #[error("invalid raw payload reference: {0}")]
    InvalidReference(String),
    #[error("supabase storage error: {0}")]
    Storage(String),
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
}

pub fn resolve_storage_bucket() -> String {
    std::env::var("SUPABASE_STORAGE_BUCKET")
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| DEFAULT_BUCKET.to_string())
}

fn resolve_project_url() -> Result<String, RawPayloadStoreError> {
    std::env::var("SUPABASE_PROJECT_URL")
        .ok()
        .map(|value| value.trim().trim_end_matches('/').to_string())
        .filter(|value| !value.is_empty())
        .ok_or(RawPayloadStoreError::MissingProjectUrl)
}

fn resolve_service_key() -> Result<String, RawPayloadStoreError> {
    std::env::var("SUPABASE_SECRET_KEY")
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .ok_or(RawPayloadStoreError::MissingServiceKey)
}

fn build_object_path(envelope_id: Uuid, received_at: DateTime<Utc>) -> String {
    let date = received_at.format("%Y/%m/%d");
    format!("{}/{}/{}.bin", DEFAULT_PREFIX, date, envelope_id)
}

fn build_object_url(base: &str, bucket: &str, path: &str) -> String {
    format!("{}/storage/v1/object/{}/{}", base, bucket, path)
}

fn is_duplicate_bucket_response(status: StatusCode, body: &str) -> bool {
    if status == StatusCode::CONFLICT {
        return true;
    }
    if status == StatusCode::BAD_REQUEST {
        let lower = body.to_ascii_lowercase();
        return lower.contains("duplicate")
            || lower.contains("already exists")
            || lower.contains("\"statuscode\":\"409\"");
    }
    false
}

fn to_supabase_ref(bucket: &str, path: &str) -> String {
    format!("supabase://{}/{}", bucket, path)
}

fn parse_supabase_ref(reference: &str) -> Result<(String, String), RawPayloadStoreError> {
    if let Some(value) = reference.strip_prefix("supabase://") {
        let mut parts = value.splitn(2, '/');
        let bucket = parts
            .next()
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
            .ok_or_else(|| RawPayloadStoreError::InvalidReference(reference.to_string()))?;
        let path = parts
            .next()
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
            .ok_or_else(|| RawPayloadStoreError::InvalidReference(reference.to_string()))?;
        return Ok((bucket.to_string(), path.to_string()));
    }
    if reference.starts_with("http://") || reference.starts_with("https://") {
        return Ok((String::new(), reference.to_string()));
    }
    Err(RawPayloadStoreError::InvalidReference(
        reference.to_string(),
    ))
}

async fn ensure_bucket_ready(
    client: &Client,
    base: &str,
    bucket: &str,
    key: &str,
) -> Result<(), RawPayloadStoreError> {
    if BUCKET_READY.get().is_some() {
        return Ok(());
    }

    let url = format!("{}/storage/v1/bucket", base);
    let response = client
        .post(url)
        .header("Authorization", format!("Bearer {}", key))
        .header("apikey", key)
        .json(&json!({
            "id": bucket,
            "name": bucket,
            "public": false
        }))
        .send()
        .await?;

    let status = response.status();
    let body = response.text().await.unwrap_or_default();
    if status.is_success() || is_duplicate_bucket_response(status, &body) {
        let _ = BUCKET_READY.set(());
        return Ok(());
    }
    Err(RawPayloadStoreError::Storage(format!(
        "bucket create failed (status {}): {}",
        status, body
    )))
}

fn ensure_bucket_ready_blocking(
    client: &reqwest::blocking::Client,
    base: &str,
    bucket: &str,
    key: &str,
) -> Result<(), RawPayloadStoreError> {
    if BUCKET_READY.get().is_some() {
        return Ok(());
    }

    let url = format!("{}/storage/v1/bucket", base);
    let response = client
        .post(url)
        .header("Authorization", format!("Bearer {}", key))
        .header("apikey", key)
        .json(&json!({
            "id": bucket,
            "name": bucket,
            "public": false
        }))
        .send()?;

    let status = response.status();
    let body = response.text().unwrap_or_default();
    if status.is_success() || is_duplicate_bucket_response(status, &body) {
        let _ = BUCKET_READY.set(());
        return Ok(());
    }
    Err(RawPayloadStoreError::Storage(format!(
        "bucket create failed (status {}): {}",
        status, body
    )))
}

pub async fn upload_raw_payload(
    envelope_id: Uuid,
    received_at: DateTime<Utc>,
    raw_payload: &[u8],
) -> Result<String, RawPayloadStoreError> {
    if raw_payload.is_empty() {
        return Err(RawPayloadStoreError::Storage(
            "raw payload is empty".to_string(),
        ));
    }

    let base = resolve_project_url()?;
    let key = resolve_service_key()?;
    let bucket = resolve_storage_bucket();
    let path = build_object_path(envelope_id, received_at);
    let url = build_object_url(&base, &bucket, &path);

    let client = Client::new();
    ensure_bucket_ready(&client, &base, &bucket, &key).await?;

    let response = client
        .post(url)
        .header("Authorization", format!("Bearer {}", key))
        .header("apikey", &key)
        .header("x-upsert", "true")
        .body(raw_payload.to_vec())
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(RawPayloadStoreError::Storage(format!(
            "upload failed (status {}): {}",
            status, body
        )));
    }

    Ok(to_supabase_ref(&bucket, &path))
}

pub fn upload_raw_payload_blocking(
    envelope_id: Uuid,
    received_at: DateTime<Utc>,
    raw_payload: &[u8],
) -> Result<String, RawPayloadStoreError> {
    if raw_payload.is_empty() {
        return Err(RawPayloadStoreError::Storage(
            "raw payload is empty".to_string(),
        ));
    }

    let base = resolve_project_url()?;
    let key = resolve_service_key()?;
    let bucket = resolve_storage_bucket();
    let path = build_object_path(envelope_id, received_at);
    let url = build_object_url(&base, &bucket, &path);

    let client = reqwest::blocking::Client::new();
    ensure_bucket_ready_blocking(&client, &base, &bucket, &key)?;

    let response = client
        .post(url)
        .header("Authorization", format!("Bearer {}", key))
        .header("apikey", &key)
        .header("x-upsert", "true")
        .body(raw_payload.to_vec())
        .send()?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().unwrap_or_default();
        return Err(RawPayloadStoreError::Storage(format!(
            "upload failed (status {}): {}",
            status, body
        )));
    }

    Ok(to_supabase_ref(&bucket, &path))
}

pub fn download_raw_payload(reference: &str) -> Result<Vec<u8>, RawPayloadStoreError> {
    let base = resolve_project_url()?;
    let key = resolve_service_key()?;

    let (bucket, path_or_url) = parse_supabase_ref(reference)?;
    let url = if bucket.is_empty() {
        path_or_url
    } else {
        build_object_url(&base, &bucket, &path_or_url)
    };

    let client = reqwest::blocking::Client::new();
    let response = client
        .get(url)
        .header("Authorization", format!("Bearer {}", key))
        .header("apikey", key)
        .send()?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().unwrap_or_default();
        return Err(RawPayloadStoreError::Storage(format!(
            "download failed (status {}): {}",
            status, body
        )));
    }

    let bytes = response.bytes()?.to_vec();
    Ok(bytes)
}
