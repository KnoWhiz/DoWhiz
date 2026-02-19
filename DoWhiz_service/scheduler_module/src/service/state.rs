use std::collections::HashMap;
use std::sync::Mutex;

use chrono::{DateTime, Utc};

use crate::index_store::TaskRef;
use crate::slack_store::SlackStore;

use super::config::ServiceConfig;

#[derive(Clone)]
pub(super) struct AppState {
    pub(super) config: std::sync::Arc<ServiceConfig>,
    pub(super) slack_store: std::sync::Arc<SlackStore>,
}

/// Information about a running task for monitoring
#[derive(Clone, Debug)]
pub(super) struct TaskClaim {
    pub(super) task_id: String,
    pub(super) user_id: String,
    pub(super) started_at: DateTime<Utc>,
    pub(super) thread_id: Option<String>,
    pub(super) retry_count: u32,
}

#[derive(Default)]
pub(super) struct SchedulerClaims {
    pub(super) running_tasks: HashMap<String, TaskClaim>,
    pub(super) running_users: HashMap<String, usize>,
}

pub(super) enum ClaimResult {
    Claimed,
    UserBusy,
    TaskBusy,
}

impl SchedulerClaims {
    pub(super) fn try_claim(
        &mut self,
        task_ref: &TaskRef,
        user_limit: usize,
        retry_count: u32,
    ) -> ClaimResult {
        let active = self
            .running_users
            .get(&task_ref.user_id)
            .copied()
            .unwrap_or(0);
        if active >= user_limit {
            return ClaimResult::UserBusy;
        }
        if self.running_tasks.contains_key(&task_ref.task_id) {
            return ClaimResult::TaskBusy;
        }
        self.running_users
            .insert(task_ref.user_id.clone(), active + 1);
        let claim = TaskClaim {
            task_id: task_ref.task_id.clone(),
            user_id: task_ref.user_id.clone(),
            started_at: Utc::now(),
            thread_id: None, // TaskRef doesn't have thread_id; it's tracked in the task itself
            retry_count,
        };
        self.running_tasks.insert(task_ref.task_id.clone(), claim);
        ClaimResult::Claimed
    }

    pub(super) fn release(&mut self, task_ref: &TaskRef) {
        if let Some(active) = self.running_users.get_mut(&task_ref.user_id) {
            if *active <= 1 {
                self.running_users.remove(&task_ref.user_id);
            } else {
                *active -= 1;
            }
        }
        self.running_tasks.remove(&task_ref.task_id);
    }

    /// Find tasks that have been running longer than the timeout
    pub(super) fn find_stale_tasks(&self, timeout_secs: u64) -> Vec<TaskClaim> {
        let now = Utc::now();
        let timeout = chrono::Duration::seconds(timeout_secs as i64);
        self.running_tasks
            .values()
            .filter(|claim| now - claim.started_at > timeout)
            .cloned()
            .collect()
    }

    /// Force release a task by task_id (used by watchdog)
    pub(super) fn force_release(&mut self, task_id: &str) -> Option<TaskClaim> {
        if let Some(claim) = self.running_tasks.remove(task_id) {
            if let Some(active) = self.running_users.get_mut(&claim.user_id) {
                if *active <= 1 {
                    self.running_users.remove(&claim.user_id);
                } else {
                    *active -= 1;
                }
            }
            Some(claim)
        } else {
            None
        }
    }
}

pub(super) struct ConcurrencyLimiter {
    max: usize,
    in_flight: Mutex<usize>,
}

impl ConcurrencyLimiter {
    pub(super) fn new(max: usize) -> Self {
        Self {
            max,
            in_flight: Mutex::new(0),
        }
    }

    pub(super) fn try_acquire(&self) -> bool {
        let mut in_flight = self
            .in_flight
            .lock()
            .expect("concurrency limiter lock poisoned");
        if *in_flight >= self.max {
            return false;
        }
        *in_flight += 1;
        true
    }

    pub(super) fn release(&self) {
        let mut in_flight = self
            .in_flight
            .lock()
            .expect("concurrency limiter lock poisoned");
        if *in_flight > 0 {
            *in_flight -= 1;
        }
    }
}
