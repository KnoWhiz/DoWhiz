use std::collections::HashSet;
use std::env;
use std::sync::{Mutex, OnceLock};
use std::thread;
use std::time::Duration;

use mongodb::bson::{doc, Document};
use mongodb::error::ErrorKind;
use mongodb::options::{ClientOptions, IndexOptions};
use mongodb::sync::{Client, Collection, Database};
use mongodb::IndexModel;
use tracing::warn;

use crate::storage_backend::StorageBackend;

#[derive(Debug, thiserror::Error)]
pub enum MongoStoreError {
    #[error("MONGODB_URI must be set")]
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
    let mut options = ClientOptions::parse(uri).run()?;
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
    db.run_command(doc! { "ping": 1 }).run()?;
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

pub fn ensure_index_compatible(
    collection: &Collection<Document>,
    model: IndexModel,
) -> Result<(), mongodb::error::Error> {
    let cache_key = index_cache_key(collection, &model);
    let mut cache = ensured_indexes()
        .lock()
        .expect("index cache mutex poisoned");
    if cache.contains(&cache_key) {
        return Ok(());
    }

    let mut attempt = 0usize;
    loop {
        match collection.create_index(model.clone()).run() {
            Ok(_) => {
                cache.insert(cache_key.clone());
                return Ok(());
            }
            Err(err) if is_ignorable_index_conflict(&err) => {
                warn!(
                    error = %err,
                    collection = collection.name(),
                    "ignoring existing conflicting index definition; keeping remote index options"
                );
                cache.insert(cache_key.clone());
                return Ok(());
            }
            Err(err) => {
                let Some(retry_after_ms) = retry_after_ms_for_index_create(&err) else {
                    return Err(err);
                };
                if attempt >= 7 {
                    return Err(err);
                }
                attempt += 1;
                let sleep_ms = retry_after_ms.clamp(25, 2_000) + (attempt as u64 * 25);
                warn!(
                    error = %err,
                    collection = collection.name(),
                    attempt,
                    sleep_ms,
                    "mongodb metadata throttle while creating index; retrying"
                );
                thread::sleep(Duration::from_millis(sleep_ms));
            }
        }
    }
}

fn is_ignorable_index_conflict(err: &mongodb::error::Error) -> bool {
    let ErrorKind::Command(command_error) = err.kind.as_ref() else {
        return false;
    };
    if command_error.code == 67 && command_error.code_name == "CannotCreateIndex" {
        let message = command_error.message.to_ascii_lowercase();
        if message.contains("cannot create unique index over")
            || message.contains("cannot create unique index when collection contains documents")
        {
            // Cosmos sharded collections can reject some unique indexes, and legacy
            // collections may already contain rows that prevent unique index creation.
            // This workload remains correct via id-scoped upserts/filters.
            return true;
        }
    }
    if matches!(command_error.code, 48 | 68 | 85 | 86) {
        return true;
    }
    matches!(
        command_error.code_name.as_str(),
        "NamespaceExists" | "IndexAlreadyExists" | "IndexOptionsConflict" | "IndexKeySpecsConflict"
    ) || command_error
        .message
        .contains("already exists with different options")
}

fn retry_after_ms_for_index_create(err: &mongodb::error::Error) -> Option<u64> {
    let ErrorKind::Command(command_error) = err.kind.as_ref() else {
        return None;
    };
    if command_error.code != 16500 && command_error.code_name != "RequestRateTooLarge" {
        return None;
    }
    if let Some(position) = command_error.message.find("RetryAfterMs=") {
        let value = &command_error.message[position + "RetryAfterMs=".len()..];
        let digits: String = value.chars().take_while(|ch| ch.is_ascii_digit()).collect();
        if let Ok(parsed) = digits.parse::<u64>() {
            return Some(parsed);
        }
    }
    Some(250)
}

fn ensured_indexes() -> &'static Mutex<HashSet<String>> {
    static ENSURED: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();
    ENSURED.get_or_init(|| Mutex::new(HashSet::new()))
}

fn index_cache_key(collection: &Collection<Document>, model: &IndexModel) -> String {
    let namespace = collection.namespace();
    let unique = model
        .options
        .as_ref()
        .and_then(|options| options.unique)
        .unwrap_or(false);
    format!(
        "{}.{}:{}:unique={}",
        namespace.db, namespace.coll, model.keys, unique
    )
}

