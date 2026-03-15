//! Notion API CLI for agent use.
//!
//! Provides commands for agents to interact with Notion pages and comments
//! without browser automation.
//!
//! Usage:
//!   notion_api_cli read-page --page-id <id> [--workspace-id <ws>]
//!   notion_api_cli get-comments --page-id <id> [--workspace-id <ws>]
//!   notion_api_cli reply --discussion-id <id> --content <text> [--workspace-id <ws>]
//!   notion_api_cli create-comment --page-id <id> --content <text> [--workspace-id <ws>]

use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    dotenvy::dotenv().ok();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage();
        return ExitCode::FAILURE;
    }

    let command = &args[1];
    match command.as_str() {
        "read-page" => cmd_read_page(&args[2..]),
        "get-comments" => cmd_get_comments(&args[2..]),
        "reply" => cmd_reply(&args[2..]),
        "create-comment" => cmd_create_comment(&args[2..]),
        "search" => cmd_search(&args[2..]),
        "help" | "--help" | "-h" => {
            print_usage();
            ExitCode::SUCCESS
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            print_usage();
            ExitCode::FAILURE
        }
    }
}

fn print_usage() {
    eprintln!(
        r#"Notion API CLI

Usage:
  notion_api_cli <command> [options]

Commands:
  read-page        Read a Notion page and its content
    --page-id <id>       Page ID (32-char UUID without dashes)
    --workspace-id <ws>  Workspace ID (optional, auto-detected if not provided)

  get-comments     Get comments on a page
    --page-id <id>       Page ID
    --workspace-id <ws>  Workspace ID (optional)

  reply            Reply to an existing comment thread
    --discussion-id <id> Discussion ID from the comment thread
    --content <text>     Reply content
    --workspace-id <ws>  Workspace ID (optional)

  create-comment   Create a new comment on a page
    --page-id <id>       Page ID
    --content <text>     Comment content
    --workspace-id <ws>  Workspace ID (optional)

  search           Search for pages
    --query <text>       Search query
    --workspace-id <ws>  Workspace ID (optional)

Environment:
  EMPLOYEE_ID              Required for OAuth token lookup
  NOTION_DEFAULT_WORKSPACE Default workspace ID if not specified

Output:
  JSON to stdout on success, error message to stderr on failure.
"#
    );
}

fn cmd_read_page(args: &[String]) -> ExitCode {
    let (page_id, workspace_id) = match parse_page_args(args) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}", e);
            return ExitCode::FAILURE;
        }
    };

    let Some(page_id) = page_id else {
        eprintln!("Error: --page-id is required");
        return ExitCode::FAILURE;
    };

    let employee_id = match env::var("EMPLOYEE_ID") {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Error: EMPLOYEE_ID environment variable is required");
            return ExitCode::FAILURE;
        }
    };

    let workspace_id = workspace_id
        .or_else(|| env::var("NOTION_DEFAULT_WORKSPACE").ok())
        .unwrap_or_else(|| "default".to_string());

    // Use the API client
    match scheduler_module::notion_browser::NotionApiClient::from_env(&employee_id) {
        Ok(client) => match client.get_page_content(&workspace_id, &page_id) {
            Ok(content) => {
                let output = serde_json::json!({
                    "page": {
                        "id": content.page.id,
                        "title": content.page.title,
                        "url": content.page.url,
                        "created_time": content.page.created_time,
                        "last_edited_time": content.page.last_edited_time,
                    },
                    "blocks": content.blocks.iter().map(|b| {
                        serde_json::json!({
                            "id": b.id,
                            "type": b.block_type,
                            "text": b.text_content,
                            "has_children": b.has_children,
                        })
                    }).collect::<Vec<_>>(),
                });
                println!("{}", serde_json::to_string_pretty(&output).unwrap());
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("API Error: {}", e);
                ExitCode::FAILURE
            }
        },
        Err(e) => {
            eprintln!("Failed to create API client: {}", e);
            ExitCode::FAILURE
        }
    }
}

fn cmd_get_comments(args: &[String]) -> ExitCode {
    let (page_id, workspace_id) = match parse_page_args(args) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}", e);
            return ExitCode::FAILURE;
        }
    };

    let Some(page_id) = page_id else {
        eprintln!("Error: --page-id is required");
        return ExitCode::FAILURE;
    };

    let employee_id = match env::var("EMPLOYEE_ID") {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Error: EMPLOYEE_ID environment variable is required");
            return ExitCode::FAILURE;
        }
    };

    let workspace_id = workspace_id
        .or_else(|| env::var("NOTION_DEFAULT_WORKSPACE").ok())
        .unwrap_or_else(|| "default".to_string());

    match scheduler_module::notion_browser::NotionApiClient::from_env(&employee_id) {
        Ok(client) => match client.get_comments(&workspace_id, &page_id) {
            Ok(comments) => {
                let output: Vec<_> = comments
                    .iter()
                    .map(|c| {
                        serde_json::json!({
                            "id": c.id,
                            "discussion_id": c.discussion_id,
                            "parent_id": c.parent_id,
                            "created_by": {
                                "id": c.created_by.id,
                                "name": c.created_by.name,
                            },
                            "created_time": c.created_time,
                            "text": c.plain_text(),
                        })
                    })
                    .collect();
                println!("{}", serde_json::to_string_pretty(&output).unwrap());
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("API Error: {}", e);
                ExitCode::FAILURE
            }
        },
        Err(e) => {
            eprintln!("Failed to create API client: {}", e);
            ExitCode::FAILURE
        }
    }
}

