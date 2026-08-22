//! Abstract machine configuration for PlazaVM backends.

use plaza_foundation::config::machine_section::MachineSection;
use plaza_foundation::core::CapabilityPolicy;
use std::path::PathBuf;

/// The target operating system space to boot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperatingSystemTarget {
    Auto,
    Linux,
    Windows,
    Bsd,
}

impl Default for OperatingSystemTarget {
    fn default() -> Self {
        Self::Auto
    }
}

/// The finalized, typed configuration passed to a runtime backend.
///
/// Backends do not parse `plaza.yaml` directly. They receive this
/// strongly-typed config which has already passed security checks.
#[derive(Debug, Clone)]
pub struct MachineConfig {
    /// The unique workspace ID.
    pub workspace_id: String,
    
    /// The specific instance ID for this execution.
    pub instance_id: String,
    
    /// The architecture and hardware sizing limits.
    pub machine: MachineSection,
    
    /// The security capabilities granted to the runtime.
    pub capabilities: CapabilityPolicy,
    
    /// The target OS space (Linux, Windows, BSD).
    pub os_target: OperatingSystemTarget,
    
    /// The path to the resolved, bootable virtual block device image.
    pub boot_device: PathBuf,
    
    /// Host to guest volume mounts (HostPath -> GuestPath).
    pub volume_mounts: std::collections::HashMap<PathBuf, String>,
    
    /// Host to guest port forwarding (HostPort -> GuestPort).
    pub port_forwards: std::collections::HashMap<u16, u16>,
    
    /// Environment variables to inject into the guest execution context.
    pub env_vars: std::collections::HashMap<String, String>,
    
    /// Optional path to the kernel file for direct kernel boot (e.g. vmlinuz)
    pub kernel_path: Option<PathBuf>,
    
    /// Optional path to the initrd file for direct kernel boot (e.g. initramfs)
    pub initrd_path: Option<PathBuf>,
    
    /// Optional kernel arguments for direct kernel boot (e.g. root=/dev/vda)
    pub kernel_args: Option<String>,
    
    /// Optional path to the modloop image containing kernel modules
    pub modloop_path: Option<PathBuf>,
}

impl Default for MachineConfig {
    fn default() -> Self {
        Self {
            workspace_id: "default_workspace".into(),
            instance_id: "default_instance".into(),
            machine: MachineSection::default(),
            capabilities: CapabilityPolicy::default(),
            os_target: OperatingSystemTarget::default(),
            boot_device: PathBuf::from("default_boot.img"),
            volume_mounts: std::collections::HashMap::new(),
            port_forwards: std::collections::HashMap::new(),
            env_vars: std::collections::HashMap::new(),
            kernel_path: None,
            initrd_path: None,
            kernel_args: None,
            modloop_path: None,
        }
    }
}
