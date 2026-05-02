use std::time::Instant;

use bollard::container::{
    KillContainerOptions, LogsOptions, RemoveContainerOptions, WaitContainerOptions,
};
use bollard::Docker;
use futures_util::StreamExt;
use tokio::time::{timeout, Duration};

use crate::sandbox::SandboxConfig;

/// The result of executing code in a Docker container.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExecutionResult {
    /// Process exit code (None if timed out)
    pub exit_code: Option<i64>,
    /// Captured stdout
    pub stdout: String,
    /// Captured stderr
    pub stderr: String,
    /// Execution duration in milliseconds
    pub duration_ms: u64,
    /// Whether the execution timed out
    pub timed_out: bool,
}

/// Manages Docker container lifecycle for code execution.
pub struct DockerExecutor {
    docker: Docker,
    sandbox: SandboxConfig,
}

/// Files to be written into the container workspace.
pub struct WorkspaceFiles {
    pub cargo_toml: String,
    pub lib_rs: String,
    pub test_rs: Option<String>,
    /// If true, write lib_rs content as src/main.rs instead of src/lib.rs
    pub as_binary: bool,
}

impl DockerExecutor {
    /// Create a new executor connected to the local Docker daemon.
    pub fn new(sandbox: SandboxConfig) -> Result<Self, bollard::errors::Error> {
        let docker = Docker::connect_with_local_defaults()?;
        Ok(Self { docker, sandbox })
    }

    /// Execute code in a sandboxed container.
    ///
    /// Creates a container, copies files, runs the command, collects output,
    /// and cleans up — enforcing the configured wall-clock timeout.
    /// An optional `timeout_override` can be provided to override the sandbox default.
    pub async fn execute(
        &self,
        files: WorkspaceFiles,
        cmd: Vec<String>,
        timeout_override: Option<u64>,
    ) -> Result<ExecutionResult, ExecutorError> {
        let container_name = format!("rustgym-run-{}", uuid::Uuid::new_v4());
        let start = Instant::now();

        // Create the container
        let config = self.sandbox.to_container_config(cmd);
        let options = SandboxConfig::create_options(&container_name);

        self.docker
            .create_container(Some(options), config)
            .await
            .map_err(ExecutorError::Docker)?;

        // Upload workspace files as a tar archive
        let tar_bytes = build_tar_archive(&files)?;
        self.docker
            .upload_to_container(
                &container_name,
                Some(bollard::container::UploadToContainerOptions {
                    path: "/workspace".to_string(),
                    ..Default::default()
                }),
                tar_bytes.into(),
            )
            .await
            .map_err(ExecutorError::Docker)?;

        // Start the container
        self.docker
            .start_container::<String>(&container_name, None)
            .await
            .map_err(ExecutorError::Docker)?;

        // Wait for completion with timeout enforcement
        let timeout_secs = timeout_override.unwrap_or(self.sandbox.timeout_secs);
        let timeout_duration = Duration::from_secs(timeout_secs);
        let wait_result = timeout(timeout_duration, self.wait_for_container(&container_name)).await;

        let (exit_code, timed_out) = match wait_result {
            Ok(Ok(code)) => (Some(code), false),
            Ok(Err(e)) => {
                // Docker error during wait — clean up and return error
                self.cleanup_container(&container_name).await;
                return Err(e);
            }
            Err(_) => {
                // Timeout elapsed — kill the container
                tracing::warn!(container = %container_name, "Container execution timed out, killing");
                let _ = self
                    .docker
                    .kill_container(
                        &container_name,
                        Some(KillContainerOptions { signal: "SIGKILL" }),
                    )
                    .await;
                (None, true)
            }
        };

        // Collect logs
        let (stdout, stderr) = self.collect_logs(&container_name).await;

        let duration_ms = start.elapsed().as_millis() as u64;

        // Remove the container
        self.cleanup_container(&container_name).await;

        Ok(ExecutionResult {
            exit_code,
            stdout,
            stderr,
            duration_ms,
            timed_out,
        })
    }

