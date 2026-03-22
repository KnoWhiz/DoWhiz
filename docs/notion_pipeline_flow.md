# Notion Pipeline Flow

## Step 1: Notion sends email notification to oliver@dowhiz.com

When someone @mentions Oliver in Notion, Notion sends an email from notify@mail.notion.so to oliver@dowhiz.com. Postmark receives it and forwards to the inbound gateway.

---

## Step 2: Inbound Gateway receives Postmark webhook

**// handlers.rs:69-148**

```rust
pub(super) async fn ingest_postmark(
    State(state): State<Arc<GatewayState>>,
    headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    // Parse Postmark payload
    let payload: PostmarkInboundPayload = serde_json::from_slice(&body)?;

    // Find which employee this email is for
    let address = find_service_address(&payload,
        &state.employee_directory.service_addresses);
    let route = resolve_route(Channel::Email, &normalize_email(&address), &state);

    // Parse into InboundMessage
    let adapter = PostmarkInboundAdapter::new(...);
    let message = adapter.parse(&body)?;

    // Build envelope and enqueue
    let envelope = build_envelope(route, Channel::Email, external_message_id, &message,
        &body).await?;
    enqueue_envelope(state.queue.clone(), envelope).await
}
```

---

## Step 3: Build queue payload - strip html_body for Notion emails

**// handlers.rs:728-742**

```rust
fn build_queue_payload(channel: Channel, message: &InboundMessage) -> IngestionPayload {
    let mut payload = IngestionPayload::from_inbound(message);
    if channel == Channel::Email {
        payload.attachments.clear();

        // Strip html_body for Notion emails to avoid PayloadTooLarge errors
        let sender_lower = message.sender.to_lowercase();
        if sender_lower.contains("notion.so") || sender_lower.contains("notion.com") {
            payload.html_body = None;
        }
    }
    payload
}
```

---

## Step 4: Envelope enqueued to Azure Service Bus

**// handlers.rs:934-970**

```rust
pub(super) async fn build_envelope(
    route: RouteDecision,
    channel: Channel,
    external_message_id: Option<String>,
    message: &InboundMessage,
    raw_payload: &[u8],
) -> Result<IngestionEnvelope, ...> {
    let queue_payload = build_queue_payload(channel, message);

    Ok(IngestionEnvelope {
        id: Uuid::new_v4(),
        channel,
        tenant_id: route.tenant_id,
        employee_id: route.employee_id,
        payload: queue_payload,
        raw_payload_ref: storage_ref,
        ...
    })
}

async fn enqueue_envelope(queue: Arc<dyn IngestionQueue>, envelope: IngestionEnvelope)
-> ... {
    queue.enqueue(&envelope).await?;
    info!("enqueued ingestion envelope id={}", envelope.id);
}
```

---

## Step 5: Global Worker (Consumer Loop) dequeues and processes envelope

**// ingestion.rs:57-80 (consumer loop)**

```rust
let handle = thread::spawn(move || loop {
    match runtime.block_on(queue.dequeue()) {
        Ok(Some(envelope)) => {
            match process_inbound_message(&config, ..., &envelope) {
                Ok(()) => info!("ingestion processed successfully"),
                Err(err) => error!("ingestion failed: {}", err),
            }
        }
        ...
    }
});
```

---

## Step 6: Process inbound message - check for Notion forward pattern

**// ingestion.rs:129-172**

```rust
match envelope.channel {
    Channel::Email => {
        let (payload, raw_payload) = resolve_email_payload(envelope)?;
        let subject = payload.subject.as_deref().unwrap_or("");

        // Check if this looks like a Notion notification (by subject pattern)
        let looks_like_notion = subject.contains("mentioned you")
            || subject.contains("replied to")
            || subject.contains("commented in")
            || subject.contains("commented on")
            || subject.contains("发表了评论")
            || subject.contains("中提及了您");

        let sender = envelope.payload.sender.trim();

        // If NOT notion-like AND sender is blacklisted, skip
        if !looks_like_notion && is_blacklisted_email_sender(sender, ...) {
            info!("skipping blacklisted sender: {}", sender);
            return Ok(());
        }

        // If notion-like AND sender is blacklisted, allow it through
        // Explicit override of oliver@dowhiz.com email blacklist
        if looks_like_notion && is_blacklisted_email_sender(sender, ...) {
            info!("allowing forwarded Notion email from blacklisted sender: {}", sender);
        }

        process_inbound_payload(config, user_store, ..., &payload, &raw_payload, ...)
    }
}
```

---

## Step 7: Process inbound email - check *NOTION_EMAIL_DETECTION_DISABLED=1*

**// email.rs:50-109**

