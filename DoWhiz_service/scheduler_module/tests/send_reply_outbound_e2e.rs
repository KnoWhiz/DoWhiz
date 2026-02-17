use mockito::{Matcher, Server};
use scheduler_module::{channel::Channel, ModuleExecutor, Scheduler, SendReplyTask, TaskKind};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::Duration;
use tempfile::TempDir;

static ENV_MUTEX: Mutex<()> = Mutex::new(());

struct EnvGuard {
    key: &'static str,
    original: Option<String>,
}

impl EnvGuard {
    fn set(key: &'static str, value: impl AsRef<std::ffi::OsStr>) -> Self {
        let original = env::var(key).ok();
        env::set_var(key, value);
        Self { key, original }
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        match &self.original {
            Some(value) => env::set_var(self.key, value),
            None => env::remove_var(self.key),
        }
    }
}

fn write_text_file(temp: &TempDir, name: &str, contents: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let path = temp.path().join(name);
    fs::write(&path, contents)?;
    Ok(path)
}

fn create_attachments_dir(temp: &TempDir) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let dir = temp.path().join("attachments");
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

fn base_send_task(channel: Channel, html_path: PathBuf, attachments_dir: PathBuf) -> SendReplyTask {
    SendReplyTask {
        channel,
        subject: String::new(),
        html_path,
        attachments_dir,
        from: None,
        to: vec![],
        cc: vec![],
        bcc: vec![],
        in_reply_to: None,
        references: None,
        archive_root: None,
        thread_epoch: None,
        thread_state_path: None,
    }
}

#[test]
fn send_reply_slack_uses_mock() -> Result<(), Box<dyn std::error::Error>> {
    let _lock = ENV_MUTEX.lock().unwrap();
    let mut server = Server::new();

    let slack_mock = server
        .mock("POST", "/chat.postMessage")
        .match_header("authorization", "Bearer xoxb-test")
        .match_header("content-type", "application/json")
        .match_body(Matcher::Regex("\\\"channel\\\":\\\"C123\\\"".to_string()))
        .match_body(Matcher::Regex("Hello Slack".to_string()))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"ok":true,"ts":"1700000000.123"}"#)
        .expect(1)
        .create();

    let _guard_token = EnvGuard::set("SLACK_BOT_TOKEN", "xoxb-test");
    let _guard_api = EnvGuard::set("SLACK_API_BASE_URL", server.url());

    let temp = TempDir::new()?;
    let html_path = write_text_file(&temp, "slack_message.txt", "Hello Slack")?;
    let attachments_dir = create_attachments_dir(&temp)?;

    let mut task = base_send_task(Channel::Slack, html_path, attachments_dir);
    task.to = vec!["C123".to_string()];

    let db_path = temp.path().join("tasks.db");
    let mut scheduler = Scheduler::load(&db_path, ModuleExecutor::default())?;
    scheduler.add_one_shot_in(Duration::from_secs(0), TaskKind::SendReply(task))?;
    scheduler.tick()?;

    slack_mock.assert();
    Ok(())
}

#[test]
fn send_reply_discord_uses_mock() -> Result<(), Box<dyn std::error::Error>> {
    let _lock = ENV_MUTEX.lock().unwrap();
    let mut server = Server::new();

    let discord_mock = server
        .mock("POST", "/api/v10/channels/987654/messages")
        .match_header("authorization", "Bot discord-test")
        .match_header("content-type", "application/json")
        .match_body(Matcher::Regex("Hello Discord".to_string()))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"msg-999","timestamp":"2024-01-01T00:00:00Z","channel_id":"987654"}"#)
        .expect(1)
        .create();

    let _guard_token = EnvGuard::set("DISCORD_BOT_TOKEN", "discord-test");
    let _guard_api = EnvGuard::set("DISCORD_API_BASE_URL", format!("{}/api/v10", server.url()));

    let temp = TempDir::new()?;
    let html_path = write_text_file(&temp, "discord_message.txt", "Hello Discord")?;
    let attachments_dir = create_attachments_dir(&temp)?;

    let mut task = base_send_task(Channel::Discord, html_path, attachments_dir);
    task.to = vec!["987654".to_string()];

    let db_path = temp.path().join("tasks.db");
    let mut scheduler = Scheduler::load(&db_path, ModuleExecutor::default())?;
    scheduler.add_one_shot_in(Duration::from_secs(0), TaskKind::SendReply(task))?;
    scheduler.tick()?;

    discord_mock.assert();
    Ok(())
}

#[test]
fn send_reply_sms_uses_mock() -> Result<(), Box<dyn std::error::Error>> {
    let _lock = ENV_MUTEX.lock().unwrap();
    let mut server = Server::new();

    let sms_mock = server
        .mock("POST", "/2010-04-01/Accounts/AC123/Messages.json")
        .match_header("authorization", Matcher::Regex("^Basic ".to_string()))
        .match_body(Matcher::AllOf(vec![
            Matcher::Regex("To=%2B15551234567".to_string()),
            Matcher::Regex("From=%2B15557654321".to_string()),
            Matcher::Regex("Body=Hello".to_string()),
        ]))
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body(r#"{"sid":"SM123","status":"queued"}"#)
        .expect(1)
        .create();

    let _guard_sid = EnvGuard::set("TWILIO_ACCOUNT_SID", "AC123");
    let _guard_token = EnvGuard::set("TWILIO_AUTH_TOKEN", "twilio-test");
    let _guard_api = EnvGuard::set("TWILIO_API_BASE_URL", server.url());

    let temp = TempDir::new()?;
    let html_path = write_text_file(&temp, "sms_message.txt", "Hello SMS")?;
    let attachments_dir = create_attachments_dir(&temp)?;

    let mut task = base_send_task(Channel::Sms, html_path, attachments_dir);
    task.from = Some("+15557654321".to_string());
    task.to = vec!["+15551234567".to_string()];

    let db_path = temp.path().join("tasks.db");
    let mut scheduler = Scheduler::load(&db_path, ModuleExecutor::default())?;
    scheduler.add_one_shot_in(Duration::from_secs(0), TaskKind::SendReply(task))?;
    scheduler.tick()?;

    sms_mock.assert();
    Ok(())
}
