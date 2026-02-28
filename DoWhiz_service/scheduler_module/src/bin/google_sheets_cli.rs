//! Google Sheets CLI tool for Agent to interact with Google Sheets.
//!
//! This CLI provides commands for the digital employee to:
//! - List spreadsheets and read content
//! - List and reply to comments
//! - Update cell values
//! - Append rows

use scheduler_module::adapters::google_sheets::{
    GoogleSheetsInboundAdapter, GoogleSheetsOutboundAdapter,
};
use scheduler_module::google_auth::{GoogleAuth, GoogleAuthConfig};
use std::collections::HashSet;
use std::env;
use std::process::exit;

fn print_usage() {
    eprintln!(
        r##"Usage: google-sheets <command> [arguments]

Commands:
  list-spreadsheets                     List all spreadsheets shared with you
  read-spreadsheet <id>                 Read spreadsheet content as CSV
  read-values <id> <range>              Read specific range (e.g., "Sheet1!A1:C10")
  get-metadata <id>                     Get spreadsheet metadata (sheet names, etc.)

Comment Operations:
  list-comments <id>                    List comments on a spreadsheet
  reply-comment <id> <comment_id> <msg> Reply to a comment

Edit Operations:
  update-values <id> <range> <json>     Update cells (JSON: [["a","b"],["c","d"]])
  append-rows <id> <range> <json>       Append rows to a range
  batch-update <id> <json>              Send batch update requests

Examples:
  google-sheets list-spreadsheets
  google-sheets read-values 1abc... "Sheet1!A1:D10"
  google-sheets update-values 1abc... "Sheet1!A1:B2" '[["Hello","World"],["Foo","Bar"]]'
  google-sheets append-rows 1abc... "Sheet1!A:D" '[["new","row","data","here"]]'

Environment Variables:
  GOOGLE_ACCESS_TOKEN    - Pre-generated access token (for sandbox environments)
  GOOGLE_CLIENT_ID       - Google OAuth client ID
  GOOGLE_CLIENT_SECRET   - Google OAuth client secret
  GOOGLE_REFRESH_TOKEN   - Google OAuth refresh token
"##
    );
}

fn get_auth() -> Result<GoogleAuth, String> {
    dotenvy::dotenv().ok();

    let mut config = GoogleAuthConfig::from_env();

    if config.access_token.is_none() {
        if let Ok(token) = std::fs::read_to_string(".google_access_token") {
            let token = token.trim().to_string();
            if !token.is_empty() {
                eprintln!("[google-sheets] Read access token from .google_access_token file");
                config.access_token = Some(token);
            }
        }
    }

    let has_access_token = config.access_token.is_some();
    let has_refresh_token = config.refresh_token.is_some();
    let has_client_id = config.client_id.is_some();
    eprintln!(
        "[google-sheets] Auth config: access_token={}, refresh_token={}, client_id={}, valid={}",
        has_access_token,
        has_refresh_token,
        has_client_id,
        config.is_valid()
    );

    if !config.is_valid() {
        return Err("Google OAuth credentials not configured. Set GOOGLE_ACCESS_TOKEN (for sandbox) or GOOGLE_CLIENT_ID, GOOGLE_CLIENT_SECRET, and GOOGLE_REFRESH_TOKEN.".to_string());
    }

    GoogleAuth::new(config).map_err(|e| format!("Failed to initialize Google auth: {}", e))
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        exit(1);
    }

    let command = &args[1];

    let result = match command.as_str() {
        "list-spreadsheets" => cmd_list_spreadsheets(),
        "read-spreadsheet" => {
            if args.len() < 3 {
                eprintln!("Error: spreadsheet ID required");
                print_usage();
                exit(1);
            }
            cmd_read_spreadsheet(&args[2])
        }
        "read-values" => {
            if args.len() < 4 {
                eprintln!("Error: spreadsheet ID and range required");
                print_usage();
                exit(1);
            }
            cmd_read_values(&args[2], &args[3])
        }
        "get-metadata" => {
            if args.len() < 3 {
                eprintln!("Error: spreadsheet ID required");
                print_usage();
                exit(1);
            }
            cmd_get_metadata(&args[2])
        }
        "list-comments" => {
            if args.len() < 3 {
                eprintln!("Error: spreadsheet ID required");
                print_usage();
                exit(1);
            }
            cmd_list_comments(&args[2])
        }
        "reply-comment" => {
            if args.len() < 5 {
                eprintln!("Error: spreadsheet ID, comment ID, and message required");
                print_usage();
                exit(1);
            }
            cmd_reply_comment(&args[2], &args[3], &args[4])
        }
        "update-values" => {
            if args.len() < 5 {
                eprintln!("Error: spreadsheet ID, range, and JSON values required");
                print_usage();
                exit(1);
            }
            cmd_update_values(&args[2], &args[3], &args[4])
        }
        "append-rows" => {
            if args.len() < 5 {
                eprintln!("Error: spreadsheet ID, range, and JSON values required");
                print_usage();
                exit(1);
            }
            cmd_append_rows(&args[2], &args[3], &args[4])
        }
        "batch-update" => {
            if args.len() < 4 {
                eprintln!("Error: spreadsheet ID and JSON requests required");
                print_usage();
                exit(1);
            }
            cmd_batch_update(&args[2], &args[3])
        }
        "--help" | "-h" | "help" => {
            print_usage();
            exit(0);
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            print_usage();
            exit(1);
        }
    };

    match result {
        Ok(output) => {
            println!("{}", output);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(1);
        }
    }
}

