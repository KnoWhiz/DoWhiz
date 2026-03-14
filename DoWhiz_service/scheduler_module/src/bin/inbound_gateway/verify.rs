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
/// Returns the decrypted echostr if verification succeeds.
/// WeChat sends: GET /wechat/webhook?msg_signature=xxx&timestamp=xxx&nonce=xxx&echostr=xxx
///
/// When EncodingAESKey is configured:
/// - echostr is encrypted and must be decrypted
/// - Signature = SHA1(sort([token, timestamp, nonce, echostr]))
/// - Decrypt echostr using AES-256-CBC
pub(super) fn verify_wechat(
    msg_signature: Option<&str>,
    timestamp: Option<&str>,
    nonce: Option<&str>,
    echostr: Option<&str>,
) -> Result<String, &'static str> {
    let token = env::var("WECHAT_TOKEN").ok();
    let encoding_aes_key = env::var("WECHAT_ENCODING_AES_KEY").ok();

    let Some(token) = token.filter(|value| !value.trim().is_empty()) else {
        // If token not configured, just return echostr (allows testing)
        return echostr.map(|e| e.to_string()).ok_or("missing_echostr");
    };

    let signature = msg_signature.ok_or("missing_signature")?;
    let timestamp = timestamp.ok_or("missing_timestamp")?;
    let nonce = nonce.ok_or("missing_nonce")?;
    let echostr = echostr.ok_or("missing_echostr")?;

    // WeChat signature: SHA1(sort([token, timestamp, nonce, echostr]))
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

    // If EncodingAESKey is configured, decrypt the echostr
    if let Some(aes_key_str) = encoding_aes_key.filter(|v| !v.trim().is_empty()) {
        return decrypt_wechat_echostr(echostr, &aes_key_str);
    }

    Ok(echostr.to_string())
}

/// Decrypt WeChat echostr using AES-256-CBC.
/// AESKey = Base64_Decode(EncodingAESKey + "=")
/// IV = first 16 bytes of AESKey
/// Message format after decryption: random(16B) + msg_len(4B, big endian) + msg + receiveid
fn decrypt_wechat_echostr(echostr: &str, encoding_aes_key: &str) -> Result<String, &'static str> {
    use aes::cipher::{block_padding::NoPadding, BlockDecryptMut, KeyIvInit};
    use base64::engine::{GeneralPurpose, GeneralPurposeConfig, DecodePaddingMode};
    type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;

    // Derive AESKey: Base64_Decode(EncodingAESKey + "=")
    // Use lenient decoder because WeChat's EncodingAESKey may have non-zero trailing bits
    let trimmed_key = encoding_aes_key.trim();
    let aes_key_b64 = format!("{}=", trimmed_key);

    let lenient_engine = GeneralPurpose::new(
        &base64::alphabet::STANDARD,
        GeneralPurposeConfig::new()
            .with_decode_padding_mode(DecodePaddingMode::Indifferent)
            .with_decode_allow_trailing_bits(true),
    );
    let aes_key = lenient_engine
        .decode(&aes_key_b64)
        .map_err(|e| {
            tracing::error!("base64 decode error: {:?}", e);
            "invalid_encoding_aes_key"
        })?;

    if aes_key.len() != 32 {
        return Err("invalid_aes_key_length");
    }

    // Decode the encrypted echostr from Base64
    let encrypted = base64::engine::general_purpose::STANDARD
        .decode(echostr)
        .map_err(|_| "invalid_echostr_base64")?;

    // IV is first 16 bytes of AESKey
    let iv: [u8; 16] = aes_key[..16].try_into().map_err(|_| "iv_error")?;
    let key: [u8; 32] = aes_key.try_into().map_err(|_| "key_error")?;

    // Decrypt using AES-256-CBC
    let mut buf = encrypted.clone();
    let decryptor = Aes256CbcDec::new(&key.into(), &iv.into());
    let decrypted = decryptor
        .decrypt_padded_mut::<NoPadding>(&mut buf)
        .map_err(|_| "decryption_failed")?;

    // Remove PKCS#7 padding
    let decrypted = remove_pkcs7_padding(decrypted)?;

    // Message format: random(16B) + msg_len(4B) + msg + receiveid
    if decrypted.len() < 20 {
        return Err("decrypted_too_short");
    }

    // Skip 16 random bytes
    let content = &decrypted[16..];

    // Read msg_len (4 bytes, big endian / network byte order)
    let msg_len = u32::from_be_bytes(
        content[0..4]
            .try_into()
            .map_err(|_| "msg_len_parse_error")?,
    ) as usize;

    if content.len() < 4 + msg_len {
        return Err("msg_length_mismatch");
    }

    // Extract the message
    let msg = &content[4..4 + msg_len];

    String::from_utf8(msg.to_vec()).map_err(|_| "invalid_utf8")
}

