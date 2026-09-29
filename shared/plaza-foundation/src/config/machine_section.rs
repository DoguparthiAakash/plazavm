//! Machine section of `plaza.yaml`.
//!
//! Describes the virtual hardware configuration for a PlazaVM workspace.
//! This is backend-independent — neither QEMU nor v86 types should appear here.

use crate::core::types::Architecture;
use serde::{Deserialize, Serialize};

/// The `machine:` section of `plaza.yaml`.
///
/// # Example
///
/// ```yaml
/// machine:
///   architecture: x86_64
///   cpu:
///     cores: 4
///   memory:
///     size: 4096MiB
///   display:
///     enabled: true
///   console:
///     serial: true
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MachineSection {
    /// Target CPU architecture.
    #[serde(default = "default_arch")]
    pub architecture: Architecture,

    /// CPU configuration.
    #[serde(default)]
    pub cpu: CpuSection,

    /// Memory configuration.
    #[serde(default)]
    pub memory: MemorySection,

    /// Display configuration.
    #[serde(default)]
    pub display: DisplaySection,

    /// Console configuration.
    #[serde(default)]
    pub console: ConsoleSection,

    /// Firmware configuration.
    #[serde(default)]
    pub firmware: FirmwareSection,

    /// Boot configuration.
    #[serde(default)]
    pub boot: BootSection,
}

fn default_arch() -> Architecture {
    Architecture::X86_64
}

impl Default for MachineSection {
    fn default() -> Self {
        Self {
            architecture: Architecture::X86_64,
            cpu: CpuSection::default(),
            memory: MemorySection::default(),
            display: DisplaySection::default(),
            console: ConsoleSection::default(),
            firmware: FirmwareSection::default(),
            boot: BootSection::default(),
        }
    }
}

/// CPU configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuSection {
    /// Number of virtual CPU cores.
    #[serde(default = "default_cores")]
    pub cores: u32,
}

fn default_cores() -> u32 {
    2
}

impl Default for CpuSection {
    fn default() -> Self {
        Self { cores: 2 }
    }
}

/// Memory configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySection {
    /// Memory size as a human-readable string (e.g., `"2048MiB"`, `"4Gi"`).
    #[serde(default = "default_memory_size")]
    pub size: String,
}

fn default_memory_size() -> String {
    "256MiB".into()
}

impl Default for MemorySection {
    fn default() -> Self {
        Self {
            size: "2048MiB".into(),
        }
    }
}

/// Display configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplaySection {
    /// Whether the virtual display is enabled.
    #[serde(default = "default_true")]
    pub enabled: bool,
}

fn default_true() -> bool {
    true
}

impl Default for DisplaySection {
    fn default() -> Self {
        Self { enabled: true }
    }
}

/// Console configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConsoleSection {
    /// Enable serial console.
    #[serde(default)]
    pub serial: bool,
}

/// Firmware configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirmwareSection {
    /// Firmware type (e.g., `"bios"`, `"uefi"`).
    #[serde(default = "default_firmware_type")]
    pub firmware_type: String,
}

fn default_firmware_type() -> String {
    "bios".into()
}

impl Default for FirmwareSection {
    fn default() -> Self {
        Self {
            firmware_type: "bios".into(),
        }
    }
}

/// Boot configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BootSection {
    /// Boot order (e.g., `["disk", "cdrom"]`).
    #[serde(default = "default_boot_order")]
    pub order: Vec<String>,
}

fn default_boot_order() -> Vec<String> {
    vec!["disk".into()]
}

impl Default for BootSection {
    fn default() -> Self {
        Self {
            order: vec!["disk".into()],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_minimal() {
        let yaml = r#"
architecture: x86_64
cpu:
  cores: 4
memory:
  size: 4096MiB
"#;
        let section: MachineSection = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(section.cpu.cores, 4);
        assert_eq!(section.memory.size, "4096MiB");
        assert_eq!(section.architecture, Architecture::X86_64);
    }

    #[test]
    fn defaults_are_sane() {
        let section = MachineSection::default();
        assert_eq!(section.cpu.cores, 2);
        assert_eq!(section.memory.size, "2048MiB");
        assert!(section.display.enabled);
        assert!(!section.console.serial);
        assert_eq!(section.firmware.firmware_type, "bios");
    }
}
