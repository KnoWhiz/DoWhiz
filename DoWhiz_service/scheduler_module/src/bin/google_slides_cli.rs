//! Google Slides CLI tool for Agent to interact with Google Slides.
//!
//! This CLI provides commands for the digital employee to:
//! - List presentations and read content
//! - List and reply to comments
//! - Insert/replace text
//! - Create/delete slides

use scheduler_module::adapters::google_slides::{
    GoogleSlidesInboundAdapter, GoogleSlidesOutboundAdapter,
};
use scheduler_module::google_auth::{GoogleAuth, GoogleAuthConfig};
use std::collections::HashSet;
use std::env;
use std::process::exit;

fn print_usage() {
    eprintln!(
        r##"Usage: google-slides <command> [arguments]

Commands:
  list-presentations                     List all presentations shared with you
  read-presentation <id>                 Read presentation content as plain text
  get-presentation <id>                  Get presentation structure (JSON)

Comment Operations:
  list-comments <id>                     List comments on a presentation
  reply-comment <id> <comment_id> <msg>  Reply to a comment

Edit Operations:
  replace-all-text <id> --find="old" --replace="new" [--match-case]
  insert-text <id> --object-id="shape_id" --text="text" [--index=0]
  create-slide <id> [--layout=BLANK] [--index=0]
  delete-slide <id> --slide-id="slide_object_id"
  batch-update <id> <json>               Send batch update requests

Examples:
  google-slides list-presentations
  google-slides read-presentation 1abc...
  google-slides replace-all-text 1abc... --find="Hello" --replace="Hi"
  google-slides create-slide 1abc... --layout=TITLE_AND_BODY

Environment Variables:
  GOOGLE_ACCESS_TOKEN    - Pre-generated access token (for sandbox environments)
  GOOGLE_CLIENT_ID       - Google OAuth client ID
  GOOGLE_CLIENT_SECRET   - Google OAuth client secret
  GOOGLE_REFRESH_TOKEN   - Google OAuth refresh token

Predefined Layouts:
  BLANK, TITLE, TITLE_AND_BODY, TITLE_AND_TWO_COLUMNS, TITLE_ONLY,
  SECTION_HEADER, ONE_COLUMN_TEXT, MAIN_POINT, BIG_NUMBER
"##
    );
}

fn parse_arg(args: &[String], flag: &str) -> Option<String> {
    for arg in args {
        if arg.starts_with(&format!("{}=", flag)) {
            return arg.split('=').nth(1).map(|s| s.to_string());
        }
        if arg == flag {
            let idx = args.iter().position(|a| a == arg)?;
            return args.get(idx + 1).cloned();
        }
    }
    None
}

