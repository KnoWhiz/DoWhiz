use chrono::{DateTime, Utc};
use postgres_native_tls::MakeTlsConnector;
use r2d2::{Pool, PooledConnection};
use r2d2_postgres::PostgresConnectionManager;
use serde::Serialize;
use std::collections::BTreeMap;
use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tracing::{error, info, warn};
use uuid::Uuid;

use crate::env_alias::var_with_scale_oliver;

type PgPool = Pool<PostgresConnectionManager<MakeTlsConnector>>;
type PgConn = PooledConnection<PostgresConnectionManager<MakeTlsConnector>>;

fn query_count_or_zero_on_missing_table(
    conn: &mut PgConn,
    query: &str,
) -> Result<i64, AccountStoreError> {
    match conn.query_one(query, &[]) {
        Ok(row) => Ok(row.get(0)),
        Err(err) if err.code() == Some(&postgres::error::SqlState::UNDEFINED_TABLE) => {
            warn!("account_store metrics query skipped missing table: {}", query);
            Ok(0)
        }
        Err(err) => Err(err.into()),
    }
}

fn parse_bool_env(value: &str) -> bool {
    matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "1" | "true" | "yes" | "on"
    )
}

fn account_store_allow_invalid_certs() -> bool {
    if let Some(value) = var_with_scale_oliver("ACCOUNT_STORE_TLS_ALLOW_INVALID_CERTS") {
        return parse_bool_env(&value);
    }
    if let Some(value) = var_with_scale_oliver("INGESTION_QUEUE_TLS_ALLOW_INVALID_CERTS") {
        return parse_bool_env(&value);
    }
    env::var("DEPLOY_TARGET")
        .ok()
        .map(|value| value.trim().eq_ignore_ascii_case("staging"))
        .unwrap_or(false)
}

