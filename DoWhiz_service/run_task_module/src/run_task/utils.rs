use std::io::Read;
use std::process::{Command, ExitStatus, Output, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use super::errors::RunTaskError;

const DEFAULT_SCHEDULER_TASK_TIMEOUT_SECS: u64 = 600;
const DEFAULT_RUN_TASK_TIMEOUT_SECS: u64 = 36000;
const WATCHDOG_HEADROOM_SECS: u64 = 30;
const MIN_RUN_TASK_TIMEOUT_SECS: u64 = 30;

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

fn parse_timeout_secs(key: &str) -> Option<u64> {
    std::env::var(key)
        .ok()
        .and_then(|value| value.trim().parse::<u64>().ok())
        .filter(|value| *value > 0)
}

pub(super) fn run_task_timeout() -> Duration {
    let requested_timeout_secs =
        parse_timeout_secs("RUN_TASK_TIMEOUT_SECS").unwrap_or(DEFAULT_RUN_TASK_TIMEOUT_SECS);
    let task_timeout_secs = parse_timeout_secs("TASK_TIMEOUT_SECS").unwrap_or_else(|| {
        DEFAULT_SCHEDULER_TASK_TIMEOUT_SECS.max(
            requested_timeout_secs
                .saturating_add(WATCHDOG_HEADROOM_SECS)
                .max(MIN_RUN_TASK_TIMEOUT_SECS),
        )
    });
    // Keep run_task timeout below watchdog timeout to avoid stale-task retry storms.
    let watchdog_budget_secs = task_timeout_secs
        .saturating_sub(WATCHDOG_HEADROOM_SECS)
        .max(MIN_RUN_TASK_TIMEOUT_SECS);
    let timeout_secs = requested_timeout_secs.min(watchdog_budget_secs);
    Duration::from_secs(timeout_secs)
}

/// Spawns a thread to continuously drain a pipe into a buffer.
/// This prevents the pipe buffer from filling up and blocking the child process.
fn spawn_pipe_drainer<R: Read + Send + 'static>(
    pipe: R,
    buffer: Arc<Mutex<Vec<u8>>>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut pipe = pipe;
        let mut chunk = [0u8; 8192];
        loop {
            match pipe.read(&mut chunk) {
                Ok(0) => break, // EOF
                Ok(n) => {
                    if let Ok(mut buf) = buffer.lock() {
                        buf.extend_from_slice(&chunk[..n]);
                    }
                }
                Err(_) => break,
            }
        }
    })
}

