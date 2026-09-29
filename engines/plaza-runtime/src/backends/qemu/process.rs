//! Safe process management for QEMU execution.
//!
//! Architecture:
//!   - Serial output is broadcast to all subscribers via `broadcast::Sender<SerialChunk>`
//!   - Serial writes are serialized through an `mpsc::Sender<Vec<u8>>` channel
//!   - A readiness trigger task periodically sends `echo '<marker>'` until confirmed
//!   - exec_with_output subscribes to the broadcast and reads between start/end markers

use crate::backends::qemu::adapter::QemuAdapter;
use crate::backends::qemu::qmp::QmpClient;
use plaza_foundation::core::{PlazaError, PlazaResult};
use std::net::{SocketAddr, TcpListener};
use std::process::Stdio;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Child, Command};
use tokio::sync::{broadcast, mpsc, watch};
use tracing::{debug, info, warn};

/// A chunk of serial output (bytes from the guest).
#[derive(Clone, Debug)]
pub struct SerialChunk(pub Vec<u8>);

/// Default broadcast channel capacity.
const SERIAL_BROADCAST_CAPACITY: usize = 256;

/// Represents a running QEMU process.
pub struct QemuProcess {
    child: Child,
    qmp: QmpClient,
    ready_rx: Option<mpsc::Receiver<()>>,
    /// Watch channel for readiness signal (true = ready).
    ready_watch: watch::Receiver<bool>,
    /// Send bytes to the guest serial port (thread-safe, multi-producer).
    serial_writer_tx: mpsc::Sender<Vec<u8>>,
    /// Broadcast sender for serial output — allows multiple listeners.
    serial_broadcast: broadcast::Sender<SerialChunk>,
}

impl QemuProcess {
    /// Spawn QEMU with the given adapter arguments.
    pub async fn spawn(
        adapter: QemuAdapter,
        _instance_id: &str,
        readiness_marker: &str,
    ) -> PlazaResult<Self> {
        // Find an open port for QMP on localhost
        let listener = TcpListener::bind("127.0.0.1:0")
            .map_err(|e| PlazaError::process(format!("Failed to bind QMP port: {}", e)))?;
        let port = listener.local_addr().unwrap().port();
        drop(listener);
        let socket_addr: SocketAddr = format!("127.0.0.1:{}", port).parse().unwrap();

        let qmp_arg = format!("tcp:127.0.0.1:{},server=on,wait=off", port);
        let (mut binary, mut args) = adapter.into_command();
        args.push("-qmp".into());
        args.push(qmp_arg.into());

        let serial_listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .map_err(|e| PlazaError::process(format!("Failed to bind serial port: {}", e)))?;
        let serial_port = serial_listener.local_addr().unwrap().port();
        args.push("-serial".into());
        args.push(format!("tcp:127.0.0.1:{}", serial_port).into());

        info!("Spawning QEMU from {:?}", binary);

        #[cfg(windows)]
        let mut child = {
            use std::os::windows::process::CommandExt;
            const DETACHED_PROCESS: u32 = 0x00000008;
            const CREATE_NO_WINDOW: u32 = 0x08000000;
            Command::new(&binary)
                .args(&args)
                .stdin(Stdio::null())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .creation_flags(DETACHED_PROCESS | CREATE_NO_WINDOW)
                .spawn()
                .map_err(|e| PlazaError::process(format!("Failed to spawn QEMU: {}", e)))?
        };
        #[cfg(not(windows))]
        let mut child = Command::new(&binary)
            .args(&args)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| PlazaError::process(format!("Failed to spawn QEMU: {}", e)))?;

        // Wait for serial connection from QEMU
        let (serial_stream, _) =
            tokio::time::timeout(Duration::from_secs(10), serial_listener.accept())
                .await
                .map_err(|_| {
                    PlazaError::process("QEMU did not connect to serial port in time")
                })?
                .map_err(|e| {
                    PlazaError::process(format!("Failed to accept serial connection: {}", e))
                })?;

        let (mut serial_rx, mut serial_tx) = serial_stream.into_split();

        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();
        let (ready_tx, ready_rx) = mpsc::channel(1);
        let (ready_watch_tx, ready_watch) = watch::channel(false);
        let marker = readiness_marker.to_string();

        // Create broadcast channel for serial output
        let (serial_broadcast, _) = broadcast::channel(SERIAL_BROADCAST_CAPACITY);
        let broadcast_tx = serial_broadcast.clone();

        // Create mpsc channel for serial writes (thread-safe, multi-producer)
        let (serial_writer_tx, mut serial_writer_rx) = mpsc::channel::<Vec<u8>>(64);

