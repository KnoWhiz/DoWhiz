use azure_storage::StorageCredentials;
use azure_storage_blobs::prelude::*;
use futures::StreamExt;
use std::env;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tracing::{error, info};
use uuid::Uuid;

use crate::memory_store::DEFAULT_MEMO_CONTENT;

#[derive(Debug, thiserror::Error)]
pub enum BlobStoreError {
    #[error("missing AZURE_STORAGE_CONNECTION_STRING")]
    MissingConnectionString,
    #[error("missing AZURE_STORAGE_CONTAINER")]
    MissingContainer,
    #[error("invalid connection string: {0}")]
    InvalidConnectionString(String),
    #[error("azure error: {0}")]
    Azure(String),
}

/// Client for reading/writing memo.md files to Azure Blob Storage
#[derive(Clone)]
pub struct BlobStore {
    container_client: Arc<ContainerClient>,
    container_name: String,
}

/// Parse a connection string into (account_name, account_key)
fn parse_connection_string(connection_string: &str) -> Result<(String, String), BlobStoreError> {
    let mut account_name = None;
    let mut account_key = None;

    for part in connection_string.split(';') {
        if let Some(val) = part.strip_prefix("AccountName=") {
            account_name = Some(val.to_string());
        } else if let Some(val) = part.strip_prefix("AccountKey=") {
            account_key = Some(val.to_string());
        }
    }

    match (account_name, account_key) {
        (Some(name), Some(key)) => Ok((name, key)),
        _ => Err(BlobStoreError::InvalidConnectionString(
            "missing AccountName or AccountKey".to_string(),
        )),
    }
}

impl BlobStore {
    /// Create a new BlobStore from environment variables
    pub fn from_env() -> Result<Self, BlobStoreError> {
        let connection_string = env::var("AZURE_STORAGE_CONNECTION_STRING")
            .ok()
            .filter(|s| !s.trim().is_empty())
            .ok_or(BlobStoreError::MissingConnectionString)?;

        let container_name = env::var("AZURE_STORAGE_CONTAINER")
            .ok()
            .filter(|s| !s.trim().is_empty())
            .ok_or(BlobStoreError::MissingContainer)?;

        Self::new(&connection_string, &container_name)
    }

    /// Create a new BlobStore with explicit credentials
    pub fn new(connection_string: &str, container_name: &str) -> Result<Self, BlobStoreError> {
        let (account_name, account_key) = parse_connection_string(connection_string)?;

        let storage_credentials = StorageCredentials::access_key(&account_name, account_key);

        let container_client = Arc::new(
            BlobServiceClient::new(&account_name, storage_credentials)
                .container_client(container_name),
        );

        Ok(Self {
            container_client,
            container_name: container_name.to_string(),
        })
    }

    /// Get the blob path for an account's memo.md (new layout)
    fn memo_blob_path(account_id: Uuid) -> String {
        format!("accounts/{}/memo/memo.md", account_id)
    }

    /// Legacy memo path (pre-unified layout)
    fn legacy_memo_blob_path(account_id: Uuid) -> String {
        format!("{}/memo.md", account_id)
    }

    /// Memo path for a legacy user (non-unified account)
    fn user_memo_blob_path(user_id: &str) -> String {
        format!("users/{}/memo/memo.md", user_id)
    }


    /// Read memo.md for an account, returns default content if not found
    pub async fn read_memo(&self, account_id: Uuid) -> Result<String, BlobStoreError> {
        let primary = Self::memo_blob_path(account_id);
        match self.read_text_blob(&primary).await {
            Ok(content) => {
                info!(
                    "Read memo.md for account {} ({} bytes)",
                    account_id,
                    content.len()
                );
                Ok(content)
            }
            Err(BlobStoreError::Azure(err)) if is_not_found(&err) => {
                // Try legacy path
                let legacy = Self::legacy_memo_blob_path(account_id);
                match self.read_text_blob(&legacy).await {
                    Ok(content) => {
                        info!(
                            "Read legacy memo for account {} ({} bytes)",
                            account_id,
                            content.len()
                        );
                        Ok(content)
                    }
                    Err(BlobStoreError::Azure(err)) if is_not_found(&err) => {
                        info!(
                            "Memo not found for account {}, returning default",
                            account_id
                        );
                        Ok(DEFAULT_MEMO_CONTENT.to_string())
                    }
                    Err(err) => {
                        error!("Failed to read memo for account {}: {}", account_id, err);
                        Err(err)
                    }
                }
            }
            Err(err) => {
                error!("Failed to read memo for account {}: {}", account_id, err);
                Err(err)
            }
        }
    }

