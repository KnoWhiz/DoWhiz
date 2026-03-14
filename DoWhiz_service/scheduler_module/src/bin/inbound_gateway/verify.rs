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

/// Verify WeChat webhook callback URL.
/// Returns the echostr if verification succeeds.
/// WeChat sends: GET /wechat/webhook?msg_signature=xxx&timestamp=xxx&nonce=xxx&echostr=xxx
pub(super) fn verify_wechat(
    msg_signature: Option<&str>,
    timestamp: Option<&str>,
    nonce: Option<&str>,
    echostr: Option<&str>,
) -> Result<String, &'static str> {
    let token = env::var("WECHAT_TOKEN").ok();
    let Some(token) = token.filter(|value| !value.trim().is_empty()) else {
        // If token not configured, just return echostr (allows testing)
        return echostr.map(|e| e.to_string()).ok_or("missing_echostr");
    };

    let signature = msg_signature.ok_or("missing_signature")?;
    let timestamp = timestamp.ok_or("missing_timestamp")?;
    let nonce = nonce.ok_or("missing_nonce")?;
    let echostr = echostr.ok_or("missing_echostr")?;

    // WeChat signature: SHA1(sort([token, timestamp, nonce, echostr]))
    // When encryption is enabled (EncodingAESKey configured), echostr is included
    let mut parts = vec![token.as_str(), timestamp, nonce, echostr];
    parts.sort();
    let data = parts.join("");

    use sha1::{Digest, Sha1};
    let mut hasher = Sha1::new();
    hasher.update(data.as_bytes());
    let result = hasher.finalize();
    let expected = hex::encode(result);

    if expected != signature {
        return Err("invalid_signature");
    }

    Ok(echostr.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use sha1::{Digest, Sha1};

    // ==================== WeChat Verification Tests ====================

    #[test]
    fn verify_wechat_returns_echostr_when_no_token_configured() {
        // When WECHAT_TOKEN is not set, should just return echostr
        std::env::remove_var("WECHAT_TOKEN");

        let result = verify_wechat(
            Some("signature"),
            Some("1234567890"),
            Some("nonce"),
            Some("test_echostr"),
        );

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test_echostr");
    }

    #[test]
    fn verify_wechat_returns_error_when_missing_echostr_no_token() {
        std::env::remove_var("WECHAT_TOKEN");

        let result = verify_wechat(Some("sig"), Some("ts"), Some("nonce"), None);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "missing_echostr");
    }

    #[test]
    fn verify_wechat_validates_signature_when_token_set() {
        let token = "test_token_12345";
        std::env::set_var("WECHAT_TOKEN", token);

        let timestamp = "1609459200";
        let nonce = "random_nonce";
        let echostr = "challenge_string";

        // Calculate the expected signature: SHA1(sort([token, timestamp, nonce, echostr]))
        let mut parts = vec![token, timestamp, nonce, echostr];
        parts.sort();
        let data = parts.join("");
        let mut hasher = Sha1::new();
        hasher.update(data.as_bytes());
        let valid_signature = hex::encode(hasher.finalize());

        let result = verify_wechat(
            Some(&valid_signature),
            Some(timestamp),
            Some(nonce),
            Some(echostr),
        );

        // Clean up env var
        std::env::remove_var("WECHAT_TOKEN");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "challenge_string");
    }

    #[test]
    fn verify_wechat_rejects_invalid_signature() {
        std::env::set_var("WECHAT_TOKEN", "secret_token");

        let result = verify_wechat(
            Some("invalid_signature"),
            Some("1234567890"),
            Some("nonce123"),
            Some("echostr"),
        );

        std::env::remove_var("WECHAT_TOKEN");

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "invalid_signature");
    }

    #[test]
    fn verify_wechat_requires_signature_when_token_set() {
        std::env::set_var("WECHAT_TOKEN", "token");

        let result = verify_wechat(None, Some("ts"), Some("nonce"), Some("echo"));

        std::env::remove_var("WECHAT_TOKEN");

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "missing_signature");
    }

    #[test]
    fn verify_wechat_requires_timestamp_when_token_set() {
        std::env::set_var("WECHAT_TOKEN", "token");

        let result = verify_wechat(Some("sig"), None, Some("nonce"), Some("echo"));

        std::env::remove_var("WECHAT_TOKEN");

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "missing_timestamp");
    }

    #[test]
    fn verify_wechat_requires_nonce_when_token_set() {
        std::env::set_var("WECHAT_TOKEN", "token");

        let result = verify_wechat(Some("sig"), Some("ts"), None, Some("echo"));

        std::env::remove_var("WECHAT_TOKEN");

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "missing_nonce");
    }

    #[test]
    fn verify_wechat_requires_echostr_when_token_set() {
        std::env::set_var("WECHAT_TOKEN", "token");

        let result = verify_wechat(Some("sig"), Some("ts"), Some("nonce"), None);

        std::env::remove_var("WECHAT_TOKEN");

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "missing_echostr");
    }

    #[test]
    fn verify_wechat_ignores_empty_token() {
        std::env::set_var("WECHAT_TOKEN", "   ");

        let result = verify_wechat(
            Some("any_signature"),
            Some("ts"),
            Some("nonce"),
            Some("echostr"),
        );

        std::env::remove_var("WECHAT_TOKEN");

        // Empty token is treated as not configured, so should pass
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "echostr");
    }

    #[test]
    fn verify_wechat_signature_sort_order() {
        // Test that the signature algorithm sorts parts correctly
        // This is critical: WeChat sorts [token, timestamp, nonce, echostr] alphabetically
        let token = "zzz_token";
        let timestamp = "aaa_timestamp";
        let nonce = "mmm_nonce";
        let echostr = "bbb_echo";

        std::env::set_var("WECHAT_TOKEN", token);

        // Sorted: ["aaa_timestamp", "bbb_echo", "mmm_nonce", "zzz_token"]
        let mut parts = vec![token, timestamp, nonce, echostr];
        parts.sort();
        assert_eq!(parts, vec!["aaa_timestamp", "bbb_echo", "mmm_nonce", "zzz_token"]);

        let data = parts.join("");
        let mut hasher = Sha1::new();
        hasher.update(data.as_bytes());
        let valid_signature = hex::encode(hasher.finalize());

        let result = verify_wechat(
            Some(&valid_signature),
            Some(timestamp),
            Some(nonce),
            Some(echostr),
        );

        std::env::remove_var("WECHAT_TOKEN");

        assert!(result.is_ok());
    }

    // ==================== WhatsApp Verification Tests ====================

    #[test]
    fn verify_whatsapp_subscription_requires_subscribe_mode() {
        std::env::set_var("WHATSAPP_VERIFY_TOKEN", "my_token");

        let result = verify_whatsapp_subscription(
            Some("webhook"), // wrong mode
            Some("my_token"),
            Some("challenge123"),
        );

        std::env::remove_var("WHATSAPP_VERIFY_TOKEN");

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "invalid_mode");
    }

    #[test]
    fn verify_whatsapp_subscription_validates_token() {
        std::env::set_var("WHATSAPP_VERIFY_TOKEN", "correct_token");

        let result = verify_whatsapp_subscription(
            Some("subscribe"),
            Some("wrong_token"),
            Some("challenge"),
        );

        std::env::remove_var("WHATSAPP_VERIFY_TOKEN");

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "token_mismatch");
    }

    #[test]
    fn verify_whatsapp_subscription_success() {
        std::env::set_var("WHATSAPP_VERIFY_TOKEN", "my_secret_token");

        let result = verify_whatsapp_subscription(
            Some("subscribe"),
            Some("my_secret_token"),
            Some("hub.challenge.12345"),
        );

        std::env::remove_var("WHATSAPP_VERIFY_TOKEN");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "hub.challenge.12345");
    }
}
