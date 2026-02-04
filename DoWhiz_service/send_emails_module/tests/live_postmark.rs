use send_emails_module::{send_email, SendEmailParams};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn load_env_from_repo() {
    let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    loop {
        let candidate = dir.join(".env");
        if candidate.exists() {
            let _ = dotenvy::from_path(candidate);
            break;
        }
        match dir.parent() {
            Some(parent) => dir = parent.to_path_buf(),
            None => break,
        }
    }
}

fn require_live_config() -> (String, String) {
    let live = env::var("POSTMARK_LIVE_TEST").unwrap_or_default();
    assert_eq!(
        live, "1",
        "POSTMARK_LIVE_TEST must be 1 to run live Postmark tests"
    );
    let token = env::var("POSTMARK_SERVER_TOKEN")
        .expect("POSTMARK_SERVER_TOKEN must be set for live Postmark tests");
    let recipient =
        env::var("POSTMARK_TEST_TO").unwrap_or_else(|_| "mini-mouse@deep-tutor.com".to_string());
    (token, recipient)
}

fn unique_subject(prefix: &str) -> String {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("{} {}", prefix, ts)
}

fn poll_for_delivery(token: &str, message_id: &str, timeout: Duration) -> Result<(), String> {
    let client = reqwest::blocking::Client::builder()
        .no_proxy()
        .timeout(Duration::from_secs(20))
        .build()
        .map_err(|err| format!("client build failed: {}", err))?;
    let url = format!(
        "https://api.postmarkapp.com/messages/outbound/{}/details",
        message_id
    );
    let start = SystemTime::now();

    loop {
        let response = client
            .get(&url)
            .header("Accept", "application/json")
            .header("X-Postmark-Server-Token", token)
            .send();

        let elapsed = start.elapsed().unwrap_or_default();
        let response = match response {
            Ok(response) => response,
            Err(err) => {
                if elapsed >= timeout {
                    return Err(format!("timed out waiting for delivery event: {}", err));
                }
                std::thread::sleep(Duration::from_secs(5));
                continue;
            }
        };

        let status = response.status();
        let body = response
            .text()
            .map_err(|err| format!("failed to read response: {}", err))?;

        if status.is_success() {
            let json: serde_json::Value =
                serde_json::from_str(&body).map_err(|err| format!("json error: {}", err))?;
            if let Some(events) = json.get("MessageEvents").and_then(|value| value.as_array()) {
                for event in events {
                    let event_type = event.get("Type").and_then(|value| value.as_str());
                    match event_type {
                        Some("Delivered") => return Ok(()),
                        Some("Bounced") | Some("Failed") => {
                            return Err(format!("delivery failed: {}", event_type.unwrap()))
                        }
                        _ => {}
                    }
                }
            }
        }

        if elapsed >= timeout {
            return Err("timed out waiting for delivery event".to_string());
        }
        std::thread::sleep(Duration::from_secs(5));
    }
}

fn poll_outbound_message(
    token: &str,
    recipient: &str,
    subject: &str,
    message_id: &str,
) -> Result<serde_json::Value, String> {
    let client = reqwest::blocking::Client::builder()
        .no_proxy()
        .timeout(Duration::from_secs(20))
        .build()
        .map_err(|err| format!("client build failed: {}", err))?;
    let start = SystemTime::now();

    loop {
        let response = client
            .get("https://api.postmarkapp.com/messages/outbound")
            .header("Accept", "application/json")
            .header("X-Postmark-Server-Token", token)
            .query(&[
                ("recipient", recipient),
                ("subject", subject),
                ("count", "20"),
                ("offset", "0"),
            ])
            .send();

        let elapsed = start.elapsed().unwrap_or_default();
        let response = match response {
            Ok(response) => response,
            Err(err) => {
                if elapsed >= Duration::from_secs(60) {
                    return Err(format!("message not found in outbound search: {}", err));
                }
                std::thread::sleep(Duration::from_secs(5));
                continue;
            }
        };

        let body = response
            .text()
            .map_err(|err| format!("failed to read search response: {}", err))?;
        let json: serde_json::Value =
            serde_json::from_str(&body).map_err(|err| format!("json error: {}", err))?;

        let messages = json
            .get("Messages")
            .and_then(|value| value.as_array())
            .ok_or_else(|| "missing Messages array".to_string())?;

        for message in messages {
            let id = message.get("MessageID").and_then(|value| value.as_str());
            if id == Some(message_id) {
                return Ok(message.clone());
            }
        }

        if elapsed >= Duration::from_secs(60) {
            return Err("message not found in outbound search".to_string());
        }
        std::thread::sleep(Duration::from_secs(5));
    }
}

