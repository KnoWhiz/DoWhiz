use azure_storage::StorageCredentials;
use azure_storage_blobs::prelude::*;
use futures::StreamExt;
use std::env;
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

    /// Get the blob path for an account's memo.md
    fn memo_blob_path(account_id: Uuid) -> String {
        format!("{}/memo.md", account_id)
    }

    /// Read memo.md for an account, returns default content if not found
    pub async fn read_memo(&self, account_id: Uuid) -> Result<String, BlobStoreError> {
        let blob_path = Self::memo_blob_path(account_id);
        let blob_client = self.container_client.blob_client(&blob_path);

        match blob_client.get_content().await {
            Ok(data) => {
                let content = String::from_utf8(data)
                    .map_err(|e| BlobStoreError::Azure(format!("invalid UTF-8: {}", e)))?;
                info!(
                    "Read memo.md for account {} ({} bytes)",
                    account_id,
                    content.len()
                );
                Ok(content)
            }
            Err(e) => {
                // Check if it's a 404 (blob not found)
                let error_str = e.to_string();
                if error_str.contains("BlobNotFound") || error_str.contains("404") {
                    info!(
                        "Memo not found for account {}, returning default",
                        account_id
                    );
                    Ok(DEFAULT_MEMO_CONTENT.to_string())
                } else {
                    error!("Failed to read memo for account {}: {}", account_id, e);
                    Err(BlobStoreError::Azure(e.to_string()))
                }
            }
        }
    }

    /// Write memo.md for an account
    pub async fn write_memo(&self, account_id: Uuid, content: &str) -> Result<(), BlobStoreError> {
        let blob_path = Self::memo_blob_path(account_id);
        let blob_client = self.container_client.blob_client(&blob_path);

        blob_client
            .put_block_blob(content.as_bytes().to_vec())
            .content_type("text/markdown")
            .await
            .map_err(|e| {
                error!("Failed to write memo for account {}: {}", account_id, e);
                BlobStoreError::Azure(e.to_string())
            })?;

        info!(
            "Wrote memo.md for account {} ({} bytes)",
            account_id,
            content.len()
        );
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
                // Parse account_id from path like "uuid/memo.md"
                if let Some(account_str) = blob.name.strip_suffix("/memo.md") {
                    if let Ok(account_id) = Uuid::parse_str(account_str) {
                        account_ids.push(account_id);
                    }
                }
            }
        }

        Ok(account_ids)
    }
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
