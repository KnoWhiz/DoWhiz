use axum::body::Bytes;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Arc;
use stripe::{
    CheckoutSession, CheckoutSessionMode, Client, CreateCheckoutSession,
    CreateCheckoutSessionLineItems, CreateCheckoutSessionLineItemsPriceData,
    CreateCheckoutSessionLineItemsPriceDataProductData, Currency, EventObject, EventType, Webhook,
};
use tracing::{error, info, warn};
use uuid::Uuid;

use crate::account_store::AccountStore;

use super::auth::{extract_bearer_token, validate_supabase_token};

/// State for billing routes
#[derive(Clone)]
pub struct BillingState {
    pub account_store: Arc<AccountStore>,
    pub stripe_client: Client,
    pub webhook_secret: String,
    pub supabase_url: String,
    pub frontend_url: String,
}

impl BillingState {
    pub fn from_env(account_store: Arc<AccountStore>) -> Option<Self> {
        let stripe_secret = env::var("STRIPE_SECRET_KEY").ok()?;
        let webhook_secret = env::var("STRIPE_WEBHOOK_SECRET").ok()?;
        let supabase_url = env::var("SUPABASE_PROJECT_URL")
            .unwrap_or_else(|_| "https://resmseutzmwumflevfqw.supabase.co".to_string());
        let frontend_url =
            env::var("FRONTEND_URL").unwrap_or_else(|_| "https://dowhiz.com".to_string());

        Some(Self {
            account_store,
            stripe_client: Client::new(stripe_secret),
            webhook_secret,
            supabase_url,
            frontend_url,
        })
    }
}

// ============================================================================
// Request/Response types
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct CheckoutRequest {
    pub hours: u32,
}

#[derive(Debug, Serialize)]
pub struct CheckoutResponse {
    pub checkout_url: String,
}

#[derive(Debug, Serialize)]
pub struct BalanceResponse {
    pub purchased_hours: f64,
    pub used_hours: f64,
    pub balance_hours: f64,
}

// ============================================================================
// GET /billing/balance
// ============================================================================

