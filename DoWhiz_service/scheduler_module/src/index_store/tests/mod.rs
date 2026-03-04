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
    let user_a = format!("user_a_{}", Uuid::new_v4());
    let user_b = format!("user_b_{}", Uuid::new_v4());

    store.sync_user_tasks(&user_a, &[due_task.clone()]).unwrap();
    store
        .sync_user_tasks(&user_b, &[future_task.clone()])
        .unwrap();

    let due_users = store.due_user_ids(now, 10_000).unwrap();
    assert!(due_users.contains(&user_a));
    assert!(!due_users.contains(&user_b));
}

#[test]
fn sync_user_tasks_dedupes_repeated_task_ids() {
    let temp = TempDir::new().unwrap();
    let db_path = temp.path().join("task_index.db");
    let store = IndexStore::new(db_path).unwrap();

    let now = Utc::now();
    let task_id = Uuid::new_v4();
    let first_due = now + Duration::minutes(5);
    let latest_due = now + Duration::minutes(1);

    let mut first = ScheduledTask {
        id: task_id,
        kind: TaskKind::Noop,
        schedule: Schedule::OneShot { run_at: first_due },
        enabled: true,
        created_at: now,
        last_run: None,
    };
    let second = ScheduledTask {
        id: task_id,
        kind: TaskKind::Noop,
        schedule: Schedule::OneShot { run_at: latest_due },
        enabled: true,
        created_at: now,
        last_run: None,
    };

    let user_id = format!("user_a_{}", Uuid::new_v4());

    store
        .sync_user_tasks(&user_id, &[first.clone(), second.clone()])
        .unwrap();
    let refs = store
        .due_task_refs(now + Duration::minutes(2), 10_000)
        .unwrap();
    let matching: Vec<_> = refs
        .into_iter()
        .filter(|task_ref| task_ref.user_id == user_id)
        .collect();
    assert_eq!(matching.len(), 1);
    assert_eq!(matching[0].task_id, task_id.to_string());

    // Switching the first copy to disabled should not remove the still-enabled duplicate.
    first.enabled = false;
    store.sync_user_tasks(&user_id, &[first, second]).unwrap();
    let refs = store
        .due_task_refs(now + Duration::minutes(2), 10_000)
        .unwrap();
    let matching: Vec<_> = refs
        .into_iter()
        .filter(|task_ref| task_ref.user_id == user_id)
        .collect();
    assert_eq!(matching.len(), 1);
}
