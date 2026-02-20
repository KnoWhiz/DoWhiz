use chrono::{DateTime, Utc};
use postgres_native_tls::MakeTlsConnector;
use r2d2::{Pool, PooledConnection};
use r2d2_postgres::PostgresConnectionManager;
use std::env;
use tracing::error;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Account {
    pub id: Uuid,
    pub auth_user_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct AccountIdentifier {
    pub id: Uuid,
    pub account_id: Uuid,
    pub identifier_type: String,
    pub identifier: String,
    pub verified: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, thiserror::Error)]
pub enum AccountStoreError {
    #[error("postgres error: {0}")]
    Postgres(#[from] postgres::Error),
    #[error("pool error: {0}")]
    Pool(#[from] r2d2::Error),
    #[error("missing SUPABASE_DB_URL")]
    MissingDbUrl,
    #[error("account not found")]
    NotFound,
    #[error("identifier already linked to another account")]
    IdentifierTaken,
    #[error("config error: {0}")]
    Config(String),
}

/// Custom error handler that logs connection errors
#[derive(Debug)]
struct LoggingErrorHandler;

impl r2d2::HandleError<postgres::Error> for LoggingErrorHandler {
    fn handle_error(&self, err: postgres::Error) {
        error!("account_store postgres pool error: {:?}", err);
    }
}

#[derive(Clone)]
pub struct AccountStore {
    pool: Option<Pool<PostgresConnectionManager<MakeTlsConnector>>>,
}

impl AccountStore {
    pub fn from_env() -> Result<Self, AccountStoreError> {
        let db_url = env::var("SUPABASE_DB_URL")
            .ok()
            .filter(|v| !v.trim().is_empty())
            .ok_or(AccountStoreError::MissingDbUrl)?;
        Self::new(&db_url)
    }

    pub fn new(db_url: &str) -> Result<Self, AccountStoreError> {
        let config: postgres::Config = db_url.parse()?;

        let mut tls_builder = native_tls::TlsConnector::builder();
        if env::var("INGESTION_QUEUE_TLS_ALLOW_INVALID_CERTS")
            .map(|v| matches!(v.to_lowercase().as_str(), "1" | "true" | "yes"))
            .unwrap_or(false)
        {
            tls_builder.danger_accept_invalid_certs(true);
            tls_builder.danger_accept_invalid_hostnames(true);
        }
        let tls_connector = tls_builder
            .build()
            .map_err(|e| AccountStoreError::Config(e.to_string()))?;
        let tls = MakeTlsConnector::new(tls_connector);

        let manager = PostgresConnectionManager::new(config, tls);
        let pool = Pool::builder()
            .max_size(50)
            .min_idle(Some(2))
            .connection_timeout(std::time::Duration::from_secs(5))
            .idle_timeout(Some(std::time::Duration::from_secs(60)))
            .error_handler(Box::new(LoggingErrorHandler))
            .build(manager)?;

        Ok(Self { pool: Some(pool) })
    }

    fn conn(
        &self,
    ) -> Result<PooledConnection<PostgresConnectionManager<MakeTlsConnector>>, AccountStoreError>
    {
        let pool = self
            .pool
            .as_ref()
            .ok_or_else(|| AccountStoreError::Config("account store pool dropped".to_string()))?;
        Ok(pool.get()?)
    }

    /// Create a new account linked to a Supabase auth user
    pub fn create_account(&self, auth_user_id: Uuid) -> Result<Account, AccountStoreError> {
        let mut conn = self.conn()?;
        let id = Uuid::new_v4();
        let row = conn.query_one(
            "INSERT INTO accounts (id, auth_user_id, created_at)
             VALUES ($1, $2, NOW())
             RETURNING id, auth_user_id, created_at",
            &[&id, &auth_user_id],
        )?;
        Ok(Account {
            id: row.get(0),
            auth_user_id: row.get(1),
            created_at: row.get(2),
        })
    }

    /// Get account by Supabase auth user ID
    pub fn get_account_by_auth_user(
        &self,
        auth_user_id: Uuid,
    ) -> Result<Option<Account>, AccountStoreError> {
        let mut conn = self.conn()?;
        let row = conn.query_opt(
            "SELECT id, auth_user_id, created_at FROM accounts WHERE auth_user_id = $1",
            &[&auth_user_id],
        )?;
        Ok(row.map(|r| Account {
            id: r.get(0),
            auth_user_id: r.get(1),
            created_at: r.get(2),
        }))
    }

    /// Get account by ID
    pub fn get_account(&self, account_id: Uuid) -> Result<Option<Account>, AccountStoreError> {
        let mut conn = self.conn()?;
        let row = conn.query_opt(
            "SELECT id, auth_user_id, created_at FROM accounts WHERE id = $1",
            &[&account_id],
        )?;
        Ok(row.map(|r| Account {
            id: r.get(0),
            auth_user_id: r.get(1),
            created_at: r.get(2),
        }))
    }

    /// Look up account by channel identifier (for message routing)
    pub fn get_account_by_identifier(
        &self,
        identifier_type: &str,
        identifier: &str,
    ) -> Result<Option<Account>, AccountStoreError> {
        let mut conn = self.conn()?;
        let row = conn.query_opt(
            "SELECT a.id, a.auth_user_id, a.created_at
             FROM accounts a
             JOIN account_identifiers ai ON ai.account_id = a.id
             WHERE ai.identifier_type = $1 AND ai.identifier = $2 AND ai.verified = true",
            &[&identifier_type, &identifier],
        )?;
        Ok(row.map(|r| Account {
            id: r.get(0),
            auth_user_id: r.get(1),
            created_at: r.get(2),
        }))
    }

    /// Create a new identifier link (unverified by default)
    pub fn create_identifier(
        &self,
        account_id: Uuid,
        identifier_type: &str,
        identifier: &str,
    ) -> Result<AccountIdentifier, AccountStoreError> {
        let mut conn = self.conn()?;
        let id = Uuid::new_v4();

        // Check if identifier is already taken by another account
        let existing = conn.query_opt(
            "SELECT account_id FROM account_identifiers
             WHERE identifier_type = $1 AND identifier = $2",
            &[&identifier_type, &identifier],
        )?;

        if let Some(row) = existing {
            let existing_account: Uuid = row.get(0);
            if existing_account != account_id {
                return Err(AccountStoreError::IdentifierTaken);
            }
        }

        let row = conn.query_one(
            "INSERT INTO account_identifiers (id, account_id, identifier_type, identifier, verified, created_at)
             VALUES ($1, $2, $3, $4, false, NOW())
             ON CONFLICT (identifier_type, identifier) DO UPDATE SET account_id = $2
             RETURNING id, account_id, identifier_type, identifier, verified, created_at",
            &[&id, &account_id, &identifier_type, &identifier],
        )?;

        Ok(AccountIdentifier {
            id: row.get(0),
            account_id: row.get(1),
            identifier_type: row.get(2),
            identifier: row.get(3),
            verified: row.get(4),
            created_at: row.get(5),
        })
    }

    /// Mark an identifier as verified
    pub fn verify_identifier(
        &self,
        account_id: Uuid,
        identifier_type: &str,
        identifier: &str,
    ) -> Result<(), AccountStoreError> {
        let mut conn = self.conn()?;
        let updated = conn.execute(
            "UPDATE account_identifiers
             SET verified = true
             WHERE account_id = $1 AND identifier_type = $2 AND identifier = $3",
            &[&account_id, &identifier_type, &identifier],
        )?;
        if updated == 0 {
            return Err(AccountStoreError::NotFound);
        }
        Ok(())
    }

    /// Delete an identifier link
    pub fn delete_identifier(
        &self,
        account_id: Uuid,
        identifier_type: &str,
        identifier: &str,
    ) -> Result<(), AccountStoreError> {
        let mut conn = self.conn()?;
        let deleted = conn.execute(
            "DELETE FROM account_identifiers
             WHERE account_id = $1 AND identifier_type = $2 AND identifier = $3",
            &[&account_id, &identifier_type, &identifier],
        )?;
        if deleted == 0 {
            return Err(AccountStoreError::NotFound);
        }
        Ok(())
    }

    /// List all identifiers for an account
    pub fn list_identifiers(
        &self,
        account_id: Uuid,
    ) -> Result<Vec<AccountIdentifier>, AccountStoreError> {
        let mut conn = self.conn()?;
        let rows = conn.query(
            "SELECT id, account_id, identifier_type, identifier, verified, created_at
             FROM account_identifiers
             WHERE account_id = $1
             ORDER BY created_at",
            &[&account_id],
        )?;
        Ok(rows
            .iter()
            .map(|r| AccountIdentifier {
                id: r.get(0),
                account_id: r.get(1),
                identifier_type: r.get(2),
                identifier: r.get(3),
                verified: r.get(4),
                created_at: r.get(5),
            })
            .collect())
    }

    /// Delete an account and all its identifiers (CASCADE)
    pub fn delete_account(&self, account_id: Uuid) -> Result<(), AccountStoreError> {
        let mut conn = self.conn()?;
        let deleted = conn.execute("DELETE FROM accounts WHERE id = $1", &[&account_id])?;
        if deleted == 0 {
            return Err(AccountStoreError::NotFound);
        }
        Ok(())
    }
}

impl Drop for AccountStore {
    fn drop(&mut self) {
        if let Some(pool) = self.pool.take() {
            std::thread::spawn(move || drop(pool));
        }
    }
}

// ============================================================================
// Global AccountStore accessor
// ============================================================================

use std::sync::Arc;

/// Lazy-initialized global AccountStore
static ACCOUNT_STORE: std::sync::OnceLock<Option<Arc<AccountStore>>> = std::sync::OnceLock::new();

/// Get or initialize the global AccountStore (returns None if not configured)
pub fn get_global_account_store() -> Option<Arc<AccountStore>> {
    ACCOUNT_STORE
        .get_or_init(|| match AccountStore::from_env() {
            Ok(store) => {
                tracing::info!("AccountStore initialized for account lookups");
                Some(Arc::new(store))
            }
            Err(e) => {
                tracing::info!(
                    "AccountStore not available ({}), account lookups disabled",
                    e
                );
                None
            }
        })
        .clone()
}

/// Map a channel type to the identifier_type used in account_identifiers table
pub fn channel_to_identifier_type(channel: &crate::channel::Channel) -> &'static str {
    use crate::channel::Channel;
    match channel {
        Channel::Email => "email",
        Channel::Sms => "phone",
        Channel::WhatsApp => "phone",
        Channel::Telegram => "telegram",
        Channel::Slack => "slack",
        Channel::Discord => "discord",
        Channel::BlueBubbles => "phone",
        Channel::GoogleDocs => "email",
    }
}

/// Look up account by channel and identifier
/// Returns the account_id if found and verified, None otherwise
pub fn lookup_account_by_channel(
    channel: &crate::channel::Channel,
    identifier: &str,
) -> Option<Uuid> {
    let store = get_global_account_store()?;
    let identifier_type = channel_to_identifier_type(channel);

    match store.get_account_by_identifier(identifier_type, identifier) {
        Ok(Some(account)) => {
            tracing::debug!(
                "Found account {} for {}:{}",
                account.id,
                identifier_type,
                identifier
            );
            Some(account.id)
        }
        Ok(None) => {
            tracing::debug!(
                "No account found for {}:{}, using local storage",
                identifier_type,
                identifier
            );
            None
        }
        Err(e) => {
            tracing::warn!(
                "Error looking up account for {}:{}: {}, using local storage",
                identifier_type,
                identifier,
                e
            );
            None
        }
    }
}