        // Spawn serial write task: receives bytes from the channel and writes to QEMU
        tokio::spawn(async move {
            use tokio::io::AsyncWriteExt;
            while let Some(data) = serial_writer_rx.recv().await {
                if serial_tx.write_all(&data).await.is_err() {
                    break; // serial closed
                }
                let _ = serial_tx.flush().await;
            }
        });

        // Spawn serial reader: reads from QEMU and broadcasts to all subscribers
        tokio::spawn(async move {
            let mut buf = [0u8; 1024];
            while let Ok(n) = tokio::io::AsyncReadExt::read(&mut serial_rx, &mut buf).await {
                if n == 0 {
                    break;
                }
                let chunk = buf[..n].to_vec();
                if let Ok(s) = std::str::from_utf8(&chunk) {
                    print!("{}", s);
                }
                let _ = broadcast_tx.send(SerialChunk(chunk));
            }
        });

        // Readiness marker detector — subscribes to the broadcast
        let mut ready_subscriber = serial_broadcast.subscribe();
        let marker_clone = marker.clone();
        tokio::spawn(async move {
            let mut line_buf = String::new();
            loop {
                match ready_subscriber.recv().await {
                    Ok(SerialChunk(data)) => {
                        if let Ok(text) = String::from_utf8(data) {
                            line_buf.push_str(&text);
                            if line_buf.contains(&marker_clone) {
                                let _ = ready_tx.send(()).await;
                                let _ = ready_watch_tx.send(true);
                                line_buf.clear();
                            }
                            if let Some(last_nl) = line_buf.rfind('\n') {
                                line_buf = line_buf[last_nl + 1..].to_string();
                            }
                            if line_buf.len() > 4096 {
                                line_buf = line_buf[line_buf.len() - 1024..].to_string();
                            }
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(_)) => continue,
                    Err(broadcast::error::RecvError::Closed) => break,
                }
            }
        });

        // Readiness trigger: periodically sends `echo '<marker>'` via the
        // serial writer channel until the readiness marker is confirmed.
        let trigger_tx = serial_writer_tx.clone();
        let marker_for_trigger = marker.clone();
        let mut ready_watch_for_trigger = ready_watch.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(2));
            interval.tick().await; // skip first immediate tick

            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        let cmd = format!("echo {}\n", marker_for_trigger);
                        if trigger_tx.send(cmd.into_bytes()).await.is_err() {
                            break;
                        }
                    }
                    result = ready_watch_for_trigger.changed() => {
                        if result.is_ok() {
                            info!("Readiness confirmed, stopping trigger");
                        }
                        break;
                    }
                }
            }
        });

        // Drain stderr in background
        tokio::spawn(async move {
            let mut reader = BufReader::new(stderr).lines();
            while let Ok(Some(line)) = reader.next_line().await {
                debug!("QEMU stderr: {}", line);
            }
        });

        let mut qmp = QmpClient::new(socket_addr);
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

        Ok(Self {
            child,
            qmp,
            ready_rx: Some(ready_rx),
            ready_watch,
            serial_writer_tx,
            serial_broadcast,
        })
    }

    /// Subscribe to the serial broadcast channel.
    pub fn subscribe_serial(&self) -> broadcast::Receiver<SerialChunk> {
        self.serial_broadcast.subscribe()
    }

    /// Wait for the readiness marker from the serial output.
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
        let cmd = format!("{}\n", command);
        self.serial_writer_tx
            .send(cmd.into_bytes())
            .await
            .map_err(|_| PlazaError::process("Serial writer channel closed"))?;
        Ok(())
    }

    /// Execute a command in the guest and capture its output.
    pub async fn exec_with_output(
        &mut self,
        cmd: &str,
        timeout: Duration,
    ) -> PlazaResult<String> {
        let start_marker = format!("__PLAZA_EXEC_START_{}__", uuid::Uuid::new_v4());
        let end_marker = format!("__PLAZA_EXEC_END_{}__", uuid::Uuid::new_v4());

        // Subscribe BEFORE sending to avoid missing the response
        let mut subscriber = self.serial_broadcast.subscribe();

        // Send a blank line first to ensure we have a clean prompt,
        // then send the wrapped command.
        // Using printf to avoid shell quoting issues.
        self.send_command("").await?;
        tokio::time::sleep(Duration::from_millis(200)).await;

        let wrapped = format!(
            "echo {start}; {cmd}; echo {end}",
            start = start_marker,
            cmd = cmd,
            end = end_marker,
        );
        self.send_command(&wrapped).await?;

        let mut output = String::new();
        let mut found_start = false;
        let deadline = tokio::time::Instant::now() + timeout;

        loop {
            let remaining = deadline.saturating_duration_since(tokio::time::Instant::now());
            if remaining.is_zero() {
                break;
            }

            match tokio::time::timeout(remaining, subscriber.recv()).await {
                Ok(Ok(SerialChunk(data))) => {
                    if let Ok(text) = String::from_utf8(data) {
                        if found_start {
                            if let Some(idx) = text.find(&end_marker) {
                                output.push_str(&text[..idx]);
                                break;
                            }
                            output.push_str(&text);
                        } else {
                            if let Some(idx) = text.find(&start_marker) {
                                found_start = true;
                                let after = &text[idx + start_marker.len()..];
                                let after = after.strip_prefix('\n').unwrap_or(after);
                                if let Some(end_idx) = after.find(&end_marker) {
                                    output.push_str(&after[..end_idx]);
                                    break;
                                }
                                output.push_str(after);
                            }
                        }
                    }
                }
                Ok(Err(broadcast::error::RecvError::Lagged(_))) => continue,
                Ok(Err(broadcast::error::RecvError::Closed)) => {
                    return Err(PlazaError::process("Serial channel closed unexpectedly"));
                }
                Err(_) => break,
            }
        }

        Ok(output.trim_end().to_string())
    }

    /// Get a mutable reference to the QMP client.
    pub fn qmp_mut(&mut self) -> &mut QmpClient {
        &mut self.qmp
    }

    /// Query comprehensive VM metrics via QMP.
    pub async fn query_metrics(&mut self) -> PlazaResult<serde_json::Value> {
        let mut metrics = serde_json::Map::new();

        if let Ok(status) = self.qmp.query_status().await {
            metrics.insert("status".into(), serde_json::Value::String(status));
        }
        if let Ok(cpus) = self.qmp.query_cpus().await {
            metrics.insert("cpus".into(), cpus);
        }
        if let Ok(mem) = self.qmp.query_memstats().await {
            metrics.insert("memory".into(), mem);
        }
        if let Ok(blocks) = self.qmp.query_blockstats().await {
            metrics.insert("block_stats".into(), blocks);
        }
        if let Ok(version) = self.qmp.query_version().await {
            metrics.insert("version".into(), version);
        }

        Ok(serde_json::Value::Object(metrics))
    }

    /// Pause the guest.
    pub async fn pause(&mut self) -> PlazaResult<()> {
        self.qmp.stop().await
    }

    /// Resume the guest.
    pub async fn resume(&mut self) -> PlazaResult<()> {
        self.qmp.cont().await
    }

    /// Reset the guest.
    pub async fn reset(&mut self) -> PlazaResult<()> {
        self.qmp.system_reset().await
    }

    /// Create a VM snapshot.
    pub async fn snapshot(&mut self, tag: &str) -> PlazaResult<()> {
        self.qmp.savevm(tag).await
    }

    /// Restore a VM snapshot.
    pub async fn restore_snapshot(&mut self, tag: &str) -> PlazaResult<()> {
        self.qmp.loadvm(tag).await
    }

    /// Delete a VM snapshot.
    pub async fn delete_snapshot(&mut self, tag: &str) -> PlazaResult<()> {
        self.qmp.delvm(tag).await
    }

    /// List VM snapshots.
    pub async fn list_snapshots(&mut self) -> PlazaResult<serde_json::Value> {
        self.qmp.query_snapshots().await
    }

    /// Gracefully stop the process via QMP.
    pub async fn stop(&mut self) -> PlazaResult<()> {
        info!("Sending system_powerdown and quit via QMP");
        let _ = self.qmp.system_powerdown().await;
        let _ = self.qmp.quit().await;

        match tokio::time::timeout(Duration::from_secs(5), self.child.wait()).await {
            Ok(Ok(_)) => Ok(()),
            Ok(Err(e)) => Err(PlazaError::process(format!(
                "Failed to wait on QEMU: {}",
                e
            ))),
            Err(_) => {
                warn!("QEMU did not exit gracefully, force killing");
                let _ = self.child.kill().await;
                Ok(())
            }
        }
    }

    /// Query the status of the guest via QMP.
    pub async fn status(&mut self) -> PlazaResult<String> {
        self.qmp.query_status().await
    }

    /// Forcefully kill the process.
    pub async fn force_kill(&mut self) -> PlazaResult<()> {
        warn!("Force killing QEMU process");
        self.child
            .kill()
            .await
            .map_err(|e| PlazaError::process(format!("Failed to kill QEMU process: {}", e)))
    }

    /// Get the OS process ID.
    pub fn id(&self) -> Option<u32> {
        self.child.id()
    }
}
