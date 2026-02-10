use scheduler_module::index_store::IndexStore;
use scheduler_module::service::{
    process_inbound_payload, PostmarkInbound, ServiceConfig, DEFAULT_INBOUND_BODY_MAX_BYTES,
};
use scheduler_module::user_store::UserStore;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;
use tempfile::TempDir;

fn first_dir(root: &Path) -> PathBuf {
    let mut entries = fs::read_dir(root).expect("read dir");
    while let Some(entry) = entries.next() {
        let path = entry.expect("dir entry").path();
        if path.is_dir() {
            return path;
        }
    }
    panic!("no directory found");
}

fn assert_clean_html(html: &str) {
    let lower = html.to_ascii_lowercase();
    assert!(html.contains("Hi @bingran-you"), "missing mention");
    assert!(html.contains("New comment on"), "missing comment text");
    assert!(
        html.contains("https://github.com/KnoWhiz/DoWhiz/issues/102"),
        "missing issue link"
    );
    assert!(html.contains("avatar.png"), "missing image");
    assert!(!lower.contains("unsubscribe"), "footer still present");
    assert!(
        !lower.contains("display:none"),
        "hidden block still present"
    );
    assert!(!lower.contains("<script"), "script tag still present");
    assert!(!lower.contains("beacon"), "tracking pixel still present");
    assert!(!html.contains("style="), "style attribute still present");
    assert!(!html.contains("class="), "class attribute still present");
    assert!(!html.contains("Hidden text"), "hidden text still present");
}

#[test]
fn inbound_email_html_is_sanitized() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let temp = TempDir::new()?;
    let root = temp.path();
    let users_root = root.join("users");
    let state_root = root.join("state");
    fs::create_dir_all(&users_root)?;
    fs::create_dir_all(&state_root)?;

    let config = ServiceConfig {
        host: "127.0.0.1".to_string(),
        port: 0,
        workspace_root: root.join("workspaces"),
        scheduler_state_path: state_root.join("tasks.db"),
        processed_ids_path: state_root.join("processed_ids.txt"),
        users_root: users_root.clone(),
        users_db_path: state_root.join("users.db"),
        task_index_path: state_root.join("task_index.db"),
        codex_model: "gpt-5.2-codex".to_string(),
        codex_disabled: true,
        scheduler_poll_interval: Duration::from_millis(50),
        scheduler_max_concurrency: 1,
        scheduler_user_max_concurrency: 1,
        inbound_body_max_bytes: DEFAULT_INBOUND_BODY_MAX_BYTES,
        skills_source_dir: None,
    };

    let user_store = UserStore::new(&config.users_db_path)?;
    let index_store = IndexStore::new(&config.task_index_path)?;

    let html_body = r#"
<html>
  <head>
    <style>.footer{color:#999}</style>
    <script>alert('x')</script>
  </head>
  <body>
    <div>
      <p>Hi @bingran-you,</p>
      <p>New comment on <a href="https://github.com/KnoWhiz/DoWhiz/issues/102">issue #102</a>.</p>
      <img src="https://github.com/images/avatar.png" alt="avatar" width="24" height="24" style="border-radius:12px" />
    </div>
    <div style="display:none">Hidden text</div>
    <img src="https://github.com/notifications/beacon/abc?pixel=true" width="1" height="1" />
    <p class="footer">Reply to this email directly, view it on GitHub, or <a href="https://github.com/notifications/unsubscribe">unsubscribe</a>.</p>
  </body>
</html>
"#;

    let payload_value = serde_json::json!({
        "From": "Alice <alice@example.com>",
        "To": "Service <service@example.com>",
        "Subject": "Issue update",
        "TextBody": "Plain text fallback",
        "HtmlBody": html_body,
        "Headers": [{"Name": "Message-ID", "Value": "<msg-1@example.com>"}]
    });
    let inbound_raw = serde_json::to_string(&payload_value)?;
    let payload: PostmarkInbound = serde_json::from_str(&inbound_raw)?;
    process_inbound_payload(
        &config,
        &user_store,
        &index_store,
        &payload,
        inbound_raw.as_bytes(),
    )?;

    let user = user_store.get_or_create_user("alice@example.com")?;
    let user_paths = user_store.user_paths(&config.users_root, &user.user_id);
    let workspace = first_dir(&user_paths.workspaces_root);

    let email_html = fs::read_to_string(workspace.join("incoming_email").join("email.html"))?;
    assert_clean_html(&email_html);

    let entry_dir = first_dir(&workspace.join("incoming_email").join("entries"));
    let entry_html = fs::read_to_string(entry_dir.join("email.html"))?;
    assert_clean_html(&entry_html);

    Ok(())
}
