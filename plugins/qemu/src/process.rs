//! Safe process management for QEMU execution.

use crate::adapter::QemuAdapter;
use crate::qmp::QmpClient;
use plaza_foundation::core::{PlazaError, PlazaResult};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};

/// Represents a running QEMU process.
pub struct QemuProcess {
    child: Child,
    qmp: QmpClient,
    socket_path: PathBuf,
}

impl QemuProcess {
    /// Spawn QEMU with the given adapter arguments.
    pub async fn spawn(adapter: QemuAdapter, instance_id: &str) -> PlazaResult<Self> {
        // Prepare QMP socket path (e.g. named pipe on Windows or Unix socket)
        let socket_path = std::env::temp_dir().join(format!("plaza-qmp-{}.sock", instance_id));

        // Add QMP arguments (using standard QEMU arguments)
        // -qmp unix:/path/to/socket,server=on,wait=off
        let qmp_arg = format!("unix:{},server=on,wait=off", socket_path.display());
        let (binary, mut args) = adapter.into_command();
        args.push("-qmp".into());
        args.push(qmp_arg.into());

        let child = Command::new(&binary)
            .args(&args)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| PlazaError::process(format!("Failed to spawn QEMU: {}", e)))?;

        let qmp = QmpClient::new(socket_path.clone());
        
        // Connect to QMP
        qmp.connect().await?;

        Ok(Self {
            child,
            qmp,
            socket_path,
        })
    }

    /// Gracefully stop the process via QMP.
    pub async fn stop(&mut self) -> PlazaResult<()> {
        self.qmp.quit().await?;
        
        // Wait for process to exit
        match self.child.wait() {
            Ok(_) => {
                let _ = std::fs::remove_file(&self.socket_path);
                Ok(())
            },
            Err(e) => Err(PlazaError::process(format!("Failed to wait on QEMU: {}", e))),
        }
    }

    /// Forcefully kill the process.
    pub fn force_kill(&mut self) -> PlazaResult<()> {
        let res = self.child.kill().map_err(|e| {
            PlazaError::process(format!("Failed to kill QEMU process: {}", e))
        });
        let _ = std::fs::remove_file(&self.socket_path);
        res
    }
}
