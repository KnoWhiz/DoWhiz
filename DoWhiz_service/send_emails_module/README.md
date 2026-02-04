# send_emails_module

Send HTML emails with attachments via the Postmark API. This module reads a reply HTML file and a flat attachments directory, then sends a single email with comma-separated To/Cc/Bcc.

## Usage

Environment variables (loaded from `.env` if present):
- `POSTMARK_SERVER_TOKEN` (required)
- `OUTBOUND_FROM` (optional, defaults to `oliver@dowhiz.com`)
- `POSTMARK_API_BASE_URL` (optional, override Postmark base URL for tests)
- `POSTMARK_LIVE_TEST` (required by tests, must be `1`)

Example:
```rust
use send_emails_module::{send_email, SendEmailParams};
use std::path::PathBuf;

let params = SendEmailParams {
    subject: "Hello".to_string(),
    html_path: PathBuf::from("/path/to/reply_email_draft.html"),
    attachments_dir: PathBuf::from("/path/to/reply_email_attachments"),
    to: vec!["mini-mouse@deep-tutor.com".to_string()],
    cc: Vec::new(),
    bcc: Vec::new(),
    in_reply_to: None,
    references: None,
};

let response = send_email(&params)?;
println!("Sent: {}", response.message_id);
```

## Folder structure

- `DoWhiz_service/send_emails_module/src/lib.rs` : Postmark send logic and request payload construction.
- `DoWhiz_service/send_emails_module/tests/` : Live Postmark tests (send + delivery status polling).

## Notes

- Attachments are all files in `reply_email_attachments/` (no subfolders).
- Tests run live against Postmark and will fail unless `POSTMARK_LIVE_TEST=1` and credentials are set.
- Use `POSTMARK_TEST_TO` to control the live-test recipient address.
- Use `POSTMARK_BATCH_COUNT` to control the batch live-test size (capped at 5).
