//! Google Slides CLI tool for Agent to interact with Google Slides.
//!
//! This CLI provides commands for the digital employee to:
//! - List presentations and read content
//! - List and reply to comments
//! - Insert/replace text
//! - Create/delete slides

use scheduler_module::adapters::google_common::{GoogleDriveClient, PermissionRole};
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
  get-presentation <id> [--json|--analyze]  Get presentation structure
    --json     Output raw JSON
    --analyze  Show detailed element analysis with sizes and capacities
  analyze-slide <id> <slide_id>          Analyze a specific slide's layout and capacity

Comment Operations:
  list-comments <id>                     List comments on a presentation
  reply-comment <id> <comment_id> <msg>  Reply to a comment

Edit Operations:
  replace-all-text <id> --find="old" --replace="new" [--match-case]
  insert-text <id> --object-id="shape_id" --text="text" [--index=0]
  create-slide <id> [--layout=BLANK] [--index=0]
  delete-slide <id> --slide-id="slide_object_id"
  insert-image <id> --url="https://..." --page-id="slide_id" [--x=0] [--y=0] [--width=200] [--height=200]
  batch-update <id> <json>               Send batch update requests

Smart Layout:
  find-space <id> <slide_id> [--min-width=100] [--min-height=100]  Find available space on slide
  search-image --query="keyword" [--count=5] [--orientation=landscape|portrait|squarish]

Presentation Management:
  create-presentation --title="My Presentation"  Create a new presentation

Sharing & Permissions:
  share <file_id> --email="user@example.com" --role="writer" [--notify]
  get-link <file_id>                           Get shareable link for a file
  list-permissions <file_id>                   List who has access to a file
  remove-permission <file_id> <permission_id>  Remove access from a file

Examples:
  google-slides list-presentations
  google-slides read-presentation 1abc...
  google-slides get-presentation 1abc... --analyze
  google-slides analyze-slide 1abc... p.SLIDE_ID
  google-slides replace-all-text 1abc... --find="Hello" --replace="Hi"
  google-slides create-slide 1abc... --layout=TITLE_AND_BODY
  google-slides insert-image 1abc... --url="https://example.com/image.png" --page-id=p.abc123

Environment Variables:
  GOOGLE_ACCESS_TOKEN    - Pre-generated access token (for sandbox environments)
  GOOGLE_CLIENT_ID       - Google OAuth client ID
  GOOGLE_CLIENT_SECRET   - Google OAuth client secret
  GOOGLE_REFRESH_TOKEN   - Google OAuth refresh token
  UNSPLASH_ACCESS_KEY    - (optional) Unsplash API key for image search

Predefined Layouts:
  BLANK, TITLE, TITLE_AND_BODY, TITLE_AND_TWO_COLUMNS, TITLE_ONLY,
  SECTION_HEADER, ONE_COLUMN_TEXT, MAIN_POINT, BIG_NUMBER

Text Capacity Guidelines:
  - Title placeholders: ~50 characters recommended
  - Subtitle: ~80 characters recommended
  - Body text: ~500 characters per text box
  - Use --analyze to see actual capacities based on element sizes
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

