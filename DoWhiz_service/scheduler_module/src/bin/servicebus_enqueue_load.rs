use chrono::Utc;
use scheduler_module::channel::{Channel, ChannelMetadata};
use scheduler_module::ingestion::{IngestionEnvelope, IngestionPayload};
use scheduler_module::ingestion_queue::build_queue_from_env;
use scheduler_module::service_bus_queue::resolve_service_bus_config_from_env;
use std::env;
use uuid::Uuid;

fn parse_arg(args: &[String], key: &str, default: &str) -> String {
    args.windows(2)
        .find(|window| window[0] == key)
        .map(|window| window[1].clone())
        .unwrap_or_else(|| default.to_string())
}

fn parse_arg_usize(args: &[String], key: &str, default: usize) -> usize {
    parse_arg(args, key, &default.to_string())
        .parse::<usize>()
        .ok()
        .filter(|value| *value > 0)
        .unwrap_or(default)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(cfg) = resolve_service_bus_config_from_env() {
        println!(
            "SERVICEBUS_CONFIG namespace={:?} queue={:?} policy={:?}",
            cfg.namespace, cfg.queue_name, cfg.policy_name
        );
    }

    let args: Vec<String> = env::args().collect();
    let count = parse_arg_usize(&args, "--count", 200);
    let employee_id = parse_arg(&args, "--employee-id", "little_bear");
    let recipient = parse_arg(&args, "--recipient", "dowhiz@deep-tutor.com");
    let reply_to = parse_arg(&args, "--reply-to", "proto@dowhiz.com");
    let fixed_thread_id = parse_arg(&args, "--thread-id", "");
    let run_id = parse_arg(
        &args,
        "--run-id",
        &format!("sbload_{}", Utc::now().format("%Y%m%dT%H%M%SZ")),
    );

    let queue = build_queue_from_env(None)?;
    let start = Utc::now();
    println!("RUN_ID={run_id}");
    println!("START_UTC={}", start.to_rfc3339());

    for idx in 0..count {
        let now = Utc::now();
        let thread_id = if fixed_thread_id.trim().is_empty() {
            format!("{run_id}-thread-{idx:04}")
        } else {
            fixed_thread_id.clone()
        };
        let message_id = format!("{run_id}-msg-{idx:04}-{}", Uuid::new_v4());
        let dedupe_key = format!("{run_id}-{idx}-{}", Uuid::new_v4());
        let subject = format!("[{run_id}] Parallel load task {idx}");
        let text_body = format!(
            "[{run_id}] Task {idx}: Read this email and reply with one concise sentence confirming completion and include task id {idx}."
        );

        let envelope = IngestionEnvelope {
            envelope_id: Uuid::new_v4(),
            received_at: now,
            tenant_id: None,
            employee_id: employee_id.clone(),
            channel: Channel::Email,
            external_message_id: Some(message_id.clone()),
            dedupe_key,
            payload: IngestionPayload {
                sender: format!("parallel-test+{idx}@example.com"),
                sender_name: Some("Parallel Test".to_string()),
                recipient: recipient.clone(),
                subject: Some(subject),
                text_body: Some(text_body),
                html_body: None,
                thread_id,
                message_id: Some(message_id),
                attachments: Vec::new(),
                reply_to: vec![reply_to.clone()],
                metadata: ChannelMetadata::default(),
            },
            raw_payload_ref: None,
            account_id: None,
        };

        queue.enqueue(&envelope)?;
        if (idx + 1) % 25 == 0 {
            println!("Sent {}/{}", idx + 1, count);
        }
    }

    let end = Utc::now();
    let duration_ms = end.signed_duration_since(start).num_milliseconds().max(0) as f64;
    println!("END_UTC={}", end.to_rfc3339());
    println!("SEND_DURATION_SECONDS={:.3}", duration_ms / 1000.0);
    println!("SENT={count}");
    Ok(())
}
