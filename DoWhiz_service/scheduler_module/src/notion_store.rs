//! Notion OAuth credential store for multi-workspace support.
//!
//! Stores OAuth access tokens and workspace info per user/workspace.

use chrono::{DateTime, Utc};
use mongodb::bson::{doc, Bson, DateTime as BsonDateTime, Document};
use mongodb::options::{IndexOptions, UpdateOptions};
use mongodb::sync::Collection;
use mongodb::IndexModel;

use crate::mongo_store::{create_client_from_env, database_from_env, ensure_index_compatible};

/// A Notion OAuth credential record.
#[derive(Debug, Clone)]
pub struct NotionCredential {
    pub account_id: uuid::Uuid,
    pub workspace_id: String,
    pub workspace_name: Option<String>,
    pub access_token: String,
    pub bot_id: String,
    pub owner_user_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, thiserror::Error)]
pub enum NotionStoreError {
    #[error("mongodb error: {0}")]
    Mongo(#[from] mongodb::error::Error),
    #[error("credential not found for workspace: {0}")]
    NotFound(String),
    #[error("mongo config error: {0}")]
    MongoConfig(String),
}

/// Store for Notion OAuth credentials.
#[derive(Debug, Clone)]
pub struct NotionStore {
    credentials: Collection<Document>,
}

impl NotionStore {
    /// Create a new NotionStore.
    pub fn new() -> Result<Self, NotionStoreError> {
        let client = create_client_from_env()
            .map_err(|e| NotionStoreError::MongoConfig(e.to_string()))?;
        let db = database_from_env(&client);
        let credentials = db.collection::<Document>("notion_credentials");

        // Create indexes
        let store = Self { credentials };
        store.ensure_indexes()?;

        Ok(store)
    }

    fn ensure_indexes(&self) -> Result<(), NotionStoreError> {
        // Unique index on account_id + workspace_id
        ensure_index_compatible(
            &self.credentials,
            IndexModel::builder()
                .keys(doc! { "account_id": 1, "workspace_id": 1 })
                .options(IndexOptions::builder().unique(Some(true)).build())
                .build(),
        )?;

        // Index on workspace_id for lookups
        ensure_index_compatible(
            &self.credentials,
            IndexModel::builder()
                .keys(doc! { "workspace_id": 1 })
                .build(),
        )?;

        Ok(())
    }

    /// Save or update a credential for an account/workspace.
    pub fn upsert_credential(
        &self,
        credential: &NotionCredential,
    ) -> Result<(), NotionStoreError> {
        let now = BsonDateTime::from_chrono(Utc::now());

        self.credentials.update_one(
            doc! {
                "account_id": credential.account_id.to_string(),
                "workspace_id": credential.workspace_id.as_str(),
            },
            doc! {
                "$set": {
                    "account_id": credential.account_id.to_string(),
                    "workspace_id": credential.workspace_id.as_str(),
                    "workspace_name": credential.workspace_name.clone().map(Bson::from).unwrap_or(Bson::Null),
                    "access_token": credential.access_token.as_str(),
                    "bot_id": credential.bot_id.as_str(),
                    "owner_user_id": credential.owner_user_id.clone().map(Bson::from).unwrap_or(Bson::Null),
                    "updated_at": now,
                },
                "$setOnInsert": {
                    "created_at": now,
                }
            },
            UpdateOptions::builder().upsert(true).build(),
        )?;

        Ok(())
    }

    /// Get credential by account_id and workspace_id.
    pub fn get_credential(
        &self,
        account_id: uuid::Uuid,
        workspace_id: &str,
    ) -> Result<NotionCredential, NotionStoreError> {
        let doc = self
            .credentials
            .find_one(
                doc! {
                    "account_id": account_id.to_string(),
                    "workspace_id": workspace_id,
                },
                None,
            )?
            .ok_or_else(|| NotionStoreError::NotFound(workspace_id.to_string()))?;

        Self::doc_to_credential(doc)
    }