```rust
pub fn process_inbound_payload(
    config: &ServiceConfig,
    ...
    payload: &PostmarkInbound,
    raw_payload: &[u8],
    ...
) -> Result<(), BoxError> {
    let sender = payload.from.as_deref().unwrap_or("").trim();
    let subject = payload.subject.as_deref().unwrap_or("");

    // Check if Notion email detection is disabled
    let notion_email_disabled = std::env::var("NOTION_EMAIL_DETECTION_DISABLED")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);

    if !notion_email_disabled && is_notion_sender(sender) {
        // Detect Notion notification type from email content
        if let Some(notification) = detect_notion_email(
            sender,
            subject,
            payload.text_body.as_deref(),
            payload.html_body.as_deref(),
        ) {
            info!("detected Notion email notification type={:?}", notification.notification_type);

            // Route to specialized Notion handler - service/ dir is up one level (thus we use super,
            // and "loop back" to the Notion inbound channel handler)
            return super::inbound::process_notion_email(
                config, user_store, index_store, account_store,
                payload, raw_payload, &notification,
            );
        }
    } else if notion_email_disabled && is_notion_sender(sender) {
        info!("Notion email detection disabled (NOTION_EMAIL_DETECTION_DISABLED=1),
            skipping");
        // Falls through to regular email processing below...
    }

    // Regular email processing (creates task with Channel::Email)
    let requester = resolve_inbound_requester(payload, raw_payload)?;
    // ... creates workspace, schedules RunTask with Channel::Email
}
```

> **If NOTION_EMAIL_DETECTION_DISABLED=1, fallback to Email. In ACI container, Codex falls back to manual 2FA verification**

---

## Step 8a: IF NOTION DETECTION ENABLED → process_notion_email

**// notion_email.rs:40-106**

```rust
pub(crate) fn process_notion_email(
    config: &ServiceConfig,
    ...
    notification: &NotionEmailNotification,
) -> Result<(), BoxError> {
    info!("processing Notion email notification type={:?} actor={:?} page={:?}",
        notification.notification_type, notification.actor_name, notification.page_title);

    // LOOP PREVENTION: Skip self-notifications
    if let Some(actor) = &notification.actor_name {
        let actor_lower = actor.to_lowercase();

        // Check against display_name (e.g., "Oliver")
        if let Some(name) = config.employee_profile.display_name.as_deref() {
            if actor_lower == name.to_lowercase() {
                info!("skipping self-notification from employee '{}'", actor);
                return Ok(());
            }
        }

        // Check against Notion integration names
        if actor_lower.starts_with("dowhiz_") || actor_lower == "dowhiz" {
            info!("skipping self-notification from Notion integration '{}'", actor);
            return Ok(());
        }
    }

    // Continue processing…
    let thread_key = create_notion_thread_key(notification, email_payload);

    // Ensure workspace exists
    let workspace = ensure_thread_workspace(
            &user_paths,
            &user.user_id,
            &thread_key,
            &config.employee_profile,
            config.skills_source_dir.as_deref(),
    )?;
}
```

> <u>workspace</u> is of type PathBuf (just a path string)

---

## Step 9a: Get Notion OAuth token from store

**// notion_email.rs:157-291**

```rust
let (access_token, credential_account_id): (Option<String>, Option<uuid::Uuid>) =
    if let Ok(token) = std::env::var("NOTION_API_TOKEN") {
        (Some(token), None)
    } else {
        match NotionStore::new() {
            Ok(store) => {
                // Try workspace_id from email URL
                if let Some(ref ws_id) = notification.workspace_id {
                    info!("Looking for Notion token by workspace_id: {}", ws_id);

                    //Get the access_token from notion_store DB , given workspace_id
                    match store.get_credential_by_workspace(ws_id) {
                        Ok(credential) => {
                            info!("Found Notion token by workspace_id '{}'", ws_id);
                            (Some(credential.access_token), Some(credential.account_id))
                        }
                        Err(e) => {
                            warn!("No Notion token found for workspace_id '{}': {}", ws_id, e);
                            // Try fallback methods...
                        }
                    }
                }
            }
            Err(e) => (None, None)
        }
    };
```

```rust
// notion_store.rs - stores the following fields in MongoDB, during OAuth
pub struct NotionCredential {
    pub workspace_id: String,     // **populated from Notion OAuth response, the index var**
    pub workspace_name: Option<String>,
    pub access_token: String,     // the OAuth token
    pub account_id: Uuid,         // which DoWhiz user owns this
    pub created_at: DateTime<Utc>,
}
```

> <u>notification.workspace_id</u> is a separate concept from the local <u>workspace</u>

---

## Step 10a: Write Notion context to workspace (the local <u>workspace</u>, not Notion workspace)

**// notion_email.rs:293-300, 435-478**

