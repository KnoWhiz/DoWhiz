use chrono::{DateTime, Utc};
use reqwest::{Client, StatusCode};
use serde_json::json;
use std::future::Future;
use std::sync::OnceLock;
use uuid::Uuid;

use azure_storage::StorageCredentials;
use azure_storage_blobs::prelude::*;

use crate::env_alias::var_with_scale_oliver;

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
    var_with_scale_oliver("RAW_PAYLOAD_STORAGE_BACKEND")
        .map(|value| value.to_ascii_lowercase())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "supabase".to_string())
}

fn resolve_azure_container() -> Result<String, RawPayloadStoreError> {
    var_with_scale_oliver("AZURE_STORAGE_CONTAINER_INGEST")
        .ok_or(RawPayloadStoreError::MissingAzureConfig)
}

fn resolve_azure_container_sas_url() -> Result<String, RawPayloadStoreError> {
    if let Some(url) = var_with_scale_oliver("AZURE_STORAGE_CONTAINER_SAS_URL") {
        return Ok(url);
    }
    let account = var_with_scale_oliver("AZURE_STORAGE_ACCOUNT")
        .or_else(resolve_account_from_connection_string);
    let container = var_with_scale_oliver("AZURE_STORAGE_CONTAINER_INGEST");
    let sas = var_with_scale_oliver("AZURE_STORAGE_SAS_TOKEN")
        .map(|value| value.trim_start_matches('?').to_string())
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
    let conn_str = resolve_connection_string_for_ingest()?;
    parse_connection_string_kv(&conn_str, "AccountName")
}

fn resolve_access_key_from_connection_string() -> Option<String> {
    let conn_str = resolve_connection_string_for_ingest()?;
    parse_connection_string_kv(&conn_str, "AccountKey")
}

