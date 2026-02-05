use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadState {
    pub thread_id: String,
    pub epoch: u64,
    pub last_email_seq: u64,
    pub last_message_id: Option<String>,
    pub updated_at: String,
}

impl ThreadState {
    pub fn new(thread_id: String, message_id: Option<String>) -> Self {
        Self {
            thread_id,
            epoch: 1,
            last_email_seq: 1,
            last_message_id: message_id,
            updated_at: Utc::now().to_rfc3339(),
        }
    }

    pub fn bump(&mut self, message_id: Option<String>) {
        self.epoch = self.epoch.saturating_add(1);
        self.last_email_seq = self.last_email_seq.saturating_add(1);
        self.last_message_id = message_id;
        self.updated_at = Utc::now().to_rfc3339();
    }
}

pub fn load_thread_state(path: &Path) -> Option<ThreadState> {
    let raw = fs::read_to_string(path).ok()?;
    serde_json::from_str(&raw).ok()
}

pub fn write_thread_state(path: &Path, state: &ThreadState) -> Result<(), io::Error> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let serialized = serde_json::to_string_pretty(state)
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
    fs::write(path, serialized)?;
    Ok(())
}

pub fn bump_thread_state(
    path: &Path,
    thread_id: &str,
    message_id: Option<String>,
) -> Result<ThreadState, io::Error> {
    let mut state = load_thread_state(path)
        .unwrap_or_else(|| ThreadState::new(thread_id.to_string(), message_id.clone()));
    if state.thread_id != thread_id {
        state = ThreadState::new(thread_id.to_string(), message_id.clone());
    } else if state.epoch == 0 || state.last_email_seq == 0 {
        state.epoch = 1;
        state.last_email_seq = 1;
        state.last_message_id = message_id.clone();
        state.updated_at = Utc::now().to_rfc3339();
    } else {
        state.bump(message_id.clone());
    }
    write_thread_state(path, &state)?;
    Ok(state)
}

pub fn current_thread_epoch(path: &Path) -> Option<u64> {
    load_thread_state(path).map(|state| state.epoch)
}

pub fn default_thread_state_path(workspace_dir: &Path) -> PathBuf {
    workspace_dir.join("thread_state.json")
}

pub fn find_thread_state_path(start: &Path) -> Option<PathBuf> {
    let mut current = Some(start);
    while let Some(path) = current {
        let candidate = path.join("thread_state.json");
        if candidate.exists() {
            return Some(candidate);
        }
        current = path.parent();
    }
    None
}