fn get_auth() -> Result<GoogleAuth, String> {
    dotenvy::dotenv().ok();

    let mut config = GoogleAuthConfig::from_env();

    if config.access_token.is_none() {
        if let Ok(token) = std::fs::read_to_string(".google_access_token") {
            let token = token.trim().to_string();
            if !token.is_empty() {
                eprintln!("[google-slides] Read access token from .google_access_token file");
                config.access_token = Some(token);
            }
        }
    }

    let has_access_token = config.access_token.is_some();
    let has_refresh_token = config.refresh_token.is_some();
    let has_client_id = config.client_id.is_some();
    eprintln!(
        "[google-slides] Auth config: access_token={}, refresh_token={}, client_id={}, valid={}",
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
        "list-presentations" => cmd_list_presentations(),
        "read-presentation" => {
            if args.len() < 3 {
                eprintln!("Error: presentation ID required");
                print_usage();
                exit(1);
            }
            cmd_read_presentation(&args[2])
        }
        "get-presentation" => {
            if args.len() < 3 {
                eprintln!("Error: presentation ID required");
                print_usage();
                exit(1);
            }
            cmd_get_presentation(&args[2])
        }
        "list-comments" => {
            if args.len() < 3 {
                eprintln!("Error: presentation ID required");
                print_usage();
                exit(1);
            }
            cmd_list_comments(&args[2])
        }
        "reply-comment" => {
            if args.len() < 5 {
                eprintln!("Error: presentation ID, comment ID, and message required");
                print_usage();
                exit(1);
            }
            cmd_reply_comment(&args[2], &args[3], &args[4])
        }
        "replace-all-text" => {
            if args.len() < 3 {
                eprintln!("Error: presentation ID required");
                print_usage();
                exit(1);
            }
            let find = parse_arg(&args, "--find").unwrap_or_default();
            let replace = parse_arg(&args, "--replace").unwrap_or_default();
            if find.is_empty() || replace.is_empty() {
                eprintln!("Error: --find and --replace are required");
                exit(1);
            }
            let match_case = args.iter().any(|a| a == "--match-case");
            cmd_replace_all_text(&args[2], &find, &replace, match_case)
        }
        "insert-text" => {
            if args.len() < 3 {
                eprintln!("Error: presentation ID required");
                print_usage();
                exit(1);
            }
            let object_id = parse_arg(&args, "--object-id").unwrap_or_default();
            let text = parse_arg(&args, "--text").unwrap_or_default();
            if object_id.is_empty() || text.is_empty() {
                eprintln!("Error: --object-id and --text are required");
                exit(1);
            }
            let index = parse_arg(&args, "--index").and_then(|s| s.parse().ok());
            cmd_insert_text(&args[2], &object_id, &text, index)
        }
        "create-slide" => {
            if args.len() < 3 {
                eprintln!("Error: presentation ID required");
                print_usage();
                exit(1);
            }
            let layout = parse_arg(&args, "--layout");
            let index = parse_arg(&args, "--index").and_then(|s| s.parse().ok());
            cmd_create_slide(&args[2], layout.as_deref(), index)
        }
        "delete-slide" => {
            if args.len() < 3 {
                eprintln!("Error: presentation ID required");
                print_usage();
                exit(1);
            }
            let slide_id = parse_arg(&args, "--slide-id").unwrap_or_default();
            if slide_id.is_empty() {
                eprintln!("Error: --slide-id is required");
                exit(1);
            }
            cmd_delete_slide(&args[2], &slide_id)
        }
        "batch-update" => {
            if args.len() < 4 {
                eprintln!("Error: presentation ID and JSON requests required");
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

fn cmd_list_presentations() -> Result<String, String> {
    let auth = get_auth()?;
    let adapter = GoogleSlidesInboundAdapter::new(auth, HashSet::new());

    let presentations = adapter
        .list_shared_presentations()
        .map_err(|e| format!("Failed to list presentations: {}", e))?;

    let mut output = String::new();
    output.push_str(&format!("Found {} presentations:\n\n", presentations.len()));
    for pres in presentations {
        output.push_str(&format!(
            "- {} ({})\n",
            pres.name.as_deref().unwrap_or("Untitled"),
            pres.id
        ));
    }
    Ok(output)
}

fn cmd_read_presentation(presentation_id: &str) -> Result<String, String> {
    let auth = get_auth()?;
    let adapter = GoogleSlidesInboundAdapter::new(auth, HashSet::new());

    adapter
        .read_presentation_content(presentation_id)
        .map_err(|e| format!("Failed to read presentation: {}", e))
}

fn cmd_get_presentation(presentation_id: &str) -> Result<String, String> {
    let auth = get_auth()?;
    let adapter = GoogleSlidesInboundAdapter::new(auth, HashSet::new());

    let presentation = adapter
        .get_presentation(presentation_id)
        .map_err(|e| format!("Failed to get presentation: {}", e))?;

    // Extract useful info
    let mut output = String::new();
    output.push_str("Presentation Structure:\n\n");

    if let Some(title) = presentation.get("title").and_then(|t| t.as_str()) {
        output.push_str(&format!("Title: {}\n", title));
    }

    if let Some(slides) = presentation.get("slides").and_then(|s| s.as_array()) {
        output.push_str(&format!("\nSlides ({}):\n", slides.len()));
        for (i, slide) in slides.iter().enumerate() {
            if let Some(object_id) = slide.get("objectId").and_then(|o| o.as_str()) {
                output.push_str(&format!("  {}. {} ", i + 1, object_id));

                // Try to get slide title or first text
                if let Some(elements) = slide.get("pageElements").and_then(|e| e.as_array()) {
                    for elem in elements {
                        if let Some(shape) = elem.get("shape") {
                            if let Some(text) = shape.get("text") {
                                if let Some(elements) = text.get("textElements").and_then(|e| e.as_array()) {
                                    for te in elements {
                                        if let Some(tr) = te.get("textRun") {
                                            if let Some(content) = tr.get("content").and_then(|c| c.as_str()) {
                                                let preview = content.trim();
                                                if !preview.is_empty() && preview.len() > 1 {
                                                    let short = if preview.len() > 40 {
                                                        format!("{}...", &preview[..40])
                                                    } else {
                                                        preview.to_string()
                                                    };
                                                    output.push_str(&format!("\"{}\"", short));
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                output.push('\n');
            }
        }
    }

    Ok(output)
}

fn cmd_list_comments(presentation_id: &str) -> Result<String, String> {
    let auth = get_auth()?;
    let adapter = GoogleSlidesInboundAdapter::new(auth, HashSet::new());

    let comments = adapter
        .list_comments(presentation_id)
        .map_err(|e| format!("Failed to list comments: {}", e))?;

    let mut output = String::new();
    output.push_str(&format!("Found {} comments:\n\n", comments.len()));
    for comment in comments {
        let author = comment
            .author
            .as_ref()
            .and_then(|a| a.display_name.as_deref())
            .unwrap_or("Unknown");
        let resolved = if comment.resolved == Some(true) { " [RESOLVED]" } else { "" };
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

fn cmd_reply_comment(presentation_id: &str, comment_id: &str, message: &str) -> Result<String, String> {
    let auth = get_auth()?;
    let adapter = GoogleSlidesOutboundAdapter::new(auth);

    let reply = adapter
        .reply_to_comment(presentation_id, comment_id, message)
        .map_err(|e| format!("Failed to reply: {}", e))?;

    Ok(format!("Successfully posted reply (id={})", reply.id))
}

fn cmd_replace_all_text(
    presentation_id: &str,
    find: &str,
    replace: &str,
    match_case: bool,
) -> Result<String, String> {
    let auth = get_auth()?;
    let adapter = GoogleSlidesOutboundAdapter::new(auth);

    adapter
        .replace_all_text(presentation_id, find, replace, match_case)
        .map_err(|e| format!("Failed to replace text: {}", e))?;

    Ok(format!(
        "Successfully replaced all occurrences of \"{}\" with \"{}\"",
        find, replace
    ))
}

fn cmd_insert_text(
    presentation_id: &str,
    object_id: &str,
    text: &str,
    index: Option<i32>,
) -> Result<String, String> {
    let auth = get_auth()?;
    let adapter = GoogleSlidesOutboundAdapter::new(auth);

    adapter
        .insert_text(presentation_id, object_id, text, index)
        .map_err(|e| format!("Failed to insert text: {}", e))?;

    Ok(format!(
        "Successfully inserted \"{}\" into shape {}",
        text, object_id
    ))
}

fn cmd_create_slide(
    presentation_id: &str,
    layout: Option<&str>,
    index: Option<i32>,
) -> Result<String, String> {
    let auth = get_auth()?;
    let adapter = GoogleSlidesOutboundAdapter::new(auth);

    let slide_id = adapter
        .create_slide(presentation_id, None, index, layout)
        .map_err(|e| format!("Failed to create slide: {}", e))?;

    Ok(format!("Successfully created slide (id={})", slide_id))
}

fn cmd_delete_slide(presentation_id: &str, slide_id: &str) -> Result<String, String> {
    let auth = get_auth()?;
    let adapter = GoogleSlidesOutboundAdapter::new(auth);

    adapter
        .delete_slide(presentation_id, slide_id)
        .map_err(|e| format!("Failed to delete slide: {}", e))?;

    Ok(format!("Successfully deleted slide {}", slide_id))
}

fn cmd_batch_update(presentation_id: &str, json: &str) -> Result<String, String> {
    let auth = get_auth()?;
    let adapter = GoogleSlidesOutboundAdapter::new(auth);

    let requests: Vec<serde_json::Value> =
        serde_json::from_str(json).map_err(|e| format!("Invalid JSON: {}", e))?;

    let response = adapter
        .batch_update(presentation_id, requests)
        .map_err(|e| format!("Failed to batch update: {}", e))?;

    Ok(serde_json::to_string_pretty(&response).unwrap_or_else(|_| "Success".to_string()))
}
