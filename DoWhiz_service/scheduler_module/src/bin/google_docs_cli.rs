//! Google Docs CLI tool for Agent to interact with Google Docs.
//!
//! This CLI provides commands for the digital employee to:
//! - Read document content
//! - Apply direct edits
//! - Apply suggestions (revision marks)
//! - Apply or discard suggestions

use scheduler_module::adapters::google_docs::GoogleDocsOutboundAdapter;
use scheduler_module::google_auth::{GoogleAuth, GoogleAuthConfig};
use std::env;
use std::process::exit;

fn print_usage() {
    eprintln!(
        r##"Usage: google-docs <command> [arguments]

Commands:
  list-documents                     List all documents shared with you
  read-document <doc_id>             Read document content as plain text
  list-comments <doc_id>             List comments on a document
  read-comment <doc_id> <comment_id> Read a specific comment
  reply-comment <doc_id> <comment_id> <message>  Reply to a comment

Direct Edit Operations:
  apply-edit <doc_id> --find="text" --replace="new text"
  insert-text <doc_id> --after="anchor" --text="text to insert"
  delete-text <doc_id> --find="text to delete"
  insert-image <doc_id> --url="https://..." [--after="anchor text"] [--index=1] [--width=200] [--height=150]

Image Search (Unsplash):
  search-image --query="landscape mountains" [--count=5] [--orientation=landscape|portrait|squarish]

Style Operations:
  get-styles <doc_id>                Get existing styles from document (headings, fonts, colors)
  set-style <doc_id> --find="text" [--color="#FF0000"] [--font="Arial"] [--size=12] [--bold] [--italic]

Suggesting Mode Operations:
  mark-deletion <doc_id> --find="text to mark"
  insert-suggestion <doc_id> --after="anchor" --text="suggestion text"
  suggest-replace <doc_id> --find="old text" --replace="new text"
  apply-suggestions <doc_id>
  discard-suggestions <doc_id>

Environment Variables:
  GOOGLE_ACCESS_TOKEN    - Pre-generated access token (for sandbox environments)
  GOOGLE_CLIENT_ID       - Google OAuth client ID
  GOOGLE_CLIENT_SECRET   - Google OAuth client secret
  GOOGLE_REFRESH_TOKEN   - Google OAuth refresh token
  EMPLOYEE_ID            - (optional) Employee ID for per-employee tokens
  UNSPLASH_ACCESS_KEY    - (optional) Unsplash API key for image search

Note: In sandbox environments without network access, set GOOGLE_ACCESS_TOKEN
      to a pre-generated token. This avoids the need for OAuth token refresh.

Style Tips:
  - Use get-styles first to see what styles exist in the document
  - Match existing heading colors and fonts for consistency
  - Color format: "#RRGGBB" (e.g., "#1B263B" for dark blue)
"##
    );
}

/// Process escape sequences in a string (e.g., \n -> newline, \t -> tab)
fn unescape_string(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.peek() {
                Some('n') => {
                    result.push('\n');
                    chars.next();
                }
                Some('t') => {
                    result.push('\t');
                    chars.next();
                }
                Some('r') => {
                    result.push('\r');
                    chars.next();
                }
                Some('\\') => {
                    result.push('\\');
                    chars.next();
                }
                _ => result.push(c),
            }
        } else {
            result.push(c);
        }
    }
    result
}

fn parse_arg(args: &[String], flag: &str) -> Option<String> {
    for arg in args {
        if arg.starts_with(&format!("{}=", flag)) {
            let value = arg.split('=').nth(1)?.to_string();
            return Some(unescape_string(&value));
        }
        // Also handle --flag "value" style
        if arg == flag {
            // Find next arg
            let idx = args.iter().position(|a| a == arg)?;
            return args.get(idx + 1).map(|s| unescape_string(s));
        }
    }
    None
}

