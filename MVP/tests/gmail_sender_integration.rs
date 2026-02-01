use std::time::Duration;

use tempfile::TempDir;

use dowhiz_mvp::gmail_sender::{GmailSender, GmailSenderConfig, SendTiming};

fn integration_enabled() -> bool {
    std::env::var("GMAIL_INTEGRATION").map(|v| v == "1" || v.eq_ignore_ascii_case("true")).unwrap_or(false)
}

fn build_recipients() -> Vec<String> {
    if let Ok(list) = std::env::var("GMAIL_TEST_RECIPIENTS") {
        return list
            .split(',')
            .map(|addr| addr.trim().to_string())
            .filter(|addr| !addr.is_empty())
            .collect();
    }
    vec![
        "deep-tutor@deep-tutor.com".to_string(),
        "mini-mouse@deep-tutor.com".to_string(),
    ]
}

fn write_fixture_files(temp_dir: &TempDir) -> (std::path::PathBuf, std::path::PathBuf) {
    let html_path = temp_dir.path().join("reply_email_draft.html");
    std::fs::write(&html_path, "<html><body><p>Integration test</p></body></html>").unwrap();

    let attachments_dir = temp_dir.path().join("reply_email_attachments");
    std::fs::create_dir_all(&attachments_dir).unwrap();
    std::fs::write(attachments_dir.join("integration.txt"), b"integration attachment").unwrap();

    (html_path, attachments_dir)
}

fn build_config() -> GmailSenderConfig {
    let mut config = GmailSenderConfig::default();
    if let Ok(path) = std::env::var("GMAIL_CLIENT_SECRET_PATH") {
        config.credentials_path = path.into();
    }
    if let Ok(path) = std::env::var("GMAIL_TOKEN_PATH") {
        config.token_path = path.into();
    }
    config
}

fn ensure_refresh_token(config: &GmailSenderConfig) {
    if std::env::var("GMAIL_REFRESH_TOKEN").is_ok() {
        return;
    }
    if let Ok(contents) = std::fs::read_to_string(&config.token_path) {
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(&contents) {
            if value
                .get("refresh_token")
                .and_then(|v| v.as_str())
                .map(|s| !s.is_empty())
                .unwrap_or(false)
            {
                return;
            }
        }
    }
    if let Ok(refresh_token) = std::env::var("GMAIL_REFRESH_TOKEN_FALLBACK") {
        let payload = serde_json::json!({
            "refresh_token": refresh_token,
        });
        let _ = std::fs::write(&config.token_path, serde_json::to_string_pretty(&payload).unwrap());
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn integration_send_with_attachment() {
    if !integration_enabled() {
        eprintln!("GMAIL_INTEGRATION not set; skipping real Gmail API test.");
        return;
    }

    let config = build_config();
    ensure_refresh_token(&config);
    let sender = GmailSender::new(config).unwrap();

    let temp_dir = TempDir::new().unwrap();
    let (html_path, attachments_dir) = write_fixture_files(&temp_dir);

    let recipients = build_recipients();
    let results = sender
        .send_email(
            "[DoWhiz] Integration Test - Attachment",
            html_path,
            attachments_dir,
            &recipients[..1],
            SendTiming::Immediate,
        )
        .await
        .unwrap();

    assert_eq!(results.len(), 1);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn integration_batch_send_with_delay() {
    if !integration_enabled() {
        eprintln!("GMAIL_INTEGRATION not set; skipping real Gmail API test.");
        return;
    }

    let config = build_config();
    ensure_refresh_token(&config);
    let sender = GmailSender::new(config).unwrap();

    let temp_dir = TempDir::new().unwrap();
    let (html_path, attachments_dir) = write_fixture_files(&temp_dir);

    let recipients = build_recipients();
    let delay = Duration::from_secs(5);

    let start = std::time::Instant::now();
    let results = sender
        .send_email(
            "[DoWhiz] Integration Test - Batch Delay",
            html_path,
            attachments_dir,
            &recipients,
            SendTiming::Delay(delay),
        )
        .await
        .unwrap();

    assert_eq!(results.len(), recipients.len());
    assert!(start.elapsed() >= delay);
}