pub(super) fn run_command_with_timeout(
    mut cmd: Command,
    timeout: Duration,
    label: &'static str,
) -> Result<Output, RunTaskError> {
    cmd.stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut child = cmd.spawn().map_err(RunTaskError::Io)?;
    let start = Instant::now();

    // Take ownership of stdout/stderr and spawn drainer threads
    // This prevents pipe buffer from filling up and blocking the child
    let stdout_buf = Arc::new(Mutex::new(Vec::new()));
    let stderr_buf = Arc::new(Mutex::new(Vec::new()));

    let stdout_handle = child
        .stdout
        .take()
        .map(|pipe| spawn_pipe_drainer(pipe, Arc::clone(&stdout_buf)));
    let stderr_handle = child
        .stderr
        .take()
        .map(|pipe| spawn_pipe_drainer(pipe, Arc::clone(&stderr_buf)));

    // Poll for exit or timeout
    let status: ExitStatus;
    let timed_out;
    loop {
        if let Some(s) = child.try_wait().map_err(RunTaskError::Io)? {
            status = s;
            timed_out = false;
            break;
        }

        if start.elapsed() >= timeout {
            let _ = child.kill();
            status = child.wait().map_err(RunTaskError::Io)?;
            timed_out = true;
            break;
        }

        thread::sleep(Duration::from_millis(200));
    }

    // Wait for drainer threads to finish
    if let Some(h) = stdout_handle {
        let _ = h.join();
    }
    if let Some(h) = stderr_handle {
        let _ = h.join();
    }

    // Extract accumulated output from buffers
    let stdout = match Arc::try_unwrap(stdout_buf) {
        Ok(mutex) => mutex.into_inner().unwrap_or_default(),
        Err(arc) => arc.lock().map(|g| g.clone()).unwrap_or_default(),
    };
    let stderr = match Arc::try_unwrap(stderr_buf) {
        Ok(mutex) => mutex.into_inner().unwrap_or_default(),
        Err(arc) => arc.lock().map(|g| g.clone()).unwrap_or_default(),
    };

    if timed_out {
        let mut combined = String::new();
        combined.push_str(&String::from_utf8_lossy(&stdout));
        combined.push_str(&String::from_utf8_lossy(&stderr));
        return Err(RunTaskError::CommandTimeout {
            command: label,
            timeout_secs: timeout.as_secs(),
            output: tail_string(&combined, 2000),
        });
    }

    Ok(Output {
        status,
        stdout,
        stderr,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    struct EnvVarGuard {
        key: &'static str,
        previous: Option<String>,
    }

    impl EnvVarGuard {
        fn set(key: &'static str, value: &'static str) -> Self {
            let previous = std::env::var(key).ok();
            std::env::set_var(key, value);
            Self { key, previous }
        }

        fn unset(key: &'static str) -> Self {
            let previous = std::env::var(key).ok();
            std::env::remove_var(key);
            Self { key, previous }
        }
    }

    impl Drop for EnvVarGuard {
        fn drop(&mut self) {
            if let Some(value) = &self.previous {
                std::env::set_var(self.key, value);
            } else {
                std::env::remove_var(self.key);
            }
        }
    }

    #[test]
    fn run_task_timeout_defaults_to_watchdog_budget_minus_headroom() {
        let _lock = ENV_LOCK.lock().expect("env lock");
        let _guards = vec![
            EnvVarGuard::unset("RUN_TASK_TIMEOUT_SECS"),
            EnvVarGuard::unset("TASK_TIMEOUT_SECS"),
        ];

        assert_eq!(run_task_timeout(), Duration::from_secs(36000));
    }

    #[test]
    fn run_task_timeout_respects_shorter_explicit_override() {
        let _lock = ENV_LOCK.lock().expect("env lock");
        let _guards = vec![
            EnvVarGuard::set("RUN_TASK_TIMEOUT_SECS", "120"),
            EnvVarGuard::unset("TASK_TIMEOUT_SECS"),
        ];

        assert_eq!(run_task_timeout(), Duration::from_secs(120));
    }

    #[test]
    fn run_task_timeout_caps_explicit_value_to_watchdog_budget() {
        let _lock = ENV_LOCK.lock().expect("env lock");
        let _guards = vec![
            EnvVarGuard::set("RUN_TASK_TIMEOUT_SECS", "36000"),
            EnvVarGuard::unset("TASK_TIMEOUT_SECS"),
        ];

        assert_eq!(run_task_timeout(), Duration::from_secs(36000));
    }

    #[test]
    fn run_task_timeout_uses_custom_task_timeout_budget() {
        let _lock = ENV_LOCK.lock().expect("env lock");
        let _guards = vec![
            EnvVarGuard::unset("RUN_TASK_TIMEOUT_SECS"),
            EnvVarGuard::set("TASK_TIMEOUT_SECS", "900"),
        ];

        assert_eq!(run_task_timeout(), Duration::from_secs(870));
    }

    #[test]
    fn run_task_timeout_caps_to_custom_task_timeout_budget() {
        let _lock = ENV_LOCK.lock().expect("env lock");
        let _guards = vec![
            EnvVarGuard::set("RUN_TASK_TIMEOUT_SECS", "880"),
            EnvVarGuard::set("TASK_TIMEOUT_SECS", "900"),
        ];

        assert_eq!(run_task_timeout(), Duration::from_secs(870));
    }

    #[test]
    fn run_task_timeout_ignores_invalid_values() {
        let _lock = ENV_LOCK.lock().expect("env lock");
        let _guards = vec![
            EnvVarGuard::set("RUN_TASK_TIMEOUT_SECS", "abc"),
            EnvVarGuard::set("TASK_TIMEOUT_SECS", "0"),
        ];

        assert_eq!(run_task_timeout(), Duration::from_secs(36000));
    }
}