fn cmd_list_spreadsheets() -> Result<String, String> {
    let auth = get_auth()?;
    let adapter = GoogleSheetsInboundAdapter::new(auth, HashSet::new());

    let sheets = adapter
        .list_shared_spreadsheets()
        .map_err(|e| format!("Failed to list spreadsheets: {}", e))?;

    let mut output = String::new();
    output.push_str(&format!("Found {} spreadsheets:\n\n", sheets.len()));
    for sheet in sheets {
        output.push_str(&format!(
            "- {} ({})\n",
            sheet.name.as_deref().unwrap_or("Untitled"),
            sheet.id
        ));
    }
    Ok(output)
}

fn cmd_read_spreadsheet(spreadsheet_id: &str) -> Result<String, String> {
    let auth = get_auth()?;
    let adapter = GoogleSheetsInboundAdapter::new(auth, HashSet::new());

    adapter
        .read_spreadsheet_content(spreadsheet_id)
        .map_err(|e| format!("Failed to read spreadsheet: {}", e))
}

fn cmd_read_values(spreadsheet_id: &str, range: &str) -> Result<String, String> {
    let auth = get_auth()?;
    let adapter = GoogleSheetsInboundAdapter::new(auth, HashSet::new());

    let values = adapter
        .read_spreadsheet_values(spreadsheet_id, range)
        .map_err(|e| format!("Failed to read values: {}", e))?;

    Ok(serde_json::to_string_pretty(&values).unwrap_or_else(|_| values.to_string()))
}

fn cmd_get_metadata(spreadsheet_id: &str) -> Result<String, String> {
    let auth = get_auth()?;
    let adapter = GoogleSheetsOutboundAdapter::new(auth);

    let metadata = adapter
        .get_spreadsheet_metadata(spreadsheet_id)
        .map_err(|e| format!("Failed to get metadata: {}", e))?;

    let mut output = String::new();
    output.push_str("Spreadsheet Metadata:\n\n");

    if let Some(props) = metadata.get("properties") {
        if let Some(title) = props.get("title").and_then(|t| t.as_str()) {
            output.push_str(&format!("Title: {}\n", title));
        }
    }

    if let Some(sheets) = metadata.get("sheets").and_then(|s| s.as_array()) {
        output.push_str(&format!("\nSheets ({}):\n", sheets.len()));
        for sheet in sheets {
            if let Some(props) = sheet.get("properties") {
                let title = props
                    .get("title")
                    .and_then(|t| t.as_str())
                    .unwrap_or("Untitled");
                let sheet_id = props.get("sheetId").and_then(|i| i.as_i64()).unwrap_or(0);
                let index = props.get("index").and_then(|i| i.as_i64()).unwrap_or(0);
                output.push_str(&format!(
                    "  - {} (id={}, index={})\n",
                    title, sheet_id, index
                ));
            }
        }
    }

    Ok(output)
}

