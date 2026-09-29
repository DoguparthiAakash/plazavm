//! QEMU Machine Protocol (QMP) client.
//!
//! Uses JSON-RPC over a local TCP socket (cross-platform for PlazaVM) to
//! control and interrogate a running QEMU instance.

use plaza_foundation::core::{PlazaError, PlazaResult};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};
use tracing::debug;

/// QMP Client.
pub struct QmpClient {
    address: SocketAddr,
    stream: Option<TcpStream>,
}

#[derive(Serialize)]
struct QmpCommand {
    execute: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    arguments: Option<serde_json::Value>,
}

#[derive(Deserialize, Debug)]
struct QmpResponse {
    #[serde(default)]
    r#return: Option<serde_json::Value>,
    #[serde(default)]
    error: Option<QmpError>,
}

#[derive(Deserialize, Debug)]
struct QmpError {
    class: String,
    desc: String,
}

impl QmpClient {
    /// Create a new QMP client tied to a socket address.
    pub fn new(address: SocketAddr) -> Self {
        Self {
            address,
            stream: None,
        }
    }

    /// Connect to the QMP socket and perform the initial handshake.
    pub async fn connect(&mut self) -> PlazaResult<()> {
        let mut retries = 0;
        let stream = loop {
            match TcpStream::connect(self.address).await {
                Ok(s) => break s,
                Err(e) => {
                    retries += 1;
                    if retries > 100 {
                        return Err(PlazaError::process(format!(
                            "Failed to connect to QMP socket after 100 retries: {}",
                            e
                        )));
                    }
                    tokio::time::sleep(Duration::from_millis(50)).await;
                }
            }
        };

        // Handshake involves reading the greeting and sending qmp_capabilities
        let mut reader = BufReader::new(stream);
        let mut greeting = String::new();
        timeout(Duration::from_secs(5), reader.read_line(&mut greeting))
            .await
            .map_err(|_| PlazaError::process("QMP connection timeout"))?
            .map_err(|e| PlazaError::process(format!("QMP read error: {}", e)))?;

        debug!("QMP Greeting: {}", greeting.trim());

        // We must extract the stream back to write
        let mut stream = reader.into_inner();

        // Send qmp_capabilities to exit negotiation mode
        let cap_cmd = serde_json::to_string(&QmpCommand {
            execute: "qmp_capabilities".to_string(),
            arguments: None,
        })
        .unwrap()
            + "\n";

        stream
            .write_all(cap_cmd.as_bytes())
            .await
            .map_err(|e| PlazaError::process(format!("QMP write error: {}", e)))?;

        let mut reader = BufReader::new(stream);
        let mut cap_resp = String::new();
        timeout(Duration::from_secs(5), reader.read_line(&mut cap_resp))
            .await
            .map_err(|_| PlazaError::process("QMP cap timeout"))?
            .map_err(|e| PlazaError::process(format!("QMP cap read error: {}", e)))?;

        debug!("QMP Cap Response: {}", cap_resp.trim());

        self.stream = Some(reader.into_inner());

        Ok(())
    }

    /// Send a 'quit' command to gracefully shut down QEMU.
    pub async fn quit(&mut self) -> PlazaResult<()> {
        let _ = self.execute("quit", None).await;
        Ok(())
    }

    pub async fn system_powerdown(&mut self) -> PlazaResult<()> {
        let _ = self.execute("system_powerdown", None).await;
        Ok(())
    }

    /// Query the status of the guest.
    pub async fn query_status(&mut self) -> PlazaResult<String> {
        let resp = self.execute("query-status", None).await?;
        if let Some(status) = resp.get("status") {
            Ok(status.as_str().unwrap_or("unknown").to_string())
        } else {
            Ok("unknown".to_string())
        }
    }

    /// Query CPU statistics.
    pub async fn query_cpus(&mut self) -> PlazaResult<serde_json::Value> {
        self.execute("query-cpus", None).await
    }

    /// Query memory statistics.
    pub async fn query_memstats(&mut self) -> PlazaResult<serde_json::Value> {
        self.execute("query-memstats", None).await
    }

    /// Query block device statistics.
    pub async fn query_blockstats(&mut self) -> PlazaResult<serde_json::Value> {
        self.execute("query-blockstats", None).await
    }

