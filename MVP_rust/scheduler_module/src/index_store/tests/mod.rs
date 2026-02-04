use super::IndexStore;
use crate::{Schedule, ScheduledTask, TaskKind};
use chrono::{Duration, Utc};
use tempfile::TempDir;
use uuid::Uuid;

#[test]
fn sync_user_tasks_and_query_due_users() {
    let temp = TempDir::new().unwrap();
    let db_path = temp.path().join("task_index.db");
    let store = IndexStore::new(db_path).unwrap();

    let now = Utc::now();
    let past = now - Duration::minutes(5);
    let future = now + Duration::minutes(10);

    let due_task = ScheduledTask {
        id: Uuid::new_v4(),
        kind: TaskKind::Noop,
        schedule: Schedule::OneShot { run_at: past },
        enabled: true,
        created_at: now,
        last_run: None,
    };
    let future_task = ScheduledTask {
        id: Uuid::new_v4(),
        kind: TaskKind::Noop,
        schedule: Schedule::OneShot { run_at: future },
        enabled: true,
        created_at: now,
        last_run: None,
    };

    store
        .sync_user_tasks("user_a", &[due_task.clone()])
        .unwrap();
    store
        .sync_user_tasks("user_b", &[future_task.clone()])
        .unwrap();

    let due_users = store.due_user_ids(now, 10).unwrap();
    assert_eq!(due_users, vec!["user_a".to_string()]);
}