#[derive(Debug, Clone)]
pub struct Account {
    pub id: Uuid,
    pub auth_user_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub tokens_to_hours: Option<f64>,
    pub purchased_hours: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct BalanceInfo {
    pub purchased_hours: f64,
    pub used_hours: f64,
    pub balance_hours: f64,
}

#[derive(Debug, Clone)]
pub struct Payment {
    pub stripe_session_id: String,
    pub account_id: Uuid,
    pub amount_cents: i32,
    pub hours_purchased: f64,
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

#[derive(Debug, Clone, Serialize)]
pub struct AccountMetricsSnapshot {
    pub accounts_total: i64,
    pub identifiers_total: i64,
    pub verified_identifiers_total: i64,
    pub identifiers_by_type: BTreeMap<String, i64>,
    pub purchased_hours_total: f64,
    pub used_hours_total: f64,
    pub payments_total: i64,
    pub payments_last_24h: i64,
}

#[derive(Debug, Clone)]
pub struct EmailVerificationToken {
    pub token: String,
    pub account_id: Uuid,
    pub email: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, thiserror::Error)]
pub enum AccountStoreError {
    #[error("postgres error: {0}")]
    Postgres(#[from] postgres::Error),
    #[error("pool error: {0}")]
    Pool(#[from] r2d2::Error),
    #[error("missing SUPABASE_DB_URL and SUPABASE_POOLER_URL")]
    MissingDbUrl,
    #[error("account not found")]
    NotFound,
    #[error("identifier already linked to another account")]
    IdentifierTaken,
    #[error("verification token expired or invalid")]
    TokenInvalid,
    #[error("config error: {0}")]
    Config(String),
}

/// Custom error handler that logs connection errors
#[derive(Debug)]
struct LoggingErrorHandler {
    pool_name: &'static str,
}

impl r2d2::HandleError<postgres::Error> for LoggingErrorHandler {
    fn handle_error(&self, err: postgres::Error) {
        error!(
            "account_store {} postgres pool error: {:?}",
            self.pool_name, err
        );
    }
}

#[derive(Clone)]
pub struct AccountStore {
    primary_pool: Option<PgPool>,
    fallback_pool: Option<PgPool>,
    prefer_fallback: Arc<AtomicBool>,
}

impl AccountStore {
    pub fn from_env() -> Result<Self, AccountStoreError> {
        let primary_db_url = env::var("SUPABASE_DB_URL")
            .ok()
            .filter(|v| !v.trim().is_empty())
            .map(|v| v.trim().to_string());
        let fallback_db_url = env::var("SUPABASE_POOLER_URL")
            .ok()
            .filter(|v| !v.trim().is_empty())
            .map(|v| v.trim().to_string());

        if primary_db_url.is_none() && fallback_db_url.is_none() {
            return Err(AccountStoreError::MissingDbUrl);
        }

        let primary_pool = match primary_db_url {
            Some(db_url) => match Self::build_pool(&db_url, "primary") {
                Ok(pool) => Some(pool),
                Err(err) => {
                    if fallback_db_url.is_some() {
                        warn!(
                            "account_store failed to initialize SUPABASE_DB_URL pool ({}), will rely on SUPABASE_POOLER_URL fallback",
                            err
                        );
                        None
                    } else {
                        return Err(err);
                    }
                }
            },
            None => None,
        };

        let fallback_pool = match fallback_db_url {
            Some(db_url) => Some(Self::build_pool(&db_url, "pooler_fallback")?),
            None => None,
        };

        if primary_pool.is_none() && fallback_pool.is_none() {
            return Err(AccountStoreError::MissingDbUrl);
        }

        if primary_pool.is_none() && fallback_pool.is_some() {
            info!("account_store initialized with SUPABASE_POOLER_URL only");
        }

        Ok(Self {
            primary_pool,
            fallback_pool,
            prefer_fallback: Arc::new(AtomicBool::new(false)),
        })
    }

    pub fn new(db_url: &str) -> Result<Self, AccountStoreError> {
        let primary_pool = Self::build_pool(db_url, "primary")?;
        Ok(Self {
            primary_pool: Some(primary_pool),
            fallback_pool: None,
            prefer_fallback: Arc::new(AtomicBool::new(false)),
        })
    }

    fn build_pool(db_url: &str, pool_name: &'static str) -> Result<PgPool, AccountStoreError> {
        let config: postgres::Config = db_url.parse()?;

        let mut tls_builder = native_tls::TlsConnector::builder();
        if account_store_allow_invalid_certs() {
            warn!(
                "account_store {} TLS verification relaxed (invalid certs/hostnames allowed)",
                pool_name
            );
            tls_builder.danger_accept_invalid_certs(true);
            tls_builder.danger_accept_invalid_hostnames(true);
        }
        let tls_connector = tls_builder
            .build()
            .map_err(|e| AccountStoreError::Config(e.to_string()))?;
        let tls = MakeTlsConnector::new(tls_connector);

        let manager = PostgresConnectionManager::new(config, tls);
        Pool::builder()
            .max_size(10)
            .min_idle(Some(0))
            .connection_timeout(std::time::Duration::from_secs(10))
            .idle_timeout(Some(std::time::Duration::from_secs(30)))
            .max_lifetime(Some(std::time::Duration::from_secs(300)))
            .test_on_check_out(true)
            .error_handler(Box::new(LoggingErrorHandler { pool_name }))
            .build(manager)
            .map_err(AccountStoreError::from)
    }

    fn conn(&self) -> Result<PgConn, AccountStoreError> {
        if self.prefer_fallback.load(Ordering::Relaxed) {
            if let Some(pool) = self.fallback_pool.as_ref() {
                match pool.get() {
                    Ok(conn) => return Ok(conn),
                    Err(fallback_err) => {
                        warn!(
                            "account_store pooler fallback connection failed: {}, retrying primary db",
                            fallback_err
                        );
                    }
                }
            }
        }

        if let Some(pool) = self.primary_pool.as_ref() {
            match pool.get() {
                Ok(conn) => return Ok(conn),
                Err(primary_err) => {
                    if let Some(fallback_pool) = self.fallback_pool.as_ref() {
                        warn!(
                            "account_store primary db connection failed ({}), switching to SUPABASE_POOLER_URL",
                            primary_err
                        );
                        self.prefer_fallback.store(true, Ordering::Relaxed);
                        return Ok(fallback_pool.get()?);
                    }
                    return Err(primary_err.into());
                }
            }
        }

        if let Some(pool) = self.fallback_pool.as_ref() {
            return Ok(pool.get()?);
        }

        Err(AccountStoreError::Config(
            "account store pools dropped".to_string(),
        ))
    }

    /// Get a connection from the pool (primarily for tests)
    pub fn get_conn(
        &self,
    ) -> Result<PooledConnection<PostgresConnectionManager<MakeTlsConnector>>, AccountStoreError>
    {
        self.conn()
    }

    /// Create a new account linked to a Supabase auth user
    pub fn create_account(&self, auth_user_id: Uuid) -> Result<Account, AccountStoreError> {
        let mut conn = self.conn()?;
        let id = Uuid::new_v4();
        let row = conn.query_one(
            "INSERT INTO accounts (id, auth_user_id, created_at, tokens_to_hours, purchased_hours)
             VALUES ($1, $2, NOW(), 0, 0)
             RETURNING id, auth_user_id, created_at, tokens_to_hours::float8, purchased_hours::float8",
            &[&id, &auth_user_id],
        )?;
        Ok(Account {
            id: row.get(0),
            auth_user_id: row.get(1),
            created_at: row.get(2),
            tokens_to_hours: row.get(3),
            purchased_hours: row.get(4),
        })
    }

    /// Get account by Supabase auth user ID
    pub fn get_account_by_auth_user(
        &self,
        auth_user_id: Uuid,
    ) -> Result<Option<Account>, AccountStoreError> {
        let mut conn = self.conn()?;
        let row = conn.query_opt(
            "SELECT id, auth_user_id, created_at, tokens_to_hours::float8, purchased_hours::float8
             FROM accounts WHERE auth_user_id = $1",
            &[&auth_user_id],
        )?;
        Ok(row.map(|r| Account {
            id: r.get(0),
            auth_user_id: r.get(1),
            created_at: r.get(2),
            tokens_to_hours: r.get(3),
            purchased_hours: r.get(4),
        }))
    }

    /// Get account by ID
    pub fn get_account(&self, account_id: Uuid) -> Result<Option<Account>, AccountStoreError> {
        let mut conn = self.conn()?;
        let row = conn.query_opt(
            "SELECT id, auth_user_id, created_at, tokens_to_hours::float8, purchased_hours::float8
             FROM accounts WHERE id = $1",
            &[&account_id],
        )?;
        Ok(row.map(|r| Account {
            id: r.get(0),
            auth_user_id: r.get(1),
            created_at: r.get(2),
            tokens_to_hours: r.get(3),
            purchased_hours: r.get(4),
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
            "SELECT a.id, a.auth_user_id, a.created_at, a.tokens_to_hours::float8, a.purchased_hours::float8
             FROM accounts a
             JOIN account_identifiers ai ON ai.account_id = a.id
             WHERE ai.identifier_type = $1 AND ai.identifier = $2 AND ai.verified = true",
            &[&identifier_type, &identifier],
        )?;
        Ok(row.map(|r| Account {
            id: r.get(0),
            auth_user_id: r.get(1),
            created_at: r.get(2),
            tokens_to_hours: r.get(3),
            purchased_hours: r.get(4),
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

        // TODO: Auto-verify for MVP. Add real verification (SMS/email codes) later.
        let row = conn.query_one(
            "INSERT INTO account_identifiers (id, account_id, identifier_type, identifier, verified, created_at)
             VALUES ($1, $2, $3, $4, true, NOW())
             ON CONFLICT (identifier_type, identifier) DO UPDATE SET account_id = $2, verified = true
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

    /// Add tokens to an account's running total
    pub fn add_tokens(&self, account_id: Uuid, tokens: i64) -> Result<(), AccountStoreError> {
        let mut conn = self.conn()?;
        conn.execute(
            "UPDATE accounts
             SET tokens = COALESCE(tokens, 0) + $1,
                 tokens_to_hours = (COALESCE(tokens, 0) + $1)::numeric / 20000000
             WHERE id = $2",
            &[&tokens, &account_id],
        )?;
        Ok(())
    }

    /// Return global account metrics for internal dashboarding.
    pub fn metrics_snapshot(&self) -> Result<AccountMetricsSnapshot, AccountStoreError> {
        let mut conn = self.conn()?;

        let accounts_total = query_count_or_zero_on_missing_table(
            &mut conn,
            "SELECT COUNT(*)::bigint FROM accounts",
        )?;
        let identifiers_total = query_count_or_zero_on_missing_table(
            &mut conn,
            "SELECT COUNT(*)::bigint FROM account_identifiers",
        )?;
        let verified_identifiers_total = query_count_or_zero_on_missing_table(
            &mut conn,
            "SELECT COUNT(*)::bigint FROM account_identifiers WHERE verified = true",
        )?;
        let payments_total = query_count_or_zero_on_missing_table(
            &mut conn,
            "SELECT COUNT(*)::bigint FROM payments",
        )?;
        let payments_last_24h = query_count_or_zero_on_missing_table(
            &mut conn,
            "SELECT COUNT(*)::bigint FROM payments WHERE created_at >= NOW() - interval '24 hours'",
        )?;

        let mut identifiers_by_type = BTreeMap::new();
        match conn.query(
            "SELECT identifier_type, COUNT(*)::bigint
             FROM account_identifiers
             WHERE verified = true
             GROUP BY identifier_type
             ORDER BY identifier_type",
            &[],
        ) {
            Ok(rows) => {
                for row in rows {
                    let identifier_type: String = row.get(0);
                    let count: i64 = row.get(1);
                    identifiers_by_type.insert(identifier_type, count);
                }
            }
            Err(err) if err.code() == Some(&postgres::error::SqlState::UNDEFINED_TABLE) => {
                warn!("account_store metrics identifier breakdown skipped: missing table");
            }
            Err(err) => return Err(err.into()),
        }

        let (purchased_hours_total, used_hours_total) = match conn.query_one(
            "SELECT
                COALESCE(SUM(purchased_hours::float8), 0),
                COALESCE(SUM(tokens_to_hours::float8), 0)
             FROM accounts",
            &[],
        ) {
            Ok(row) => (row.get(0), row.get(1)),
            Err(err) if err.code() == Some(&postgres::error::SqlState::UNDEFINED_TABLE) => {
                warn!("account_store metrics usage totals skipped: missing accounts table");
                (0.0, 0.0)
            }
            Err(err) => return Err(err.into()),
        };

        Ok(AccountMetricsSnapshot {
            accounts_total,
            identifiers_total,
            verified_identifiers_total,
            identifiers_by_type,
            purchased_hours_total,
            used_hours_total,
            payments_total,
            payments_last_24h,
        })
    }

    // =========================================================================
    // Billing methods
    // =========================================================================

    /// Get balance info for an account
    pub fn get_balance(&self, account_id: Uuid) -> Result<BalanceInfo, AccountStoreError> {
        let account = self
            .get_account(account_id)?
            .ok_or(AccountStoreError::NotFound)?;

        let purchased = account.purchased_hours.unwrap_or(0.0);
        let used = account.tokens_to_hours.unwrap_or(0.0);

        Ok(BalanceInfo {
            purchased_hours: purchased,
            used_hours: used,
            balance_hours: purchased - used,
        })
    }

    /// Check if account has sufficient balance (allows 1 hour grace)
    pub fn has_sufficient_balance(&self, account_id: Uuid) -> Result<bool, AccountStoreError> {
        let balance = self.get_balance(account_id)?;
        // Block if used_hours exceeds purchased_hours + 1 (1 hour grace)
        Ok(balance.used_hours <= balance.purchased_hours + 1.0)
    }

    /// Add purchased hours to an account (called after successful payment)
    pub fn add_purchased_hours(
        &self,
        account_id: Uuid,
        hours: f64,
    ) -> Result<(), AccountStoreError> {
        let mut conn = self.conn()?;
        conn.execute(
            "UPDATE accounts
             SET purchased_hours = COALESCE(purchased_hours, 0) + $1::float8
             WHERE id = $2",
            &[&hours, &account_id],
        )?;
        Ok(())
    }

    /// Record a payment in the audit table (stripe_session_id is primary key)
    pub fn record_payment(
        &self,
        account_id: Uuid,
        stripe_session_id: &str,
        amount_cents: i32,
        hours: f64,
    ) -> Result<(), AccountStoreError> {
        let mut conn = self.conn()?;
        // Use ON CONFLICT DO NOTHING for idempotency
        conn.execute(
            "INSERT INTO payments (stripe_session_id, account_id, amount_cents, hours_purchased, created_at)
             VALUES ($1, $2, $3, $4::float8, NOW())
             ON CONFLICT (stripe_session_id) DO NOTHING",
            &[&stripe_session_id, &account_id, &amount_cents, &hours],
        )?;
        Ok(())
    }

    /// Check if a payment has already been recorded (for idempotency)
    pub fn payment_exists(&self, stripe_session_id: &str) -> Result<bool, AccountStoreError> {
        let mut conn = self.conn()?;
        let row = conn.query_opt(
            "SELECT stripe_session_id FROM payments WHERE stripe_session_id = $1",
            &[&stripe_session_id],
        )?;
        Ok(row.is_some())
    }

    /// Create an email verification token (expires in 24 hours)
    pub fn create_email_verification_token(
        &self,
        account_id: Uuid,
        email: &str,
    ) -> Result<EmailVerificationToken, AccountStoreError> {
        let mut conn = self.conn()?;
        let token = Uuid::new_v4().to_string();
        let expires_at = Utc::now() + chrono::Duration::hours(24);

        // Delete any existing tokens for this email
        conn.execute(
            "DELETE FROM email_verification_tokens WHERE email = $1",
            &[&email],
        )?;

        let row = conn.query_one(
            "INSERT INTO email_verification_tokens (token, account_id, email, expires_at, created_at)
             VALUES ($1, $2, $3, $4, NOW())
             RETURNING token, account_id, email, expires_at, created_at",
            &[&token, &account_id, &email, &expires_at],
        )?;

        Ok(EmailVerificationToken {
            token: row.get(0),
            account_id: row.get(1),
            email: row.get(2),
            expires_at: row.get(3),
            created_at: row.get(4),
        })
    }

    /// Verify an email token and link the email to the account
    pub fn verify_email_token(&self, token: &str) -> Result<AccountIdentifier, AccountStoreError> {
        let normalized_token = token.trim();
        if normalized_token.is_empty() {
            return Err(AccountStoreError::TokenInvalid);
        }
        if Uuid::parse_str(normalized_token).is_err() {
            return Err(AccountStoreError::TokenInvalid);
        }
        let mut conn = self.conn()?;

        // Look up the token
        let row = conn.query_opt(
            "SELECT token, account_id, email, expires_at
             FROM email_verification_tokens
             WHERE token::text = $1",
            &[&normalized_token],
        )?;

        let verification = match row {
            Some(r) => EmailVerificationToken {
                token: r.get(0),
                account_id: r.get(1),
                email: r.get(2),
                expires_at: r.get(3),
                created_at: Utc::now(), // Not needed for verification
            },
            None => return Err(AccountStoreError::TokenInvalid),
        };

        // Check if expired
        if Utc::now() > verification.expires_at {
            // Delete expired token
            conn.execute(
                "DELETE FROM email_verification_tokens WHERE token::text = $1",
                &[&normalized_token],
            )?;
            return Err(AccountStoreError::TokenInvalid);
        }

        // Create or update the identifier as verified
        let id = Uuid::new_v4();
        let row = conn.query_one(
            "INSERT INTO account_identifiers (id, account_id, identifier_type, identifier, verified, created_at)
             VALUES ($1, $2, 'email', $3, true, NOW())
             ON CONFLICT (identifier_type, identifier) DO UPDATE SET account_id = $2, verified = true
             RETURNING id, account_id, identifier_type, identifier, verified, created_at",
            &[&id, &verification.account_id, &verification.email],
        )?;

        // Delete the used token
        conn.execute(
            "DELETE FROM email_verification_tokens WHERE token::text = $1",
            &[&normalized_token],
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
}

impl Drop for AccountStore {
    fn drop(&mut self) {
        let primary_pool = self.primary_pool.take();
        let fallback_pool = self.fallback_pool.take();
        if primary_pool.is_some() || fallback_pool.is_some() {
            std::thread::spawn(move || {
                drop(primary_pool);
                drop(fallback_pool);
            });
        }
    }
}

// ============================================================================
// Global AccountStore accessor
// ============================================================================

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
        Channel::GoogleDocs | Channel::GoogleSheets | Channel::GoogleSlides => "email",
    }
}

fn identifier_lookup_candidates(identifier_type: &str, identifier: &str) -> Vec<String> {
    let trimmed = identifier.trim();
    if trimmed.is_empty() {
        return Vec::new();
    }

    let mut candidates = vec![trimmed.to_string()];
    match identifier_type {
        "email" | "github" => {
            let lower = trimmed.to_ascii_lowercase();
            if lower != trimmed {
                candidates.push(lower);
            }
        }
        "slack" => {
            let upper = trimmed.to_ascii_uppercase();
            if upper != trimmed {
                candidates.push(upper);
            }
        }
        "phone" => {
            if let Some(normalized) = crate::user_store::normalize_phone(trimmed) {
                if normalized != trimmed {
                    candidates.push(normalized);
                }
            }
        }
        _ => {}
    }

    let mut seen = std::collections::HashSet::new();
    candidates
        .into_iter()
        .filter(|value| seen.insert(value.clone()))
        .collect()
}

/// Look up account by identifier type and identifier.
/// Returns the account_id if found and verified, None otherwise.
pub fn lookup_account_by_identifier(identifier_type: &str, identifier: &str) -> Option<Uuid> {
    let store = get_global_account_store()?;
    let candidates = identifier_lookup_candidates(identifier_type, identifier);
    if candidates.is_empty() {
        return None;
    }

    for candidate in candidates {
        match store.get_account_by_identifier(identifier_type, &candidate) {
            Ok(Some(account)) => {
                tracing::debug!(
                    "Found account {} for {}:{}",
                    account.id,
                    identifier_type,
                    candidate
                );
                return Some(account.id);
            }
            Ok(None) => {
                continue;
            }
            Err(e) => {
                tracing::warn!(
                    "Error looking up account for {}:{}: {}",
                    identifier_type,
                    candidate,
                    e
                );
            }
        }
    }

    tracing::debug!(
        "No account found for {}:{}, using local storage",
        identifier_type,
        identifier
    );
    None
}

/// Look up account by channel and identifier
/// Returns the account_id if found and verified, None otherwise
pub fn lookup_account_by_channel(
    channel: &crate::channel::Channel,
    identifier: &str,
) -> Option<Uuid> {
    let identifier_type = channel_to_identifier_type(channel);
    lookup_account_by_identifier(identifier_type, identifier)
}

#[cfg(test)]
mod tests {
    use super::account_store_allow_invalid_certs;
    use std::env;
    use std::sync::{Mutex, OnceLock};

    fn env_lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

    fn reset_tls_env() {
        env::remove_var("DEPLOY_TARGET");
        env::remove_var("INGESTION_QUEUE_TLS_ALLOW_INVALID_CERTS");
        env::remove_var("ACCOUNT_STORE_TLS_ALLOW_INVALID_CERTS");
        env::remove_var("SCALE_OLIVER_INGESTION_QUEUE_TLS_ALLOW_INVALID_CERTS");
        env::remove_var("SCALE_OLIVER_ACCOUNT_STORE_TLS_ALLOW_INVALID_CERTS");
    }

    #[test]
    fn account_store_tls_defaults_enabled_on_staging() {
        let _guard = env_lock().lock().expect("env lock");
        reset_tls_env();
        env::set_var("DEPLOY_TARGET", "staging");
        assert!(account_store_allow_invalid_certs());
        reset_tls_env();
    }

    #[test]
    fn account_store_tls_defaults_disabled_on_production() {
        let _guard = env_lock().lock().expect("env lock");
        reset_tls_env();
        env::set_var("DEPLOY_TARGET", "production");
        assert!(!account_store_allow_invalid_certs());
        reset_tls_env();
    }

    #[test]
    fn account_store_tls_respects_explicit_override() {
        let _guard = env_lock().lock().expect("env lock");
        reset_tls_env();
        env::set_var("DEPLOY_TARGET", "staging");
        env::set_var("ACCOUNT_STORE_TLS_ALLOW_INVALID_CERTS", "0");
        assert!(!account_store_allow_invalid_certs());
        env::set_var("ACCOUNT_STORE_TLS_ALLOW_INVALID_CERTS", "1");
        assert!(account_store_allow_invalid_certs());
        reset_tls_env();
    }
}
