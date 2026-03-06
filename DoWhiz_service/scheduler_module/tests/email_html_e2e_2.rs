mod test_support;

use scheduler_module::employee_config::{EmployeeDirectory, EmployeeProfile};
use scheduler_module::index_store::IndexStore;
use scheduler_module::service::{
    process_inbound_payload, PostmarkInbound, ServiceConfig, DEFAULT_INBOUND_BODY_MAX_BYTES,
};
use scheduler_module::user_store::UserStore;
use std::collections::{HashMap, HashSet};
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

fn assert_complex_html_sanitized(html: &str) {
    let lower = html.to_ascii_lowercase();
    assert!(html.contains("Hi @bingran-you"), "missing mention");
    assert!(html.contains("Build failed on"), "missing comment text");
    assert!(
        html.contains("https://github.com/KnoWhiz/DoWhiz/pull/2042"),
        "missing pull request link"
    );
    assert!(html.contains("avatar.png"), "missing image");
    assert!(
        html.contains(">malicious link</a>"),
        "anchor text should remain"
    );
    assert!(
        !lower.contains("unsubscribe"),
        "footer marker should be removed"
    );
    assert!(
        !lower.contains("manage notifications"),
        "footer should be removed"
    );
    assert!(
        !lower.contains("display:none"),
        "hidden block still present"
    );
    assert!(!lower.contains("<script"), "script tag still present");
    assert!(!lower.contains("<style"), "style tag still present");
    assert!(!lower.contains("open.gif"), "tracking pixel still present");
    assert!(!lower.contains("pixel.gif"), "1x1 pixel should be removed");
    assert!(
        !lower.contains("javascript:"),
        "unsafe link should be stripped"
    );
    assert!(
        !lower.contains("preheader hidden text"),
        "hidden preheader should be removed"
    );
    assert!(
        !lower.contains("should not appear"),
        "aria-hidden block should be removed"
    );
    assert!(!html.contains("style="), "style attribute still present");
    assert!(!html.contains("class="), "class attribute still present");
}

fn assert_text_fallback(html: &str) {
    assert!(
        html.starts_with("<pre>"),
        "fallback should wrap plain text with <pre>"
    );
    assert!(html.ends_with("</pre>"), "fallback should close </pre>");
    assert!(
        html.contains("Line 1 &lt;keep&gt;"),
        "expected escaped angle brackets in fallback text"
    );
    assert!(
        html.contains("Line 2 &amp; data"),
        "expected escaped ampersand in fallback text"
    );
    let lower = html.to_ascii_lowercase();
    assert!(
        !lower.contains("unsubscribe"),
        "footer text should not remain"
    );
    assert!(
        !lower.contains("tracking"),
        "tracking marker should not remain"
    );
}

fn test_employee_directory() -> (EmployeeProfile, EmployeeDirectory) {
    let addresses = vec!["service@example.com".to_string()];
    let address_set: HashSet<String> = addresses
        .iter()
        .map(|value| value.to_ascii_lowercase())
        .collect();
    let employee = EmployeeProfile {
        id: "test-employee".to_string(),
        display_name: None,
        runner: "codex".to_string(),
        model: None,
        addresses: addresses.clone(),
        address_set: address_set.clone(),
        runtime_root: None,
        agents_path: None,
        claude_path: None,
        soul_path: None,
        skills_dir: None,
        discord_enabled: false,
        slack_enabled: false,
        bluebubbles_enabled: false,
    };
    let mut employee_by_id = HashMap::new();
    employee_by_id.insert(employee.id.clone(), employee.clone());
    let mut service_addresses = HashSet::new();
    service_addresses.extend(address_set);
    let directory = EmployeeDirectory {
        employees: vec![employee.clone()],
        employee_by_id,
        default_employee_id: Some(employee.id.clone()),
        service_addresses,
    };
    (employee, directory)
}

fn setup_service_for_test(
    test_name: &str,
) -> Result<
    Option<(TempDir, ServiceConfig, UserStore, IndexStore)>,
    Box<dyn std::error::Error + Send + Sync>,
> {
    let Some(ingestion_db_url) = test_support::require_supabase_db_url(test_name) else {
        return Ok(None);
    };
    let temp = TempDir::new()?;
    let root = temp.path();
    let users_root = root.join("users");
    let state_root = root.join("state");
    fs::create_dir_all(&users_root)?;
    fs::create_dir_all(&state_root)?;

    let (employee_profile, employee_directory) = test_employee_directory();
    let config = ServiceConfig {
        host: "127.0.0.1".to_string(),
        port: 0,
        employee_id: employee_profile.id.clone(),
        employee_config_path: root.join("employee.toml"),
        employee_profile,
        employee_directory,
        workspace_root: root.join("workspaces"),
        scheduler_state_path: state_root.join("tasks.db"),
        processed_ids_path: state_root.join("processed_ids.txt"),
        ingestion_db_url,
        ingestion_poll_interval: Duration::from_millis(50),
        users_root: users_root.clone(),
        users_db_path: state_root.join("users.db"),
        task_index_path: state_root.join("task_index.db"),
        codex_model: "gpt-5.4".to_string(),
        codex_disabled: true,
        scheduler_poll_interval: Duration::from_millis(50),
        scheduler_max_concurrency: 1,
        scheduler_user_max_concurrency: 1,
        inbound_body_max_bytes: DEFAULT_INBOUND_BODY_MAX_BYTES,
        skills_source_dir: None,
        slack_bot_token: None,
        slack_bot_user_id: None,
        slack_store_path: state_root.join("slack.db"),
        slack_client_id: None,
        slack_client_secret: None,
        slack_redirect_uri: None,
        discord_bot_token: None,
        discord_bot_user_id: None,
        google_docs_enabled: false,
        bluebubbles_url: None,
        bluebubbles_password: None,
        telegram_bot_token: None,
        whatsapp_access_token: None,
        whatsapp_phone_number_id: None,
        whatsapp_verify_token: None,
    };

    let user_store = UserStore::new(&config.users_db_path)?;
    let index_store = IndexStore::new(&config.task_index_path)?;
    Ok(Some((temp, config, user_store, index_store)))
}