fn cmd_reply(args: &[String]) -> ExitCode {
    let mut discussion_id = None;
    let mut content = None;
    let mut workspace_id = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--discussion-id" => {
                i += 1;
                discussion_id = args.get(i).cloned();
            }
            "--content" => {
                i += 1;
                content = args.get(i).cloned();
            }
            "--workspace-id" => {
                i += 1;
                workspace_id = args.get(i).cloned();
            }
            _ => {}
        }
        i += 1;
    }

    let Some(discussion_id) = discussion_id else {
        eprintln!("Error: --discussion-id is required");
        return ExitCode::FAILURE;
    };

    let Some(content) = content else {
        eprintln!("Error: --content is required");
        return ExitCode::FAILURE;
    };

    let employee_id = match env::var("EMPLOYEE_ID") {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Error: EMPLOYEE_ID environment variable is required");
            return ExitCode::FAILURE;
        }
    };

    let workspace_id = workspace_id
        .or_else(|| env::var("NOTION_DEFAULT_WORKSPACE").ok())
        .unwrap_or_else(|| "default".to_string());

    match scheduler_module::notion_browser::NotionApiClient::from_env(&employee_id) {
        Ok(client) => match client.reply_to_comment(&workspace_id, &discussion_id, &content) {
            Ok(comment) => {
                let output = serde_json::json!({
                    "success": true,
                    "comment_id": comment.id,
                    "discussion_id": comment.discussion_id,
                    "text": comment.plain_text(),
                });
                println!("{}", serde_json::to_string_pretty(&output).unwrap());
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("API Error: {}", e);
                ExitCode::FAILURE
            }
        },
        Err(e) => {
            eprintln!("Failed to create API client: {}", e);
            ExitCode::FAILURE
        }
    }
}

fn cmd_create_comment(args: &[String]) -> ExitCode {
    let mut page_id = None;
    let mut content = None;
    let mut workspace_id = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--page-id" => {
                i += 1;
                page_id = args.get(i).cloned();
            }
            "--content" => {
                i += 1;
                content = args.get(i).cloned();
            }
            "--workspace-id" => {
                i += 1;
                workspace_id = args.get(i).cloned();
            }
            _ => {}
        }
        i += 1;
    }

    let Some(page_id) = page_id else {
        eprintln!("Error: --page-id is required");
        return ExitCode::FAILURE;
    };

    let Some(content) = content else {
        eprintln!("Error: --content is required");
        return ExitCode::FAILURE;
    };

    let employee_id = match env::var("EMPLOYEE_ID") {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Error: EMPLOYEE_ID environment variable is required");
            return ExitCode::FAILURE;
        }
    };

    let workspace_id = workspace_id
        .or_else(|| env::var("NOTION_DEFAULT_WORKSPACE").ok())
        .unwrap_or_else(|| "default".to_string());

    match scheduler_module::notion_browser::NotionApiClient::from_env(&employee_id) {
        Ok(client) => match client.create_comment(&workspace_id, &page_id, &content) {
            Ok(comment) => {
                let output = serde_json::json!({
                    "success": true,
                    "comment_id": comment.id,
                    "discussion_id": comment.discussion_id,
                    "page_id": comment.parent_id,
                    "text": comment.plain_text(),
                });
                println!("{}", serde_json::to_string_pretty(&output).unwrap());
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("API Error: {}", e);
                ExitCode::FAILURE
            }
        },
        Err(e) => {
            eprintln!("Failed to create API client: {}", e);
            ExitCode::FAILURE
        }
    }
}

fn cmd_search(args: &[String]) -> ExitCode {
    let mut query = None;
    let mut workspace_id = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--query" => {
                i += 1;
                query = args.get(i).cloned();
            }
            "--workspace-id" => {
                i += 1;
                workspace_id = args.get(i).cloned();
            }
            _ => {}
        }
        i += 1;
    }

    let Some(query) = query else {
        eprintln!("Error: --query is required");
        return ExitCode::FAILURE;
    };

    let employee_id = match env::var("EMPLOYEE_ID") {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Error: EMPLOYEE_ID environment variable is required");
            return ExitCode::FAILURE;
        }
    };

    let workspace_id = workspace_id
        .or_else(|| env::var("NOTION_DEFAULT_WORKSPACE").ok())
        .unwrap_or_else(|| "default".to_string());

    match scheduler_module::notion_browser::NotionApiClient::from_env(&employee_id) {
        Ok(client) => match client.search_pages(&workspace_id, &query) {
            Ok(pages) => {
                let output: Vec<_> = pages
                    .iter()
                    .map(|p| {
                        serde_json::json!({
                            "id": p.id,
                            "title": p.title,
                            "url": p.url,
                        })
                    })
                    .collect();
                println!("{}", serde_json::to_string_pretty(&output).unwrap());
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("API Error: {}", e);
                ExitCode::FAILURE
            }
        },
        Err(e) => {
            eprintln!("Failed to create API client: {}", e);
            ExitCode::FAILURE
        }
    }
}

fn parse_page_args(args: &[String]) -> Result<(Option<String>, Option<String>), String> {
    let mut page_id = None;
    let mut workspace_id = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--page-id" => {
                i += 1;
                page_id = args.get(i).cloned();
            }
            "--workspace-id" => {
                i += 1;
                workspace_id = args.get(i).cloned();
            }
            arg if arg.starts_with("--") => {
                return Err(format!("Unknown argument: {}", arg));
            }
            _ => {}
        }
        i += 1;
    }

    Ok((page_id, workspace_id))
}
