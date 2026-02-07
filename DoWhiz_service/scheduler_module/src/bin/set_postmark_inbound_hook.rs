use serde_json::json;
use std::env;

type BoxError = Box<dyn std::error::Error + Send + Sync>;

fn parse_args() -> Result<Option<String>, String> {
    let mut hook_url = None;
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--hook-url" => {
                hook_url = args.next();
            }
            "--help" | "-h" => {
                return Err(help_text());
            }
            _ => {
                return Err(format!("unknown argument: {}", arg));
            }
        }
    }
    Ok(hook_url)
}

fn help_text() -> String {
    [
        "Set Postmark inbound hook URL",
        "",
        "Usage:",
        "  cargo run -p scheduler_module --bin set_postmark_inbound_hook -- --hook-url https://YOUR-NGROK-URL.ngrok-free.dev/postmark/inbound",
        "",
        "Environment:",
        "  POSTMARK_SERVER_TOKEN (required)",
        "  POSTMARK_INBOUND_HOOK_URL (optional default for --hook-url)",
    ]
    .join("\n")
}

fn main() -> Result<(), BoxError> {
    dotenvy::dotenv().ok();

    let hook_url = match parse_args() {
        Ok(value) => value,
        Err(msg) => {
            eprintln!("{}", msg);
            return Ok(());
        }
    };

    let hook_url = hook_url
        .or_else(|| env::var("POSTMARK_INBOUND_HOOK_URL").ok())
        .ok_or("missing --hook-url or POSTMARK_INBOUND_HOOK_URL")?;

    let token =
        env::var("POSTMARK_SERVER_TOKEN").map_err(|_| "POSTMARK_SERVER_TOKEN must be set")?;

    let payload = json!({ "InboundHookUrl": hook_url });
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()?;
    let response = client
        .put("https://api.postmarkapp.com/server")
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .header("X-Postmark-Server-Token", token)
        .json(&payload)
        .send()?;

    let status = response.status();
    let body = response.text().unwrap_or_default();
    if !status.is_success() {
        return Err(format!("Postmark error {}: {}", status, body).into());
    }

    println!("Set InboundHookUrl ok: {}", payload["InboundHookUrl"]);
    Ok(())
}
