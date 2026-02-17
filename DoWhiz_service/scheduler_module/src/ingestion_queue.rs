use chrono::Utc;
use rusqlite::{params, Connection, OptionalExtension};
use std::path::{Path, PathBuf};
use uuid::Uuid;

use crate::ingestion::IngestionEnvelope;

#[derive(Debug, thiserror::Error)]
pub enum IngestionQueueError {
    #[error("sqlite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid envelope id {0}")]
    InvalidEnvelopeId(String),
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

#[derive(Debug, Clone)]
pub struct IngestionQueue {
    path: PathBuf,
}

impl IngestionQueue {
    pub fn new(path: impl Into<PathBuf>) -> Result<Self, IngestionQueueError> {
        let queue = Self { path: path.into() };
        queue.ensure_schema()?;
        Ok(queue)
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn enqueue(&self, envelope: &IngestionEnvelope) -> Result<EnqueueResult, IngestionQueueError> {
        let conn = self.open()?;
        let payload_json = serde_json::to_string(envelope)?;
        let now = Utc::now().to_rfc3339();
        let inserted = conn.execute(
            "INSERT OR IGNORE INTO ingestion_queue\n                (id, tenant_id, employee_id, channel, external_message_id, dedupe_key, payload_json, status, created_at, attempts)\n             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 'pending', ?8, 0)",
            params![
                envelope.envelope_id.to_string(),
                envelope.tenant_id.as_deref(),
                envelope.employee_id.as_str(),
                envelope.channel.to_string(),
                envelope.external_message_id.as_deref(),
                envelope.dedupe_key.as_str(),
                payload_json,
                now,
            ],
        )?;

        Ok(EnqueueResult {
            inserted: inserted > 0,
        })
    }

    pub fn claim_next(
        &self,
        employee_id: &str,
    ) -> Result<Option<QueuedEnvelope>, IngestionQueueError> {
        let mut conn = self.open()?;
        let tx = conn.transaction()?;
        let row = tx
            .query_row(
                "SELECT id, payload_json\n                 FROM ingestion_queue\n                 WHERE status = 'pending' AND employee_id = ?1\n                 ORDER BY created_at\n                 LIMIT 1",
                params![employee_id],
                |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
            )
            .optional()?;

        let Some((id_raw, payload_json)) = row else {
            return Ok(None);
        };

        let now = Utc::now().to_rfc3339();
        let updated = tx.execute(
            "UPDATE ingestion_queue\n             SET status = 'processing', locked_at = ?1, attempts = attempts + 1\n             WHERE id = ?2 AND status = 'pending'",
            params![now, id_raw.as_str()],
        )?;

        if updated == 0 {
            tx.commit()?;
            return Ok(None);
        }

        let envelope: IngestionEnvelope = serde_json::from_str(&payload_json)?;
        tx.commit()?;
        let id =
            Uuid::parse_str(&id_raw).map_err(|_| IngestionQueueError::InvalidEnvelopeId(id_raw))?;
        Ok(Some(QueuedEnvelope { id, envelope }))
    }

    pub fn mark_done(&self, id: &Uuid) -> Result<(), IngestionQueueError> {
        let conn = self.open()?;
        let now = Utc::now().to_rfc3339();
        conn.execute(
            "UPDATE ingestion_queue\n             SET status = 'done', processed_at = ?1\n             WHERE id = ?2",
            params![now, id.to_string()],
        )?;
        Ok(())
    }

    pub fn mark_failed(&self, id: &Uuid, error: &str) -> Result<(), IngestionQueueError> {
        let conn = self.open()?;
        let now = Utc::now().to_rfc3339();
        conn.execute(
            "UPDATE ingestion_queue\n             SET status = 'failed', processed_at = ?1, last_error = ?2\n             WHERE id = ?3",
            params![now, error, id.to_string()],
        )?;
        Ok(())
    }

    fn open(&self) -> Result<Connection, IngestionQueueError> {
        let conn = Connection::open(&self.path)?;
        Ok(conn)
    }

    fn ensure_schema(&self) -> Result<(), IngestionQueueError> {
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let conn = self.open()?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS ingestion_queue (\n                id TEXT PRIMARY KEY,\n                tenant_id TEXT,\n                employee_id TEXT NOT NULL,\n                channel TEXT NOT NULL,\n                external_message_id TEXT,\n                dedupe_key TEXT NOT NULL UNIQUE,\n                payload_json TEXT NOT NULL,\n                status TEXT NOT NULL,\n                created_at TEXT NOT NULL,\n                locked_at TEXT,\n                processed_at TEXT,\n                attempts INTEGER NOT NULL DEFAULT 0,\n                last_error TEXT\n            )",
            [],
        )?;
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_ingestion_queue_pending\n             ON ingestion_queue(employee_id, status, created_at)",
            [],
        )?;
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_ingestion_queue_dedupe\n             ON ingestion_queue(dedupe_key)",
            [],
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::channel::{Channel, ChannelMetadata, InboundMessage};
    use crate::ingestion::{encode_raw_payload, IngestionPayload};
    use tempfile::TempDir;

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
            raw_payload_b64: encode_raw_payload(&message.raw_payload),
        }
    }

    #[test]
    fn enqueue_and_claim_roundtrip() {
        let temp = TempDir::new().expect("tempdir");
        let db_path = temp.path().join("ingest.db");
        let queue = IngestionQueue::new(db_path).expect("queue");

        let envelope = sample_envelope("emp", "dedupe-1");
        let result = queue.enqueue(&envelope).expect("enqueue");
        assert!(result.inserted);

        let claimed = queue.claim_next("emp").expect("claim");
        assert!(claimed.is_some());
        let claimed = claimed.unwrap();
        assert_eq!(claimed.envelope.dedupe_key, "dedupe-1");

        queue.mark_done(&claimed.id).expect("done");
    }

    #[test]
    fn enqueue_dedupe_prevents_duplicates() {
        let temp = TempDir::new().expect("tempdir");
        let db_path = temp.path().join("ingest.db");
        let queue = IngestionQueue::new(db_path).expect("queue");

        let envelope = sample_envelope("emp", "dedupe-2");
        let first = queue.enqueue(&envelope).expect("enqueue");
        let second = queue.enqueue(&envelope).expect("enqueue");
        assert!(first.inserted);
        assert!(!second.inserted);
    }
}