fn get_auth() -> Result<GoogleAuth, String> {
    dotenvy::dotenv().ok();

    let mut config = GoogleAuthConfig::from_env();

    // If no access token from env, try reading from file in current directory
    // (Codex sandbox may not pass environment variables to tools it spawns)
    if config.access_token.is_none() {
        if let Ok(token) = std::fs::read_to_string(".google_access_token") {
            let token = token.trim().to_string();
            if !token.is_empty() {
                eprintln!("[google-docs] Read access token from .google_access_token file");
                config.access_token = Some(token);
            }
        }
    }

    // Debug: show what credentials are available
    let has_access_token = config.access_token.is_some();
    let has_refresh_token = config.refresh_token.is_some();
    let has_client_id = config.client_id.is_some();
    eprintln!(
        "[google-docs] Auth config: access_token={}, refresh_token={}, client_id={}, valid={}",
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
        "list-documents" => cmd_list_documents(),
        "read-document" => {
            if args.len() < 3 {
                eprintln!("Error: document ID required");
                print_usage();
                exit(1);
            }
            cmd_read_document(&args[2])
        }
        "list-comments" => {
            if args.len() < 3 {
                eprintln!("Error: document ID required");
                print_usage();
                exit(1);
            }
            cmd_list_comments(&args[2])
        }
        "reply-comment" => {
            if args.len() < 5 {
                eprintln!("Error: document ID, comment ID, and message required");
                print_usage();
                exit(1);
            }
            cmd_reply_comment(&args[2], &args[3], &args[4])
        }
        "apply-edit" => {
            if args.len() < 3 {
                eprintln!("Error: document ID required");
                print_usage();
                exit(1);
            }
            let find = parse_arg(&args, "--find").unwrap_or_default();
            let replace = parse_arg(&args, "--replace").unwrap_or_default();
            if find.is_empty() || replace.is_empty() {
                eprintln!("Error: --find and --replace are required");
                exit(1);
            }
            cmd_apply_edit(&args[2], &find, &replace)
        }
        "insert-text" => {
            if args.len() < 3 {
                eprintln!("Error: document ID required");
                print_usage();
                exit(1);
            }
            let after = parse_arg(&args, "--after").unwrap_or_default();
            let text = parse_arg(&args, "--text").unwrap_or_default();
            if after.is_empty() || text.is_empty() {
                eprintln!("Error: --after and --text are required");
                exit(1);
            }
            cmd_insert_text(&args[2], &after, &text)
        }
        "delete-text" => {
            if args.len() < 3 {
                eprintln!("Error: document ID required");
                print_usage();
                exit(1);
            }
            let find = parse_arg(&args, "--find").unwrap_or_default();
            if find.is_empty() {
                eprintln!("Error: --find is required");
                exit(1);
            }
            cmd_delete_text(&args[2], &find)
        }
        "insert-image" => {
            if args.len() < 3 {
                eprintln!("Error: document ID required");
                print_usage();
                exit(1);
            }
            let url = parse_arg(&args, "--url").unwrap_or_default();
            if url.is_empty() {
                eprintln!("Error: --url is required");
                exit(1);
            }
            let after = parse_arg(&args, "--after");
            let index = parse_arg(&args, "--index").and_then(|s| s.parse().ok());
            let width = parse_arg(&args, "--width").and_then(|s| s.parse().ok());
            let height = parse_arg(&args, "--height").and_then(|s| s.parse().ok());
            cmd_insert_image(&args[2], &url, after.as_deref(), index, width, height)
        }
        "mark-deletion" => {
            if args.len() < 3 {
                eprintln!("Error: document ID required");
                print_usage();
                exit(1);
            }
            let find = parse_arg(&args, "--find").unwrap_or_default();
            if find.is_empty() {
                eprintln!("Error: --find is required");
                exit(1);
            }
            cmd_mark_deletion(&args[2], &find)
        }
        "insert-suggestion" => {
            if args.len() < 3 {
                eprintln!("Error: document ID required");
                print_usage();
                exit(1);
            }
            let after = parse_arg(&args, "--after").unwrap_or_default();
            let text = parse_arg(&args, "--text").unwrap_or_default();
            if after.is_empty() || text.is_empty() {
                eprintln!("Error: --after and --text are required");
                exit(1);
            }
            cmd_insert_suggestion(&args[2], &after, &text)
        }
        "suggest-replace" => {
            if args.len() < 3 {
                eprintln!("Error: document ID required");
                print_usage();
                exit(1);
            }
            let find = parse_arg(&args, "--find").unwrap_or_default();
            let replace = parse_arg(&args, "--replace").unwrap_or_default();
            if find.is_empty() || replace.is_empty() {
                eprintln!("Error: --find and --replace are required");
                exit(1);
            }
            cmd_suggest_replace(&args[2], &find, &replace)
        }
        "apply-suggestions" => {
            if args.len() < 3 {
                eprintln!("Error: document ID required");
                print_usage();
                exit(1);
            }
            cmd_apply_suggestions(&args[2])
        }
        "discard-suggestions" => {
            if args.len() < 3 {
                eprintln!("Error: document ID required");
                print_usage();
                exit(1);
            }
            cmd_discard_suggestions(&args[2])
        }
        "get-styles" => {
            if args.len() < 3 {
                eprintln!("Error: document ID required");
                print_usage();
                exit(1);
            }
            cmd_get_styles(&args[2])
        }
        "set-style" => {
            if args.len() < 3 {
                eprintln!("Error: document ID required");
                print_usage();
                exit(1);
            }
            let find = parse_arg(&args, "--find").unwrap_or_default();
            if find.is_empty() {
                eprintln!("Error: --find is required");
                exit(1);
            }
            let color = parse_arg(&args, "--color");
            let font = parse_arg(&args, "--font");
            let size = parse_arg(&args, "--size").and_then(|s| s.parse::<f64>().ok());
            let bold = args.iter().any(|a| a == "--bold");
            let italic = args.iter().any(|a| a == "--italic");
            let no_bold = args.iter().any(|a| a == "--no-bold");
            let no_italic = args.iter().any(|a| a == "--no-italic");

            let bold_opt = if bold {
                Some(true)
            } else if no_bold {
                Some(false)
            } else {
                None
            };
            let italic_opt = if italic {
                Some(true)
            } else if no_italic {
                Some(false)
            } else {
                None
            };

            cmd_set_style(
                &args[2],
                &find,
                color.as_deref(),
                font.as_deref(),
                size,
                bold_opt,
                italic_opt,
            )
        }
        "search-image" => {
            let query = parse_arg(&args, "--query").unwrap_or_default();
            if query.is_empty() {
                eprintln!("Error: --query is required");
                exit(1);
            }
            let count = parse_arg(&args, "--count").and_then(|s| s.parse().ok());
            let orientation = parse_arg(&args, "--orientation");
            cmd_search_image(&query, count, orientation.as_deref())
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

fn cmd_list_documents() -> Result<String, String> {
    let auth = get_auth()?;
    let adapter = scheduler_module::adapters::google_docs::GoogleDocsInboundAdapter::new(
        auth,
        std::collections::HashSet::new(),
    );

    let docs = adapter
        .list_shared_documents()
        .map_err(|e| format!("Failed to list documents: {}", e))?;

    let mut output = String::new();
    output.push_str(&format!("Found {} documents:\n\n", docs.len()));
    for doc in docs {
        output.push_str(&format!(
            "- {} ({})\n",
            doc.name.as_deref().unwrap_or("Untitled"),
            doc.id
        ));
    }
    Ok(output)
}

fn cmd_read_document(doc_id: &str) -> Result<String, String> {
    let auth = get_auth()?;
    let adapter = scheduler_module::adapters::google_docs::GoogleDocsInboundAdapter::new(
        auth,
        std::collections::HashSet::new(),
    );

    adapter
        .read_document_content(doc_id)
        .map_err(|e| format!("Failed to read document: {}", e))
}

fn cmd_list_comments(doc_id: &str) -> Result<String, String> {
    let auth = get_auth()?;
    let adapter = scheduler_module::adapters::google_docs::GoogleDocsInboundAdapter::new(
        auth,
        std::collections::HashSet::new(),
    );

    let comments = adapter
        .list_comments(doc_id)
        .map_err(|e| format!("Failed to list comments: {}", e))?;

    let mut output = String::new();
    output.push_str(&format!("Found {} comments:\n\n", comments.len()));
    for comment in comments {
        let author = comment
            .author
            .as_ref()
            .and_then(|a| a.display_name.as_deref())
            .unwrap_or("Unknown");
        output.push_str(&format!(
            "- [{}] {}: {}\n",
            comment.id, author, comment.content
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

fn cmd_reply_comment(doc_id: &str, comment_id: &str, message: &str) -> Result<String, String> {
    let auth = get_auth()?;
    let adapter = GoogleDocsOutboundAdapter::new(auth);

    let reply = adapter
        .reply_to_comment(doc_id, comment_id, message)
        .map_err(|e| format!("Failed to reply: {}", e))?;

    Ok(format!("Successfully posted reply (id={})", reply.id))
}

fn cmd_apply_edit(doc_id: &str, find: &str, replace: &str) -> Result<String, String> {
    let auth = get_auth()?;
    let adapter = GoogleDocsOutboundAdapter::new(auth);

    // For direct edit, we use suggest_replace then apply_suggestions
    adapter
        .suggest_replace(doc_id, find, replace)
        .map_err(|e| format!("Failed to mark edit: {}", e))?;

    adapter
        .apply_suggestions(doc_id)
        .map_err(|e| format!("Failed to apply edit: {}", e))?;

    Ok(format!(
        "Successfully replaced \"{}\" with \"{}\"",
        find, replace
    ))
}

fn cmd_insert_text(doc_id: &str, after: &str, text: &str) -> Result<String, String> {
    let auth = get_auth()?;
    let adapter = GoogleDocsOutboundAdapter::new(auth);

    // For direct insert, add as suggestion then apply
    adapter
        .insert_suggestion(doc_id, after, text)
        .map_err(|e| format!("Failed to mark insertion: {}", e))?;

    adapter
        .apply_suggestions(doc_id)
        .map_err(|e| format!("Failed to apply insertion: {}", e))?;

    Ok(format!(
        "Successfully inserted \"{}\" after \"{}\"",
        text, after
    ))
}

fn cmd_delete_text(doc_id: &str, find: &str) -> Result<String, String> {
    let auth = get_auth()?;
    let adapter = GoogleDocsOutboundAdapter::new(auth);

    // For direct delete, mark for deletion then apply
    adapter
        .mark_deletion(doc_id, find)
        .map_err(|e| format!("Failed to mark deletion: {}", e))?;

    adapter
        .apply_suggestions(doc_id)
        .map_err(|e| format!("Failed to apply deletion: {}", e))?;

    Ok(format!("Successfully deleted \"{}\"", find))
}

fn cmd_insert_image(
    doc_id: &str,
    url: &str,
    after: Option<&str>,
    index: Option<i64>,
    width: Option<f64>,
    height: Option<f64>,
) -> Result<String, String> {
    let auth = get_auth()?;
    let adapter = GoogleDocsOutboundAdapter::new(auth);

    let result = if let Some(after_text) = after {
        // Insert after specific text
        adapter
            .insert_image_after_text(doc_id, url, after_text, width, height)
            .map_err(|e| format!("Failed to insert image: {}", e))?
    } else if let Some(idx) = index {
        // Insert at specific index
        adapter
            .insert_image(doc_id, url, idx, width, height)
            .map_err(|e| format!("Failed to insert image: {}", e))?
    } else {
        // Insert at end of document
        let end_idx = adapter
            .get_document_end_index(doc_id)
            .map_err(|e| format!("Failed to get document end index: {}", e))?;
        adapter
            .insert_image(doc_id, url, end_idx, width, height)
            .map_err(|e| format!("Failed to insert image: {}", e))?
    };

    let location = if let Some(after_text) = after {
        format!("after \"{}\"", after_text)
    } else if let Some(idx) = index {
        format!("at index {}", idx)
    } else {
        "at end of document".to_string()
    };

    Ok(format!(
        "Successfully inserted image {} (id={})",
        location, result
    ))
}

fn cmd_mark_deletion(doc_id: &str, find: &str) -> Result<String, String> {
    let auth = get_auth()?;
    let adapter = GoogleDocsOutboundAdapter::new(auth);

    adapter
        .mark_deletion(doc_id, find)
        .map_err(|e| format!("Failed to mark deletion: {}", e))?;

    Ok(format!(
        "Successfully marked \"{}\" for deletion (red strikethrough)",
        find
    ))
}

fn cmd_insert_suggestion(doc_id: &str, after: &str, text: &str) -> Result<String, String> {
    let auth = get_auth()?;
    let adapter = GoogleDocsOutboundAdapter::new(auth);

    adapter
        .insert_suggestion(doc_id, after, text)
        .map_err(|e| format!("Failed to insert suggestion: {}", e))?;

    Ok(format!(
        "Successfully inserted suggestion \"{}\" (blue) after \"{}\"",
        text, after
    ))
}

fn cmd_suggest_replace(doc_id: &str, find: &str, replace: &str) -> Result<String, String> {
    let auth = get_auth()?;
    let adapter = GoogleDocsOutboundAdapter::new(auth);

    adapter
        .suggest_replace(doc_id, find, replace)
        .map_err(|e| format!("Failed to suggest replacement: {}", e))?;

    Ok(format!(
        "Successfully suggested replacing \"{}\" (red strikethrough) with \"{}\" (blue)",
        find, replace
    ))
}

fn cmd_apply_suggestions(doc_id: &str) -> Result<String, String> {
    let auth = get_auth()?;
    let adapter = GoogleDocsOutboundAdapter::new(auth);

    adapter
        .apply_suggestions(doc_id)
        .map_err(|e| format!("Failed to apply suggestions: {}", e))?;

    Ok(
        "Successfully applied all suggestions (deleted red text, normalized blue text to black)"
            .to_string(),
    )
}

fn cmd_discard_suggestions(doc_id: &str) -> Result<String, String> {
    let auth = get_auth()?;
    let adapter = GoogleDocsOutboundAdapter::new(auth);

    adapter
        .discard_suggestions(doc_id)
        .map_err(|e| format!("Failed to discard suggestions: {}", e))?;

    Ok("Successfully discarded all suggestions (deleted blue text, restored red text)".to_string())
}

fn cmd_get_styles(doc_id: &str) -> Result<String, String> {
    let auth = get_auth()?;
    let adapter = GoogleDocsOutboundAdapter::new(auth);

    let styles = adapter
        .get_document_styles(doc_id)
        .map_err(|e| format!("Failed to get document styles: {}", e))?;

    let mut output = String::new();
    output.push_str("Document Styles:\n\n");

    // Helper to format style info
    fn format_style(
        name: &str,
        style: &Option<scheduler_module::adapters::google_docs::TextStyleInfo>,
    ) -> String {
        match style {
            Some(s) => {
                let mut parts = Vec::new();
                if let Some(c) = &s.foreground_color {
                    parts.push(format!("color={}", c));
                }
                if let Some(f) = &s.font_family {
                    parts.push(format!("font=\"{}\"", f));
                }
                if let Some(sz) = s.font_size {
                    parts.push(format!("size={}pt", sz));
                }
                if let Some(true) = s.bold {
                    parts.push("bold".to_string());
                }
                if let Some(true) = s.italic {
                    parts.push("italic".to_string());
                }
                if parts.is_empty() {
                    format!("{}: (default)\n", name)
                } else {
                    format!("{}: {}\n", name, parts.join(", "))
                }
            }
            None => format!("{}: (not defined)\n", name),
        }
    }

    output.push_str("=== Named Styles ===\n");
    output.push_str(&format_style("Title", &styles.title));
    output.push_str(&format_style("Subtitle", &styles.subtitle));
    output.push_str(&format_style("Heading 1", &styles.heading_1));
    output.push_str(&format_style("Heading 2", &styles.heading_2));
    output.push_str(&format_style("Heading 3", &styles.heading_3));
    output.push_str(&format_style("Heading 4", &styles.heading_4));
    output.push_str(&format_style("Normal Text", &styles.normal_text));

    output.push_str("\n=== Actual Styles Found in Document ===\n");
    if let Some((text, style)) = &styles.heading_1_sample {
        let preview = if text.len() > 40 {
            format!("{}...", &text[..40])
        } else {
            text.clone()
        };
        output.push_str(&format!("H1 Sample: \"{}\"\n", preview));
        output.push_str(&format!(
            "  -> {}\n",
            format_style("Style", &Some(style.clone())).trim()
        ));
    }
    if let Some((text, style)) = &styles.heading_2_sample {
        let preview = if text.len() > 40 {
            format!("{}...", &text[..40])
        } else {
            text.clone()
        };
        output.push_str(&format!("H2 Sample: \"{}\"\n", preview));
        output.push_str(&format!(
            "  -> {}\n",
            format_style("Style", &Some(style.clone())).trim()
        ));
    }
    if let Some((text, style)) = &styles.heading_3_sample {
        let preview = if text.len() > 40 {
            format!("{}...", &text[..40])
        } else {
            text.clone()
        };
        output.push_str(&format!("H3 Sample: \"{}\"\n", preview));
        output.push_str(&format!(
            "  -> {}\n",
            format_style("Style", &Some(style.clone())).trim()
        ));
    }

    output.push_str("\n=== Usage Tips ===\n");
    output.push_str("To apply consistent styles, use the colors and fonts shown above.\n");
    output.push_str("Example: google-docs set-style <doc_id> --find=\"My Heading\" --color=\"#1B263B\" --bold\n");

    Ok(output)
}

fn cmd_set_style(
    doc_id: &str,
    find: &str,
    color: Option<&str>,
    font: Option<&str>,
    size: Option<f64>,
    bold: Option<bool>,
    italic: Option<bool>,
) -> Result<String, String> {
    let auth = get_auth()?;
    let adapter = GoogleDocsOutboundAdapter::new(auth);

    // At least one style property must be specified
    if color.is_none() && font.is_none() && size.is_none() && bold.is_none() && italic.is_none() {
        return Err("At least one style property must be specified (--color, --font, --size, --bold, --italic)".to_string());
    }

    adapter
        .set_text_style(doc_id, find, color, font, size, bold, italic)
        .map_err(|e| format!("Failed to set style: {}", e))?;

    let mut applied = Vec::new();
    if let Some(c) = color {
        applied.push(format!("color={}", c));
    }
    if let Some(f) = font {
        applied.push(format!("font=\"{}\"", f));
    }
    if let Some(s) = size {
        applied.push(format!("size={}pt", s));
    }
    if let Some(true) = bold {
        applied.push("bold".to_string());
    }
    if let Some(false) = bold {
        applied.push("no-bold".to_string());
    }
    if let Some(true) = italic {
        applied.push("italic".to_string());
    }
    if let Some(false) = italic {
        applied.push("no-italic".to_string());
    }

    Ok(format!(
        "Successfully applied style to \"{}\": {}",
        find,
        applied.join(", ")
    ))
}

fn cmd_search_image(
    query: &str,
    count: Option<u32>,
    orientation: Option<&str>,
) -> Result<String, String> {
    dotenvy::dotenv().ok();

    let client = scheduler_module::adapters::image_search::UnsplashClient::from_env()?;

    let results = client
        .search_images(query, count, orientation)
        .map_err(|e| format!("Failed to search images: {}", e))?;

    if results.results.is_empty() {
        return Ok(format!("No images found for query: {}", query));
    }

    let mut output = String::new();
    output.push_str(&format!(
        "Found {} images for \"{}\" (showing {}):\n\n",
        results.total,
        query,
        results.results.len()
    ));

    for (i, image) in results.results.iter().enumerate() {
        let description = image.get_description();
        let desc_preview = if description.len() > 60 {
            format!("{}...", &description[..60])
        } else {
            description.clone()
        };

        output.push_str(&format!("{}. {}\n", i + 1, desc_preview));
        output.push_str(&format!("   ID: {}\n", image.id));
        output.push_str(&format!("   Size: {}x{} ({})\n",
            image.width,
            image.height,
            if image.width > image.height { "landscape" }
            else if image.height > image.width { "portrait" }
            else { "square" }
        ));
        output.push_str(&format!("   URL (regular): {}\n", image.urls.regular));
        output.push_str(&format!("   Attribution: {}\n", image.get_attribution()));
        output.push_str("\n");
    }

    output.push_str("Usage: Copy the URL and use insert-image to add to document:\n");
    output.push_str("  google-docs insert-image <doc_id> --url=\"<URL>\" --after=\"anchor text\"\n");

    Ok(output)
}