fn has_flag(args: &[String], flag: &str) -> bool {
    args.iter().any(|a| a == flag)
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
            let json_mode = has_flag(&args, "--json");
            let analyze_mode = has_flag(&args, "--analyze");
            cmd_get_presentation(&args[2], json_mode, analyze_mode)
        }
        "analyze-slide" => {
            if args.len() < 4 {
                eprintln!("Error: presentation ID and slide ID required");
                print_usage();
                exit(1);
            }
            cmd_analyze_slide(&args[2], &args[3])
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
        "insert-image" => {
            if args.len() < 3 {
                eprintln!("Error: presentation ID required");
                print_usage();
                exit(1);
            }
            let url = parse_arg(&args, "--url").unwrap_or_default();
            let page_id = parse_arg(&args, "--page-id").unwrap_or_default();
            if url.is_empty() || page_id.is_empty() {
                eprintln!("Error: --url and --page-id are required");
                exit(1);
            }
            let x = parse_arg(&args, "--x")
                .and_then(|s| s.parse().ok())
                .unwrap_or(100.0);
            let y = parse_arg(&args, "--y")
                .and_then(|s| s.parse().ok())
                .unwrap_or(100.0);
            let width = parse_arg(&args, "--width").and_then(|s| s.parse().ok());
            let height = parse_arg(&args, "--height").and_then(|s| s.parse().ok());
            cmd_insert_image(&args[2], &url, &page_id, x, y, width, height)
        }
        "batch-update" => {
            if args.len() < 4 {
                eprintln!("Error: presentation ID and JSON requests required");
                print_usage();
                exit(1);
            }
            cmd_batch_update(&args[2], &args[3])
        }
        "find-space" => {
            if args.len() < 4 {
                eprintln!("Error: presentation ID and slide ID required");
                print_usage();
                exit(1);
            }
            let min_width = parse_arg(&args, "--min-width")
                .and_then(|s| s.parse().ok())
                .unwrap_or(100.0);
            let min_height = parse_arg(&args, "--min-height")
                .and_then(|s| s.parse().ok())
                .unwrap_or(100.0);
            cmd_find_space(&args[2], &args[3], min_width, min_height)
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
        "create-presentation" => {
            let title = parse_arg(&args, "--title").unwrap_or_else(|| "Untitled Presentation".to_string());
            cmd_create_presentation(&title)
        }
        "share" => {
            if args.len() < 3 {
                eprintln!("Error: file ID required");
                print_usage();
                exit(1);
            }
            let email = parse_arg(&args, "--email").unwrap_or_default();
            let role = parse_arg(&args, "--role").unwrap_or_else(|| "writer".to_string());
            if email.is_empty() {
                eprintln!("Error: --email is required");
                exit(1);
            }
            let notify = has_flag(&args, "--notify");
            cmd_share_file(&args[2], &email, &role, notify)
        }
        "get-link" => {
            if args.len() < 3 {
                eprintln!("Error: file ID required");
                print_usage();
                exit(1);
            }
            cmd_get_link(&args[2])
        }
        "list-permissions" => {
            if args.len() < 3 {
                eprintln!("Error: file ID required");
                print_usage();
                exit(1);
            }
            cmd_list_permissions(&args[2])
        }
        "remove-permission" => {
            if args.len() < 4 {
                eprintln!("Error: file ID and permission ID required");
                print_usage();
                exit(1);
            }
            cmd_remove_permission(&args[2], &args[3])
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

fn cmd_get_presentation(
    presentation_id: &str,
    json_mode: bool,
    analyze_mode: bool,
) -> Result<String, String> {
    let auth = get_auth()?;
    let adapter = GoogleSlidesInboundAdapter::new(auth, HashSet::new());

    let presentation = adapter
        .get_presentation(presentation_id)
        .map_err(|e| format!("Failed to get presentation: {}", e))?;

    // Raw JSON output
    if json_mode {
        return serde_json::to_string_pretty(&presentation)
            .map_err(|e| format!("Failed to serialize: {}", e));
    }

    let mut output = String::new();
    output.push_str("Presentation Structure:\n\n");

    if let Some(title) = presentation.get("title").and_then(|t| t.as_str()) {
        output.push_str(&format!("Title: {}\n", title));
    }

    // Extract page size
    if let Some(page_size) = presentation.get("pageSize") {
        let width = extract_dimension(page_size.get("width"));
        let height = extract_dimension(page_size.get("height"));
        output.push_str(&format!(
            "Page Size: {:.0}pt x {:.0}pt ({:.1}\" x {:.1}\")\n",
            width,
            height,
            width / 72.0,
            height / 72.0
        ));
    }

    if let Some(slides) = presentation.get("slides").and_then(|s| s.as_array()) {
        output.push_str(&format!("\nSlides ({}):\n", slides.len()));

        for (i, slide) in slides.iter().enumerate() {
            if let Some(object_id) = slide.get("objectId").and_then(|o| o.as_str()) {
                output.push_str(&format!("\n  {}. Slide: {}\n", i + 1, object_id));

                if let Some(elements) = slide.get("pageElements").and_then(|e| e.as_array()) {
                    if analyze_mode {
                        // Detailed element analysis
                        for elem in elements {
                            if let Some(elem_id) = elem.get("objectId").and_then(|o| o.as_str()) {
                                let elem_type = get_element_type(elem);
                                let (x, y, width, height) = extract_element_bounds(elem);

                                output.push_str(&format!("     - {} ({})\n", elem_id, elem_type));
                                output.push_str(&format!(
                                    "       Position: ({:.0}, {:.0}), Size: {:.0} x {:.0} pt\n",
                                    x, y, width, height
                                ));

                                // For shapes with text, show text info
                                if let Some(shape) = elem.get("shape") {
                                    if let Some(placeholder) = shape.get("placeholder") {
                                        if let Some(ptype) =
                                            placeholder.get("type").and_then(|t| t.as_str())
                                        {
                                            output.push_str(&format!(
                                                "       Placeholder: {}\n",
                                                ptype
                                            ));
                                        }
                                    }

                                    if let Some(text) = shape.get("text") {
                                        let (text_content, char_count, font_size) =
                                            extract_text_info(text);
                                        let capacity =
                                            estimate_text_capacity(width, height, font_size);

                                        output.push_str(&format!(
                                            "       Font Size: {:.0}pt\n",
                                            font_size
                                        ));
                                        output.push_str(&format!(
                                            "       Text: {} chars",
                                            char_count
                                        ));
                                        if capacity > 0 {
                                            let usage_pct = (char_count as f64 / capacity as f64
                                                * 100.0)
                                                .min(100.0);
                                            output.push_str(&format!(
                                                " / ~{} capacity ({:.0}% used)",
                                                capacity, usage_pct
                                            ));
                                        }
                                        output.push('\n');

                                        if !text_content.is_empty() {
                                            let preview = if text_content.len() > 60 {
                                                format!("{}...", &text_content[..60])
                                            } else {
                                                text_content
                                            };
                                            output.push_str(&format!(
                                                "       Content: \"{}\"\n",
                                                preview.replace('\n', "\\n")
                                            ));
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        // Simple output - just show first text
                        for elem in elements {
                            if let Some(shape) = elem.get("shape") {
                                if let Some(text) = shape.get("text") {
                                    if let Some(elements) =
                                        text.get("textElements").and_then(|e| e.as_array())
                                    {
                                        for te in elements {
                                            if let Some(tr) = te.get("textRun") {
                                                if let Some(content) =
                                                    tr.get("content").and_then(|c| c.as_str())
                                                {
                                                    let preview = content.trim();
                                                    if !preview.is_empty() && preview.len() > 1 {
                                                        let short = if preview.len() > 40 {
                                                            format!("{}...", &preview[..40])
                                                        } else {
                                                            preview.to_string()
                                                        };
                                                        output.push_str(&format!(
                                                            "     \"{}\"",
                                                            short
                                                        ));
                                                        break;
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
        }
    }

    if analyze_mode {
        output.push_str("\n--- Capacity Guidelines ---\n");
        output.push_str(
            "Title (24pt): ~50 chars | Subtitle (18pt): ~80 chars | Body (14pt): ~500 chars\n",
        );
        output.push_str("Warning: >80% capacity may cause text overflow\n");
    }

    Ok(output)
}

/// Extract dimension in points from Google Slides dimension object
fn extract_dimension(dim: Option<&serde_json::Value>) -> f64 {
    dim.and_then(|d| {
        let magnitude = d.get("magnitude").and_then(|m| m.as_f64()).unwrap_or(0.0);
        let unit = d.get("unit").and_then(|u| u.as_str()).unwrap_or("PT");
        Some(match unit {
            "EMU" => magnitude / 914400.0 * 72.0, // EMU to points
            "PT" => magnitude,
            _ => magnitude,
        })
    })
    .unwrap_or(0.0)
}

/// Get element type string
fn get_element_type(elem: &serde_json::Value) -> &'static str {
    if elem.get("shape").is_some() {
        "Shape"
    } else if elem.get("image").is_some() {
        "Image"
    } else if elem.get("table").is_some() {
        "Table"
    } else if elem.get("line").is_some() {
        "Line"
    } else if elem.get("video").is_some() {
        "Video"
    } else {
        "Unknown"
    }
}

/// Extract element position and size from transform
/// Note: Images have size stored with scaleX/scaleY transforms that need to be applied
fn extract_element_bounds(elem: &serde_json::Value) -> (f64, f64, f64, f64) {
    let transform = elem.get("transform");
    let size = elem.get("size");

    // Get translation (position) - always in EMU
    let translate_x = transform
        .and_then(|t| t.get("translateX"))
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0)
        / 914400.0
        * 72.0; // EMU to points

    let translate_y = transform
        .and_then(|t| t.get("translateY"))
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0)
        / 914400.0
        * 72.0;

    // Get scale factors (images use these to set actual display size)
    let scale_x = transform
        .and_then(|t| t.get("scaleX"))
        .and_then(|v| v.as_f64())
        .unwrap_or(1.0);

    let scale_y = transform
        .and_then(|t| t.get("scaleY"))
        .and_then(|v| v.as_f64())
        .unwrap_or(1.0);

    // Get base size
    let base_width = size
        .and_then(|s| s.get("width"))
        .map(|w| extract_dimension(Some(w)))
        .unwrap_or(0.0);

    let base_height = size
        .and_then(|s| s.get("height"))
        .map(|h| extract_dimension(Some(h)))
        .unwrap_or(0.0);

    // Apply scale to get actual displayed size
    let width = base_width * scale_x.abs();
    let height = base_height * scale_y.abs();

    (translate_x, translate_y, width, height)
}

/// Extract text content, character count, and font size from text object
fn extract_text_info(text: &serde_json::Value) -> (String, usize, f64) {
    let mut content = String::new();
    let mut font_size = 14.0; // default

    if let Some(elements) = text.get("textElements").and_then(|e| e.as_array()) {
        for te in elements {
            if let Some(tr) = te.get("textRun") {
                if let Some(c) = tr.get("content").and_then(|c| c.as_str()) {
                    content.push_str(c);
                }
                // Extract font size from style
                if let Some(style) = tr.get("style") {
                    if let Some(fs) = style.get("fontSize") {
                        font_size = extract_dimension(Some(fs));
                    }
                }
            }
        }
    }

    let char_count = content.trim().chars().count();
    (content.trim().to_string(), char_count, font_size)
}

/// Estimate text capacity based on box size and font
fn estimate_text_capacity(width: f64, height: f64, font_size: f64) -> usize {
    if width <= 0.0 || height <= 0.0 || font_size <= 0.0 {
        return 0;
    }

    // Rough estimation: average char width ~0.5 * font_size, line height ~1.2 * font_size
    let char_width = font_size * 0.5;
    let line_height = font_size * 1.4;

    let chars_per_line = (width / char_width) as usize;
    let lines = (height / line_height) as usize;

    chars_per_line.saturating_mul(lines)
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
    presentation_id: &str,
    comment_id: &str,
    message: &str,
) -> Result<String, String> {
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

fn cmd_analyze_slide(presentation_id: &str, slide_id: &str) -> Result<String, String> {
    let auth = get_auth()?;
    let adapter = GoogleSlidesInboundAdapter::new(auth, HashSet::new());

    let presentation = adapter
        .get_presentation(presentation_id)
        .map_err(|e| format!("Failed to get presentation: {}", e))?;

    // Find the specific slide
    let slides = presentation
        .get("slides")
        .and_then(|s| s.as_array())
        .ok_or("No slides found")?;

    let slide = slides
        .iter()
        .find(|s| s.get("objectId").and_then(|o| o.as_str()) == Some(slide_id))
        .ok_or_else(|| format!("Slide {} not found", slide_id))?;

    let mut output = String::new();
    output.push_str(&format!("=== Slide Analysis: {} ===\n\n", slide_id));

    // Page size
    if let Some(page_size) = presentation.get("pageSize") {
        let width = extract_dimension(page_size.get("width"));
        let height = extract_dimension(page_size.get("height"));
        output.push_str(&format!(
            "Page Size: {:.0}pt x {:.0}pt ({:.1}\" x {:.1}\")\n\n",
            width,
            height,
            width / 72.0,
            height / 72.0
        ));
    }

    if let Some(elements) = slide.get("pageElements").and_then(|e| e.as_array()) {
        output.push_str(&format!("Elements ({}):\n\n", elements.len()));

        for elem in elements {
            if let Some(elem_id) = elem.get("objectId").and_then(|o| o.as_str()) {
                let elem_type = get_element_type(elem);
                let (x, y, width, height) = extract_element_bounds(elem);

                output.push_str(&format!("┌─ {} [{}]\n", elem_id, elem_type));
                output.push_str(&format!("│  Position: ({:.0}, {:.0}) pt\n", x, y));
                output.push_str(&format!(
                    "│  Size: {:.0} x {:.0} pt ({:.1}\" x {:.1}\")\n",
                    width,
                    height,
                    width / 72.0,
                    height / 72.0
                ));

                if let Some(shape) = elem.get("shape") {
                    // Placeholder info
                    if let Some(placeholder) = shape.get("placeholder") {
                        if let Some(ptype) = placeholder.get("type").and_then(|t| t.as_str()) {
                            output.push_str(&format!("│  Placeholder Type: {}\n", ptype));
                        }
                    }

                    // Text analysis
                    if let Some(text) = shape.get("text") {
                        let (text_content, char_count, font_size) = extract_text_info(text);
                        let capacity = estimate_text_capacity(width, height, font_size);

                        output.push_str(&format!("│  Font Size: {:.0}pt\n", font_size));

                        if capacity > 0 {
                            let usage_pct =
                                (char_count as f64 / capacity as f64 * 100.0).min(100.0);
                            let status = if usage_pct > 90.0 {
                                "⚠️  OVERFLOW RISK"
                            } else if usage_pct > 70.0 {
                                "⚡ Near capacity"
                            } else {
                                "✓ OK"
                            };

                            output.push_str(&format!(
                                "│  Text: {} / ~{} chars ({:.0}%) {}\n",
                                char_count, capacity, usage_pct, status
                            ));
                            output.push_str(&format!(
                                "│  Remaining: ~{} chars\n",
                                capacity.saturating_sub(char_count)
                            ));
                        } else {
                            output.push_str(&format!("│  Text: {} chars\n", char_count));
                        }

                        if !text_content.is_empty() {
                            let preview = if text_content.len() > 80 {
                                format!("{}...", &text_content[..80])
                            } else {
                                text_content
                            };
                            output.push_str(&format!(
                                "│  Content: \"{}\"\n",
                                preview.replace('\n', "\\n")
                            ));
                        }
                    }
                } else if elem.get("image").is_some() {
                    output.push_str("│  (Image element - no text capacity)\n");
                }

                output.push_str("└─────────────────────────────────\n\n");
            }
        }
    }

    output.push_str("=== Recommendations ===\n");
    output.push_str("• Title: Keep under 50 characters\n");
    output.push_str("• Subtitle/Body: Keep under 80% capacity to avoid overflow\n");
    output.push_str("• Use bullet points for long content\n");

    Ok(output)
}

fn cmd_insert_image(
    presentation_id: &str,
    url: &str,
    page_id: &str,
    x: f64,
    y: f64,
    width: Option<f64>,
    height: Option<f64>,
) -> Result<String, String> {
    let auth = get_auth()?;
    let adapter = GoogleSlidesOutboundAdapter::new(auth);

    let object_id = adapter
        .insert_image(presentation_id, url, page_id, x, y, width, height)
        .map_err(|e| format!("Failed to insert image: {}", e))?;

    Ok(format!("Successfully inserted image (id={})", object_id))
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

/// Find available space on a slide for image placement.
/// Returns recommended positions where an image can be placed without overlapping existing content.
fn cmd_find_space(
    presentation_id: &str,
    slide_id: &str,
    min_width: f64,
    min_height: f64,
) -> Result<String, String> {
    let auth = get_auth()?;
    let adapter = GoogleSlidesOutboundAdapter::new(auth);

    let presentation = adapter
        .get_presentation(presentation_id)
        .map_err(|e| format!("Failed to get presentation: {}", e))?;

    // Get slide dimensions (default to standard 16:9)
    let page_size = presentation.get("pageSize");
    let slide_width = page_size
        .and_then(|ps| ps.get("width"))
        .map(|w| extract_dimension(Some(w)))
        .unwrap_or(720.0); // 10 inches in points
    let slide_height = page_size
        .and_then(|ps| ps.get("height"))
        .map(|h| extract_dimension(Some(h)))
        .unwrap_or(405.0); // 5.625 inches in points

    // Find the target slide
    let slides = presentation.get("slides").and_then(|s| s.as_array());
    let target_slide = slides.and_then(|slides| {
        slides.iter().find(|s| {
            s.get("objectId")
                .and_then(|id| id.as_str())
                .map(|id| id == slide_id)
                .unwrap_or(false)
        })
    });

    let Some(slide) = target_slide else {
        return Err(format!("Slide {} not found in presentation", slide_id));
    };

    // Collect all element bounding boxes with safety padding
    let mut occupied_regions: Vec<(f64, f64, f64, f64)> = Vec::new();
    let element_padding = 15.0; // Add padding around elements to prevent overlap

    if let Some(elements) = slide.get("pageElements").and_then(|e| e.as_array()) {
        for elem in elements {
            let (x, y, w, h) = extract_element_bounds(elem);
            if w > 0.0 && h > 0.0 {
                // Add padding around the element
                let padded_x = (x - element_padding).max(0.0);
                let padded_y = (y - element_padding).max(0.0);
                let padded_w = w + element_padding * 2.0;
                let padded_h = h + element_padding * 2.0;
                occupied_regions.push((padded_x, padded_y, padded_w, padded_h));
            }
        }
    }

    let mut output = String::new();
    output.push_str(&format!(
        "=== Available Space Analysis for Slide {} ===\n\n",
        slide_id
    ));
    output.push_str(&format!(
        "Slide size: {:.0} x {:.0} pt ({:.1}\" x {:.1}\")\n",
        slide_width,
        slide_height,
        slide_width / 72.0,
        slide_height / 72.0
    ));
    output.push_str(&format!(
        "Minimum space requested: {:.0} x {:.0} pt\n\n",
        min_width, min_height
    ));

    output.push_str(&format!("Existing elements: {}\n", occupied_regions.len()));
    for (i, (x, y, w, h)) in occupied_regions.iter().enumerate() {
        output.push_str(&format!(
            "  {}. ({:.0}, {:.0}) size {:.0}x{:.0}\n",
            i + 1,
            x,
            y,
            w,
            h
        ));
    }
    output.push_str("\n");

    // Find available spaces using a simple grid-based approach
    let grid_step = 50.0; // 50 points step
    let margin = 20.0; // margin from edges
    let mut available_positions: Vec<(f64, f64, f64, f64)> = Vec::new();

    // Check various positions
    let positions_to_check = vec![
        // Bottom right (common for images)
        (
            slide_width - min_width - margin,
            slide_height - min_height - margin,
        ),
        // Bottom left
        (margin, slide_height - min_height - margin),
        // Top right
        (slide_width - min_width - margin, margin),
        // Center bottom
        (
            (slide_width - min_width) / 2.0,
            slide_height - min_height - margin,
        ),
        // Center
        (
            (slide_width - min_width) / 2.0,
            (slide_height - min_height) / 2.0,
        ),
    ];

    for (check_x, check_y) in positions_to_check {
        if check_x < margin || check_y < margin {
            continue;
        }

        let mut overlaps = false;
        for (ox, oy, ow, oh) in &occupied_regions {
            // Check for overlap
            let rect1_right = check_x + min_width;
            let rect1_bottom = check_y + min_height;
            let rect2_right = ox + ow;
            let rect2_bottom = oy + oh;

            if check_x < rect2_right
                && rect1_right > *ox
                && check_y < rect2_bottom
                && rect1_bottom > *oy
            {
                overlaps = true;
                break;
            }
        }

        if !overlaps {
            available_positions.push((check_x, check_y, min_width, min_height));
        }
    }

    // Also try grid search for more options
    let mut y = margin;
    while y + min_height <= slide_height - margin && available_positions.len() < 10 {
        let mut x = margin;
        while x + min_width <= slide_width - margin {
            let mut overlaps = false;
            for (ox, oy, ow, oh) in &occupied_regions {
                let rect1_right = x + min_width;
                let rect1_bottom = y + min_height;
                let rect2_right = ox + ow;
                let rect2_bottom = oy + oh;

                if x < rect2_right && rect1_right > *ox && y < rect2_bottom && rect1_bottom > *oy {
                    overlaps = true;
                    break;
                }
            }

            if !overlaps {
                // Check if this position is not too close to existing positions
                let mut too_close = false;
                for (px, py, _, _) in &available_positions {
                    if (x - px).abs() < grid_step && (y - py).abs() < grid_step {
                        too_close = true;
                        break;
                    }
                }
                if !too_close {
                    available_positions.push((x, y, min_width, min_height));
                }
            }
            x += grid_step;
        }
        y += grid_step;
    }

    if available_positions.is_empty() {
        output.push_str("⚠️ No available space found for the requested size.\n");
        output.push_str("Consider:\n");
        output.push_str("  - Using a smaller image size\n");
        output.push_str("  - Removing or repositioning existing elements\n");
        output.push_str("  - Creating a new slide\n");
    } else {
        output.push_str(&format!(
            "✓ Found {} available positions:\n\n",
            available_positions.len()
        ));
        for (i, (x, y, w, h)) in available_positions.iter().take(5).enumerate() {
            let location = if *y > slide_height * 0.6 {
                if *x > slide_width * 0.6 {
                    "bottom-right"
                } else if *x < slide_width * 0.4 {
                    "bottom-left"
                } else {
                    "bottom-center"
                }
            } else if *y < slide_height * 0.4 {
                if *x > slide_width * 0.6 {
                    "top-right"
                } else if *x < slide_width * 0.4 {
                    "top-left"
                } else {
                    "top-center"
                }
            } else {
                "center"
            };

            output.push_str(&format!(
                "{}. Position: ({:.0}, {:.0}) - {} area\n",
                i + 1,
                x,
                y,
                location
            ));
            output.push_str(&format!(
                "   Command: google-slides insert-image {} --url=\"<URL>\" --page-id={} --x={:.0} --y={:.0} --width={:.0} --height={:.0}\n\n",
                presentation_id, slide_id, x, y, w, h
            ));
        }

        output.push_str("Tip: The first position is usually the best choice.\n");
    }

    Ok(output)
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
        output.push_str(&format!(
            "   Size: {}x{} ({})\n",
            image.width,
            image.height,
            if image.width > image.height {
                "landscape"
            } else if image.height > image.width {
                "portrait"
            } else {
                "square"
            }
        ));
        output.push_str(&format!("   URL (regular): {}\n", image.urls.regular));
        output.push_str(&format!("   Attribution: {}\n", image.get_attribution()));
        output.push_str("\n");
    }

    output.push_str("Usage:\n");
    output.push_str("  1. First find available space: google-slides find-space <id> <slide_id>\n");
    output.push_str("  2. Then insert the image: google-slides insert-image <id> --url=\"<URL>\" --page-id=<slide_id> --x=<x> --y=<y>\n");

    Ok(output)
}

fn cmd_create_presentation(title: &str) -> Result<String, String> {
    let auth = get_auth()?;
    let adapter = GoogleSlidesOutboundAdapter::new(auth);

    let presentation_id = adapter
        .create_presentation(title)
        .map_err(|e| format!("Failed to create presentation: {}", e))?;

    let mut output = String::new();
    output.push_str(&format!("Created presentation: {}\n", title));
    output.push_str(&format!("Presentation ID: {}\n", presentation_id));
    output.push_str(&format!(
        "Link: https://docs.google.com/presentation/d/{}/edit\n",
        presentation_id
    ));
    output.push_str("\nNext steps:\n");
    output.push_str("  - Add slides: google-slides create-slide <id> --layout=TITLE_AND_BODY\n");
    output.push_str("  - Share with user: google-slides share <id> --email=\"user@example.com\" --role=\"writer\"\n");
    output.push_str("  - Get shareable link: google-slides get-link <id>\n");

    Ok(output)
}

fn cmd_share_file(file_id: &str, email: &str, role: &str, notify: bool) -> Result<String, String> {
    let auth = get_auth()?;
    let client = GoogleDriveClient::new(auth);

    let permission_role = match role.to_lowercase().as_str() {
        "reader" | "read" | "view" => PermissionRole::Reader,
        "commenter" | "comment" => PermissionRole::Commenter,
        "writer" | "write" | "edit" => PermissionRole::Writer,
        _ => {
            return Err(format!(
                "Invalid role '{}'. Valid roles: reader, commenter, writer",
                role
            ));
        }
    };

    let result = client
        .share_file(file_id, email, permission_role, notify)
        .map_err(|e| format!("Failed to share file: {}", e))?;

    let mut output = String::new();
    output.push_str(&format!("Shared file {} with {}\n", file_id, email));
    output.push_str(&format!("Role: {}\n", result.role));
    output.push_str(&format!("Permission ID: {}\n", result.permission_id));
    if notify {
        output.push_str("Email notification sent.\n");
    }

    Ok(output)
}

fn cmd_get_link(file_id: &str) -> Result<String, String> {
    let auth = get_auth()?;
    let client = GoogleDriveClient::new(auth);

    let links = client
        .get_sharing_link(file_id)
        .map_err(|e| format!("Failed to get link: {}", e))?;

    let mut output = String::new();
    output.push_str(&format!("File ID: {}\n", file_id));

    if let Some(view_link) = &links.web_view_link {
        output.push_str(&format!("View/Edit Link: {}\n", view_link));
    }

    if let Some(download_link) = &links.web_content_link {
        output.push_str(&format!("Download Link: {}\n", download_link));
    }

    if links.web_view_link.is_none() && links.web_content_link.is_none() {
        output.push_str("No links available. The file may not be shared yet.\n");
        output.push_str("Use 'google-slides share <id> --email=\"...\"' to share the file first.\n");
    }

    Ok(output)
}

fn cmd_list_permissions(file_id: &str) -> Result<String, String> {
    let auth = get_auth()?;
    let client = GoogleDriveClient::new(auth);

    let permissions = client
        .list_permissions(file_id)
        .map_err(|e| format!("Failed to list permissions: {}", e))?;

    let mut output = String::new();
    output.push_str(&format!(
        "Permissions for file {} ({} total):\n\n",
        file_id,
        permissions.len()
    ));

    for perm in permissions {
        let id = perm.get("id").and_then(|v| v.as_str()).unwrap_or("unknown");
        let perm_type = perm.get("type").and_then(|v| v.as_str()).unwrap_or("unknown");
        let role = perm.get("role").and_then(|v| v.as_str()).unwrap_or("unknown");
        let email = perm
            .get("emailAddress")
            .and_then(|v| v.as_str())
            .unwrap_or("-");
        let name = perm
            .get("displayName")
            .and_then(|v| v.as_str())
            .unwrap_or("-");

        output.push_str(&format!("- ID: {}\n", id));
        output.push_str(&format!("  Type: {}, Role: {}\n", perm_type, role));
        if email != "-" {
            output.push_str(&format!("  Email: {}\n", email));
        }
        if name != "-" {
            output.push_str(&format!("  Name: {}\n", name));
        }
        output.push_str("\n");
    }

    Ok(output)
}

fn cmd_remove_permission(file_id: &str, permission_id: &str) -> Result<String, String> {
    let auth = get_auth()?;
    let client = GoogleDriveClient::new(auth);

    client
        .remove_permission(file_id, permission_id)
        .map_err(|e| format!("Failed to remove permission: {}", e))?;

    Ok(format!(
        "Removed permission {} from file {}",
        permission_id, file_id
    ))
}
