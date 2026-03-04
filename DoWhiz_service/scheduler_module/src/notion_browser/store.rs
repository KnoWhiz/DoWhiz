//! MongoDB storage for tracking processed Notion notifications.

use chrono::{DateTime, Utc};
use mongodb::bson::{doc, Document};
use mongodb::options::{IndexOptions, UpdateOptions};
use mongodb::sync::Collection;
use mongodb::IndexModel;
use tracing::{debug, info};

use super::NotionError;

/// MongoDB store for tracking processed Notion notifications.
///
/// This prevents duplicate processing of notifications across restarts
/// and ensures each @mention is only handled once.
pub struct MongoNotionProcessedStore {
    collection: Collection<Document>,
}

impl MongoNotionProcessedStore {
    /// Create a new processed store using the given MongoDB collection.
    pub fn new(collection: Collection<Document>) -> Self {
        Self { collection }
    }

    /// Create a store from environment configuration.
    ///
    /// Uses `MONGODB_URI` and creates the `notion_processed_notifications` collection
    /// in the employee-specific database.
    pub fn from_env(employee_id: &str) -> Result<Self, NotionError> {
        use crate::mongo_store::{create_client_from_env, database_from_env};

        let client = create_client_from_env()
            .map_err(|e| NotionError::ConfigError(format!("MongoDB config error: {}", e)))?;

        let db = database_from_env(&client);
        let collection = db.collection("notion_processed_notifications");

        let store = Self::new(collection);
        store.ensure_indexes()?;

        info!(
            "Initialized MongoNotionProcessedStore for employee {}",
            employee_id
        );

        Ok(store)
    }

    /// Ensure required indexes exist on the collection.
    fn ensure_indexes(&self) -> Result<(), NotionError> {
        use crate::mongo_store::ensure_index_compatible;

        // Unique index on notification_id
        let notification_index = IndexModel::builder()
            .keys(doc! { "notification_id": 1 })
            .options(IndexOptions::builder().unique(true).build())
            .build();

        // Index on processed_at for cleanup queries
        let timestamp_index = IndexModel::builder()
            .keys(doc! { "processed_at": -1 })
            .build();

        // Index on workspace_id for workspace-specific queries
        let workspace_index = IndexModel::builder()
            .keys(doc! { "workspace_id": 1, "processed_at": -1 })
            .build();

        ensure_index_compatible(&self.collection, notification_index)
            .map_err(|e| NotionError::StorageError(e.to_string()))?;

        ensure_index_compatible(&self.collection, timestamp_index)
            .map_err(|e| NotionError::StorageError(e.to_string()))?;

        ensure_index_compatible(&self.collection, workspace_index)
            .map_err(|e| NotionError::StorageError(e.to_string()))?;

        Ok(())
    }

    /// Check if a notification has already been processed.
    pub fn is_processed(&self, notification_id: &str) -> Result<bool, NotionError> {
        let filter = doc! { "notification_id": notification_id };

        let count = self
            .collection
            .count_documents(filter, None)
            .map_err(|e| NotionError::StorageError(format!("Failed to check processed: {}", e)))?;

        Ok(count > 0)
    }

    /// Mark a notification as processed.
    pub fn mark_processed(
        &self,
        notification_id: &str,
        workspace_id: Option<&str>,
        page_id: Option<&str>,
    ) -> Result<(), NotionError> {
        let now = Utc::now();

        let document = doc! {
            "notification_id": notification_id,
            "workspace_id": workspace_id,
            "page_id": page_id,
            "processed_at": mongodb::bson::DateTime::from_chrono(now),
        };

        // Use upsert to handle duplicates gracefully
        let filter = doc! { "notification_id": notification_id };
        let update = doc! { "$setOnInsert": document };
        let options = UpdateOptions::builder().upsert(true).build();

        self.collection
            .update_one(filter, update, options)
            .map_err(|e| NotionError::StorageError(format!("Failed to mark processed: {}", e)))?;

        debug!("Marked notification {} as processed", notification_id);
        Ok(())
    }

    /// Get all notification IDs processed since a given time.
    pub fn get_processed_since(&self, since: DateTime<Utc>) -> Result<Vec<String>, NotionError> {
        let filter = doc! {
            "processed_at": { "$gte": mongodb::bson::DateTime::from_chrono(since) }
        };

        let cursor = self
            .collection
            .find(filter, None)
            .map_err(|e| NotionError::StorageError(format!("Failed to query processed: {}", e)))?;

        let mut ids = Vec::new();
        for result in cursor {
            if let Ok(doc) = result {
                if let Ok(id) = doc.get_str("notification_id") {
                    ids.push(id.to_string());
                }
            }
        }

        Ok(ids)
    }

    /// Get all processed notification IDs for a specific workspace.
    pub fn get_processed_for_workspace(&self, workspace_id: &str) -> Result<Vec<String>, NotionError> {
        let filter = doc! { "workspace_id": workspace_id };

        let cursor = self
            .collection
            .find(filter, None)
            .map_err(|e| NotionError::StorageError(format!("Failed to query processed: {}", e)))?;

        let mut ids = Vec::new();
        for result in cursor {
            if let Ok(doc) = result {
                if let Ok(id) = doc.get_str("notification_id") {
                    ids.push(id.to_string());
                }
            }
        }

        Ok(ids)
    }

    /// Clean up old processed records (older than retention days).
    pub fn cleanup_old_records(&self, retention_days: i64) -> Result<u64, NotionError> {
        let cutoff = Utc::now() - chrono::Duration::days(retention_days);
        let filter = doc! {
            "processed_at": { "$lt": mongodb::bson::DateTime::from_chrono(cutoff) }
        };

        let result = self
            .collection
            .delete_many(filter, None)
            .map_err(|e| NotionError::StorageError(format!("Failed to cleanup: {}", e)))?;

        if result.deleted_count > 0 {
            info!(
                "Cleaned up {} old Notion notification records (older than {} days)",
                result.deleted_count, retention_days
            );
        }

        Ok(result.deleted_count)
    }

    /// Get statistics about processed notifications.
    pub fn get_stats(&self) -> Result<ProcessedStats, NotionError> {
        let total = self
            .collection
            .count_documents(doc! {}, None)
            .map_err(|e| NotionError::StorageError(format!("Failed to count: {}", e)))?;

        let today_start = Utc::now()
            .date_naive()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc();

        let today = self
            .collection
            .count_documents(
                doc! {
                    "processed_at": { "$gte": mongodb::bson::DateTime::from_chrono(today_start) }
                },
                None,
            )
            .map_err(|e| NotionError::StorageError(format!("Failed to count today: {}", e)))?;

        Ok(ProcessedStats {
            total_processed: total,
            processed_today: today,
        })
    }
}

/// Statistics about processed notifications.
#[derive(Debug, Clone)]
pub struct ProcessedStats {
    pub total_processed: u64,
    pub processed_today: u64,
}
