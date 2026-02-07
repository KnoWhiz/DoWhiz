use chrono::{DateTime, Utc};
use rusqlite::{params, Connection};
use std::fs;
use std::path::PathBuf;

use crate::{Schedule, ScheduledTask};

#[derive(Debug)]
pub struct IndexStore {
    path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct TaskRef {
    pub task_id: String,
    pub user_id: String,
}

#[derive(Debug, thiserror::Error)]
pub enum IndexStoreError {
    #[error("sqlite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

impl IndexStore {
    pub fn new(path: impl Into<PathBuf>) -> Result<Self, IndexStoreError> {
        let store = Self { path: path.into() };
        let _ = store.open()?;
        Ok(store)
    }

    pub fn sync_user_tasks(
        &self,
        user_id: &str,
        tasks: &[ScheduledTask],
    ) -> Result<(), IndexStoreError> {
        let mut conn = self.open()?;
        let tx = conn.transaction()?;
        tx.execute(
            "DELETE FROM task_index WHERE user_id = ?1",
            params![user_id],
        )?;
        {
            let mut stmt = tx.prepare(
                "INSERT INTO task_index (task_id, user_id, next_run, enabled)
                 VALUES (?1, ?2, ?3, ?4)",
            )?;
            for task in tasks {
                if !task.enabled {
                    continue;
                }
                let next_run = match &task.schedule {
                    Schedule::Cron { next_run, .. } => next_run,
                    Schedule::OneShot { run_at } => run_at,
                };
                stmt.execute(params![
                    task.id.to_string(),
                    user_id,
                    format_datetime(next_run),
                    1i64
                ])?;
            }
        }
        tx.commit()?;
        Ok(())
    }

    pub fn due_user_ids(
        &self,
        now: DateTime<Utc>,
        limit: usize,
    ) -> Result<Vec<String>, IndexStoreError> {
        let conn = self.open()?;
        let mut stmt = conn.prepare(
            "SELECT DISTINCT user_id
             FROM task_index
             WHERE enabled = 1 AND next_run <= ?1
             ORDER BY next_run
             LIMIT ?2",
        )?;
        let rows = stmt.query_map(params![format_datetime(&now), limit as i64], |row| {
            row.get::<_, String>(0)
        })?;
        let mut user_ids = Vec::new();
        for row in rows {
            user_ids.push(row?);
        }
        Ok(user_ids)
    }

    pub fn due_task_refs(
        &self,
        now: DateTime<Utc>,
        limit: usize,
    ) -> Result<Vec<TaskRef>, IndexStoreError> {
        let conn = self.open()?;
        let mut stmt = conn.prepare(
            "SELECT task_id, user_id
             FROM task_index
             WHERE enabled = 1 AND next_run <= ?1
             ORDER BY next_run
             LIMIT ?2",
        )?;
        let rows = stmt.query_map(params![format_datetime(&now), limit as i64], |row| {
            Ok(TaskRef {
                task_id: row.get::<_, String>(0)?,
                user_id: row.get::<_, String>(1)?,
            })
        })?;
        let mut task_refs = Vec::new();
        for row in rows {
            task_refs.push(row?);
        }
        Ok(task_refs)
    }

    fn open(&self) -> Result<Connection, IndexStoreError> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }
        let conn = Connection::open(&self.path)?;
        conn.execute_batch(INDEX_SCHEMA)?;
        Ok(conn)
    }
}

const INDEX_SCHEMA: &str = r#"
CREATE TABLE IF NOT EXISTS task_index (
    task_id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    next_run TEXT NOT NULL,
    enabled INTEGER NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_task_index_next_run ON task_index (next_run);
CREATE INDEX IF NOT EXISTS idx_task_index_user_id ON task_index (user_id);
"#;

fn format_datetime(value: &DateTime<Utc>) -> String {
    value.to_rfc3339()
}

#[cfg(test)]
mod tests;