    /// Write memo.md for an account
    pub async fn write_memo(&self, account_id: Uuid, content: &str) -> Result<(), BlobStoreError> {
        let blob_path = Self::memo_blob_path(account_id);
        self.write_blob_bytes(&blob_path, content.as_bytes())
            .await
            .map_err(|e| {
                error!("Failed to write memo for account {}: {}", account_id, e);
                e
            })?;

        info!(
            "Wrote memo.md for account {} ({} bytes)",
            account_id,
            content.len()
        );
        Ok(())
    }

    /// Read memo.md for a legacy user (non-unified account)
    pub async fn read_user_memo(&self, user_id: &str) -> Result<String, BlobStoreError> {
        let blob_path = Self::user_memo_blob_path(user_id);
        match self.read_text_blob(&blob_path).await {
            Ok(content) => {
                info!(
                    "Read memo.md for user {} ({} bytes)",
                    user_id,
                    content.len()
                );
                Ok(content)
            }
            Err(BlobStoreError::Azure(err)) if is_not_found(&err) => {
                info!("Memo not found for user {}, returning default", user_id);
                Ok(DEFAULT_MEMO_CONTENT.to_string())
            }
            Err(err) => {
                error!("Failed to read memo for user {}: {}", user_id, err);
                Err(err)
            }
        }
    }

    /// Write memo.md for a legacy user (non-unified account)
    pub async fn write_user_memo(&self, user_id: &str, content: &str) -> Result<(), BlobStoreError> {
        let blob_path = Self::user_memo_blob_path(user_id);
        self.write_blob_bytes(&blob_path, content.as_bytes())
            .await
            .map_err(|e| {
                error!("Failed to write memo for user {}: {}", user_id, e);
                e
            })?;
        info!(
            "Wrote memo.md for user {} ({} bytes)",
            user_id,
            content.len()
        );
        Ok(())
    }

    /// Write raw bytes to a blob path
    pub async fn write_blob_bytes(
        &self,
        blob_path: &str,
        bytes: &[u8],
    ) -> Result<(), BlobStoreError> {
        let blob_client = self.container_client.blob_client(blob_path);
        blob_client
            .put_block_blob(bytes.to_vec())
            .await
            .map_err(|e| BlobStoreError::Azure(e.to_string()))?;
        Ok(())
    }

    /// Upload a local file to Azure Blob Storage
    pub async fn upload_file(
        &self,
        local_path: &Path,
        blob_path: &str,
    ) -> Result<(), BlobStoreError> {
        let bytes = std::fs::read(local_path)
            .map_err(|e| BlobStoreError::Azure(e.to_string()))?;
        self.write_blob_bytes(blob_path, &bytes)
            .await
    }

    /// Upload a directory recursively to Azure Blob Storage
    pub async fn upload_dir(
        &self,
        local_root: &Path,
        blob_prefix: &str,
    ) -> Result<(), BlobStoreError> {
        if !local_root.exists() {
            return Ok(());
        }

        let mut files = Vec::new();
        collect_files(local_root, &mut files)
            .map_err(|e| BlobStoreError::Azure(e.to_string()))?;

        for file_path in files {
            let relative = file_path
                .strip_prefix(local_root)
                .map_err(|e| BlobStoreError::Azure(e.to_string()))?;
            let relative_str = relative
                .to_string_lossy()
                .replace(std::path::MAIN_SEPARATOR, "/");
            let blob_path = format!("{}/{}", blob_prefix.trim_end_matches('/'), relative_str);
            self.upload_file(&file_path, &blob_path).await?;
        }
        Ok(())
    }

    /// Delete memo.md for an account (used when account is deleted)
    pub async fn delete_memo(&self, account_id: Uuid) -> Result<(), BlobStoreError> {
        let blob_path = Self::memo_blob_path(account_id);
        let blob_client = self.container_client.blob_client(&blob_path);

        match blob_client.delete().await {
            Ok(_) => {
                info!("Deleted memo.md for account {}", account_id);
                Ok(())
            }
            Err(e) => {
                let error_str = e.to_string();
                // Ignore if already deleted
                if error_str.contains("BlobNotFound") || error_str.contains("404") {
                    info!("Memo already deleted for account {}", account_id);
                    Ok(())
                } else {
                    error!("Failed to delete memo for account {}: {}", account_id, e);
                    Err(BlobStoreError::Azure(e.to_string()))
                }
            }
        }
    }

