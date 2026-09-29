//! Workspace file transfer between host and guest.
//!
//! Uses base64-encoded data sent through the serial console
//! to transfer files between host and guest environments.

use plaza_foundation::core::{PlazaError, PlazaResult};
use std::path::{Path, PathBuf};
use tracing::{debug, info};

/// Maximum chunk size for serial transfer (to avoid serial buffer overflow).
const MAX_CHUNK_SIZE: usize = 4096;

/// Encode a file's contents as base64 for serial transfer.
pub fn encode_file_for_transfer(path: &Path) -> PlazaResult<String> {
    let content = std::fs::read(path).map_err(|e| {
        PlazaError::storage(format!("Failed to read file {:?}: {}", path, e))
    })?;

    use base64::Engine;
    let encoded = base64::engine::general_purpose::STANDARD.encode(&content);
    Ok(encoded)
}

/// Decode base64-encoded content and write to a file.
pub fn decode_and_write(encoded: &str, output_path: &Path) -> PlazaResult<()> {
    use base64::Engine;
    let content = base64::engine::general_purpose::STANDARD
        .decode(encoded)
        .map_err(|e| PlazaError::storage(format!("Failed to decode base64: {}", e)))?;

    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| {
            PlazaError::storage(format!("Failed to create directory {:?}: {}", parent, e))
        })?;
    }

    std::fs::write(output_path, &content).map_err(|e| {
        PlazaError::storage(format!("Failed to write file {:?}: {}", output_path, e))
    })?;

    Ok(())
}

/// Generate a shell script that writes base64 data to a file in the guest.
pub fn generate_guest_write_script(
    encoded_data: &str,
    guest_path: &str,
) -> String {
    // Split into chunks to avoid serial buffer issues
    let chunks: Vec<&str> = encoded_data
        .as_bytes()
        .chunks(MAX_CHUNK_SIZE)
        .map(|c| std::str::from_utf8(c).unwrap_or(""))
        .collect();

    let mut script = format!("echo '' > {}\n", guest_path);

    for chunk in &chunks {
        script.push_str(&format!(
            "printf '{}' >> {}\n",
            chunk, guest_path
        ));
    }

    script.push_str(&format!(
        "echo '{}' | base64 -d > {}\n",
        encoded_data, guest_path
    ));

    script
}

/// Generate a shell script that reads a file from the guest and outputs base64.
pub fn generate_guest_read_script(guest_path: &str) -> String {
    format!("base64 {}\n", guest_path)
}

/// Prepare a file for transfer to guest.
pub fn prepare_transfer_to_guest(
    host_path: &Path,
    guest_path: &str,
) -> PlazaResult<TransferPayload> {
    let encoded = encode_file_for_transfer(host_path)?;
    let filename = host_path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    let file_size = std::fs::metadata(host_path)
        .map(|m| m.len())
        .unwrap_or(0);

    Ok(TransferPayload {
        filename,
        guest_path: guest_path.to_string(),
        encoded_data: encoded,
        file_size,
    })
}

/// A prepared file transfer payload.
#[derive(Debug, Clone)]
pub struct TransferPayload {
    pub filename: String,
    pub guest_path: String,
    pub encoded_data: String,
    pub file_size: u64,
}

/// Estimate transfer time based on data size and serial baud rate.
pub fn estimate_transfer_time(payload: &TransferPayload) -> std::time::Duration {
    // Serial is typically 115200 baud = ~11520 bytes/sec
    // But TCP serial may be faster; estimate conservatively
    let bytes_per_sec = 50_000; // Conservative estimate for TCP serial
    let total_bytes = payload.encoded_data.len();
    let seconds = (total_bytes as f64 / bytes_per_sec as f64).ceil() as u64;
    std::time::Duration::from_secs(seconds.max(1))
}
