use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;

use crossbeam_channel::{bounded, Sender};
use tracing::info;
use uuid::Uuid;

use crate::blob_store::get_blob_store;
use crate::memory_diff::{apply_memory_diff, MemoryDiff};

/// A queued memory write operation
#[derive(Debug, Clone)]
pub struct MemoryWriteRequest {
    /// If set, use Azure Blob storage (unified account)
    pub account_id: Option<Uuid>,
    /// Legacy: user identifier for local storage
    pub user_id: String,
    /// Legacy: local directory for memo.md
    pub user_memory_dir: PathBuf,
    pub diff: MemoryDiff,
}

/// Internal request with completion signal
struct InternalRequest {
    request: MemoryWriteRequest,
    /// Channel to signal completion (with result)
    done: Sender<Result<(), MemoryQueueError>>,
}

/// Global memory write queue that ensures sequential writes per user
pub struct MemoryWriteQueue {
    /// Per-user/account channels for submitting diffs
    user_channels: Mutex<HashMap<String, Sender<InternalRequest>>>,
}

impl MemoryWriteQueue {
    pub fn new() -> Self {
        Self {
            user_channels: Mutex::new(HashMap::new()),
        }
    }

    /// Submit a diff to be applied to a user's memo.md
    /// Blocks until the diff is applied by the worker thread
    pub fn submit(&self, request: MemoryWriteRequest) -> Result<(), MemoryQueueError> {
        // Use account_id as key if present, otherwise user_id
        let queue_key = request
            .account_id
            .map(|id| format!("account:{}", id))
            .unwrap_or_else(|| format!("user:{}", request.user_id));

        // Create completion channel
        let (done_tx, done_rx) = bounded::<Result<(), MemoryQueueError>>(1);

        let internal_request = InternalRequest {
            request,
            done: done_tx,
        };

        // Get or create the user's channel
        let sender = {
            let mut channels = self
                .user_channels
                .lock()
                .map_err(|_| MemoryQueueError::LockPoisoned)?;

            if let Some(sender) = channels.get(&queue_key) {
                sender.clone()
            } else {
                // Create a new channel and worker for this user/account
                let (sender, receiver) = bounded::<InternalRequest>(100);

                // Spawn worker thread for this user/account
                let worker_key = queue_key.clone();
                thread::spawn(move || {
                    info!("Memory queue worker started for {}", worker_key);

                    // Create a tokio runtime for async blob operations
                    let rt = tokio::runtime::Builder::new_current_thread()
                        .enable_all()
                        .build()
                        .expect("Failed to create tokio runtime for memory queue worker");

                    for req in receiver {
                        let use_blob = memory_queue_use_blob() && get_blob_store().is_some();
                        let result = if use_blob {
                            // Use Azure Blob storage when available
                            rt.block_on(apply_diff_to_blob(&req.request))
                        } else {
                            // Use local file storage
                            apply_diff_to_file(&req.request)
                        };
                        // Signal completion (ignore send error if receiver dropped)
                        let _ = req.done.send(result);
                    }
                    info!("Memory queue worker stopped for {}", worker_key);
                });

                channels.insert(queue_key.clone(), sender.clone());
                sender
            }
        };

        // Send to queue
        sender
            .send(internal_request)
            .map_err(|_| MemoryQueueError::ChannelClosed)?;

        // Wait for worker to complete
        done_rx
            .recv()
            .map_err(|_| MemoryQueueError::ChannelClosed)?
    }
}

impl Default for MemoryWriteQueue {
    fn default() -> Self {
        Self::new()
    }
}

/// Apply a diff to the user's memo.md file (local storage)
fn apply_diff_to_file(request: &MemoryWriteRequest) -> Result<(), MemoryQueueError> {
    let memo_path = request.user_memory_dir.join("memo.md");

    // Read current content
    let current_content = if memo_path.exists() {
        fs::read_to_string(&memo_path).map_err(|e| MemoryQueueError::Io(Arc::new(e)))?
    } else {
        // Use default memo structure
        crate::memory_store::DEFAULT_MEMO_CONTENT.to_string()
    };

    // Apply diff
    let merged_content = apply_memory_diff(&current_content, &request.diff);

    // Write back
    if let Some(parent) = memo_path.parent() {
        fs::create_dir_all(parent).map_err(|e| MemoryQueueError::Io(Arc::new(e)))?;
    }
    fs::write(&memo_path, &merged_content).map_err(|e| MemoryQueueError::Io(Arc::new(e)))?;

    info!(
        "Applied memory diff for user {} ({} sections changed)",
        request.user_id,
        request.diff.changed_sections.len()
    );

    Ok(())
}

