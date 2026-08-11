//! Abstract machine configuration for PlazaVM backends.

use plaza_foundation::config::machine_section::MachineSection;
use plaza_foundation::core::CapabilityPolicy;
use std::path::PathBuf;

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
    
    /// The path to the resolved, bootable virtual block device image.
    pub boot_device: PathBuf,
}