fn ensure_users_indexes(db: &Database) -> Result<(), mongodb::error::Error> {
    let collection = db.collection::<Document>("users");
    ensure_index_compatible(
        &collection,
        IndexModel::builder()
            .keys(doc! { "user_id": 1 })
            .options(IndexOptions::builder().unique(Some(true)).build())
            .build(),
    )?;
    ensure_index_compatible(
        &collection,
        IndexModel::builder()
            .keys(doc! { "identifier_type": 1, "identifier": 1 })
            .build(),
    )?;
    ensure_index_compatible(
        &collection,
        IndexModel::builder().keys(doc! { "created_at": 1 }).build(),
    )?;
    Ok(())
}

fn ensure_tasks_indexes(db: &Database) -> Result<(), mongodb::error::Error> {
    let collection = db.collection::<Document>("tasks");
    ensure_index_compatible(
        &collection,
        IndexModel::builder()
            .keys(doc! { "owner_scope.kind": 1, "owner_scope.id": 1, "task_id": 1 })
            .build(),
    )?;
    ensure_index_compatible(
        &collection,
        IndexModel::builder()
            .keys(doc! { "owner_scope.kind": 1, "owner_scope.id": 1, "enabled": 1 })
            .build(),
    )?;
    ensure_index_compatible(
        &collection,
        IndexModel::builder()
            .keys(doc! { "enabled": 1, "schedule.next_run": 1 })
            .build(),
    )?;
    ensure_index_compatible(
        &collection,
        IndexModel::builder()
            .keys(doc! { "owner_scope.kind": 1, "owner_scope.id": 1, "created_at": 1 })
            .build(),
    )?;
    Ok(())
}

fn ensure_task_executions_indexes(db: &Database) -> Result<(), mongodb::error::Error> {
    let collection = db.collection::<Document>("task_executions");
    ensure_index_compatible(
        &collection,
        IndexModel::builder()
            .keys(doc! {
                "owner_scope.kind": 1,
                "owner_scope.id": 1,
                "task_id": 1,
                "started_at": -1
            })
            .build(),
    )?;
    ensure_index_compatible(
        &collection,
        IndexModel::builder()
            .keys(doc! { "started_at": -1 })
            .build(),
    )?;
    Ok(())
}

fn ensure_task_index_indexes(db: &Database) -> Result<(), mongodb::error::Error> {
    let collection = db.collection::<Document>("task_index");
    // user_id must come first for sharded collections (Cosmos DB shard key compatibility)
    ensure_index_compatible(
        &collection,
        IndexModel::builder()
            .keys(doc! { "user_id": 1, "task_id": 1 })
            .options(IndexOptions::builder().unique(Some(true)).build())
            .build(),
    )?;
    ensure_index_compatible(
        &collection,
        IndexModel::builder()
            .keys(doc! { "enabled": 1, "next_run": 1 })
            .build(),
    )?;
    Ok(())
}

fn ensure_account_task_views_indexes(db: &Database) -> Result<(), mongodb::error::Error> {
    let collection = db.collection::<Document>("account_task_views");
    ensure_index_compatible(
        &collection,
        IndexModel::builder()
            .keys(doc! { "account_id": 1, "task_id": 1 })
            .options(IndexOptions::builder().unique(Some(true)).build())
            .build(),
    )?;
    ensure_index_compatible(
        &collection,
        IndexModel::builder()
            .keys(doc! { "account_id": 1, "created_at": -1 })
            .build(),
    )?;
    Ok(())
}

fn ensure_slack_installation_indexes(db: &Database) -> Result<(), mongodb::error::Error> {
    let collection = db.collection::<Document>("slack_installations");
    ensure_index_compatible(
        &collection,
        IndexModel::builder()
            .keys(doc! { "team_id": 1 })
            .options(IndexOptions::builder().unique(Some(true)).build())
            .build(),
    )?;
    ensure_index_compatible(
        &collection,
        IndexModel::builder()
            .keys(doc! { "installed_at": -1 })
            .build(),
    )?;
    Ok(())
}

fn ensure_processed_comments_indexes(db: &Database) -> Result<(), mongodb::error::Error> {
    let collection = db.collection::<Document>("processed_comments");
    ensure_index_compatible(
        &collection,
        IndexModel::builder()
            .keys(doc! { "file_id": 1, "tracking_id": 1 })
            .options(IndexOptions::builder().unique(Some(true)).build())
            .build(),
    )?;
    ensure_index_compatible(
        &collection,
        IndexModel::builder()
            .keys(doc! { "file_type": 1, "processed_at": -1 })
            .build(),
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