fn build_html_file(dir: &Path, name: &str) -> PathBuf {
    let path = dir.join(name);
    let html = r#"<html><body><h1>DoWhiz Test</h1><p>This is a live test.</p></body></html>"#;
    fs::write(&path, html).expect("failed to write html body");
    path
}

#[test]
fn send_email_with_attachments_and_delivery() {
    load_env_from_repo();
    let (token, recipient) = require_live_config();

    let temp = tempfile::tempdir().expect("tempdir failed");
    let html_path = build_html_file(temp.path(), "reply_email_draft.html");
    let attachments_dir = temp.path().join("reply_email_attachments");
    fs::create_dir_all(&attachments_dir).expect("failed to create attachments dir");

    fs::write(attachments_dir.join("sample.txt"), b"sample text").unwrap();
    fs::write(attachments_dir.join("sample.csv"), b"a,b,c\n1,2,3\n").unwrap();

    let subject = unique_subject("DoWhiz live attachment test");

    let params = SendEmailParams {
        subject: subject.clone(),
        html_path,
        attachments_dir: attachments_dir.clone(),
        to: vec![recipient.clone()],
        cc: Vec::new(),
        bcc: Vec::new(),
        in_reply_to: None,
        references: None,
    };

    let response = send_email(&params).expect("postmark send failed");
    assert_eq!(response.error_code, 0, "postmark returned error");

    poll_for_delivery(&token, &response.message_id, Duration::from_secs(180))
        .expect("email was not delivered in time");

    let message = poll_outbound_message(&token, &recipient, &subject, &response.message_id)
        .expect("failed to find outbound message");
    let attachments = message
        .get("Attachments")
        .and_then(|value| value.as_array())
        .expect("missing Attachments array");
    assert!(attachments.len() >= 2, "expected attachments in outbound message");
}

#[test]
fn send_multiple_emails_batch() {
    load_env_from_repo();
    let (token, recipient) = require_live_config();

    let temp = tempfile::tempdir().expect("tempdir failed");
    let html_path = build_html_file(temp.path(), "reply_email_draft.html");
    let attachments_dir = temp.path().join("reply_email_attachments");
    fs::create_dir_all(&attachments_dir).expect("failed to create attachments dir");

    let requested = env::var("POSTMARK_BATCH_COUNT")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(2);
    let batch_count = requested.clamp(1, 5);
    if requested > batch_count {
        eprintln!(
            "Capping POSTMARK_BATCH_COUNT={} to {} to limit test cost.",
            requested, batch_count
        );
    }

    for idx in 0..batch_count {
        let subject = unique_subject(&format!("DoWhiz live batch test {}", idx + 1));
        let params = SendEmailParams {
            subject: subject.clone(),
            html_path: html_path.clone(),
            attachments_dir: attachments_dir.clone(),
            to: vec![recipient.clone()],
            cc: Vec::new(),
            bcc: Vec::new(),
            in_reply_to: None,
            references: None,
        };

        let response = send_email(&params).expect("postmark send failed");
        assert_eq!(response.error_code, 0, "postmark returned error");

        poll_for_delivery(&token, &response.message_id, Duration::from_secs(180))
            .unwrap_or_else(|err| panic!("batch email not delivered: {}", err));

        let _ = poll_outbound_message(&token, &recipient, &subject, &response.message_id)
            .expect("failed to find outbound message");
    }
}
