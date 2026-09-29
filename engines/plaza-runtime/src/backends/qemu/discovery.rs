//! Discovery and validation of QEMU binaries.

use plaza_foundation::core::{PlazaError, PlazaResult};
use std::path::PathBuf;

/// Known system QEMU installation paths on Windows.
const WINDOWS_QEMU_PATHS: &[&str] = &[
    "C:\\Program Files\\qemu\\qemu-system-{}.exe",
    "C:\\Program Files (x86)\\qemu\\qemu-system-{}.exe",
];

/// Known system QEMU installation paths on Linux.
const LINUX_QEMU_PATHS: &[&str] = &[
    "/usr/bin/qemu-system-{}",
    "/usr/local/bin/qemu-system-{}",
    "/opt/qemu/bin/qemu-system-{}",
];

/// Discovers the QEMU executable for the given architecture.
///
/// Checks:
/// 1. plaza-qemu (bundled, same dir as CLI)
/// 2. System PATH
/// 3. Known installation directories
pub fn discover_qemu(arch: &str) -> PlazaResult<PathBuf> {
    // 1. Try plaza-qemu (bundled with PlazaVM)
    let bundled_name = if cfg!(windows) {
        "plaza-qemu.exe"
    } else {
        "plaza-qemu"
    };
    let bundled_path = std::env::current_exe()
        .unwrap_or_else(|_| PathBuf::from("."))
        .parent()
        .unwrap_or_else(|| std::path::Path::new("."))
        .join(bundled_name);
    if bundled_path.exists() {
        let output = std::process::Command::new(&bundled_path)
            .arg("--version")
            .output();
        if let Ok(o) = output {
            if o.status.success() {
                return Ok(bundled_path);
            }
        }
    }

    // 2. Try PATH
    let binary_name = format!("qemu-system-{}", arch);
    if let Ok(output) = std::process::Command::new(&binary_name)
        .arg("--version")
        .output()
    {
        if output.status.success() {
            // Resolve full path from PATH
            if let Ok(path_output) = std::process::Command::new(if cfg!(windows) { "where" } else { "which" })
                .arg(&binary_name)
                .output()
            {
                let stdout = String::from_utf8_lossy(&path_output.stdout);
                if let Some(first_line) = stdout.lines().next() {
                    let p = PathBuf::from(first_line.trim());
                    if p.exists() {
                        return Ok(p);
                    }
                }
            }
            return Ok(PathBuf::from(&binary_name));
        }
    }

    // 3. Try known installation paths
    let known_paths = if cfg!(windows) {
        WINDOWS_QEMU_PATHS
    } else {
        LINUX_QEMU_PATHS
    };

    for path_template in known_paths {
        let path_str = path_template.replace("{}", arch);
        let path = PathBuf::from(&path_str);
        if path.exists() {
            if let Ok(output) = std::process::Command::new(&path)
                .arg("--version")
                .output()
            {
                if output.status.success() {
                    return Ok(path);
                }
            }
        }
    }

    Err(PlazaError::process(format!(
        "QEMU not found for architecture '{}'. \
         Install QEMU or set QEMU_PATH environment variable. \
         Searched: bundled plaza-qemu, PATH, {} known paths",
        arch,
        known_paths.len()
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discover_qemu_returns_path_or_error() {
        // Should either find QEMU or return a descriptive error
        match discover_qemu("x86_64") {
            Ok(path) => assert!(!path.as_os_str().is_empty()),
            Err(e) => {
                let msg = e.to_string();
                assert!(msg.contains("QEMU not found") || msg.contains("Failed to execute"));
            }
        }
    }
}
