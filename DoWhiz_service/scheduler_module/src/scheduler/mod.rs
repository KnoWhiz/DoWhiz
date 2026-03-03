mod actions;
mod core;
mod executor;
mod outbound;
mod reply;
mod schedule;
mod snapshot;
mod store;
mod types;
mod utils;

pub use core::Scheduler;
pub use executor::{ModuleExecutor, TaskExecutor};
pub use store::TaskStatusSummary;
pub use types::{
    RunTaskTask, Schedule, ScheduledTask, SchedulerError, SendReplyTask, TaskExecution, TaskKind,
};
pub use utils::load_google_access_token_from_service_env;

use std::path::Path;

/// Load task status summaries for the owner scope derived from `tasks_db_path`.
/// Returns an empty vector if the storage backend can't be reached.
pub fn load_tasks_with_status(tasks_db_path: &Path) -> Vec<TaskStatusSummary> {
    match store::SchedulerStore::new(tasks_db_path.to_path_buf()) {
        Ok(store) => store.list_tasks_with_status().unwrap_or_default(),
        Err(_) => Vec::new(),
    }
}

#[cfg(test)]
mod tests;
