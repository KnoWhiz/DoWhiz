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
    #[error("missing Azure blob storage configuration")]
    MissingAzureConfig,
}

pub fn resolve_storage_bucket() -> String {
    std::env::var("SUPABASE_STORAGE_BUCKET")
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| DEFAULT_BUCKET.to_string())
}

fn resolve_raw_payload_backend() -> String {
    std::env::var("RAW_PAYLOAD_STORAGE_BACKEND")
        .ok()
        .map(|value| value.trim().to_ascii_lowercase())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "supabase".to_string())
}

fn resolve_azure_container() -> Result<String, RawPayloadStoreError> {
    std::env::var("AZURE_STORAGE_CONTAINER")
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .ok_or(RawPayloadStoreError::MissingAzureConfig)
}

fn resolve_azure_container_sas_url() -> Result<String, RawPayloadStoreError> {
    if let Ok(url) = std::env::var("AZURE_STORAGE_CONTAINER_SAS_URL") {
        let trimmed = url.trim();
        if !trimmed.is_empty() {
            return Ok(trimmed.to_string());
        }
    }
    let account = std::env::var("AZURE_STORAGE_ACCOUNT")
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .or_else(resolve_account_from_connection_string);
    let container = std::env::var("AZURE_STORAGE_CONTAINER")
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());
    let sas = std::env::var("AZURE_STORAGE_SAS_TOKEN")
        .ok()
        .map(|value| value.trim().trim_start_matches('?').to_string())
        .filter(|value| !value.is_empty());
    match (account, container, sas) {
        (Some(account), Some(container), Some(sas)) => Ok(format!(
            "https://{}.blob.core.windows.net/{}?{}",
            account, container, sas
        )),
        _ => Err(RawPayloadStoreError::MissingAzureConfig),
    }
}

fn resolve_account_from_connection_string() -> Option<String> {
    let conn_str = std::env::var("AZURE_STORAGE_CONNECTION_STRING_INGEST").ok()?;
    parse_connection_string_kv(&conn_str, "AccountName")
}

