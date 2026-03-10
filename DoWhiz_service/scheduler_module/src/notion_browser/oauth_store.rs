//! MongoDB storage for Notion OAuth tokens.
//!
//! Stores OAuth access tokens obtained via Notion's Public Integration OAuth flow.
//! Tokens are stored per-workspace and do not expire (Notion tokens are long-lived).

use chrono::{DateTime, Utc};
use mongodb::bson::{doc, Document};
use mongodb::options::{IndexOptions, UpdateOptions};
use mongodb::sync::Collection;
use mongodb::IndexModel;
use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};

use super::NotionError;

/// OAuth token data for a Notion workspace.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotionOAuthToken {
    /// Notion workspace ID
    pub workspace_id: String,
    /// Human-readable workspace name
    pub workspace_name: String,
    /// OAuth access token (should be encrypted at rest in production)
    pub access_token: String,
    /// Bot user ID assigned by Notion
    pub bot_id: String,
    /// User ID of the person who authorized the integration
    pub owner_user_id: String,
    /// DoWhiz employee ID this token is associated with
    pub employee_id: String,
    /// When the token was created/refreshed
    pub created_at: DateTime<Utc>,
    /// Optional: workspace icon URL
    #[serde(default)]
    pub workspace_icon: Option<String>,
}

/// MongoDB store for Notion OAuth tokens.
pub struct NotionOAuthStore {
    collection: Option<Collection<Document>>,
}

impl NotionOAuthStore {
    /// Create a new OAuth store using the given MongoDB collection.
    pub fn new(collection: Collection<Document>) -> Self {
        Self {
            collection: Some(collection),
        }
    }

    /// Create a no-op store that doesn't persist anything.
    pub fn noop() -> Self {
        warn!("Running Notion OAuth store in noop mode - tokens will not be persisted");
        Self { collection: None }
    }

    /// Create a store from environment configuration.
    pub fn from_env(employee_id: &str) -> Result<Self, NotionError> {
        use crate::mongo_store::{create_client_from_env, database_from_env};

        let client = create_client_from_env()
            .map_err(|e| NotionError::ConfigError(format!("MongoDB config error: {}", e)))?;

        let db = database_from_env(&client);
        let collection = db.collection("notion_oauth_tokens");

        let store = Self::new(collection);
        store.ensure_indexes()?;

        info!(
            "Initialized NotionOAuthStore for employee {}",
            employee_id
        );

        Ok(store)
    }

    /// Ensure required indexes exist on the collection.
    fn ensure_indexes(&self) -> Result<(), NotionError> {
        let collection = match &self.collection {
            Some(c) => c,
            None => return Ok(()),
        };

        use crate::mongo_store::ensure_index_compatible;

        // Unique index on workspace_id + employee_id
        let workspace_index = IndexModel::builder()
            .keys(doc! { "workspace_id": 1, "employee_id": 1 })
            .options(IndexOptions::builder().unique(true).build())
            .build();

        // Index on employee_id for listing all tokens for an employee
        let employee_index = IndexModel::builder()
            .keys(doc! { "employee_id": 1 })
            .build();

        ensure_index_compatible(collection, workspace_index)
            .map_err(|e| NotionError::StorageError(e.to_string()))?;

        ensure_index_compatible(collection, employee_index)
            .map_err(|e| NotionError::StorageError(e.to_string()))?;

        Ok(())
    }

    /// Get the access token for a workspace.
    pub fn get_token(&self, workspace_id: &str, employee_id: &str) -> Result<Option<String>, NotionError> {
        let collection = match &self.collection {
            Some(c) => c,
            None => return Ok(None),
        };

        let filter = doc! {
            "workspace_id": workspace_id,
            "employee_id": employee_id,
        };

        let result = collection
            .find_one(filter, None)
            .map_err(|e| NotionError::StorageError(format!("Failed to get token: {}", e)))?;

        Ok(result.and_then(|doc| doc.get_str("access_token").ok().map(|s| s.to_string())))
    }

    /// Get the full token data for a workspace.
    pub fn get_token_data(&self, workspace_id: &str, employee_id: &str) -> Result<Option<NotionOAuthToken>, NotionError> {
        let collection = match &self.collection {
            Some(c) => c,
            None => return Ok(None),
        };

        let filter = doc! {
            "workspace_id": workspace_id,
            "employee_id": employee_id,
        };

        let result = collection
            .find_one(filter, None)
            .map_err(|e| NotionError::StorageError(format!("Failed to get token: {}", e)))?;

        match result {
            Some(doc) => {
                let token: NotionOAuthToken = mongodb::bson::from_document(doc)
                    .map_err(|e| NotionError::StorageError(format!("Failed to deserialize token: {}", e)))?;
                Ok(Some(token))
            }
            None => Ok(None),
        }
    }

    /// Store or update an OAuth token.
    pub fn store_token(&self, token: &NotionOAuthToken) -> Result<(), NotionError> {
        let collection = match &self.collection {
            Some(c) => c,
            None => return Ok(()),
        };

        let doc = mongodb::bson::to_document(token)
            .map_err(|e| NotionError::StorageError(format!("Failed to serialize token: {}", e)))?;

        let filter = doc! {
            "workspace_id": &token.workspace_id,
            "employee_id": &token.employee_id,
        };

        let update = doc! { "$set": doc };
        let options = UpdateOptions::builder().upsert(true).build();

        collection
            .update_one(filter, update, options)
            .map_err(|e| NotionError::StorageError(format!("Failed to store token: {}", e)))?;

        info!(
            "Stored OAuth token for workspace {} ({})",
            token.workspace_name, token.workspace_id
        );

        Ok(())
    }

    /// Revoke/delete an OAuth token.
    pub fn revoke_token(&self, workspace_id: &str, employee_id: &str) -> Result<bool, NotionError> {
        let collection = match &self.collection {
            Some(c) => c,
            None => return Ok(false),
        };

        let filter = doc! {
            "workspace_id": workspace_id,
            "employee_id": employee_id,
        };

        let result = collection
            .delete_one(filter, None)
            .map_err(|e| NotionError::StorageError(format!("Failed to revoke token: {}", e)))?;

        if result.deleted_count > 0 {
            info!("Revoked OAuth token for workspace {}", workspace_id);
        }

        Ok(result.deleted_count > 0)
    }

    /// List all workspaces with tokens for an employee.
    pub fn list_workspaces(&self, employee_id: &str) -> Result<Vec<NotionOAuthToken>, NotionError> {
        let collection = match &self.collection {
            Some(c) => c,
            None => return Ok(vec![]),
        };

        let filter = doc! { "employee_id": employee_id };

        let cursor = collection
            .find(filter, None)
            .map_err(|e| NotionError::StorageError(format!("Failed to list workspaces: {}", e)))?;

        let mut tokens = Vec::new();
        for result in cursor {
            match result {
                Ok(doc) => {
                    if let Ok(token) = mongodb::bson::from_document::<NotionOAuthToken>(doc) {
                        tokens.push(token);
                    }
                }
                Err(e) => {
                    debug!("Skipping invalid token document: {}", e);
                }
            }
        }

        Ok(tokens)
    }

    /// Check if we have a valid token for a workspace.
    pub fn has_token(&self, workspace_id: &str, employee_id: &str) -> Result<bool, NotionError> {
        self.get_token(workspace_id, employee_id)
            .map(|t| t.is_some())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_noop_store_returns_none() {
        let store = NotionOAuthStore::noop();
        assert!(store.get_token("ws-123", "emp-1").unwrap().is_none());
        assert!(store.list_workspaces("emp-1").unwrap().is_empty());
    }
}
