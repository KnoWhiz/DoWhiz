use scheduler_module::past_emails::hydrate_past_emails;
use std::env;
use std::path::PathBuf;

type BoxError = Box<dyn std::error::Error + Send + Sync>;

struct Args {
    archive_root: PathBuf,
    references_dir: PathBuf,
    user_id: String,
    max_attachment_mb: Option<u64>,
}

fn parse_args() -> Result<Args, String> {
    let mut archive_root = None;
    let mut references_dir = None;
    let mut user_id = None;
    let mut max_attachment_mb = None;

    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--archive-root" => {
                archive_root = args.next().map(PathBuf::from);
            }
            "--references-dir" => {
                references_dir = args.next().map(PathBuf::from);
            }
            "--user-id" => {
                user_id = args.next();
            }
            "--max-attachment-mb" => {
                let raw = args
                    .next()
                    .ok_or_else(|| "missing value for --max-attachment-mb".to_string())?;
                let parsed = raw
                    .parse::<u64>()
                    .map_err(|_| format!("invalid max attachment mb: {}", raw))?;
                max_attachment_mb = Some(parsed);
            }
            "--help" | "-h" => {
                return Err(help_text());
            }
            _ => {
                return Err(format!("unknown argument: {}", arg));
            }
        }
    }

    let archive_root = archive_root.ok_or_else(|| "missing --archive-root".to_string())?;
    let references_dir = references_dir.ok_or_else(|| "missing --references-dir".to_string())?;

    Ok(Args {
        archive_root,
        references_dir,
        user_id: user_id.unwrap_or_else(|| "unknown".to_string()),
        max_attachment_mb,
    })
}

fn help_text() -> String {
    [
        "Hydrate past_emails into a workspace references directory",
        "",
        "Usage:",
        "  cargo run -p scheduler_module --bin hydrate_past_emails -- \\",
        "    --archive-root /path/to/users/<id>/mail \\",
        "    --references-dir /path/to/workspace/references \\",
        "    --user-id <user_id>",
        "",
        "Options:",
        "  --archive-root       User mail archive root (required).",
        "  --references-dir     Workspace references directory (required).",
        "  --user-id            User id string for index.json (default: unknown).",
        "  --max-attachment-mb  Size threshold for azure-only attachments (default: 50).",
    ]
    .join("\n")
}

fn main() -> Result<(), BoxError> {
    let args = match parse_args() {
        Ok(values) => values,
        Err(msg) => {
            eprintln!("{}", msg);
            return Ok(());
        }
    };

    let max_bytes = args
        .max_attachment_mb
        .map(|mb| mb.saturating_mul(1024 * 1024));
    let report = hydrate_past_emails(
        &args.archive_root,
        &args.references_dir,
        &args.user_id,
        max_bytes,
    )?;
    println!(
        "Hydrated past_emails: {} entries, {} attachments ({} large)",
        report.entries_written, report.attachments_total, report.large_attachments
    );
    Ok(())
}