fn parse_connection_string_kv(conn_str: &str, key: &str) -> Option<String> {
    for segment in conn_str.split(';') {
        let mut parts = segment.splitn(2, '=');
        let candidate_key = parts.next()?.trim();
        let value = parts.next()?.trim();
        if candidate_key == key && !value.is_empty() {
            return Some(value.to_string());
        }
    }
    None
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

fn build_azure_blob_url(container_sas_url: &str, path: &str) -> String {
    let mut parts = container_sas_url.splitn(2, '?');
    let base = parts.next().unwrap_or("").trim_end_matches('/');
    let sas = parts.next().unwrap_or("").trim_start_matches('?');
    if sas.is_empty() {
        format!("{}/{}", base, path)
    } else {
        format!("{}/{}?{}", base, path, sas)
    }
}

fn to_azure_ref(container: &str, path: &str) -> String {
    format!("azure://{}/{}", container, path)
}

fn parse_azure_ref(reference: &str) -> Result<(String, String), RawPayloadStoreError> {
    let Some(value) = reference.strip_prefix("azure://") else {
        return Err(RawPayloadStoreError::InvalidReference(
            reference.to_string(),
        ));
    };
    let mut parts = value.splitn(2, '/');
    let container = parts
        .next()
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
        .ok_or_else(|| RawPayloadStoreError::InvalidReference(reference.to_string()))?;
    let path = parts
        .next()
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
        .ok_or_else(|| RawPayloadStoreError::InvalidReference(reference.to_string()))?;
    Ok((container.to_string(), path.to_string()))
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
    if resolve_raw_payload_backend() == "azure" {
        return upload_raw_payload_azure(envelope_id, received_at, raw_payload).await;
    }
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
    if resolve_raw_payload_backend() == "azure" {
        return upload_raw_payload_azure_blocking(envelope_id, received_at, raw_payload);
    }
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
    if reference.starts_with("azure://") {
        let (_container, path) = parse_azure_ref(reference)?;
        let container_sas_url = resolve_azure_container_sas_url()?;
        let url = build_azure_blob_url(&container_sas_url, &path);
        let client = reqwest::blocking::Client::new();
        let response = client.get(url).send()?;
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            return Err(RawPayloadStoreError::Storage(format!(
                "download failed (status {}): {}",
                status, body
            )));
        }
        let bytes = response.bytes()?.to_vec();
        return Ok(bytes);
    }
    let base = resolve_project_url()?;
    let key = resolve_service_key()?;

    let (bucket, path_or_url) = parse_supabase_ref(reference)?;
    let url = if bucket.is_empty() {
        path_or_url
    } else {
        build_object_url(&base, &bucket, &path_or_url)
    };

    let client = reqwest::blocking::Client::new();
    let response = if is_azure_blob_url(&url) {
        client.get(url).send()?
    } else {
        client
            .get(url)
            .header("Authorization", format!("Bearer {}", key))
            .header("apikey", key)
            .send()?
    };

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

fn is_azure_blob_url(url: &str) -> bool {
    let lower = url.to_ascii_lowercase();
    lower.contains("blob.core.windows.net") || lower.contains("sig=")
}

async fn upload_raw_payload_azure(
    envelope_id: Uuid,
    received_at: DateTime<Utc>,
    raw_payload: &[u8],
) -> Result<String, RawPayloadStoreError> {
    if raw_payload.is_empty() {
        return Err(RawPayloadStoreError::Storage(
            "raw payload is empty".to_string(),
        ));
    }
    let container = resolve_azure_container()?;
    let container_sas_url = resolve_azure_container_sas_url()?;
    let path = build_object_path(envelope_id, received_at);
    let url = build_azure_blob_url(&container_sas_url, &path);

    let client = Client::new();
    let response = client
        .put(url)
        .header("x-ms-blob-type", "BlockBlob")
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
    Ok(to_azure_ref(&container, &path))
}

fn upload_raw_payload_azure_blocking(
    envelope_id: Uuid,
    received_at: DateTime<Utc>,
    raw_payload: &[u8],
) -> Result<String, RawPayloadStoreError> {
    if raw_payload.is_empty() {
        return Err(RawPayloadStoreError::Storage(
            "raw payload is empty".to_string(),
        ));
    }
    let container = resolve_azure_container()?;
    let container_sas_url = resolve_azure_container_sas_url()?;
    let path = build_object_path(envelope_id, received_at);
    let url = build_azure_blob_url(&container_sas_url, &path);

    let client = reqwest::blocking::Client::new();
    let response = client
        .put(url)
        .header("x-ms-blob-type", "BlockBlob")
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
    Ok(to_azure_ref(&container, &path))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use std::env;
    use uuid::Uuid;

    #[test]
    fn azure_upload_download_roundtrip() {
        dotenvy::dotenv().ok();
        let backend = std::env::var("RAW_PAYLOAD_STORAGE_BACKEND")
            .unwrap_or_else(|_| "supabase".to_string());
        if backend.trim().to_ascii_lowercase() != "azure" {
            eprintln!("RAW_PAYLOAD_STORAGE_BACKEND is not azure; skipping.");
            return;
        }
        let payload = b"azure-roundtrip-test";
        let envelope_id = Uuid::new_v4();
        let received_at = Utc::now();
        let reference = upload_raw_payload_blocking(envelope_id, received_at, payload)
            .expect("upload");
        let downloaded = download_raw_payload(&reference).expect("download");
        assert_eq!(payload.to_vec(), downloaded);
    }

    #[test]
    fn azure_connection_string_fallback_for_account() {
        let original_account = env::var("AZURE_STORAGE_ACCOUNT").ok();
        let original_container = env::var("AZURE_STORAGE_CONTAINER").ok();
        let original_sas = env::var("AZURE_STORAGE_SAS_TOKEN").ok();
        let original_conn = env::var("AZURE_STORAGE_CONNECTION_STRING_INGEST").ok();

        env::remove_var("AZURE_STORAGE_ACCOUNT");
        env::set_var("AZURE_STORAGE_CONTAINER", "ingestion-raw");
        env::set_var("AZURE_STORAGE_SAS_TOKEN", "sig=test");
        env::set_var(
            "AZURE_STORAGE_CONNECTION_STRING_INGEST",
            "DefaultEndpointsProtocol=https;AccountName=testaccount;AccountKey=key;EndpointSuffix=core.windows.net",
        );

        let url = resolve_azure_container_sas_url().expect("sas url");
        assert_eq!(
            url,
            "https://testaccount.blob.core.windows.net/ingestion-raw?sig=test"
        );

        match original_account {
            Some(value) => env::set_var("AZURE_STORAGE_ACCOUNT", value),
            None => env::remove_var("AZURE_STORAGE_ACCOUNT"),
        }
        match original_container {
            Some(value) => env::set_var("AZURE_STORAGE_CONTAINER", value),
            None => env::remove_var("AZURE_STORAGE_CONTAINER"),
        }
        match original_sas {
            Some(value) => env::set_var("AZURE_STORAGE_SAS_TOKEN", value),
            None => env::remove_var("AZURE_STORAGE_SAS_TOKEN"),
        }
        match original_conn {
            Some(value) => env::set_var("AZURE_STORAGE_CONNECTION_STRING_INGEST", value),
            None => env::remove_var("AZURE_STORAGE_CONNECTION_STRING_INGEST"),
        }
    }
}
