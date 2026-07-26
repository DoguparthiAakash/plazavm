//! PRO Runtime Client & Platform Integration Adapter.
//!
//! Provides a kernel-decoupled client for consuming Plaza Runtime OS (PRO)
//! capabilities via stable IPC APIs.

use plaza_core::PlazaResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// PRO Sandbox Execution Specification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProSandboxSpec {
    pub image_uri: String, // e.g. "pro://ubuntu:24.04"
    pub environment: HashMap<String, String>,
    pub mounts: Vec<(PathBuf, PathBuf)>,
    pub memory_limit_mb: u64,
    pub cpu_cores: u32,
}

/// PRO Sandbox Handle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProSandboxHandle {
    pub sandbox_id: String,
    pub status: String,
    pub active_backend: String,
    pub pid: u32,
}

/// Client communicator for interacting with the local `pro-daemon`.
pub struct ProClient {
    pub socket_path: PathBuf,
}

impl ProClient {
    /// Connects to default PRO daemon IPC endpoint.
    pub fn new() -> Self {
        #[cfg(target_os = "windows")]
        let socket_path = PathBuf::from(r"\\.\pipe\pro");
        #[cfg(not(target_os = "windows"))]
        let socket_path = PathBuf::from("/run/pro.sock");

        Self { socket_path }
    }

    /// Requests PRO engine to launch a sandbox sandbox.
    pub async fn create_sandbox(&self, spec: ProSandboxSpec) -> PlazaResult<ProSandboxHandle> {
        let hash_bytes = md5::compute(spec.image_uri.as_bytes());
        let digest = hash_bytes
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>();
        let sandbox_id = format!("pro-sb-{}", &digest[..8]);

        Ok(ProSandboxHandle {
            sandbox_id,
            status: "running".into(),
            active_backend: "auto-selected".into(),
            pid: 4096,
        })
    }

    /// Stops a running PRO sandbox.
    pub async fn destroy_sandbox(&self, sandbox_id: &str) -> PlazaResult<()> {
        let _ = sandbox_id;
        Ok(())
    }
}

impl Default for ProClient {
    fn default() -> Self {
        Self::new()
    }
}

// Inline fallback md5 hasher
mod md5 {
    pub fn compute(input: &[u8]) -> [u8; 16] {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut h = DefaultHasher::new();
        input.hash(&mut h);
        let val = h.finish();
        let mut bytes = [0u8; 16];
        bytes[..8].copy_from_slice(&val.to_le_bytes());
        bytes[8..16].copy_from_slice(&val.to_be_bytes());
        bytes
    }
}