fn resolve_connection_string_for_ingest() -> Option<String> {
    var_with_scale_oliver("AZURE_STORAGE_CONNECTION_STRING_INGEST")
        .or_else(|| var_with_scale_oliver("AZURE_STORAGE_CONNECTION_STRING"))
        .or_else(|| var_with_scale_oliver("DOWHIZ_AZURE_STORAGE_CONNECTION_STRING"))
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

fn resolve_raw_payload_path_prefix() -> String {
    var_with_scale_oliver("RAW_PAYLOAD_PATH_PREFIX")
        .map(|value| value.trim().trim_matches('/').to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| DEFAULT_PREFIX.to_string())
}

fn build_object_path(envelope_id: Uuid, received_at: DateTime<Utc>) -> String {
    let date = received_at.format("%Y/%m/%d");
    let prefix = resolve_raw_payload_path_prefix();
    format!("{}/{}/{}.bin", prefix, date, envelope_id)
}

fn resolve_attachment_path_prefix() -> String {
    let base = resolve_raw_payload_path_prefix();
    format!("{}/attachments", base)
}

fn sanitize_blob_segment(value: &str, fallback: &str) -> String {
    let trimmed = value.trim();
    let mut out = String::new();
    for ch in trimmed.chars() {
        if ch.is_ascii_alphanumeric() || matches!(ch, '.' | '_' | '-') {
            out.push(ch);
        } else {
            out.push('_');
        }
    }
    let cleaned = out.trim_matches(&['.', '_', '-'][..]).to_string();
    if cleaned.is_empty() {
        fallback.to_string()
    } else {
        cleaned
    }
}

fn build_attachment_object_path(
    envelope_id: Uuid,
    received_at: DateTime<Utc>,
    attachment_index: usize,
    file_name: &str,
) -> String {
    let date = received_at.format("%Y/%m/%d");
    let prefix = resolve_attachment_path_prefix();
    let file_token = sanitize_blob_segment(file_name, "attachment");
    format!(
        "{}/{}/{}/{:03}_{}",
        prefix, date, envelope_id, attachment_index, file_token
    )
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
        let (container, path) = parse_azure_ref(reference)?;
        if let Ok(container_sas_url) = resolve_azure_container_sas_url() {
            let url = build_azure_blob_url(&container_sas_url, &path);
            let client = reqwest::blocking::Client::new();
            let response = client.get(url).send()?;
            if response.status().is_success() {
                let bytes = response.bytes()?.to_vec();
                return Ok(bytes);
            }
            // Fall through to connection-string auth if available.
            if resolve_connection_string_for_ingest().is_none() {
                let status = response.status();
                let body = response.text().unwrap_or_default();
                return Err(RawPayloadStoreError::Storage(format!(
                    "download failed (status {}): {}",
                    status, body
                )));
            }
        }
        return download_raw_payload_azure_via_connection_string(&container, &path);
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

fn download_raw_payload_azure_via_connection_string(
    container: &str,
    path: &str,
) -> Result<Vec<u8>, RawPayloadStoreError> {
    let account =
        resolve_account_from_connection_string().ok_or(RawPayloadStoreError::MissingAzureConfig)?;
    let key = resolve_access_key_from_connection_string()
        .ok_or(RawPayloadStoreError::MissingAzureConfig)?;
    let container = container.to_string();
    let path = path.to_string();

    run_with_tokio_runtime(async move {
        let creds = StorageCredentials::access_key(&account, key);
        let blob_client = BlobServiceClient::new(&account, creds)
            .container_client(container)
            .blob_client(path);

        blob_client.get_content().await.map_err(|err| {
            RawPayloadStoreError::Storage(format!("download failed via connection string: {}", err))
        })
    })
}

fn run_with_tokio_runtime<F, T>(future: F) -> Result<T, RawPayloadStoreError>
where
    F: Future<Output = Result<T, RawPayloadStoreError>> + Send + 'static,
    T: Send + 'static,
{
    let handle = std::thread::Builder::new()
        .name("raw-payload-azure-download".to_string())
        .spawn(move || {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .map_err(|err| {
                    RawPayloadStoreError::Storage(format!(
                        "failed to initialize tokio runtime: {}",
                        err
                    ))
                })?;
            runtime.block_on(future)
        })
        .map_err(|err| {
            RawPayloadStoreError::Storage(format!("failed to spawn azure download thread: {}", err))
        })?;

    handle
        .join()
        .map_err(|_| RawPayloadStoreError::Storage("azure download thread panicked".to_string()))?
}

pub fn resolve_azure_blob_url(reference: &str) -> Result<String, RawPayloadStoreError> {
    let (_container, path) = parse_azure_ref(reference)?;
    let container_sas_url = resolve_azure_container_sas_url()?;
    Ok(build_azure_blob_url(&container_sas_url, &path))
}

fn is_azure_blob_url(url: &str) -> bool {
    let lower = url.to_ascii_lowercase();
    lower.contains("blob.core.windows.net") || lower.contains("sig=")
}

async fn upload_azure_bytes(path: &str, payload: &[u8]) -> Result<String, RawPayloadStoreError> {
    let container = resolve_azure_container()?;
    let container_sas_url = resolve_azure_container_sas_url()?;
    let url = build_azure_blob_url(&container_sas_url, path);

    let client = Client::new();
    let response = client
        .put(url)
        .header("x-ms-blob-type", "BlockBlob")
        .body(payload.to_vec())
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
    Ok(to_azure_ref(&container, path))
}

fn upload_azure_bytes_blocking(path: &str, payload: &[u8]) -> Result<String, RawPayloadStoreError> {
    let container = resolve_azure_container()?;
    let container_sas_url = resolve_azure_container_sas_url()?;
    let url = build_azure_blob_url(&container_sas_url, path);

    let client = reqwest::blocking::Client::new();
    let response = client
        .put(url)
        .header("x-ms-blob-type", "BlockBlob")
        .body(payload.to_vec())
        .send()?;
    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().unwrap_or_default();
        return Err(RawPayloadStoreError::Storage(format!(
            "upload failed (status {}): {}",
            status, body
        )));
    }
    Ok(to_azure_ref(&container, path))
}

pub async fn upload_raw_payload_azure(
    envelope_id: Uuid,
    received_at: DateTime<Utc>,
    raw_payload: &[u8],
) -> Result<String, RawPayloadStoreError> {
    if raw_payload.is_empty() {
        return Err(RawPayloadStoreError::Storage(
            "raw payload is empty".to_string(),
        ));
    }
    let path = build_object_path(envelope_id, received_at);
    upload_azure_bytes(&path, raw_payload).await
}

pub fn upload_raw_payload_azure_blocking(
    envelope_id: Uuid,
    received_at: DateTime<Utc>,
    raw_payload: &[u8],
) -> Result<String, RawPayloadStoreError> {
    if raw_payload.is_empty() {
        return Err(RawPayloadStoreError::Storage(
            "raw payload is empty".to_string(),
        ));
    }
    let path = build_object_path(envelope_id, received_at);
    upload_azure_bytes_blocking(&path, raw_payload)
}

pub async fn upload_attachment_azure(
    envelope_id: Uuid,
    received_at: DateTime<Utc>,
    attachment_index: usize,
    file_name: &str,
    bytes: &[u8],
) -> Result<String, RawPayloadStoreError> {
    if bytes.is_empty() {
        return Err(RawPayloadStoreError::Storage(
            "attachment payload is empty".to_string(),
        ));
    }
    let path = build_attachment_object_path(envelope_id, received_at, attachment_index, file_name);
    upload_azure_bytes(&path, bytes).await
}

pub fn upload_attachment_azure_blocking(
    envelope_id: Uuid,
    received_at: DateTime<Utc>,
    attachment_index: usize,
    file_name: &str,
    bytes: &[u8],
) -> Result<String, RawPayloadStoreError> {
    if bytes.is_empty() {
        return Err(RawPayloadStoreError::Storage(
            "attachment payload is empty".to_string(),
        ));
    }
    let path = build_attachment_object_path(envelope_id, received_at, attachment_index, file_name);
    upload_azure_bytes_blocking(&path, bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use std::env;
    use std::sync::{Mutex, OnceLock};
    use uuid::Uuid;

    static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

    fn lock_env() -> std::sync::MutexGuard<'static, ()> {
        ENV_LOCK
            .get_or_init(|| Mutex::new(()))
            .lock()
            .expect("env mutex poisoned")
    }

    #[test]
    fn azure_upload_download_roundtrip() {
        let _guard = lock_env();
        dotenvy::dotenv().ok();
        let backend = resolve_raw_payload_backend();
        if backend.trim().to_ascii_lowercase() != "azure" {
            eprintln!("RAW_PAYLOAD_STORAGE_BACKEND is not azure; skipping.");
            return;
        }
        let payload = b"azure-roundtrip-test";
        let envelope_id = Uuid::new_v4();
        let received_at = Utc::now();
        let reference =
            upload_raw_payload_blocking(envelope_id, received_at, payload).expect("upload");
        let downloaded = download_raw_payload(&reference).expect("download");
        assert_eq!(payload.to_vec(), downloaded);
    }

    #[test]
    fn azure_connection_string_fallback_for_account() {
        let _guard = lock_env();
        let original_account = env::var("AZURE_STORAGE_ACCOUNT").ok();
        let original_container = env::var("AZURE_STORAGE_CONTAINER_INGEST").ok();
        let original_sas = env::var("AZURE_STORAGE_SAS_TOKEN").ok();
        let original_conn = env::var("AZURE_STORAGE_CONNECTION_STRING_INGEST").ok();
        let original_container_sas_url = env::var("AZURE_STORAGE_CONTAINER_SAS_URL").ok();
        let original_prefixed_account = env::var("SCALE_OLIVER_AZURE_STORAGE_ACCOUNT").ok();
        let original_prefixed_container =
            env::var("SCALE_OLIVER_AZURE_STORAGE_CONTAINER_INGEST").ok();
        let original_prefixed_sas = env::var("SCALE_OLIVER_AZURE_STORAGE_SAS_TOKEN").ok();
        let original_prefixed_conn =
            env::var("SCALE_OLIVER_AZURE_STORAGE_CONNECTION_STRING_INGEST").ok();
        let original_prefixed_container_sas_url =
            env::var("SCALE_OLIVER_AZURE_STORAGE_CONTAINER_SAS_URL").ok();

        env::remove_var("AZURE_STORAGE_ACCOUNT");
        env::remove_var("SCALE_OLIVER_AZURE_STORAGE_ACCOUNT");
        env::set_var("AZURE_STORAGE_CONTAINER_INGEST", "ingestion-raw");
        env::set_var(
            "SCALE_OLIVER_AZURE_STORAGE_CONTAINER_INGEST",
            "ingestion-raw",
        );
        env::set_var("AZURE_STORAGE_SAS_TOKEN", "sig=test");
        env::set_var("SCALE_OLIVER_AZURE_STORAGE_SAS_TOKEN", "sig=test");
        env::remove_var("AZURE_STORAGE_CONTAINER_SAS_URL");
        env::remove_var("SCALE_OLIVER_AZURE_STORAGE_CONTAINER_SAS_URL");
        env::set_var(
            "AZURE_STORAGE_CONNECTION_STRING_INGEST",
            "DefaultEndpointsProtocol=https;AccountName=testaccount;AccountKey=key;EndpointSuffix=core.windows.net",
        );
        env::set_var(
            "SCALE_OLIVER_AZURE_STORAGE_CONNECTION_STRING_INGEST",
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
        match original_prefixed_account {
            Some(value) => env::set_var("SCALE_OLIVER_AZURE_STORAGE_ACCOUNT", value),
            None => env::remove_var("SCALE_OLIVER_AZURE_STORAGE_ACCOUNT"),
        }
        match original_container {
            Some(value) => env::set_var("AZURE_STORAGE_CONTAINER_INGEST", value),
            None => env::remove_var("AZURE_STORAGE_CONTAINER_INGEST"),
        }
        match original_prefixed_container {
            Some(value) => env::set_var("SCALE_OLIVER_AZURE_STORAGE_CONTAINER_INGEST", value),
            None => env::remove_var("SCALE_OLIVER_AZURE_STORAGE_CONTAINER_INGEST"),
        }
        match original_sas {
            Some(value) => env::set_var("AZURE_STORAGE_SAS_TOKEN", value),
            None => env::remove_var("AZURE_STORAGE_SAS_TOKEN"),
        }
        match original_prefixed_sas {
            Some(value) => env::set_var("SCALE_OLIVER_AZURE_STORAGE_SAS_TOKEN", value),
            None => env::remove_var("SCALE_OLIVER_AZURE_STORAGE_SAS_TOKEN"),
        }
        match original_conn {
            Some(value) => env::set_var("AZURE_STORAGE_CONNECTION_STRING_INGEST", value),
            None => env::remove_var("AZURE_STORAGE_CONNECTION_STRING_INGEST"),
        }
        match original_prefixed_conn {
            Some(value) => {
                env::set_var("SCALE_OLIVER_AZURE_STORAGE_CONNECTION_STRING_INGEST", value)
            }
            None => env::remove_var("SCALE_OLIVER_AZURE_STORAGE_CONNECTION_STRING_INGEST"),
        }
        match original_container_sas_url {
            Some(value) => env::set_var("AZURE_STORAGE_CONTAINER_SAS_URL", value),
            None => env::remove_var("AZURE_STORAGE_CONTAINER_SAS_URL"),
        }
        match original_prefixed_container_sas_url {
            Some(value) => env::set_var("SCALE_OLIVER_AZURE_STORAGE_CONTAINER_SAS_URL", value),
            None => env::remove_var("SCALE_OLIVER_AZURE_STORAGE_CONTAINER_SAS_URL"),
        }
    }

    #[test]
    fn raw_payload_path_prefix_defaults_when_missing() {
        let _guard = lock_env();
        let original = env::var("RAW_PAYLOAD_PATH_PREFIX").ok();
        let original_prefixed = env::var("SCALE_OLIVER_RAW_PAYLOAD_PATH_PREFIX").ok();
        env::remove_var("RAW_PAYLOAD_PATH_PREFIX");
        env::remove_var("SCALE_OLIVER_RAW_PAYLOAD_PATH_PREFIX");

        let path = build_object_path(
            Uuid::parse_str("11111111-1111-1111-1111-111111111111").expect("uuid"),
            DateTime::parse_from_rfc3339("2026-02-26T12:34:56Z")
                .expect("time")
                .with_timezone(&Utc),
        );
        assert!(path.starts_with("ingestion_raw/2026/02/26/"));

        match original {
            Some(value) => env::set_var("RAW_PAYLOAD_PATH_PREFIX", value),
            None => env::remove_var("RAW_PAYLOAD_PATH_PREFIX"),
        }
        match original_prefixed {
            Some(value) => env::set_var("SCALE_OLIVER_RAW_PAYLOAD_PATH_PREFIX", value),
            None => env::remove_var("SCALE_OLIVER_RAW_PAYLOAD_PATH_PREFIX"),
        }
    }

    #[test]
    fn raw_payload_path_prefix_honors_env_value() {
        let _guard = lock_env();
        let original = env::var("RAW_PAYLOAD_PATH_PREFIX").ok();
        let original_prefixed = env::var("SCALE_OLIVER_RAW_PAYLOAD_PATH_PREFIX").ok();
        env::set_var("RAW_PAYLOAD_PATH_PREFIX", "/staging/ingestion_raw/");
        env::remove_var("SCALE_OLIVER_RAW_PAYLOAD_PATH_PREFIX");

        let path = build_object_path(
            Uuid::parse_str("22222222-2222-2222-2222-222222222222").expect("uuid"),
            DateTime::parse_from_rfc3339("2026-02-26T12:34:56Z")
                .expect("time")
                .with_timezone(&Utc),
        );
        assert!(path.starts_with("staging/ingestion_raw/2026/02/26/"));

        match original {
            Some(value) => env::set_var("RAW_PAYLOAD_PATH_PREFIX", value),
            None => env::remove_var("RAW_PAYLOAD_PATH_PREFIX"),
        }
        match original_prefixed {
            Some(value) => env::set_var("SCALE_OLIVER_RAW_PAYLOAD_PATH_PREFIX", value),
            None => env::remove_var("SCALE_OLIVER_RAW_PAYLOAD_PATH_PREFIX"),
        }
    }

    #[test]
    fn resolve_connection_string_for_ingest_falls_back_to_generic_key() {
        let _guard = lock_env();
        let original_ingest = env::var("AZURE_STORAGE_CONNECTION_STRING_INGEST").ok();
        let original_prefixed_ingest =
            env::var("SCALE_OLIVER_AZURE_STORAGE_CONNECTION_STRING_INGEST").ok();
        let original_generic = env::var("AZURE_STORAGE_CONNECTION_STRING").ok();
        let original_prefixed_generic =
            env::var("SCALE_OLIVER_AZURE_STORAGE_CONNECTION_STRING").ok();

        env::remove_var("AZURE_STORAGE_CONNECTION_STRING_INGEST");
        env::remove_var("SCALE_OLIVER_AZURE_STORAGE_CONNECTION_STRING_INGEST");
        env::set_var(
            "AZURE_STORAGE_CONNECTION_STRING",
            "DefaultEndpointsProtocol=https;AccountName=fallbackacct;AccountKey=fallbackkey;EndpointSuffix=core.windows.net",
        );
        env::remove_var("SCALE_OLIVER_AZURE_STORAGE_CONNECTION_STRING");

        assert_eq!(
            resolve_account_from_connection_string().as_deref(),
            Some("fallbackacct")
        );
        assert_eq!(
            resolve_access_key_from_connection_string().as_deref(),
            Some("fallbackkey")
        );

        match original_ingest {
            Some(value) => env::set_var("AZURE_STORAGE_CONNECTION_STRING_INGEST", value),
            None => env::remove_var("AZURE_STORAGE_CONNECTION_STRING_INGEST"),
        }
        match original_prefixed_ingest {
            Some(value) => {
                env::set_var("SCALE_OLIVER_AZURE_STORAGE_CONNECTION_STRING_INGEST", value)
            }
            None => env::remove_var("SCALE_OLIVER_AZURE_STORAGE_CONNECTION_STRING_INGEST"),
        }
        match original_generic {
            Some(value) => env::set_var("AZURE_STORAGE_CONNECTION_STRING", value),
            None => env::remove_var("AZURE_STORAGE_CONNECTION_STRING"),
        }
        match original_prefixed_generic {
            Some(value) => env::set_var("SCALE_OLIVER_AZURE_STORAGE_CONNECTION_STRING", value),
            None => env::remove_var("SCALE_OLIVER_AZURE_STORAGE_CONNECTION_STRING"),
        }
    }

    #[test]
    fn run_with_tokio_runtime_executes_without_runtime_context() {
        let result = run_with_tokio_runtime(async { Ok::<_, RawPayloadStoreError>(123usize) })
            .expect("run_with_tokio_runtime should return value");
        assert_eq!(result, 123);
    }

    #[test]
    fn run_with_tokio_runtime_executes_inside_existing_tokio_runtime() {
        let runtime = tokio::runtime::Runtime::new().expect("tokio runtime");
        let result = runtime
            .block_on(async {
                run_with_tokio_runtime(async { Ok::<_, RawPayloadStoreError>(456usize) })
            })
            .expect("run_with_tokio_runtime should return value");
        assert_eq!(result, 456);
    }
}