    /// Check if a memo exists for an account
    pub async fn memo_exists(&self, account_id: Uuid) -> Result<bool, BlobStoreError> {
        let blob_path = Self::memo_blob_path(account_id);
        let blob_client = self.container_client.blob_client(&blob_path);

        match blob_client.get_properties().await {
            Ok(_) => Ok(true),
            Err(e) => {
                let error_str = e.to_string();
                if error_str.contains("BlobNotFound") || error_str.contains("404") {
                    Ok(false)
                } else {
                    Err(BlobStoreError::Azure(e.to_string()))
                }
            }
        }
    }

    /// List all account IDs that have memos (for debugging/admin)
    pub async fn list_accounts_with_memos(&self) -> Result<Vec<Uuid>, BlobStoreError> {
        let mut account_ids = Vec::new();
        let mut stream = self.container_client.list_blobs().into_stream();

        while let Some(result) = stream.next().await {
            let response = result.map_err(|e| BlobStoreError::Azure(e.to_string()))?;
            for blob in response.blobs.blobs() {
                // Parse account_id from path like "accounts/{uuid}/memo/memo.md"
                if let Some(stripped) = blob.name.strip_prefix("accounts/") {
                    if let Some(account_str) = stripped.split('/').next() {
                        if let Ok(account_id) = Uuid::parse_str(account_str) {
                            account_ids.push(account_id);
                        }
                    }
                } else if let Some(account_str) = blob.name.strip_suffix("/memo.md") {
                    // Legacy path support
                    if let Ok(account_id) = Uuid::parse_str(account_str) {
                        account_ids.push(account_id);
                    }
                }
            }
        }

        Ok(account_ids)
    }
}

fn collect_files(dir: &Path, files: &mut Vec<PathBuf>) -> Result<(), std::io::Error> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if entry.file_type()?.is_dir() {
            collect_files(&path, files)?;
        } else if entry.file_type()?.is_file() {
            files.push(path);
        }
    }
    Ok(())
}

fn is_not_found(error: &str) -> bool {
    error.contains("BlobNotFound") || error.contains("404")
}

impl BlobStore {
    async fn read_text_blob(&self, blob_path: &str) -> Result<String, BlobStoreError> {
        let blob_client = self.container_client.blob_client(blob_path);
        match blob_client.get_content().await {
            Ok(data) => String::from_utf8(data)
                .map_err(|e| BlobStoreError::Azure(format!("invalid UTF-8: {}", e))),
            Err(e) => Err(BlobStoreError::Azure(e.to_string())),
        }
    }
}

/// Lazy-initialized global BlobStore for unified accounts
static BLOB_STORE: std::sync::OnceLock<Option<Arc<BlobStore>>> = std::sync::OnceLock::new();

/// Get or initialize the global BlobStore (returns None if not configured)
pub fn get_blob_store() -> Option<Arc<BlobStore>> {
    BLOB_STORE
        .get_or_init(|| {
            match BlobStore::from_env() {
                Ok(store) => {
                    info!("BlobStore initialized for unified memo storage");
                    Some(Arc::new(store))
                }
                Err(e) => {
                    info!("BlobStore not available ({}), using local storage only", e);
                    None
                }
            }
        })
        .clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Requires Azure credentials
    async fn test_blob_store_roundtrip() {
        dotenvy::dotenv().ok();
        let store = BlobStore::from_env().expect("BlobStore::from_env");

        let test_account_id = Uuid::new_v4();
        let test_content = "# Test Memo\n\n## Section\n- Item 1\n";

        // Write
        store
            .write_memo(test_account_id, test_content)
            .await
            .expect("write");

        // Read back
        let read_content = store.read_memo(test_account_id).await.expect("read");
        assert_eq!(read_content, test_content);

        // Delete
        store.delete_memo(test_account_id).await.expect("delete");

        // Verify returns default after delete
        let after_delete = store
            .read_memo(test_account_id)
            .await
            .expect("read after delete");
        assert_eq!(after_delete, DEFAULT_MEMO_CONTENT);
    }
}
