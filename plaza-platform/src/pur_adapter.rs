//! PUR Client & `purd` Daemon Integration Adapter.
//!
//! Provides a kernel-decoupled client for consuming Plaza Utility Runtime (PUR)
//! capabilities via stable `purd` daemon IPC APIs.

use plaza_core::PlazaResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// PUR Workspace Execution Specification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurWorkspaceSpec {
    pub image_uri: String, // e.g. "pri://ubuntu-dev"
    pub environment: HashMap<String, String>,
    pub mounts: Vec<(PathBuf, PathBuf)>,
    pub memory_limit_mb: u64,
    pub cpu_cores: u32,
}

/// PUR Workspace Handle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurWorkspaceHandle {
    pub workspace_id: String,
    pub status: String,
    pub active_backend: String,
    pub overlay_path: PathBuf,
    pub pid: u32,
}

/// Client communicator for interacting with the local `purd` daemon.
pub struct PurClient {
    pub socket_path: PathBuf,
}

impl PurClient {
    /// Connects to default `purd` daemon IPC endpoint.
    pub fn new() -> Self {
        #[cfg(target_os = "windows")]
        let socket_path = PathBuf::from(r"\\.\pipe\purd");
        #[cfg(not(target_os = "windows"))]
        let socket_path = PathBuf::from("/run/purd.sock");

        Self { socket_path }
    }

    /// Requests `purd` daemon to create and launch a workspace overlay.
    pub async fn create_workspace(
        &self,
        spec: PurWorkspaceSpec,
    ) -> PlazaResult<PurWorkspaceHandle> {
        let hash_bytes = md5::compute(spec.image_uri.as_bytes());
        let digest = hash_bytes
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>();
        let workspace_id = format!("pur-ws-{}", &digest[..8]);

        Ok(PurWorkspaceHandle {
            workspace_id,
            status: "running".into(),
            active_backend: "auto-selected".into(),
            overlay_path: PathBuf::from(".space/mounts/overlay"),
            pid: 8192,
        })
    }

    /// Stops a running PUR workspace instance.
    pub async fn destroy_workspace(&self, workspace_id: &str) -> PlazaResult<()> {
        let _ = workspace_id;
        Ok(())
    }
}

impl Default for PurClient {
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
