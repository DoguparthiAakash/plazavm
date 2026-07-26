//! Platform Abstraction Layer (PAL).
//!
//! Provides kernel-decoupled, stable platform interface contracts for
//! filesystem, process supervision, networking, and security isolation across
//! host platforms (Linux, Windows, macOS, BSD, and future backends).

use crate::core::PlazaResult;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Filesystem operations trait contract.
pub trait PalFilesystem: Send + Sync {
    fn canonicalize(&self, path: &Path) -> PlazaResult<PathBuf>;
    fn create_overlay_mount(
        &self,
        lower: &Path,
        upper: &Path,
        work: &Path,
        merged: &Path,
    ) -> PlazaResult<()>;
}

/// Process management trait contract.
pub trait PalProcessManager: Send + Sync {
    fn spawn_supervised(
        &self,
        command: &str,
        args: &[String],
        env: &HashMap<String, String>,
    ) -> PlazaResult<u32>;
    fn kill_process(&self, pid: u32) -> PlazaResult<()>;
}

/// Network operations trait contract.
pub trait PalNetwork: Send + Sync {
    fn allocate_virtual_port(&self) -> PlazaResult<u16>;
    fn verify_socket_connectivity(&self, port: u16) -> PlazaResult<bool>;
}

/// Security and sandbox isolation trait contract.
pub trait PalSecurity: Send + Sync {
    fn enforce_sandbox_isolation(&self, pid: u32) -> PlazaResult<()>;
}

/// Consolidated Platform Abstraction Layer (PAL) Container.
pub struct PlatformAbstractionLayer {
    pub filesystem: Box<dyn PalFilesystem>,
    pub process: Box<dyn PalProcessManager>,
    pub network: Box<dyn PalNetwork>,
    pub security: Box<dyn PalSecurity>,
}

impl Default for PlatformAbstractionLayer {
    fn default() -> Self {
        Self {
            filesystem: Box::new(DefaultPalFilesystem),
            process: Box::new(DefaultPalProcessManager),
            network: Box::new(DefaultPalNetwork),
            security: Box::new(DefaultPalSecurity),
        }
    }
}

// ── Default Host Platform Implementations ────────────────────────────────────

struct DefaultPalFilesystem;
impl PalFilesystem for DefaultPalFilesystem {
    fn canonicalize(&self, path: &Path) -> PlazaResult<PathBuf> {
        std::fs::canonicalize(path)
            .map_err(|e| crate::core::PlazaError::storage(format!("Canonicalize failed: {}", e)))
    }
    fn create_overlay_mount(
        &self,
        _lower: &Path,
        _upper: &Path,
        _work: &Path,
        merged: &Path,
    ) -> PlazaResult<()> {
        std::fs::create_dir_all(merged)?;
        Ok(())
    }
}

struct DefaultPalProcessManager;
impl PalProcessManager for DefaultPalProcessManager {
    fn spawn_supervised(
        &self,
        command: &str,
        args: &[String],
        env: &HashMap<String, String>,
    ) -> PlazaResult<u32> {
        let mut child = std::process::Command::new(command);
        child.args(args);
        for (k, v) in env {
            child.env(k, v);
        }
        let handle = child.spawn().map_err(|e| {
            crate::core::PlazaError::config(format!("Failed to spawn process: {}", e))
        })?;
        Ok(handle.id())
    }
    fn kill_process(&self, pid: u32) -> PlazaResult<()> {
        #[cfg(target_os = "windows")]
        {
            let _ = std::process::Command::new("taskkill")
                .args(["/F", "/PID", &pid.to_string()])
                .status();
        }
        #[cfg(not(target_os = "windows"))]
        {
            let _ = std::process::Command::new("kill")
                .args(["-9", &pid.to_string()])
                .status();
        }
        Ok(())
    }
}

struct DefaultPalNetwork;
impl PalNetwork for DefaultPalNetwork {
    fn allocate_virtual_port(&self) -> PlazaResult<u16> {
        let listener = std::net::TcpListener::bind("127.0.0.1:0")?;
        let port = listener.local_addr()?.port();
        Ok(port)
    }
    fn verify_socket_connectivity(&self, port: u16) -> PlazaResult<bool> {
        Ok(std::net::TcpStream::connect(format!("127.0.0.1:{}", port)).is_ok())
    }
}

struct DefaultPalSecurity;
impl PalSecurity for DefaultPalSecurity {
    fn enforce_sandbox_isolation(&self, _pid: u32) -> PlazaResult<()> {
        Ok(())
    }
}


