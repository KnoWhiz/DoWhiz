use std::io::Read;
use std::process::{Command, ExitStatus, Output, Stdio};
use std::sync::{Arc, Mutex};
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
        .unwrap_or(36000); // 10 hours
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
