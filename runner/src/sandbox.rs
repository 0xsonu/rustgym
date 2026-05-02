use bollard::container::{Config, CreateContainerOptions};
use bollard::models::HostConfig;

/// Configuration for the sandboxed Docker container.
#[derive(Debug, Clone)]
pub struct SandboxConfig {
    /// Docker image to use for execution
    pub image: String,
    /// Disable network access
    pub network_disabled: bool,
    /// Memory limit in bytes (default: 256MB)
    pub memory_limit: i64,
    /// CPU quota in microseconds per 100ms period (50000 = 0.5 CPU)
    pub cpu_quota: i64,
    /// Maximum number of PIDs allowed in the container
    pub pids_limit: i64,
    /// Wall-clock timeout in seconds
    pub timeout_secs: u64,
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self {
            image: "rust:latest".to_string(),
            network_disabled: true,
            memory_limit: 256 * 1024 * 1024, // 256MB
            cpu_quota: 50000,                // 0.5 CPU (50% of one core)
            pids_limit: 64,
            timeout_secs: 10,
        }
    }
}

impl SandboxConfig {
    /// Build a bollard `Config` for creating a container with these sandbox limits.
    pub fn to_container_config(&self, cmd: Vec<String>) -> Config<String> {
        let host_config = HostConfig {
            memory: Some(self.memory_limit),
            cpu_quota: Some(self.cpu_quota),
            pids_limit: Some(self.pids_limit),
            network_mode: Some("none".to_string()),
            ..Default::default()
        };

        Config {
            image: Some(self.image.clone()),
            cmd: Some(cmd),
            working_dir: Some("/workspace".to_string()),
            network_disabled: Some(self.network_disabled),
            host_config: Some(host_config),
            ..Default::default()
        }
    }

    /// Build container creation options with a unique name.
    pub fn create_options(container_name: &str) -> CreateContainerOptions<String> {
        CreateContainerOptions {
            name: container_name.to_string(),
            platform: None,
        }
    }
}
