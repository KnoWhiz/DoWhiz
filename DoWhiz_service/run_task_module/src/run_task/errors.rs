use std::fmt;
use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub enum RunTaskError {
    Io(io::Error),
    MissingEnv {
        key: &'static str,
    },
    InvalidPath {
        label: &'static str,
        path: PathBuf,
        reason: &'static str,
    },
    CodexNotFound,
    CodexFailed {
        status: Option<i32>,
        output: String,
    },
    ClaudeNotFound,
    ClaudeInstallFailed {
        output: String,
    },
    ClaudeFailed {
        status: Option<i32>,
        output: String,
    },
    DockerNotFound,
    DockerFailed {
        status: Option<i32>,
        output: String,
    },
    GitHubAuthCommandNotFound {
        command: &'static str,
    },
    GitHubAuthFailed {
        command: &'static str,
        status: Option<i32>,
        output: String,
    },
    OutputMissing {
        path: PathBuf,
        output: String,
    },
}

impl fmt::Display for RunTaskError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RunTaskError::Io(err) => write!(f, "I/O error: {}", err),
            RunTaskError::MissingEnv { key } => write!(f, "Missing environment variable: {}", key),
            RunTaskError::InvalidPath {
                label,
                path,
                reason,
            } => write!(
                f,
                "Invalid path for {}: {} ({})",
                label,
                path.display(),
                reason
            ),
            RunTaskError::CodexNotFound => write!(f, "Codex CLI not found on PATH."),
            RunTaskError::CodexFailed { status, output } => write!(
                f,
                "Codex failed (status: {:?}). Output tail:\n{}",
                status, output
            ),
            RunTaskError::ClaudeNotFound => write!(f, "Claude CLI not found on PATH."),
            RunTaskError::ClaudeInstallFailed { output } => {
                write!(f, "Failed to install Claude CLI. Output tail:\n{}", output)
            }
            RunTaskError::ClaudeFailed { status, output } => write!(
                f,
                "Claude failed (status: {:?}). Output tail:\n{}",
                status, output
            ),
            RunTaskError::DockerNotFound => write!(f, "Docker CLI not found on PATH."),
            RunTaskError::DockerFailed { status, output } => write!(
                f,
                "Docker run failed (status: {:?}). Output tail:\n{}",
                status, output
            ),
            RunTaskError::GitHubAuthCommandNotFound { command } => {
                write!(f, "GitHub auth command not found on PATH: {}", command)
            }
            RunTaskError::GitHubAuthFailed {
                command,
                status,
                output,
            } => write!(
                f,
                "GitHub auth command failed ({} status: {:?}). Output tail:\n{}",
                command, status, output
            ),
            RunTaskError::OutputMissing { path, output } => {
                write!(
                    f,
                    "Expected output not found: {}\nCodex output tail:\n{}",
                    path.display(),
                    output
                )
            }
        }
    }
}

impl std::error::Error for RunTaskError {}

impl From<io::Error> for RunTaskError {
    fn from(err: io::Error) -> Self {
        RunTaskError::Io(err)
    }
}