fn process_payload_and_load_html(
    config: &ServiceConfig,
    user_store: &UserStore,
    index_store: &IndexStore,
    payload: serde_json::Value,
    requester_email: &str,
) -> Result<(String, String), Box<dyn std::error::Error + Send + Sync>> {
    let inbound_raw = serde_json::to_string(&payload)?;
    let payload: PostmarkInbound = serde_json::from_str(&inbound_raw)?;
    process_inbound_payload(
        config,
        user_store,
        index_store,
        &payload,
        inbound_raw.as_bytes(),
    )?;

    let user = user_store.get_or_create_user("email", requester_email)?;
    let user_paths = user_store.user_paths(&config.users_root, &user.user_id);
    let workspace = first_dir(&user_paths.workspaces_root);

    let email_html = fs::read_to_string(workspace.join("incoming_email").join("email.html"))?;
    let entry_dir = first_dir(&workspace.join("incoming_email").join("entries"));
    let entry_html = fs::read_to_string(entry_dir.join("email.html"))?;
    Ok((email_html, entry_html))
}

#[test]
fn inbound_email_complex_html_is_sanitized() -> Result<(), Box<dyn std::error::Error + Send + Sync>>
{
    let Some((_temp, config, user_store, index_store)) =
        setup_service_for_test("inbound_email_complex_html_is_sanitized")?
    else {
        return Ok(());
    };

    let html_body = r#"
<html>
  <head>
    <style>.notice { color: #999; }</style>
    <script>alert('xss')</script>
  </head>
  <body>
    <!-- hidden preview -->
    <div style="display:none; max-height:0; opacity:0">Preheader hidden text</div>
    <table role="presentation" class="layout">
      <tr>
        <td>
          <p style="font-weight:600;">Hi @bingran-you,</p>
          <p>Build failed on <a href="https://github.com/KnoWhiz/DoWhiz/pull/2042" style="color:red">PR #2042</a>.</p>
          <p>See logs <a href="javascript:alert('xss')">malicious link</a>.</p>
          <img src="https://github.com/images/avatar.png" alt="avatar" width="24" height="24" style="border-radius:12px" />
          <img src="https://mail.example.com/open.gif?tracking=1" width="1" height="1" />
          <img src="https://mail.example.com/pixel.gif" style="width:1px;height:1px" />
        </td>
      </tr>
    </table>
    <section aria-hidden="true">
      <p>Should not appear</p>
    </section>
    <div id="notification-footer">
      Manage notifications and unsubscribe here.
    </div>
    <div>
      <p>Hi @bingran-you,</p>
      <p>Context block preserved.</p>
    </div>
  </body>
</html>
"#;

    let payload_value = serde_json::json!({
        "From": "Alice <alice@example.com>",
        "To": "Service <service@example.com>",
        "Subject": "Build alert",
        "TextBody": "Plain text fallback",
        "HtmlBody": html_body,
        "Headers": [{"Name": "Message-ID", "Value": "<msg-complex@example.com>"}]
    });

    let (email_html, entry_html) = process_payload_and_load_html(
        &config,
        &user_store,
        &index_store,
        payload_value,
        "alice@example.com",
    )?;
    assert_complex_html_sanitized(&email_html);
    assert_complex_html_sanitized(&entry_html);

    Ok(())
}

#[test]
fn inbound_email_falls_back_to_text_when_html_is_removed(
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let Some((_temp, config, user_store, index_store)) =
        setup_service_for_test("inbound_email_falls_back_to_text_when_html_is_removed")?
    else {
        return Ok(());
    };

    let html_body = r#"
<html>
  <body>
    <div style="display:none">tracking preheader</div>
    <img src="https://mail.example.com/open.gif?tracking=true" width="1" height="1" />
    <p class="footer">Reply to this email directly, or unsubscribe.</p>
  </body>
</html>
"#;

    let payload_value = serde_json::json!({
        "From": "Bob <bob@example.com>",
        "To": "Service <service@example.com>",
        "Subject": "Tracking-only HTML",
        "TextBody": "Line 1 <keep>\nLine 2 & data",
        "HtmlBody": html_body,
        "Headers": [{"Name": "Message-ID", "Value": "<msg-fallback@example.com>"}]
    });

    let (email_html, entry_html) = process_payload_and_load_html(
        &config,
        &user_store,
        &index_store,
        payload_value,
        "bob@example.com",
    )?;
    assert_text_fallback(&email_html);
    assert_text_fallback(&entry_html);

    Ok(())
}