    /// Get all credentials for an account.
    pub fn get_credentials_for_account(
        &self,
        account_id: uuid::Uuid,
    ) -> Result<Vec<NotionCredential>, NotionStoreError> {
        let cursor = self.credentials.find(
            doc! {
                "account_id": account_id.to_string(),
            },
            None,
        )?;

        let mut credentials = Vec::new();
        for result in cursor {
            let doc = result?;
            credentials.push(Self::doc_to_credential(doc)?);
        }

        Ok(credentials)
    }

    /// Get credential by workspace_id (for incoming webhooks).
    pub fn get_credential_by_workspace(
        &self,
        workspace_id: &str,
    ) -> Result<NotionCredential, NotionStoreError> {
        let doc = self
            .credentials
            .find_one(
                doc! {
                    "workspace_id": workspace_id,
                },
                None,
            )?
            .ok_or_else(|| NotionStoreError::NotFound(workspace_id.to_string()))?;

        Self::doc_to_credential(doc)
    }

    /// Delete a credential.
    pub fn delete_credential(
        &self,
        account_id: uuid::Uuid,
        workspace_id: &str,
    ) -> Result<bool, NotionStoreError> {
        let result = self.credentials.delete_one(
            doc! {
                "account_id": account_id.to_string(),
                "workspace_id": workspace_id,
            },
            None,
        )?;
        Ok(result.deleted_count > 0)
    }

    fn doc_to_credential(doc: Document) -> Result<NotionCredential, NotionStoreError> {
        let account_id_str = doc
            .get_str("account_id")
            .map_err(|_| NotionStoreError::NotFound("missing account_id".to_string()))?;
        let account_id = uuid::Uuid::parse_str(account_id_str)
            .map_err(|_| NotionStoreError::NotFound("invalid account_id".to_string()))?;

        let workspace_id = doc
            .get_str("workspace_id")
            .map_err(|_| NotionStoreError::NotFound("missing workspace_id".to_string()))?
            .to_string();

        let workspace_name = match doc.get("workspace_name") {
            Some(Bson::String(value)) => Some(value.to_string()),
            _ => None,
        };

        let access_token = doc
            .get_str("access_token")
            .map_err(|_| NotionStoreError::NotFound("missing access_token".to_string()))?
            .to_string();

        let bot_id = doc
            .get_str("bot_id")
            .map_err(|_| NotionStoreError::NotFound("missing bot_id".to_string()))?
            .to_string();

        let owner_user_id = match doc.get("owner_user_id") {
            Some(Bson::String(value)) => Some(value.to_string()),
            _ => None,
        };

        let created_at = match doc.get("created_at") {
            Some(Bson::DateTime(dt)) => dt.to_chrono(),
            _ => Utc::now(),
        };

        let updated_at = match doc.get("updated_at") {
            Some(Bson::DateTime(dt)) => dt.to_chrono(),
            _ => Utc::now(),
        };

        Ok(NotionCredential {
            account_id,
            workspace_id,
            workspace_name,
            access_token,
            bot_id,
            owner_user_id,
            created_at,
            updated_at,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_credential_roundtrip() {
        // This test requires MongoDB to be available
        if std::env::var("MONGODB_URI").is_err() {
            return;
        }

        let store = NotionStore::new().unwrap();
        let account_id = uuid::Uuid::new_v4();
        let workspace_id = format!("test-workspace-{}", uuid::Uuid::new_v4());

        let credential = NotionCredential {
            account_id,
            workspace_id: workspace_id.clone(),
            workspace_name: Some("Test Workspace".to_string()),
            access_token: "secret_test_token".to_string(),
            bot_id: "bot_123".to_string(),
            owner_user_id: Some("user_456".to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Upsert
        store.upsert_credential(&credential).unwrap();

        // Get
        let retrieved = store.get_credential(account_id, &workspace_id).unwrap();
        assert_eq!(retrieved.workspace_name, Some("Test Workspace".to_string()));
        assert_eq!(retrieved.access_token, "secret_test_token");

        // Delete
        let deleted = store.delete_credential(account_id, &workspace_id).unwrap();
        assert!(deleted);
    }
}
