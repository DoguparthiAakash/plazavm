//! Safe process management for QEMU execution.

use crate::backends::qemu::adapter::QemuAdapter;
use crate::backends::qemu::qmp::QmpClient;
use plaza_foundation::core::{PlazaError, PlazaResult};
use std::net::{SocketAddr, TcpListener};
use std::process::Stdio;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Child, Command};
use tokio::sync::mpsc;
use tracing::{info, warn, debug};

/// Represents a running QEMU process.
pub struct QemuProcess {
    child: Child,
    qmp: QmpClient,
    ready_rx: Option<mpsc::Receiver<()>>,
    serial_tx: Option<tokio::net::tcp::OwnedWriteHalf>,
}

impl QemuProcess {
    /// Spawn QEMU with the given adapter arguments.
    pub async fn spawn(adapter: QemuAdapter, _instance_id: &str) -> PlazaResult<Self> {
        // Find an open port for QMP on localhost
        let listener = TcpListener::bind("127.0.0.1:0")
            .map_err(|e| PlazaError::process(format!("Failed to bind QMP port: {}", e)))?;
        let port = listener.local_addr().unwrap().port();
        
        // We must drop the listener so QEMU can bind to it
        drop(listener);
        let socket_addr: SocketAddr = format!("127.0.0.1:{}", port).parse().unwrap();

        // Add QMP arguments (using standard QEMU arguments)
        // -qmp tcp:127.0.0.1:port,server=on,wait=off
        let qmp_arg = format!("tcp:127.0.0.1:{},server=on,wait=off", port);
        let (mut binary, mut args) = adapter.into_command();
        args.push("-qmp".into());
        args.push(qmp_arg.into());
        
        let serial_listener = tokio::net::TcpListener::bind("127.0.0.1:0").await
            .map_err(|e| PlazaError::process(format!("Failed to bind serial port: {}", e)))?;
        let serial_port = serial_listener.local_addr().unwrap().port();
        args.push("-serial".into());
        args.push(format!("tcp:127.0.0.1:{}", serial_port).into());

        info!("Spawning QEMU from {:?}", binary);

        let mut child = Command::new(&binary)
            .args(&args)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| PlazaError::process(format!("Failed to spawn QEMU: {}", e)))?;

        // Wait for serial connection from QEMU
        let (serial_stream, _) = tokio::time::timeout(Duration::from_secs(10), serial_listener.accept()).await
            .map_err(|_| PlazaError::process("QEMU did not connect to serial port in time"))?
            .map_err(|e| PlazaError::process(format!("Failed to accept serial connection: {}", e)))?;
            
        let (mut serial_rx, serial_tx) = serial_stream.into_split();

        // Monitor stdout for PLAZA_GUEST_READY
        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();
        let (ready_tx, ready_rx) = mpsc::channel(1);
        
        tokio::spawn(async move {
            let mut buf = [0u8; 1024];
            let mut line_buf = String::new();
            while let Ok(n) = tokio::io::AsyncReadExt::read(&mut serial_rx, &mut buf).await {
                if n == 0 { break; }
                let chunk = String::from_utf8_lossy(&buf[..n]);
                print!("{}", chunk);
                debug!("QEMU output: {}", chunk);
                line_buf.push_str(&chunk);
                if line_buf.contains(plaza_machine::OS_READY_MARKER) {
                    let _ = ready_tx.send(()).await;
                    line_buf.clear(); // prevent multiple sends
                }
                if let Some(last_nl) = line_buf.rfind('\n') {
                    // Keep only the part after the last newline
                    let remainder = line_buf[last_nl + 1..].to_string();
                    line_buf = remainder;
                }
                // Cap line_buf size just in case
                if line_buf.len() > 4096 {
                    line_buf = line_buf[line_buf.len() - 1024..].to_string();
                }
            }
        });

        tokio::spawn(async move {
            let mut reader = BufReader::new(stderr).lines();
            while let Ok(Some(line)) = reader.next_line().await {
                println!("QEMU Error: {}", line);
                debug!("QEMU Error: {}", line);
            }
        });

        let mut qmp = QmpClient::new(socket_addr);
        
        // Try connecting to QMP with timeout
        match tokio::time::timeout(Duration::from_secs(10), qmp.connect()).await {
            Ok(Ok(_)) => {
                info!("QEMU process spawned and QMP connected successfully");
            }
            Ok(Err(e)) => {
                let _ = child.kill().await;
                return Err(e);
            }
            Err(_) => {
                let _ = child.kill().await;
                return Err(PlazaError::process("QMP connection timed out"));
            }
        }

        Ok(Self { child, qmp, ready_rx: Some(ready_rx), serial_tx: Some(serial_tx) })
    }

    /// Wait for the PLAZA_GUEST_READY marker from the serial output.
    pub async fn wait_for_ready(&mut self, timeout: Duration) -> PlazaResult<()> {
        if let Some(rx) = self.ready_rx.as_mut() {
            match tokio::time::timeout(timeout, rx.recv()).await {
                Ok(Some(_)) => Ok(()),
                Ok(None) => Err(PlazaError::process("QEMU process exited before ready")),
                Err(_) => Err(PlazaError::process("Guest readiness timeout")),
            }
        } else {
            Ok(())
        }
    }

    /// Send a string command to the guest's serial console.
    pub async fn send_command(&mut self, command: &str) -> PlazaResult<()> {
        if let Some(serial_tx) = self.serial_tx.as_mut() {
            use tokio::io::AsyncWriteExt;
            let cmd = format!("{}\n", command); // Use \n as we now talk directly to the Linux guest terminal
            serial_tx.write_all(cmd.as_bytes()).await.map_err(|e| {
                PlazaError::process(format!("Failed to write to QEMU serial port: {}", e))
            })?;
            serial_tx.flush().await.map_err(|e| {
                PlazaError::process(format!("Failed to flush QEMU serial port: {}", e))
            })?;
            Ok(())
        } else {
            Err(PlazaError::process("QEMU serial port is not available"))
        }
    }

    /// Gracefully stop the process via QMP.
    pub async fn stop(&mut self) -> PlazaResult<()> {
        info!("Sending system_powerdown and quit via QMP");
        // Try to powerdown first for graceful OS shutdown, then quit
        let _ = self.qmp.system_powerdown().await;
        let _ = self.qmp.quit().await;
        
        // Wait for process to exit
        match tokio::time::timeout(Duration::from_secs(5), self.child.wait()).await {
            Ok(Ok(_)) => Ok(()),
            Ok(Err(e)) => Err(PlazaError::process(format!("Failed to wait on QEMU: {}", e))),
            Err(_) => {
                warn!("QEMU did not exit gracefully, force killing");
                let _ = self.child.kill().await;
                Ok(())
            }
        }
    }

    /// Query the status of the guest via QMP
    pub async fn status(&mut self) -> PlazaResult<String> {
        self.qmp.query_status().await
    }

    /// Forcefully kill the process.
    pub async fn force_kill(&mut self) -> PlazaResult<()> {
        warn!("Force killing QEMU process");
        self.child.kill().await.map_err(|e| {
            PlazaError::process(format!("Failed to kill QEMU process: {}", e))
        })
    }

    /// Get the OS process ID.
    pub fn id(&self) -> Option<u32> {
        self.child.id()
    }
}
