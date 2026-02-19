use std::collections::HashMap;
use std::env;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use axum::http::HeaderMap;
use base64::Engine;
use hmac::{Hmac, Mac};
use sha1::Sha1;
use sha2::Sha256;

pub(super) fn verify_slack(headers: &HeaderMap, body: &[u8]) -> Result<(), &'static str> {
    let secret = env::var("SLACK_SIGNING_SECRET").ok();
    let Some(secret) = secret.filter(|value| !value.trim().is_empty()) else {
        return Ok(());
    };
    let signature = headers
        .get("x-slack-signature")
        .and_then(|value| value.to_str().ok())
        .ok_or("missing_signature")?;
    let timestamp = headers
        .get("x-slack-request-timestamp")
        .and_then(|value| value.to_str().ok())
        .ok_or("missing_timestamp")?;
    let timestamp_value: i64 = timestamp.parse().map_err(|_| "invalid_timestamp")?;

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::from_secs(0))
        .as_secs() as i64;
    if (now - timestamp_value).abs() > 60 * 5 {
        return Err("stale_timestamp");
    }

    let base = format!("v0:{}:{}", timestamp, String::from_utf8_lossy(body));
    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).map_err(|_| "bad_secret")?;
    mac.update(base.as_bytes());
    let expected = format!("v0={}", hex::encode(mac.finalize().into_bytes()));
    if expected != signature {
        return Err("invalid_signature");
    }
    Ok(())
}

pub(super) fn verify_postmark(headers: &HeaderMap) -> Result<(), &'static str> {
    let token = env::var("POSTMARK_INBOUND_TOKEN").ok();
    let Some(token) = token.filter(|value| !value.trim().is_empty()) else {
        return Ok(());
    };
    let header = headers
        .get("x-postmark-token")
        .and_then(|value| value.to_str().ok())
        .ok_or("missing_token")?;
    if header != token {
        return Err("invalid_token");
    }
    Ok(())
}

pub(super) fn verify_bluebubbles(headers: &HeaderMap) -> Result<(), &'static str> {
    let token = env::var("BLUEBUBBLES_WEBHOOK_TOKEN").ok();
    let Some(token) = token.filter(|value| !value.trim().is_empty()) else {
        return Ok(());
    };
    let header = headers
        .get("x-bluebubbles-token")
        .and_then(|value| value.to_str().ok())
        .ok_or("missing_token")?;
    if header != token {
        return Err("invalid_token");
    }
    Ok(())
}

pub(super) fn verify_twilio(headers: &HeaderMap, body: &[u8]) -> Result<(), &'static str> {
    let token = env::var("TWILIO_AUTH_TOKEN").ok();
    let url = env::var("TWILIO_WEBHOOK_URL").ok();
    let (Some(token), Some(url)) = (token, url) else {
        return Ok(());
    };
    if token.trim().is_empty() || url.trim().is_empty() {
        return Ok(());
    }
    let signature = headers
        .get("x-twilio-signature")
        .and_then(|value| value.to_str().ok())
        .ok_or("missing_signature")?;

    let params: HashMap<String, String> =
        serde_urlencoded::from_bytes(body).map_err(|_| "bad_form")?;
    let mut keys: Vec<_> = params.keys().cloned().collect();
    keys.sort();
    let mut data = url.clone();
    for key in keys {
        if let Some(value) = params.get(&key) {
            data.push_str(&key);
            data.push_str(value);
        }
    }

    let mut mac = Hmac::<Sha1>::new_from_slice(token.as_bytes()).map_err(|_| "bad_secret")?;
    mac.update(data.as_bytes());
    let expected = base64::engine::general_purpose::STANDARD.encode(mac.finalize().into_bytes());

    if expected != signature {
        return Err("invalid_signature");
    }
    Ok(())
}

/// Verify WhatsApp webhook subscription request.
/// Returns the challenge token if verification succeeds.
pub(super) fn verify_whatsapp_subscription(
    mode: Option<&str>,
    token: Option<&str>,
    challenge: Option<&str>,
) -> Result<String, &'static str> {
    let expected_token = env::var("WHATSAPP_VERIFY_TOKEN").ok();
    let Some(expected) = expected_token.filter(|value| !value.trim().is_empty()) else {
        return Err("verify_token_not_configured");
    };

    if mode != Some("subscribe") {
        return Err("invalid_mode");
    }

    let provided_token = token.ok_or("missing_token")?;
    if provided_token != expected {
        return Err("token_mismatch");
    }

    challenge.map(|c| c.to_string()).ok_or("missing_challenge")
}