fn cmd_list_comments(spreadsheet_id: &str) -> Result<String, String> {
    let auth = get_auth()?;
    let adapter = GoogleSheetsInboundAdapter::new(auth, HashSet::new());

    let comments = adapter
        .list_comments(spreadsheet_id)
        .map_err(|e| format!("Failed to list comments: {}", e))?;

    let mut output = String::new();
    output.push_str(&format!("Found {} comments:\n\n", comments.len()));
    for comment in comments {
        let author = comment
            .author
            .as_ref()
            .and_then(|a| a.display_name.as_deref())
            .unwrap_or("Unknown");
        let resolved = if comment.resolved == Some(true) {
            " [RESOLVED]"
        } else {
            ""
        };
        output.push_str(&format!(
            "- [{}]{} {}: {}\n",
            comment.id, resolved, author, comment.content
        ));

        if let Some(replies) = comment.replies {
            for reply in replies {
                let reply_author = reply
                    .author
                    .as_ref()
                    .and_then(|a| a.display_name.as_deref())
                    .unwrap_or("Unknown");
                output.push_str(&format!(
                    "    └─ [{}] {}: {}\n",
                    reply.id, reply_author, reply.content
                ));
            }
        }
    }
    Ok(output)
}

fn cmd_reply_comment(
    spreadsheet_id: &str,
    comment_id: &str,
    message: &str,
) -> Result<String, String> {
    let auth = get_auth()?;
    let adapter = GoogleSheetsOutboundAdapter::new(auth);

    let reply = adapter
        .reply_to_comment(spreadsheet_id, comment_id, message)
        .map_err(|e| format!("Failed to reply: {}", e))?;

    Ok(format!("Successfully posted reply (id={})", reply.id))
}

fn cmd_update_values(spreadsheet_id: &str, range: &str, json: &str) -> Result<String, String> {
    let auth = get_auth()?;
    let adapter = GoogleSheetsOutboundAdapter::new(auth);

    let values: Vec<Vec<serde_json::Value>> =
        serde_json::from_str(json).map_err(|e| format!("Invalid JSON: {}", e))?;

    adapter
        .update_values(spreadsheet_id, range, values)
        .map_err(|e| format!("Failed to update values: {}", e))?;

    Ok(format!("Successfully updated values at {}", range))
}

fn cmd_append_rows(spreadsheet_id: &str, range: &str, json: &str) -> Result<String, String> {
    let auth = get_auth()?;
    let adapter = GoogleSheetsOutboundAdapter::new(auth);

    let values: Vec<Vec<serde_json::Value>> =
        serde_json::from_str(json).map_err(|e| format!("Invalid JSON: {}", e))?;

    let row_count = values.len();
    adapter
        .append_rows(spreadsheet_id, range, values)
        .map_err(|e| format!("Failed to append rows: {}", e))?;

    Ok(format!(
        "Successfully appended {} row(s) to {}",
        row_count, range
    ))
}

fn cmd_batch_update(spreadsheet_id: &str, json: &str) -> Result<String, String> {
    let auth = get_auth()?;
    let adapter = GoogleSheetsOutboundAdapter::new(auth);

    let requests: Vec<serde_json::Value> =
        serde_json::from_str(json).map_err(|e| format!("Invalid JSON: {}", e))?;

    let response = adapter
        .batch_update(spreadsheet_id, requests)
        .map_err(|e| format!("Failed to batch update: {}", e))?;

    Ok(serde_json::to_string_pretty(&response).unwrap_or_else(|_| "Success".to_string()))
}
