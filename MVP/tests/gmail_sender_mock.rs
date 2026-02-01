use std::time::Duration;

use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine as _;
use serde_json::json;
use tempfile::TempDir;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

use dowhiz_mvp::gmail_sender::{GmailSender, GmailSenderConfig, ScheduleMode, SendTiming};

fn write_fixture_files(temp_dir: &TempDir) -> (std::path::PathBuf, std::path::PathBuf) {
    let html_path = temp_dir.path().join("reply_email_draft.html");
    std::fs::write(&html_path, "<html><body><h1>Hello</h1></body></html>").unwrap();

    let attachments_dir = temp_dir.path().join("reply_email_attachments");
    std::fs::create_dir_all(&attachments_dir).unwrap();
    std::fs::write(attachments_dir.join("note.txt"), b"hello attachment").unwrap();

    (html_path, attachments_dir)
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn sends_html_with_attachment() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/gmail/v1/users/me/messages/send"))
        .and(header("authorization", "Bearer test-token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"id": "msg-1", "threadId": "th-1"})))
        .mount(&server)
        .await;

    let temp_dir = TempDir::new().unwrap();
    let (html_path, attachments_dir) = write_fixture_files(&temp_dir);

    let mut config = GmailSenderConfig::default();
    config.base_url = format!("{}/gmail/v1", server.uri());
    config.access_token_override = Some("test-token".to_string());

    let sender = GmailSender::new(config).unwrap();
    let results = sender
        .send_email(
            "Test Subject",
            html_path,
            attachments_dir,
            &["recipient@example.com".to_string()],
            SendTiming::Immediate,
        )
        .await
        .unwrap();

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].message_id, "msg-1");

    let requests = server.received_requests().await.unwrap();
    assert_eq!(requests.len(), 1);
    let body: serde_json::Value = serde_json::from_slice(&requests[0].body).unwrap();
    let raw = body["raw"].as_str().unwrap();
    let decoded = URL_SAFE_NO_PAD.decode(raw).unwrap();
    let mime = String::from_utf8_lossy(&decoded);

    assert!(mime.contains("Content-Type: multipart/mixed"));
    assert!(mime.contains("Content-Type: text/html"));
    assert!(mime.contains("Subject: Test Subject"));
    assert!(mime.contains("To: recipient@example.com"));
    assert!(mime.contains("filename=\"note.txt\""));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn delays_send_until_ready() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/gmail/v1/users/me/messages/send"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"id": "msg-2"})))
        .mount(&server)
        .await;

    let temp_dir = TempDir::new().unwrap();
    let (html_path, attachments_dir) = write_fixture_files(&temp_dir);

    let mut config = GmailSenderConfig::default();
    config.base_url = format!("{}/gmail/v1", server.uri());
    config.access_token_override = Some("test-token".to_string());

    let sender = GmailSender::new(config).unwrap();
    let start = std::time::Instant::now();

    sender
        .send_email(
            "Delayed",
            html_path,
            attachments_dir,
            &["recipient@example.com".to_string()],
            SendTiming::Delay(Duration::from_millis(200)),
        )
        .await
        .unwrap();

    assert!(start.elapsed() >= Duration::from_millis(200));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn gmail_schedule_mode_returns_error() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/gmail/v1/users/me/messages/send"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"id": "msg-3"})))
        .mount(&server)
        .await;

    let temp_dir = TempDir::new().unwrap();
    let (html_path, attachments_dir) = write_fixture_files(&temp_dir);

    let mut config = GmailSenderConfig::default();
    config.base_url = format!("{}/gmail/v1", server.uri());
    config.access_token_override = Some("test-token".to_string());
    config.schedule_mode = ScheduleMode::Gmail;

    let sender = GmailSender::new(config).unwrap();
    let err = sender
        .send_email(
            "Scheduled",
            html_path,
            attachments_dir,
            &["recipient@example.com".to_string()],
            SendTiming::Delay(Duration::from_secs(1)),
        )
        .await
        .expect_err("expected schedule not supported error");

    let message = err.to_string();
    assert!(message.contains("scheduled send"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn batch_send_multiple_recipients() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/gmail/v1/users/me/messages/send"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"id": "msg-batch"})))
        .mount(&server)
        .await;

    let temp_dir = TempDir::new().unwrap();
    let (html_path, attachments_dir) = write_fixture_files(&temp_dir);

    let mut config = GmailSenderConfig::default();
    config.base_url = format!("{}/gmail/v1", server.uri());
    config.access_token_override = Some("test-token".to_string());

    let sender = GmailSender::new(config).unwrap();
    let recipients = vec![
        "first@example.com".to_string(),
        "second@example.com".to_string(),
    ];

    let results = sender
        .send_email(
            "Batch",
            html_path,
            attachments_dir,
            &recipients,
            SendTiming::Immediate,
        )
        .await
        .unwrap();

    assert_eq!(results.len(), 2);
    let requests = server.received_requests().await.unwrap();
    assert_eq!(requests.len(), 2);
}