/// Apply a diff to Azure Blob storage (unified account storage)
async fn apply_diff_to_blob(request: &MemoryWriteRequest) -> Result<(), MemoryQueueError> {
    let blob_store = get_blob_store()
        .ok_or_else(|| MemoryQueueError::BlobStore("BlobStore not configured".to_string()))?;

    let (current_content, target_label) = if let Some(account_id) = request.account_id {
        let content = blob_store
            .read_memo(account_id)
            .await
            .map_err(|e| MemoryQueueError::BlobStore(e.to_string()))?;
        (content, format!("account {}", account_id))
    } else {
        let content = blob_store
            .read_user_memo(&request.user_id)
            .await
            .map_err(|e| MemoryQueueError::BlobStore(e.to_string()))?;
        (content, format!("user {}", request.user_id))
    };

    let merged_content = apply_memory_diff(&current_content, &request.diff);

    if let Some(account_id) = request.account_id {
        blob_store
            .write_memo(account_id, &merged_content)
            .await
            .map_err(|e| MemoryQueueError::BlobStore(e.to_string()))?;
    } else {
        blob_store
            .write_user_memo(&request.user_id, &merged_content)
            .await
            .map_err(|e| MemoryQueueError::BlobStore(e.to_string()))?;
    }

    info!(
        "Applied memory diff for {} ({} sections changed)",
        target_label,
        request.diff.changed_sections.len()
    );

    Ok(())
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum MemoryQueueError {
    #[error("IO error: {0}")]
    Io(#[source] Arc<std::io::Error>),
    #[error("Queue lock poisoned")]
    LockPoisoned,
    #[error("Channel closed")]
    ChannelClosed,
    #[error("Blob storage error: {0}")]
    BlobStore(String),
}

impl From<std::io::Error> for MemoryQueueError {
    fn from(err: std::io::Error) -> Self {
        MemoryQueueError::Io(Arc::new(err))
    }
}

/// Global singleton for the memory write queue
static MEMORY_QUEUE: std::sync::OnceLock<Arc<MemoryWriteQueue>> = std::sync::OnceLock::new();

/// Get the global memory write queue instance
pub fn global_memory_queue() -> Arc<MemoryWriteQueue> {
    MEMORY_QUEUE
        .get_or_init(|| Arc::new(MemoryWriteQueue::new()))
        .clone()
}

fn memory_queue_use_blob() -> bool {
    match env::var("MEMORY_QUEUE_USE_BLOB") {
        Ok(value) => {
            let normalized = value.trim().to_ascii_lowercase();
            !(normalized.is_empty() || normalized == "0" || normalized == "false")
        }
        Err(_) => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory_diff::SectionChange;
    use std::collections::HashMap;
    use tempfile::TempDir;

    struct EnvGuard {
        key: &'static str,
        prev: Option<String>,
    }

    impl EnvGuard {
        fn set(key: &'static str, value: &str) -> Self {
            let prev = std::env::var(key).ok();
            std::env::set_var(key, value);
            Self { key, prev }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            match &self.prev {
                Some(value) => std::env::set_var(self.key, value),
                None => std::env::remove_var(self.key),
            }
        }
    }

    #[test]
    fn test_apply_diff_to_file() {
        let _blob_guard = EnvGuard::set("MEMORY_QUEUE_USE_BLOB", "0");
        let temp = TempDir::new().expect("tempdir");
        let memory_dir = temp.path().join("memory");
        fs::create_dir_all(&memory_dir).expect("create dir");

        // Write initial memo
        let initial = r#"# Memo

## Contacts
- Alice: 555-0000

## Preferences
"#;
        fs::write(memory_dir.join("memo.md"), initial).expect("write initial");

        // Create diff
        let diff = MemoryDiff {
            changed_sections: HashMap::from([(
                "Contacts".to_string(),
                SectionChange::Added(vec!["- Bob: 555-1234".to_string()]),
            )]),
        };

        let request = MemoryWriteRequest {
            account_id: None,
            user_id: "test-user".to_string(),
            user_memory_dir: memory_dir.clone(),
            diff,
        };

        apply_diff_to_file(&request).expect("apply diff");

        let result = fs::read_to_string(memory_dir.join("memo.md")).expect("read result");
        assert!(result.contains("Alice"));
        assert!(result.contains("Bob"));
    }

    #[test]
    fn test_queue_sequential_writes() {
        let _blob_guard = EnvGuard::set("MEMORY_QUEUE_USE_BLOB", "0");
        let temp = TempDir::new().expect("tempdir");
        let memory_dir = temp.path().join("memory");
        fs::create_dir_all(&memory_dir).expect("create dir");

        let initial = r#"# Memo

## Contacts

## Preferences
"#;
        fs::write(memory_dir.join("memo.md"), initial).expect("write initial");

        let queue = MemoryWriteQueue::new();

        // Submit two diffs sequentially (both go through the queue)
        let diff1 = MemoryDiff {
            changed_sections: HashMap::from([(
                "Contacts".to_string(),
                SectionChange::Added(vec!["- Alice: 555-0000".to_string()]),
            )]),
        };

        let diff2 = MemoryDiff {
            changed_sections: HashMap::from([(
                "Contacts".to_string(),
                SectionChange::Added(vec!["- Bob: 555-1234".to_string()]),
            )]),
        };

        queue
            .submit(MemoryWriteRequest {
                account_id: None,
                user_id: "test-user".to_string(),
                user_memory_dir: memory_dir.clone(),
                diff: diff1,
            })
            .expect("submit 1");

        queue
            .submit(MemoryWriteRequest {
                account_id: None,
                user_id: "test-user".to_string(),
                user_memory_dir: memory_dir.clone(),
                diff: diff2,
            })
            .expect("submit 2");

        let result = fs::read_to_string(memory_dir.join("memo.md")).expect("read result");
        assert!(result.contains("Alice"), "Should contain Alice");
        assert!(result.contains("Bob"), "Should contain Bob");
    }

    #[test]
    fn test_concurrent_submits_serialized() {
        let _blob_guard = EnvGuard::set("MEMORY_QUEUE_USE_BLOB", "0");
        use std::sync::Arc;
        use std::thread;

        let temp = TempDir::new().expect("tempdir");
        let memory_dir = temp.path().join("memory");
        fs::create_dir_all(&memory_dir).expect("create dir");

        let initial = r#"# Memo

## Contacts

## Preferences
"#;
        fs::write(memory_dir.join("memo.md"), initial).expect("write initial");

        let queue = Arc::new(MemoryWriteQueue::new());
        let memory_dir = Arc::new(memory_dir);

        // Spawn multiple threads submitting diffs concurrently
        let handles: Vec<_> = (0..5)
            .map(|i| {
                let queue = Arc::clone(&queue);
                let memory_dir = Arc::clone(&memory_dir);
                thread::spawn(move || {
                    let diff = MemoryDiff {
                        changed_sections: HashMap::from([(
                            "Contacts".to_string(),
                            SectionChange::Added(vec![format!("- Contact{}: 555-{:04}", i, i)]),
                        )]),
                    };
                    queue
                        .submit(MemoryWriteRequest {
                            account_id: None,
                            user_id: "test-user".to_string(),
                            user_memory_dir: memory_dir.as_ref().clone(),
                            diff,
                        })
                        .expect("submit");
                })
            })
            .collect();

        for handle in handles {
            handle.join().expect("thread join");
        }

        let result = fs::read_to_string(memory_dir.join("memo.md")).expect("read result");
        // All 5 contacts should be present (no lost updates)
        for i in 0..5 {
            assert!(
                result.contains(&format!("Contact{}", i)),
                "Should contain Contact{}",
                i
            );
        }
    }
}
