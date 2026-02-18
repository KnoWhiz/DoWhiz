use std::env;
use std::io;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use super::env::{env_enabled_default, resolve_env_path};
use super::errors::RunTaskError;
use super::utils::tail_string;

pub(super) fn ensure_docker_image_available(image: &str) -> Result<(), RunTaskError> {
    if docker_image_exists(image)? {
        return Ok(());
    }

    if !env_enabled_default("RUN_TASK_DOCKER_AUTO_BUILD", true) {
        return Err(RunTaskError::DockerFailed {
            status: None,
            output: format!(
                "docker image '{}' not found and auto-build disabled (set RUN_TASK_DOCKER_AUTO_BUILD=1)",
                image
            ),
        });
    }

    let (dockerfile, context) = resolve_docker_build_paths()?;
    let mut cmd = Command::new("docker");
    cmd.args([
        "build",
        "-t",
        image,
        "-f",
        dockerfile.to_string_lossy().as_ref(),
        context.to_string_lossy().as_ref(),
    ]);

    let output = match cmd.output() {
        Ok(output) => output,
        Err(err) if err.kind() == io::ErrorKind::NotFound => {
            return Err(RunTaskError::DockerNotFound)
        }
        Err(err) => return Err(RunTaskError::Io(err)),
    };
    let mut combined_output = String::new();
    combined_output.push_str(&String::from_utf8_lossy(&output.stdout));
    combined_output.push_str(&String::from_utf8_lossy(&output.stderr));
    let output_tail = tail_string(&combined_output, 2000);

    if !output.status.success() {
        return Err(RunTaskError::DockerFailed {
            status: output.status.code(),
            output: output_tail,
        });
    }

    Ok(())
}

pub(super) fn docker_image_exists(image: &str) -> Result<bool, RunTaskError> {
    let output = match Command::new("docker")
        .args(["image", "inspect", image])
        .output()
    {
        Ok(output) => output,
        Err(err) if err.kind() == io::ErrorKind::NotFound => {
            return Err(RunTaskError::DockerNotFound)
        }
        Err(err) => return Err(RunTaskError::Io(err)),
    };

    Ok(output.status.success())
}

pub(super) fn resolve_docker_build_paths() -> Result<(PathBuf, PathBuf), RunTaskError> {
    let cwd = env::current_dir().map_err(RunTaskError::Io)?;
    let dockerfile = if let Some(path) = resolve_env_path("RUN_TASK_DOCKERFILE", &cwd) {
        path
    } else {
        let candidate = cwd.join("Dockerfile");
        if candidate.exists() {
            candidate
        } else {
            let candidate = cwd.join("..").join("Dockerfile");
            if candidate.exists() {
                candidate
            } else {
                return Err(RunTaskError::InvalidPath {
                    label: "dockerfile",
                    path: cwd.join("Dockerfile"),
                    reason: "Dockerfile not found; set RUN_TASK_DOCKERFILE",
                });
            }
        }
    };

    let context = if let Some(path) = resolve_env_path("RUN_TASK_DOCKER_BUILD_CONTEXT", &cwd) {
        path
    } else {
        dockerfile
            .parent()
            .map(PathBuf::from)
            .ok_or(RunTaskError::InvalidPath {
                label: "docker_build_context",
                path: dockerfile.clone(),
                reason: "could not resolve Dockerfile directory",
            })?
    };

    Ok((dockerfile, context))
}

pub(super) fn docker_cli_available() -> bool {
    match Command::new("docker")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
    {
        Ok(status) => status.success(),
        Err(_) => false,
    }
}