    /// Wait for the container to finish and return its exit code.
    async fn wait_for_container(&self, container_name: &str) -> Result<i64, ExecutorError> {
        let mut stream = self.docker.wait_container(
            container_name,
            Some(WaitContainerOptions {
                condition: "not-running",
            }),
        );

        if let Some(result) = stream.next().await {
            match result {
                Ok(response) => Ok(response.status_code),
                Err(e) => Err(ExecutorError::Docker(e)),
            }
        } else {
            // Stream ended without a result — treat as error
            Err(ExecutorError::ContainerWaitFailed)
        }
    }

    /// Collect stdout and stderr logs from the container.
    async fn collect_logs(&self, container_name: &str) -> (String, String) {
        let mut stdout = String::new();
        let mut stderr = String::new();

        let options = LogsOptions::<String> {
            stdout: true,
            stderr: true,
            follow: false,
            ..Default::default()
        };

        let mut stream = self.docker.logs(container_name, Some(options));

        while let Some(Ok(output)) = stream.next().await {
            match output {
                bollard::container::LogOutput::StdOut { message } => {
                    stdout.push_str(&String::from_utf8_lossy(&message));
                }
                bollard::container::LogOutput::StdErr { message } => {
                    stderr.push_str(&String::from_utf8_lossy(&message));
                }
                _ => {}
            }
        }

        (stdout, stderr)
    }

    /// Remove the container, ignoring errors (best-effort cleanup).
    async fn cleanup_container(&self, container_name: &str) {
        let options = RemoveContainerOptions {
            force: true,
            ..Default::default()
        };

        if let Err(e) = self
            .docker
            .remove_container(container_name, Some(options))
            .await
        {
            tracing::warn!(container = %container_name, error = %e, "Failed to remove container");
        }
    }
}

/// Build a tar archive containing the workspace files.
fn build_tar_archive(files: &WorkspaceFiles) -> Result<Vec<u8>, ExecutorError> {
    let mut archive = tar::Builder::new(Vec::new());

    // Add Cargo.toml
    append_file_to_tar(&mut archive, "Cargo.toml", files.cargo_toml.as_bytes())?;

    // Add source file as either src/main.rs or src/lib.rs
    let source_path = if files.as_binary {
        "src/main.rs"
    } else {
        "src/lib.rs"
    };
    append_file_to_tar(&mut archive, source_path, files.lib_rs.as_bytes())?;

    // Add tests/tests.rs if provided
    if let Some(ref test_code) = files.test_rs {
        append_file_to_tar(&mut archive, "tests/tests.rs", test_code.as_bytes())?;
    }

    let data = archive
        .into_inner()
        .map_err(|e| ExecutorError::TarBuild(e.to_string()))?;
    Ok(data)
}

/// Append a single file entry to a tar archive.
fn append_file_to_tar(
    archive: &mut tar::Builder<Vec<u8>>,
    path: &str,
    content: &[u8],
) -> Result<(), ExecutorError> {
    let mut header = tar::Header::new_gnu();
    header
        .set_path(path)
        .map_err(|e| ExecutorError::TarBuild(e.to_string()))?;
    header.set_size(content.len() as u64);
    header.set_mode(0o644);
    header.set_cksum();

    archive
        .append(&header, content)
        .map_err(|e| ExecutorError::TarBuild(e.to_string()))?;

    Ok(())
}

/// Errors that can occur during container execution.
#[derive(Debug, thiserror::Error)]
pub enum ExecutorError {
    #[error("Docker API error: {0}")]
    Docker(#[from] bollard::errors::Error),

    #[error("Container wait stream ended unexpectedly")]
    ContainerWaitFailed,

    #[error("Failed to build tar archive: {0}")]
    TarBuild(String),
}
