use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;

use crossbeam_channel::{bounded, Sender};
use tracing::info;

use crate::memory_diff::{apply_memory_diff, MemoryDiff};

/// A queued memory write operation
#[derive(Debug, Clone)]
pub struct MemoryWriteRequest {
    pub user_id: String,
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
    /// Per-user channels for submitting diffs
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
        let user_id = request.user_id.clone();

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

            if let Some(sender) = channels.get(&user_id) {
                sender.clone()
            } else {
                // Create a new channel and worker for this user
                let (sender, receiver) = bounded::<InternalRequest>(100);

                // Spawn worker thread for this user
                let worker_user_id = user_id.clone();
                thread::spawn(move || {
                    info!("Memory queue worker started for user {}", worker_user_id);
                    for req in receiver {
                        let result = apply_diff_to_file(&req.request);
                        // Signal completion (ignore send error if receiver dropped)
                        let _ = req.done.send(result);
                    }
                    info!("Memory queue worker stopped for user {}", worker_user_id);
                });

                channels.insert(user_id.clone(), sender.clone());
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

/// Apply a diff to the user's memo.md file
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

#[derive(Debug, Clone, thiserror::Error)]
pub enum MemoryQueueError {
    #[error("IO error: {0}")]
    Io(#[source] Arc<std::io::Error>),
    #[error("Queue lock poisoned")]
    LockPoisoned,
    #[error("Channel closed")]
    ChannelClosed,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory_diff::SectionChange;
    use std::collections::HashMap;
    use tempfile::TempDir;

    #[test]
    fn test_apply_diff_to_file() {
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
                user_id: "test-user".to_string(),
                user_memory_dir: memory_dir.clone(),
                diff: diff1,
            })
            .expect("submit 1");

        queue
            .submit(MemoryWriteRequest {
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