/// Remove PKCS#7 padding from decrypted data
fn remove_pkcs7_padding(data: &[u8]) -> Result<&[u8], &'static str> {
    if data.is_empty() {
        return Err("empty_data");
    }
    let padding_len = data[data.len() - 1] as usize;
    if padding_len == 0 || padding_len > 32 || padding_len > data.len() {
        return Err("invalid_padding");
    }
    // Verify all padding bytes are correct
    for &byte in &data[data.len() - padding_len..] {
        if byte as usize != padding_len {
            return Err("invalid_padding_bytes");
        }
    }
    Ok(&data[..data.len() - padding_len])
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
        std::env::remove_var("WECHAT_ENCODING_AES_KEY");

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
        std::env::remove_var("WECHAT_ENCODING_AES_KEY");

        let result = verify_wechat(Some("sig"), Some("ts"), Some("nonce"), None);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "missing_echostr");
    }

    #[test]
    fn verify_wechat_validates_signature_when_token_set() {
        let token = "test_token_12345";
        std::env::set_var("WECHAT_TOKEN", token);
        std::env::remove_var("WECHAT_ENCODING_AES_KEY");

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
        std::env::remove_var("WECHAT_ENCODING_AES_KEY");

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
        std::env::remove_var("WECHAT_ENCODING_AES_KEY");

        let result = verify_wechat(None, Some("ts"), Some("nonce"), Some("echo"));

        std::env::remove_var("WECHAT_TOKEN");

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "missing_signature");
    }

    #[test]
    fn verify_wechat_requires_timestamp_when_token_set() {
        std::env::set_var("WECHAT_TOKEN", "token");
        std::env::remove_var("WECHAT_ENCODING_AES_KEY");

        let result = verify_wechat(Some("sig"), None, Some("nonce"), Some("echo"));

        std::env::remove_var("WECHAT_TOKEN");

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "missing_timestamp");
    }

    #[test]
    fn verify_wechat_requires_nonce_when_token_set() {
        std::env::set_var("WECHAT_TOKEN", "token");
        std::env::remove_var("WECHAT_ENCODING_AES_KEY");

        let result = verify_wechat(Some("sig"), Some("ts"), None, Some("echo"));

        std::env::remove_var("WECHAT_TOKEN");

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "missing_nonce");
    }

    #[test]
    fn verify_wechat_requires_echostr_when_token_set() {
        std::env::set_var("WECHAT_TOKEN", "token");
        std::env::remove_var("WECHAT_ENCODING_AES_KEY");

        let result = verify_wechat(Some("sig"), Some("ts"), Some("nonce"), None);

        std::env::remove_var("WECHAT_TOKEN");

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "missing_echostr");
    }

    #[test]
    fn verify_wechat_ignores_empty_token() {
        std::env::set_var("WECHAT_TOKEN", "   ");
        std::env::remove_var("WECHAT_ENCODING_AES_KEY");

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
        std::env::remove_var("WECHAT_ENCODING_AES_KEY");

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

    #[test]
    fn verify_wechat_with_encryption_decrypts_echostr() {
        // Test with WeChat encryption
        // EncodingAESKey is 43 chars, Base64 decode with "=" suffix gives 32 bytes
        // Using 43 'A's which decodes to 32 zero bytes
        let token = "test_token";
        let encoding_aes_key = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";

        std::env::set_var("WECHAT_TOKEN", token);
        std::env::set_var("WECHAT_ENCODING_AES_KEY", encoding_aes_key);

        // Create a valid encrypted echostr for testing
        // The format after decryption: random(16B) + msg_len(4B) + msg + receiveid
        use aes::cipher::{block_padding::Pkcs7, BlockEncryptMut, KeyIvInit};
        use base64::Engine;
        type Aes256CbcEnc = cbc::Encryptor<aes::Aes256>;

        let aes_key = base64::engine::general_purpose::STANDARD
            .decode(format!("{}=", encoding_aes_key))
            .unwrap();
        assert_eq!(aes_key.len(), 32, "AES key should be 32 bytes");
        let iv: [u8; 16] = aes_key[..16].try_into().unwrap();
        let key: [u8; 32] = aes_key.clone().try_into().unwrap();

        // Build plaintext: random(16B) + msg_len(4B) + msg + receiveid
        let random_bytes: [u8; 16] = [0u8; 16]; // Use zeros for determinism
        let msg = b"test_echo_response";
        let msg_len = (msg.len() as u32).to_be_bytes();
        let receiveid = b"wx5823bf96d3bd56c7";

        let mut plaintext = Vec::new();
        plaintext.extend_from_slice(&random_bytes);
        plaintext.extend_from_slice(&msg_len);
        plaintext.extend_from_slice(msg);
        plaintext.extend_from_slice(receiveid);

        // Encrypt with PKCS7 padding
        let encryptor = Aes256CbcEnc::new(&key.into(), &iv.into());
        let encrypted = encryptor.encrypt_padded_vec_mut::<Pkcs7>(&plaintext);
        let echostr = base64::engine::general_purpose::STANDARD.encode(&encrypted);

        let timestamp = "1409659813";
        let nonce = "1372623149";

        // Calculate signature: SHA1(sort([token, timestamp, nonce, echostr]))
        let mut parts = vec![token, timestamp, nonce, echostr.as_str()];
        parts.sort();
        let data = parts.join("");
        let mut hasher = Sha1::new();
        hasher.update(data.as_bytes());
        let signature = hex::encode(hasher.finalize());

        let result = verify_wechat(
            Some(&signature),
            Some(timestamp),
            Some(nonce),
            Some(&echostr),
        );

        std::env::remove_var("WECHAT_TOKEN");
        std::env::remove_var("WECHAT_ENCODING_AES_KEY");

        assert!(result.is_ok(), "Expected Ok, got {:?}", result);
        assert_eq!(result.unwrap(), "test_echo_response");
    }

    #[test]
    fn verify_wechat_decryption_invalid_aes_key() {
        // Clean up any existing env vars first
        std::env::remove_var("WECHAT_ENCODING_AES_KEY");

        std::env::set_var("WECHAT_TOKEN", "token_for_invalid_test");
        std::env::set_var("WECHAT_ENCODING_AES_KEY", "short");

        let timestamp = "12345";
        let nonce = "nonce";
        let echostr = "c29tZWJhc2U2NGRhdGE="; // valid base64

        // Calculate signature
        let mut parts = vec!["token_for_invalid_test", timestamp, nonce, echostr];
        parts.sort();
        let data = parts.join("");
        let mut hasher = Sha1::new();
        hasher.update(data.as_bytes());
        let signature = hex::encode(hasher.finalize());

        let result = verify_wechat(
            Some(&signature),
            Some(timestamp),
            Some(nonce),
            Some(echostr),
        );

        std::env::remove_var("WECHAT_TOKEN");
        std::env::remove_var("WECHAT_ENCODING_AES_KEY");

        // Should fail during decryption due to invalid key length
        assert!(result.is_err(), "Expected Err, got {:?}", result);
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