async fn get_balance(
    State(state): State<BillingState>,
    headers: HeaderMap,
) -> Result<Json<BalanceResponse>, (StatusCode, String)> {
    // Extract and validate token
    let token = extract_bearer_token(&headers)
        .ok_or((StatusCode::UNAUTHORIZED, "Missing Authorization header".to_string()))?;

    let auth_user = validate_supabase_token(&state.supabase_url, &token).await?;

    // Get account - run sync DB operation on blocking thread
    let store = state.account_store.clone();
    let auth_user_id = auth_user.id;
    let account = tokio::task::spawn_blocking(move || {
        store.get_account_by_auth_user(auth_user_id)
    })
    .await
    .map_err(|e| {
        error!("Task join error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Internal error".to_string())
    })?
    .map_err(|e| {
        error!("Failed to get account: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string())
    })?
    .ok_or((StatusCode::NOT_FOUND, "Account not found".to_string()))?;

    // Get balance - run sync DB operation on blocking thread
    let store = state.account_store.clone();
    let account_id = account.id;
    let balance = tokio::task::spawn_blocking(move || {
        store.get_balance(account_id)
    })
    .await
    .map_err(|e| {
        error!("Task join error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Internal error".to_string())
    })?
    .map_err(|e| {
        error!("Failed to get balance: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string())
    })?;

    Ok(Json(BalanceResponse {
        purchased_hours: balance.purchased_hours,
        used_hours: balance.used_hours,
        balance_hours: balance.balance_hours,
    }))
}

// ============================================================================
// POST /billing/checkout
// ============================================================================

async fn create_checkout(
    State(state): State<BillingState>,
    headers: HeaderMap,
    Json(payload): Json<CheckoutRequest>,
) -> Result<Json<CheckoutResponse>, (StatusCode, String)> {
    // Validate hours
    if payload.hours == 0 {
        return Err((StatusCode::BAD_REQUEST, "Hours must be greater than 0".to_string()));
    }

    // Extract and validate token
    let token = extract_bearer_token(&headers)
        .ok_or((StatusCode::UNAUTHORIZED, "Missing Authorization header".to_string()))?;

    let auth_user = validate_supabase_token(&state.supabase_url, &token).await?;

    // Get account - run sync DB operation on blocking thread
    let store = state.account_store.clone();
    let auth_user_id = auth_user.id;
    let account = tokio::task::spawn_blocking(move || {
        store.get_account_by_auth_user(auth_user_id)
    })
    .await
    .map_err(|e| {
        error!("Task join error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Internal error".to_string())
    })?
    .map_err(|e| {
        error!("Failed to get account: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string())
    })?
    .ok_or((StatusCode::NOT_FOUND, "Account not found".to_string()))?;

    // Calculate price in cents ($10/hr)
    let amount_cents = (payload.hours as i64) * 1000; // $10 = 1000 cents

    // Create Stripe checkout session
    let success_url = format!("{}/auth/?payment=success", state.frontend_url);
    let cancel_url = format!("{}/auth/?payment=cancelled", state.frontend_url);

    let mut params = CreateCheckoutSession::new();
    params.mode = Some(CheckoutSessionMode::Payment);
    params.success_url = Some(&success_url);
    params.cancel_url = Some(&cancel_url);

    // Create line item with custom price
    let line_item = CreateCheckoutSessionLineItems {
        price_data: Some(CreateCheckoutSessionLineItemsPriceData {
            currency: Currency::USD,
            unit_amount: Some(amount_cents),
            product_data: Some(CreateCheckoutSessionLineItemsPriceDataProductData {
                name: format!("{} hours of DoWhiz employee time", payload.hours),
                ..Default::default()
            }),
            ..Default::default()
        }),
        quantity: Some(1),
        ..Default::default()
    };
    params.line_items = Some(vec![line_item]);

    // Store account_id in metadata for webhook
    let mut metadata = std::collections::HashMap::new();
    metadata.insert("account_id".to_string(), account.id.to_string());
    metadata.insert("hours".to_string(), payload.hours.to_string());
    params.metadata = Some(metadata);

    let session: CheckoutSession = CheckoutSession::create(&state.stripe_client, params)
        .await
        .map_err(|e| {
            error!("Failed to create Stripe checkout session: {}", e);
            (StatusCode::BAD_GATEWAY, "Failed to create checkout session".to_string())
        })?;

    let checkout_url = session.url.ok_or_else(|| {
        error!("Stripe session missing URL");
        (StatusCode::BAD_GATEWAY, "Invalid checkout session".to_string())
    })?;

    info!(
        "Created checkout session for account {} - {} hours (${:.2})",
        account.id,
        payload.hours,
        amount_cents as f64 / 100.0
    );

    Ok(Json(CheckoutResponse { checkout_url }))
}

// ============================================================================
// POST /billing/webhook
// ============================================================================

async fn handle_webhook(
    State(state): State<BillingState>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<StatusCode, (StatusCode, String)> {
    // Get Stripe signature header
    let signature = headers
        .get("stripe-signature")
        .and_then(|v| v.to_str().ok())
        .ok_or((StatusCode::BAD_REQUEST, "Missing Stripe signature".to_string()))?;

    // Verify webhook signature
    let payload = std::str::from_utf8(&body).map_err(|_| {
        (StatusCode::BAD_REQUEST, "Invalid payload encoding".to_string())
    })?;

    let event = Webhook::construct_event(payload, signature, &state.webhook_secret).map_err(|e| {
        warn!("Webhook signature verification failed: {}", e);
        (StatusCode::BAD_REQUEST, "Invalid signature".to_string())
    })?;

    // Only handle checkout.session.completed
    if event.type_ != EventType::CheckoutSessionCompleted {
        info!("Ignoring webhook event type: {:?}", event.type_);
        return Ok(StatusCode::OK);
    }

    // Extract checkout session from event
    let session = match event.data.object {
        EventObject::CheckoutSession(session) => session,
        _ => {
            warn!("Unexpected event object type");
            return Ok(StatusCode::OK);
        }
    };

    let session_id = session.id.as_str().to_string();

    // Check idempotency - skip if already processed (run on blocking thread)
    let store = state.account_store.clone();
    let session_id_clone = session_id.clone();
    let payment_exists = tokio::task::spawn_blocking(move || {
        store.payment_exists(&session_id_clone)
    })
    .await
    .map_err(|e| {
        error!("Task join error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Internal error".to_string())
    })?
    .map_err(|e| {
        error!("Failed to check payment existence: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string())
    })?;

    if payment_exists {
        info!("Payment {} already processed, skipping", session_id);
        return Ok(StatusCode::OK);
    }

    // Extract metadata
    let metadata = session.metadata.as_ref().ok_or_else(|| {
        error!("Checkout session missing metadata");
        (StatusCode::BAD_REQUEST, "Missing metadata".to_string())
    })?;

    let account_id_str = metadata.get("account_id").ok_or_else(|| {
        error!("Metadata missing account_id");
        (StatusCode::BAD_REQUEST, "Missing account_id in metadata".to_string())
    })?;

    let account_id: Uuid = account_id_str.parse().map_err(|_| {
        error!("Invalid account_id in metadata: {}", account_id_str);
        (StatusCode::BAD_REQUEST, "Invalid account_id".to_string())
    })?;

    let hours_str = metadata.get("hours").ok_or_else(|| {
        error!("Metadata missing hours");
        (StatusCode::BAD_REQUEST, "Missing hours in metadata".to_string())
    })?;

    let hours: f64 = hours_str.parse().map_err(|_| {
        error!("Invalid hours in metadata: {}", hours_str);
        (StatusCode::BAD_REQUEST, "Invalid hours".to_string())
    })?;

    // Get amount from session
    let amount_cents = session.amount_total.unwrap_or(0) as i32;

    // Record payment (idempotent insert) - run on blocking thread
    let store = state.account_store.clone();
    let session_id_for_record = session_id.clone();
    tokio::task::spawn_blocking(move || {
        store.record_payment(account_id, &session_id_for_record, amount_cents, hours)
    })
    .await
    .map_err(|e| {
        error!("Task join error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Internal error".to_string())
    })?
    .map_err(|e| {
        error!("Failed to record payment: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Failed to record payment".to_string())
    })?;

    // Add purchased hours to account - run on blocking thread
    let store = state.account_store.clone();
    tokio::task::spawn_blocking(move || {
        store.add_purchased_hours(account_id, hours)
    })
    .await
    .map_err(|e| {
        error!("Task join error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Internal error".to_string())
    })?
    .map_err(|e| {
        error!("Failed to add purchased hours: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update account".to_string())
    })?;

    info!(
        "Payment {} processed: {} hours added to account {}",
        session_id, hours, account_id
    );

    Ok(StatusCode::OK)
}

// ============================================================================
// Router
// ============================================================================

pub fn billing_router(state: BillingState) -> Router {
    Router::new()
        .route("/billing/balance", get(get_balance))
        .route("/billing/checkout", post(create_checkout))
        .route("/billing/webhook", post(handle_webhook))
        .with_state(state)
}
