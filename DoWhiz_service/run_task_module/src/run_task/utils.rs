use std::process::{Command, Output, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use super::errors::RunTaskError;

pub(super) fn tail_string(input: &str, max_len: usize) -> String {
    let trimmed = input.trim();
    if trimmed.len() <= max_len {
        return trimmed.to_string();
    }
    let mut start = trimmed.len().saturating_sub(max_len);
    while start < trimmed.len() && !trimmed.is_char_boundary(start) {
        start += 1;
    }
    trimmed[start..].to_string()
}

pub(super) fn run_task_timeout() -> Duration {
    let timeout_secs = std::env::var("RUN_TASK_TIMEOUT_SECS")
        .ok()
        .and_then(|value| value.trim().parse::<u64>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(1800);
    Duration::from_secs(timeout_secs)
}

pub(super) fn run_command_with_timeout(
    mut cmd: Command,
    timeout: Duration,
    label: &'static str,
) -> Result<Output, RunTaskError> {
    cmd.stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut child = cmd.spawn().map_err(RunTaskError::Io)?;
    let start = Instant::now();

    loop {
        if let Some(_) = child.try_wait().map_err(RunTaskError::Io)? {
            return child.wait_with_output().map_err(RunTaskError::Io);
        }

        if start.elapsed() >= timeout {
            let _ = child.kill();
            let output = child.wait_with_output().map_err(RunTaskError::Io)?;
            let mut combined = String::new();
            combined.push_str(&String::from_utf8_lossy(&output.stdout));
            combined.push_str(&String::from_utf8_lossy(&output.stderr));
            return Err(RunTaskError::CommandTimeout {
                command: label,
                timeout_secs: timeout.as_secs(),
                output: tail_string(&combined, 2000),
            });
        }

        thread::sleep(Duration::from_millis(200));
    }
}
