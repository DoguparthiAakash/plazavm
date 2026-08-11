//! Discovery and validation of QEMU binaries.

use plaza_foundation::core::{PlazaError, PlazaResult};
use std::path::PathBuf;
use std::process::Command;

/// Discovers the QEMU executable for the given architecture.
///
/// Ensures the executable is present and functioning.
pub fn discover_qemu(arch: &str) -> PlazaResult<PathBuf> {
    let binary_name = format!("qemu-system-{}", arch);
    
    // Check if the binary is in PATH
    let output = Command::new(&binary_name)
        .arg("--version")
        .output()
        .map_err(|e| PlazaError::process(format!("Failed to execute {}: {}", binary_name, e)))?;

    if !output.status.success() {
        return Err(PlazaError::process(format!(
            "QEMU binary {} returned non-zero status on version check",
            binary_name
        )));
    }

    // In a real implementation we might use `which` crate to resolve full path,
    // but for now relying on PATH is sufficient as long as execution succeeds.
    Ok(PathBuf::from(binary_name))
}
