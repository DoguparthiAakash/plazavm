//! Typed capability advertisement for runtime backends.

use plaza_foundation::core::types::{Architecture, OperatingSystem};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Declares what a runtime backend can and cannot do.
///
/// The Decision Engine uses this to match workspace requirements
/// against available backends during automatic runtime selection.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RuntimeCapabilities {
    // ── Operating system & architecture support ─────────────────────────────
    /// Guest operating systems this backend can run.
    pub supported_os: Vec<OperatingSystem>,
    /// CPU architectures this backend can target.
    pub supported_arch: Vec<Architecture>,

    // ── Lifecycle capabilities ──────────────────────────────────────────────
    /// Can pause and resume a running instance.
    pub can_pause: bool,
    /// Can create and restore snapshots.
    pub can_snapshot: bool,
    /// Can live-migrate a running instance to another host.
    pub can_live_migrate: bool,

    // ── Resource capabilities ───────────────────────────────────────────────
    /// Can dynamically adjust CPU allocation while running.
    pub can_resize_cpu: bool,
    /// Can dynamically adjust memory allocation while running.
    pub can_resize_memory: bool,
    /// Can hot-add a disk to a running instance.
    pub can_hot_add_disk: bool,
    /// Can pass through a host GPU to the guest.
    pub can_gpu_passthrough: bool,

    // ── Hardware / device capabilities ──────────────────────────────────────
    /// Can pass through USB devices to the guest.
    pub can_usb_passthrough: bool,
    /// Can emulate or pass through a TPM device.
    pub can_tpm: bool,
    /// Can boot from EFI/UEFI firmware.
    pub can_efi: bool,
    /// Supports running a hypervisor inside the guest.
    pub can_nested_virtualization: bool,

    // ── Networking ──────────────────────────────────────────────────────────
    /// Can bridge the guest network to the host network.
    pub can_bridge_network: bool,
    /// Can forward specific host ports to guest ports.
    pub can_port_forward: bool,
    /// Supports overlay/virtual networking between instances.
    pub supports_overlay_network: bool,

    // ── Storage ─────────────────────────────────────────────────────────────
    /// Uses overlay filesystem for layered storage.
    pub supports_overlay_fs: bool,
    /// Can mount host directories into the guest.
    pub supports_volume_mounts: bool,

    // ── Integration ─────────────────────────────────────────────────────────
    /// Provides an interactive console/shell.
    pub supports_console: bool,
    /// Can expose a VNC display.
    pub supports_vnc: bool,
    /// Can expose a SPICE display.
    pub supports_spice: bool,
    /// Can run without root/administrator privileges.
    pub supports_rootless: bool,

    // ── Extensible custom capabilities ──────────────────────────────────────
    /// Additional capabilities not covered by typed fields.
    /// Plugins can advertise arbitrary key-value capabilities here.
    #[serde(default)]
    pub custom: HashMap<String, serde_json::Value>,
}

impl RuntimeCapabilities {
    /// Check whether this backend supports a specific OS.
    pub fn supports_os(&self, os: &OperatingSystem) -> bool {
        self.supported_os.contains(os)
    }

    /// Check whether this backend supports a specific architecture.
    pub fn supports_arch(&self, arch: &Architecture) -> bool {
        self.supported_arch.contains(arch)
    }

    /// Check whether a named custom capability exists.
    pub fn has_custom(&self, name: &str) -> bool {
        self.custom.contains_key(name)
    }
}

