use postgres_native_tls::MakeTlsConnector;
use r2d2::{Pool, PooledConnection};
use r2d2_postgres::PostgresConnectionManager;
use std::env;
use tracing::error;
use uuid::Uuid;

use crate::ingestion::IngestionEnvelope;

/// Custom error handler that logs the actual connection error
#[derive(Debug)]
struct LoggingErrorHandler;

impl r2d2::HandleError<postgres::Error> for LoggingErrorHandler {
    fn handle_error(&self, err: postgres::Error) {
        error!("postgres connection pool error: {:?}", err);
    }
}

#[derive(Debug, thiserror::Error)]
pub enum IngestionQueueError {
    #[error("postgres error: {0}")]
    Postgres(#[from] postgres::Error),
    #[error("pool error: {0}")]
    Pool(#[from] r2d2::Error),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("uuid error: {0}")]
    Uuid(#[from] uuid::Error),
    #[error("missing SUPABASE_DB_URL/INGESTION_DB_URL/DATABASE_URL")]
    MissingDbUrl,
    #[error("invalid ingestion queue table name: {0}")]
    InvalidTableName(String),
    #[error("ingestion queue config error: {0}")]
    Config(String),
}

#[derive(Debug, Clone)]
pub struct EnqueueResult {
    pub inserted: bool,
}

#[derive(Debug, Clone)]
pub struct QueuedEnvelope {
    pub id: Uuid,
    pub envelope: IngestionEnvelope,
}

pub trait IngestionQueue: Send + Sync {
    fn enqueue(&self, envelope: &IngestionEnvelope) -> Result<EnqueueResult, IngestionQueueError>;
    fn claim_next(&self, employee_id: &str) -> Result<Option<QueuedEnvelope>, IngestionQueueError>;
    fn mark_done(&self, id: &Uuid) -> Result<(), IngestionQueueError>;
    fn mark_failed(&self, id: &Uuid, error: &str) -> Result<(), IngestionQueueError>;
}

#[derive(Clone)]
pub struct PostgresIngestionQueue {
    pool: Option<Pool<PostgresConnectionManager<MakeTlsConnector>>>,
    table: String,
    lease_secs: i64,
    max_attempts: i32,
}

impl PostgresIngestionQueue {
    pub fn from_env() -> Result<Self, IngestionQueueError> {
        let db_url = resolve_db_url()?;
        let table = resolve_table_name()?;
        let lease_secs = resolve_i64_env("INGESTION_QUEUE_LEASE_SECS", 60);
        let max_attempts = resolve_i32_env("INGESTION_QUEUE_MAX_ATTEMPTS", 5);
        Self::new(&db_url, &table, lease_secs, max_attempts)
    }

    pub fn new_from_url(db_url: &str) -> Result<Self, IngestionQueueError> {
        let table = resolve_table_name()?;
        let lease_secs = resolve_i64_env("INGESTION_QUEUE_LEASE_SECS", 60);
        let max_attempts = resolve_i32_env("INGESTION_QUEUE_MAX_ATTEMPTS", 5);
        Self::new(db_url, &table, lease_secs, max_attempts)
    }

    pub fn new(
        db_url: &str,
        table: &str,
        lease_secs: i64,
        max_attempts: i32,
    ) -> Result<Self, IngestionQueueError> {
        let table = sanitize_table_name(table)?;

        let config: postgres::Config = db_url.parse().map_err(IngestionQueueError::Postgres)?;
        let mut tls_builder = native_tls::TlsConnector::builder();
        if resolve_bool_env("INGESTION_QUEUE_TLS_ALLOW_INVALID_CERTS") {
            tls_builder.danger_accept_invalid_certs(true);
            tls_builder.danger_accept_invalid_hostnames(true);
        }
        let tls_connector = tls_builder
            .build()
            .map_err(|err| IngestionQueueError::Config(err.to_string()))?;
        let tls = MakeTlsConnector::new(tls_connector);

        let manager = PostgresConnectionManager::new(config, tls);
        let pool = Pool::builder()
            .max_size(4)
            .idle_timeout(Some(std::time::Duration::from_secs(300))) // Close idle connections after 5 min
            .error_handler(Box::new(LoggingErrorHandler))
            .build(manager)?;
        let queue = Self {
            pool: Some(pool),
            table,
            lease_secs,
            max_attempts,
        };
        queue.ensure_schema()?;
        Ok(queue)
    }

    fn connection(
        &self,
    ) -> Result<PooledConnection<PostgresConnectionManager<MakeTlsConnector>>, IngestionQueueError>
    {
        let pool = self
            .pool
            .as_ref()
            .expect("ingestion queue pool unavailable");
        Ok(pool.get()?)
    }

    fn ensure_schema(&self) -> Result<(), IngestionQueueError> {
        let mut conn = self.connection()?;
        let statement = format!(
            "CREATE TABLE IF NOT EXISTS {table} (
                id UUID PRIMARY KEY,
                tenant_id TEXT,
                employee_id TEXT NOT NULL,
                channel TEXT NOT NULL,
                external_message_id TEXT,
                dedupe_key TEXT NOT NULL UNIQUE,
                payload_json TEXT NOT NULL,
                status TEXT NOT NULL,
                created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                locked_at TIMESTAMPTZ,
                locked_by TEXT,
                processed_at TIMESTAMPTZ,
                attempts INTEGER NOT NULL DEFAULT 0,
                last_error TEXT,
                available_at TIMESTAMPTZ
            );
            CREATE INDEX IF NOT EXISTS {table}_pending_idx
                ON {table}(employee_id, status, created_at);
            CREATE INDEX IF NOT EXISTS {table}_available_idx
                ON {table}(status, available_at);",
            table = self.table
        );
        conn.batch_execute(&statement)?;
        Ok(())
    }

    #[cfg(test)]
    fn drop_table_for_tests(&self) {
        if let Ok(mut conn) = self.connection() {
            let _ = conn.execute(&format!("DROP TABLE IF EXISTS {}", self.table), &[]);
        }
    }
}

impl IngestionQueue for PostgresIngestionQueue {
    fn enqueue(&self, envelope: &IngestionEnvelope) -> Result<EnqueueResult, IngestionQueueError> {
        let mut conn = self.connection()?;
        let payload_json = serde_json::to_string(envelope)?;
        let row = conn.execute(
            &format!(
                "INSERT INTO {table}
                    (id, tenant_id, employee_id, channel, external_message_id, dedupe_key, payload_json, status, created_at, attempts)
                 VALUES ($1, $2, $3, $4, $5, $6, $7, 'pending', now(), 0)
                 ON CONFLICT (dedupe_key) DO NOTHING",
                table = self.table
            ),
            &[
                &envelope.envelope_id,
                &envelope.tenant_id,
                &envelope.employee_id,
                &envelope.channel.to_string(),
                &envelope.external_message_id,
                &envelope.dedupe_key,
                &payload_json,
            ],
        )?;

        Ok(EnqueueResult { inserted: row > 0 })
    }

    fn claim_next(&self, employee_id: &str) -> Result<Option<QueuedEnvelope>, IngestionQueueError> {
        let mut conn = self.connection()?;
        let instance_id = resolve_worker_instance_id(employee_id);
        let lease_secs = self.lease_secs;

        let mut tx = conn.transaction()?;
        let row = tx.query_opt(
            &format!(
                "SELECT id, payload_json
                 FROM {table}
                 WHERE employee_id = $1
                   AND (
                     status = 'pending'
                     OR (status = 'processing' AND locked_at < now() - ($2::bigint * interval '1 second'))
                   )
                   AND (available_at IS NULL OR available_at <= now())
                   AND attempts < $3
                 ORDER BY created_at
                 LIMIT 1
                 FOR UPDATE SKIP LOCKED",
                table = self.table
            ),
            &[&employee_id, &lease_secs, &self.max_attempts],
        )?;

        let Some(row) = row else {
            tx.commit()?;
            return Ok(None);
        };

        let id: Uuid = row.get(0);
        let payload_json: String = row.get(1);

        let updated = tx.execute(
            &format!(
                "UPDATE {table}
                 SET status = 'processing',
                     locked_at = now(),
                     locked_by = $2,
                     attempts = attempts + 1
                 WHERE id = $1",
                table = self.table
            ),
            &[&id, &instance_id],
        )?;

        if updated == 0 {
            tx.commit()?;
            return Ok(None);
        }

        tx.commit()?;

        let envelope: IngestionEnvelope = serde_json::from_str(&payload_json)?;
        Ok(Some(QueuedEnvelope { id, envelope }))
    }

    fn mark_done(&self, id: &Uuid) -> Result<(), IngestionQueueError> {
        let mut conn = self.connection()?;
        conn.execute(
            &format!(
                "UPDATE {table}
                 SET status = 'done',
                     processed_at = now(),
                     locked_at = NULL,
                     locked_by = NULL
                 WHERE id = $1",
                table = self.table
            ),
            &[id],
        )?;
        Ok(())
    }

    fn mark_failed(&self, id: &Uuid, error: &str) -> Result<(), IngestionQueueError> {
        let mut conn = self.connection()?;
        let attempts: i32 = conn
            .query_one(
                &format!(
                    "SELECT attempts FROM {table} WHERE id = $1",
                    table = self.table
                ),
                &[id],
            )?
            .get(0);

        let (status, available_at) = if attempts >= self.max_attempts {
            ("failed", None)
        } else {
            let backoff_secs = i64::from(attempts.max(1)).saturating_mul(5);
            ("pending", Some(backoff_secs))
        };

        if let Some(backoff_secs) = available_at {
            conn.execute(
                &format!(
                    "UPDATE {table}
                     SET status = $2,
                         processed_at = now(),
                         locked_at = NULL,
                         locked_by = NULL,
                         available_at = now() + ($3::bigint * interval '1 second'),
                         last_error = $4
                     WHERE id = $1",
                    table = self.table
                ),
                &[id, &status, &backoff_secs, &error],
            )?;
        } else {
            conn.execute(
                &format!(
                    "UPDATE {table}
                     SET status = $2,
                         processed_at = now(),
                         locked_at = NULL,
                         locked_by = NULL,
                         available_at = NULL,
                         last_error = $3
                     WHERE id = $1",
                    table = self.table
                ),
                &[id, &status, &error],
            )?;
        }
        Ok(())
    }
}

impl Drop for PostgresIngestionQueue {
    fn drop(&mut self) {
        if let Some(pool) = self.pool.take() {
            std::thread::spawn(move || drop(pool));
        }
    }
}

fn resolve_db_url() -> Result<String, IngestionQueueError> {
    env::var("INGESTION_DB_URL")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .or_else(|| {
            env::var("SUPABASE_DB_URL")
                .ok()
                .filter(|value| !value.trim().is_empty())
        })
        .or_else(|| {
            env::var("DATABASE_URL")
                .ok()
                .filter(|value| !value.trim().is_empty())
        })
        .ok_or(IngestionQueueError::MissingDbUrl)
}

fn resolve_table_name() -> Result<String, IngestionQueueError> {
    let raw = env::var("INGESTION_QUEUE_TABLE")
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "ingestion_queue".to_string());
    sanitize_table_name(&raw)
}

fn sanitize_table_name(raw: &str) -> Result<String, IngestionQueueError> {
    let parts: Vec<&str> = raw.split('.').collect();
    if parts.is_empty() || parts.len() > 2 {
        return Err(IngestionQueueError::InvalidTableName(raw.to_string()));
    }
    for part in &parts {
        if part.is_empty()
            || !part
                .chars()
                .all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
        {
            return Err(IngestionQueueError::InvalidTableName(raw.to_string()));
        }
    }
    Ok(raw.to_string())
}

fn resolve_i64_env(key: &str, default_value: i64) -> i64 {
    env::var(key)
        .ok()
        .and_then(|value| value.parse::<i64>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(default_value)
}

fn resolve_i32_env(key: &str, default_value: i32) -> i32 {
    env::var(key)
        .ok()
        .and_then(|value| value.parse::<i32>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(default_value)
}

fn resolve_bool_env(key: &str) -> bool {
    env::var(key)
        .ok()
        .map(|value| value.trim().to_ascii_lowercase())
        .map(|value| matches!(value.as_str(), "1" | "true" | "yes" | "on"))
        .unwrap_or(false)
}

fn resolve_worker_instance_id(employee_id: &str) -> String {
    if let Ok(value) = env::var("WORKER_INSTANCE_ID") {
        let trimmed = value.trim();
        if !trimmed.is_empty() {
            return trimmed.to_string();
        }
    }

    let key = format!(
        "WORKER_INSTANCE_ID_{}",
        employee_id.trim().to_ascii_uppercase()
    );
    if let Ok(value) = env::var(key) {
        let trimmed = value.trim();
        if !trimmed.is_empty() {
            return trimmed.to_string();
        }
    }

    env::var("HOSTNAME")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| format!("pid-{}", std::process::id()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::channel::{Channel, ChannelMetadata, InboundMessage};
    use crate::ingestion::IngestionPayload;
    use chrono::Utc;

    fn sample_envelope(employee_id: &str, dedupe_key: &str) -> IngestionEnvelope {
        let message = InboundMessage {
            channel: Channel::Slack,
            sender: "U123".to_string(),
            sender_name: None,
            recipient: "C123".to_string(),
            subject: None,
            text_body: Some("hello".to_string()),
            html_body: None,
            thread_id: "123.456".to_string(),
            message_id: Some("evt_1".to_string()),
            attachments: Vec::new(),
            reply_to: vec!["C123".to_string()],
            raw_payload: b"{".to_vec(),
            metadata: ChannelMetadata {
                slack_channel_id: Some("C123".to_string()),
                ..Default::default()
            },
        };

        IngestionEnvelope {
            envelope_id: Uuid::new_v4(),
            received_at: Utc::now(),
            tenant_id: Some("tenant".to_string()),
            employee_id: employee_id.to_string(),
            channel: Channel::Slack,
            external_message_id: Some("evt_1".to_string()),
            dedupe_key: dedupe_key.to_string(),
            payload: IngestionPayload::from_inbound(&message),
            raw_payload_ref: None,
        }
    }

    fn test_queue() -> PostgresIngestionQueue {
        dotenvy::dotenv().ok();
        env::set_var("INGESTION_QUEUE_TLS_ALLOW_INVALID_CERTS", "true");
        let db_url = resolve_db_url().expect("SUPABASE_DB_URL required for ingestion queue tests");
        let table = format!("ingestion_queue_test_{}", Uuid::new_v4().simple());
        PostgresIngestionQueue::new(&db_url, &table, 10, 5).expect("queue")
    }

    #[test]
    fn enqueue_and_claim_roundtrip() {
        let queue = test_queue();
        let envelope = sample_envelope("emp", "dedupe-1");
        let result = queue.enqueue(&envelope).expect("enqueue");
        assert!(result.inserted);

        let claimed = queue.claim_next("emp").expect("claim");
        assert!(claimed.is_some());
        let claimed = claimed.unwrap();
        assert_eq!(claimed.envelope.dedupe_key, "dedupe-1");

        queue.mark_done(&claimed.id).expect("done");
        queue.drop_table_for_tests();
    }

    #[test]
    fn enqueue_dedupe_prevents_duplicates() {
        let queue = test_queue();
        let envelope = sample_envelope("emp", "dedupe-2");
        let first = queue.enqueue(&envelope).expect("enqueue");
        let second = queue.enqueue(&envelope).expect("enqueue");
        assert!(first.inserted);
        assert!(!second.inserted);
        queue.drop_table_for_tests();
    }
}