```rust
write_notion_email_context(&workspace, notification, email_payload, seq,
    access_token.as_deref())?;

fn write_notion_email_context(...) -> Result<(), BoxError> {
    // Write OAuth token to .notion_env for the agent
    if let Some(token) = access_token {
        let env_path = workspace.join(".notion_env");
        std::fs::write(&env_path, format!("NOTION_API_TOKEN={}\n", token))?;
        info!("Wrote Notion API token to workspace");
    }

    // Write context as JSON
    let context_path = workspace.join(".notion_email_context.json");
    let context = serde_json::json!({
        "channel": "notion",
        "page_url": notification.page_url,
        "page_id": notification.page_id,
        "page_title": notification.page_title,
        "comment_preview": notification.comment_preview,
        "has_api_access": has_api_access,
        "instructions": "Use notion_api_cli to read the page and post replies..."
    });
    std::fs::write(&context_path, serde_json::to_string_pretty(&context)?)?;
}
```

---

## Step 11a: Schedule task with *Channel::Notion*

**// notion_email.rs:318-357**

```rust
let run_task = RunTaskTask {
    workspace_dir: workspace.clone(),
    channel: Channel::Notion,
    model_name,
    runner: config.employee_profile.runner.clone(),
    reply_to: vec![user_email.clone()],
    thread_id: Some(thread_key.clone()),
    account_id: credential_account_id,
    ...
};

let mut scheduler = Scheduler::load(&user_paths.tasks_db_path, ModuleExecutor::default())?;
let task_id = scheduler.add_one_shot_in(Duration::from_secs(0),
    TaskKind::RunTask(run_task))?;

info!("scheduled Notion task user_id={} task_id={} channel=Notion", user.user_id, task_id);
```

---

## Step 12a: Codex runs with Notion context

The agent has access to:
- `.notion_env` with `NOTION_API_TOKEN=secret_xxx`
- `.notion_email_context.json` with page_id, comment info
- `notion_api_cli` tool to read pages and post comments

---

## Step 13a: Agent completes → execute_notion_send

**// outbound.rs:811-909**

```rust
pub(crate) fn execute_notion_send(task: &SendReplyTask) -> Result<(), SchedulerError> {
    let workspace_dir = task.html_path.parent().unwrap_or(Path::new("."));

    // Check if agent already posted via Notion API (marker file written by agent)
    if workspace_dir.join(".notion_api_replied").exists() {
        // .notion_api_replied is the marker file indicating Codex already responded via Notion CLI
        info!("skipping notion send - agent already posted via API");
        return Ok(());
    }

    // Read reply text from workspace
    let text_body = fs::read_to_string(&task.html_path)?;

    // Read Notion context
    let context_path = workspace_dir.join(".notion_context.json");
    let context: serde_json::Value = serde_json::from_str(&fs::read_to_string(&context_path)?)?;

    // Queue reply for Notion browser poller (Legacy)
    let reply_request = serde_json::json!({
        "reply_text": text_body.trim(),
        "page_id": context.get("page_id"),
        "comment_id": context.get("comment_id"),
        "status": "pending"
    });

    let queue_dir = notion_reply_queue_dir(&employee_id);
    let request_path = queue_dir.join(&format!("{}.json", request_id));
    fs::write(&request_path, serde_json::to_string_pretty(&reply_request)?)?;

    info!("queued Notion reply request id={} page_id={:?}", request_id, context.get("page_id"));
    Ok(())
}
```

- Codex decides to use `notion_api_cli` to send its response directly via Notion API, or not - separate logic
  - ***This is a potential bug, as Codex can invoke the notion_api, and write to task.html_path, causing two responses: one from direct API from Codex within ACI, and one from OutboundAdapter***
  - **Fix:** Agent (prompted in `employee_notion.md`) executes `touch .notion_api_replied` after posting via `notion_api_cli`. The `execute_notion_send` function checks for this marker file and returns `Ok(())` early, preventing the duplicate send.

---

## Step 8b: IF NOTION DETECTION DISABLED → regular email flow

**// email.rs:107-109**

```rust
} else if notion_email_disabled && is_notion_sender(sender) {
    info!("Notion email detection disabled (NOTION_EMAIL_DETECTION_DISABLED=1),
        skipping");
}

// Falls through to regular email processing:
// email.rs:111+
let requester = resolve_inbound_requester(payload, raw_payload)?;
// ... creates workspace WITHOUT .notion_env or .notion_email_context.json
// ... schedules RunTask with Channel::Email
```

---

## Step 9b-13b: Regular email flow

- Task created with *Channel::Email*
- Codex runs without Notion context
- Agent tries to reply via email
- `execute_email_send` is called instead of `execute_notion_send`
- **Oliver hits captcha when sending via Gmail**
