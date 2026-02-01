# DoWhiz MVP Gmail Sender

This module sends HTML emails (with optional attachments) via the Gmail API using OAuth2. It also supports a local delay before sending.

Important: Gmail API does **not** support Gmail-side “Schedule send”. The `Delay` option in this module is a local sleep before calling `users.messages.send`.

## Prerequisites

- Rust toolchain (stable)
- A Google Cloud project with the Gmail API enabled
- OAuth client credentials JSON downloaded from Google Cloud Console

## Step 1: Place credentials

1. Download OAuth client credentials from Google Cloud Console.
2. Save the file as `.gmail_client_secret.json` in the `MVP` folder.

The file should look like this:

```
{
  "web": {
    "client_id": "...",
    "client_secret": "...",
    "redirect_uris": [],
    "auth_uri": "https://accounts.google.com/o/oauth2/auth",
    "token_uri": "https://accounts.google.com/o/oauth2/token"
  }
}
```

## Step 2: Get a refresh token

This module needs a refresh token to obtain access tokens. You can provide it in one of two ways:

Option A: environment variable

- Set `GMAIL_REFRESH_TOKEN` to your refresh token.

Option B: token file

- Create `.gmail_token.json` in the `MVP` folder:

```
{
  "refresh_token": "YOUR_REFRESH_TOKEN"
}
```

Notes:
- `GMAIL_REFRESH_TOKEN` takes precedence.
- The refresh token must have the Gmail send scope (for example, `https://www.googleapis.com/auth/gmail.send`).

## Step 3: Prepare your email inputs

You will pass these to the function:

- `reply_email_draft.html`: HTML body file
- `reply_email_attachments/`: directory of files to attach (all files are attached; can be empty)
- Subject: passed as a parameter
- Recipients: list of email addresses

## Step 4: Run tests (mocked)

From the repo root:

```
cd MVP
cargo test
```

This runs mocked tests that do not call the real Gmail API.

## Step 5: Run integration tests (real Gmail API)

Integration tests send real emails. They only run when `GMAIL_INTEGRATION=1`.

Example:

```
cd MVP
export GMAIL_INTEGRATION=1
export GMAIL_REFRESH_TOKEN="YOUR_REFRESH_TOKEN"
export GMAIL_CLIENT_SECRET_PATH="./.gmail_client_secret.json"
export GMAIL_TEST_RECIPIENTS="deep-tutor@deep-tutor.com,mini-mouse@deep-tutor.com"

cargo test --test gmail_sender_integration
```

Optional variables:
- `GMAIL_TOKEN_PATH`: override token file path (default: `.gmail_token.json`)
- `GMAIL_CLIENT_SECRET_PATH`: override credentials file path (default: `.gmail_client_secret.json`)
- `GMAIL_TEST_RECIPIENTS`: comma-separated recipients

## Example usage (Rust)

```rust
use std::time::Duration;
use dowhiz_mvp::gmail_sender::{GmailSender, GmailSenderConfig, SendTiming};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = GmailSenderConfig::default();
    let sender = GmailSender::new(config)?;

    let results = sender
        .send_email(
            "Hello from DoWhiz",
            "./reply_email_draft.html",
            "./reply_email_attachments",
            &vec!["recipient@example.com".to_string()],
            SendTiming::Delay(Duration::from_secs(60)),
        )
        .await?;

    println!("Sent: {}", results.len());
    Ok(())
}
```

## File locations

- Module: `MVP/src/gmail_sender.rs`
- Mock tests: `MVP/tests/gmail_sender_mock.rs`
- Integration tests: `MVP/tests/gmail_sender_integration.rs`
