use std::env;

use mongodb::bson::{doc, Document};
use mongodb::options::{ClientOptions, IndexOptions};
use mongodb::sync::{Client, Database};
use mongodb::IndexModel;

use crate::storage_backend::StorageBackend;

#[derive(Debug, thiserror::Error)]
pub enum MongoStoreError {
    #[error("MONGODB_URI must be set when STORAGE_BACKEND includes mongo")]
    MissingMongoUri,
    #[error("mongodb error: {0}")]
    Mongo(#[from] mongodb::error::Error),
}

pub fn create_client_from_env() -> Result<Client, MongoStoreError> {
    let uri = env::var("MONGODB_URI")
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .ok_or(MongoStoreError::MissingMongoUri)?;
    let mut options = ClientOptions::parse(uri)?;
    options.app_name = Some("DoWhizScheduler".to_string());
    Ok(Client::with_options(options)?)
}

pub fn mongo_database_name_from_env() -> String {
    let explicit = env::var("MONGODB_DATABASE")
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());
    if let Some(name) = explicit {
        return name;
    }
    let target = env::var("DEPLOY_TARGET")
        .ok()
        .map(|value| value.trim().to_ascii_lowercase())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "production".to_string());
    let employee = env::var("EMPLOYEE_ID")
        .ok()
        .map(|value| sanitize_fragment(&value))
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "default".to_string());
    format!("dowhiz_{}_{}", sanitize_fragment(&target), employee)
}

pub fn database_from_env(client: &Client) -> Database {
    client.database(&mongo_database_name_from_env())
}

pub fn health_check_from_env() -> Result<(), MongoStoreError> {
    if !StorageBackend::from_env().uses_mongo() {
        return Ok(());
    }
    let client = create_client_from_env()?;
    let db = database_from_env(&client);
    db.run_command(doc! { "ping": 1 }, None)?;
    Ok(())
}

pub fn bootstrap_indexes_from_env() -> Result<(), MongoStoreError> {
    if !StorageBackend::from_env().uses_mongo() {
        return Ok(());
    }
    let client = create_client_from_env()?;
    let db = database_from_env(&client);

    ensure_users_indexes(&db)?;
    ensure_tasks_indexes(&db)?;
    ensure_task_executions_indexes(&db)?;
    ensure_task_index_indexes(&db)?;
    ensure_account_task_views_indexes(&db)?;
    ensure_slack_installation_indexes(&db)?;
    ensure_processed_comments_indexes(&db)?;
    Ok(())
}

fn ensure_users_indexes(db: &Database) -> Result<(), mongodb::error::Error> {
    let collection = db.collection::<Document>("users");
    collection.create_index(
        IndexModel::builder()
            .keys(doc! { "user_id": 1 })
            .options(IndexOptions::builder().unique(Some(true)).build())
            .build(),
        None,
    )?;
    collection.create_index(
        IndexModel::builder()
            .keys(doc! { "identifier_type": 1, "identifier": 1 })
            .build(),
        None,
    )?;
    collection.create_index(
        IndexModel::builder().keys(doc! { "created_at": 1 }).build(),
        None,
    )?;
    Ok(())
}

fn ensure_tasks_indexes(db: &Database) -> Result<(), mongodb::error::Error> {
    let collection = db.collection::<Document>("tasks");
    collection.create_index(
        IndexModel::builder()
            .keys(doc! { "owner_scope.kind": 1, "owner_scope.id": 1, "task_id": 1 })
            .build(),
        None,
    )?;
    collection.create_index(
        IndexModel::builder()
            .keys(doc! { "owner_scope.kind": 1, "owner_scope.id": 1, "enabled": 1 })
            .build(),
        None,
    )?;
    collection.create_index(
        IndexModel::builder()
            .keys(doc! { "enabled": 1, "schedule.next_run": 1 })
            .build(),
        None,
    )?;
    collection.create_index(
        IndexModel::builder()
            .keys(doc! { "owner_scope.kind": 1, "owner_scope.id": 1, "created_at": 1 })
            .build(),
        None,
    )?;
    Ok(())
}

fn ensure_task_executions_indexes(db: &Database) -> Result<(), mongodb::error::Error> {
    let collection = db.collection::<Document>("task_executions");
    collection.create_index(
        IndexModel::builder()
            .keys(doc! {
                "owner_scope.kind": 1,
                "owner_scope.id": 1,
                "task_id": 1,
                "started_at": -1
            })
            .build(),
        None,
    )?;
    collection.create_index(
        IndexModel::builder()
            .keys(doc! { "started_at": -1 })
            .build(),
        None,
    )?;
    Ok(())
}

fn ensure_task_index_indexes(db: &Database) -> Result<(), mongodb::error::Error> {
    let collection = db.collection::<Document>("task_index");
    collection.create_index(
        IndexModel::builder()
            .keys(doc! { "task_id": 1, "user_id": 1 })
            .options(IndexOptions::builder().unique(Some(true)).build())
            .build(),
        None,
    )?;
    collection.create_index(
        IndexModel::builder()
            .keys(doc! { "enabled": 1, "next_run": 1 })
            .build(),
        None,
    )?;
    Ok(())
}

fn ensure_account_task_views_indexes(db: &Database) -> Result<(), mongodb::error::Error> {
    let collection = db.collection::<Document>("account_task_views");
    collection.create_index(
        IndexModel::builder()
            .keys(doc! { "account_id": 1, "task_id": 1 })
            .options(IndexOptions::builder().unique(Some(true)).build())
            .build(),
        None,
    )?;
    collection.create_index(
        IndexModel::builder()
            .keys(doc! { "account_id": 1, "created_at": -1 })
            .build(),
        None,
    )?;
    Ok(())
}

fn ensure_slack_installation_indexes(db: &Database) -> Result<(), mongodb::error::Error> {
    let collection = db.collection::<Document>("slack_installations");
    collection.create_index(
        IndexModel::builder()
            .keys(doc! { "team_id": 1 })
            .options(IndexOptions::builder().unique(Some(true)).build())
            .build(),
        None,
    )?;
    collection.create_index(
        IndexModel::builder()
            .keys(doc! { "installed_at": -1 })
            .build(),
        None,
    )?;
    Ok(())
}

fn ensure_processed_comments_indexes(db: &Database) -> Result<(), mongodb::error::Error> {
    let collection = db.collection::<Document>("processed_comments");
    collection.create_index(
        IndexModel::builder()
            .keys(doc! { "file_id": 1, "tracking_id": 1 })
            .options(IndexOptions::builder().unique(Some(true)).build())
            .build(),
        None,
    )?;
    collection.create_index(
        IndexModel::builder()
            .keys(doc! { "file_type": 1, "processed_at": -1 })
            .build(),
        None,
    )?;
    Ok(())
}

fn sanitize_fragment(raw: &str) -> String {
    let mut result = String::with_capacity(raw.len());
    let mut last_was_underscore = false;
    for ch in raw.chars() {
        if ch.is_ascii_alphanumeric() {
            result.push(ch.to_ascii_lowercase());
            last_was_underscore = false;
        } else if !last_was_underscore {
            result.push('_');
            last_was_underscore = true;
        }
    }
    result.trim_matches('_').to_string()
}

#[cfg(test)]
mod tests {
    use super::sanitize_fragment;

    #[test]
    fn sanitize_fragment_normalizes_separators() {
        assert_eq!(
            sanitize_fragment("Little Bear@DoWhiz.com"),
            "little_bear_dowhiz_com"
        );
        assert_eq!(sanitize_fragment("prod--west"), "prod_west");
    }
}