    /// Query VM status with full details.
    pub async fn query_status_detail(&mut self) -> PlazaResult<serde_json::Value> {
        self.execute("query-status", None).await
    }

    /// Query character devices (serial ports).
    pub async fn query_chardev(&mut self) -> PlazaResult<serde_json::Value> {
        self.execute("query-chardev", None).await
    }

    /// Query the QEMU version.
    pub async fn query_version(&mut self) -> PlazaResult<serde_json::Value> {
        self.execute("query-version", None).await
    }

    /// Take a snapshot of the current VM state.
    pub async fn savevm(&mut self, tag: &str) -> PlazaResult<()> {
        let args = serde_json::json!({"name": tag});
        self.execute("savevm", Some(args)).await?;
        Ok(())
    }

    /// Restore a VM snapshot.
    pub async fn loadvm(&mut self, tag: &str) -> PlazaResult<()> {
        let args = serde_json::json!({"name": tag});
        self.execute("loadvm", Some(args)).await?;
        Ok(())
    }

    /// Delete a VM snapshot.
    pub async fn delvm(&mut self, tag: &str) -> PlazaResult<()> {
        let args = serde_json::json!({"name": tag});
        self.execute("delvm", Some(args)).await?;
        Ok(())
    }

    /// List available VM snapshots.
    pub async fn query_snapshots(&mut self) -> PlazaResult<serde_json::Value> {
        self.execute("query-snapshots", None).await
    }

    /// Create a block backup (snapshot) of a device.
    pub async fn blockdev_backup(&mut self, device: &str, target: &str) -> PlazaResult<()> {
        let args = serde_json::json!({
            "device": device,
            "target": target,
            "sync": "full",
            "format": "raw"
        });
        self.execute("blockdev-backup", Some(args)).await?;
        Ok(())
    }

    /// Stop guest CPUs (pause).
    pub async fn stop(&mut self) -> PlazaResult<()> {
        self.execute("stop", None).await?;
        Ok(())
    }

    /// Resume guest CPUs.
    pub async fn cont(&mut self) -> PlazaResult<()> {
        self.execute("cont", None).await?;
        Ok(())
    }

    /// Reset the guest.
    pub async fn system_reset(&mut self) -> PlazaResult<()> {
        self.execute("system_reset", None).await?;
        Ok(())
    }

    /// Send a key event to the guest.
    pub async fn send_key(&mut self, key: &str, hold: bool) -> PlazaResult<()> {
        let args = serde_json::json!({
            "key": key,
            "hold": hold
        });
        self.execute("send-key", Some(args)).await?;
        Ok(())
    }

    /// Execute an arbitrary QMP command.
    pub async fn execute(
        &mut self,
        command: &str,
        args: Option<serde_json::Value>,
    ) -> PlazaResult<serde_json::Value> {
        let stream = match self.stream.as_mut() {
            Some(s) => s,
            None => return Err(PlazaError::process("QMP not connected")),
        };

        let cmd = QmpCommand {
            execute: command.to_string(),
            arguments: args,
        };

        let cmd_str = serde_json::to_string(&cmd).unwrap() + "\n";
        stream
            .write_all(cmd_str.as_bytes())
            .await
            .map_err(|e| PlazaError::process(format!("QMP write error: {}", e)))?;

        let mut reader = BufReader::new(stream);
        loop {
            let mut line = String::new();
            match timeout(Duration::from_secs(5), reader.read_line(&mut line)).await {
                Ok(Ok(0)) => {
                    return Err(PlazaError::process("QMP socket closed unexpectedly"));
                }
                Ok(Ok(_)) => {
                    // QEMU sends events as well, we skip them for now
                    if line.contains("\"event\":") {
                        continue;
                    }

                    let resp: QmpResponse = serde_json::from_str(&line)
                        .map_err(|e| PlazaError::process(format!("Failed to parse QMP: {}", e)))?;

                    if let Some(err) = resp.error {
                        return Err(PlazaError::process(format!(
                            "QMP error {}: {}",
                            err.class, err.desc
                        )));
                    }

                    return Ok(resp.r#return.unwrap_or(serde_json::Value::Null));
                }
                Ok(Err(e)) => return Err(PlazaError::process(format!("QMP read error: {}", e))),
                Err(_) => return Err(PlazaError::process("QMP response timeout")),
            }
        }
    }
}
