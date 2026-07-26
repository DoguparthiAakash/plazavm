//! Host capability data model.

use crate::core::types::{Architecture, HealthStatus};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Complete snapshot of host capabilities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostCapabilities {
    pub os: HostOs,
    pub cpu: CpuCapabilities,
    pub gpu: Vec<GpuCapabilities>,
    pub memory: MemoryInfo,
    pub storage: Vec<StorageInfo>,
    pub virtualization: VirtualizationSupport,
    pub installed_runtimes: Vec<InstalledRuntime>,
}

/// Host operating system information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostOs {
    pub name: String,
    pub version: String,
    pub arch: Architecture,
    pub kernel: String,
    pub is_headless: bool,
}

/// CPU capabilities detected on the host.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuCapabilities {
    pub arch: Architecture,
    pub model: String,
    pub vendor: CpuVendor,
    pub cores_physical: u32,
    pub cores_logical: u32,
    pub frequency_mhz: u64,
    pub features: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CpuVendor {
    Intel,
    Amd,
    Apple,
    Arm,
    Unknown,
}

/// GPU capabilities for a single GPU device.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuCapabilities {
    pub name: String,
    pub vendor: GpuVendor,
    pub vram_mb: u64,
    pub compute: GpuCompute,
    pub driver_version: Option<String>,
    pub passthrough_capable: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GpuVendor {
    Nvidia,
    Amd,
    Intel,
    Apple,
    Unknown,
}

/// GPU compute API support.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum GpuCompute {
    Cuda {
        version: String,
        compute_capability: String,
    },
    Rocm {
        version: String,
    },
    Metal {
        version: String,
    },
    Intel {
        version: String,
    },
    None,
}

/// Host memory information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryInfo {
    pub total_mb: u64,
    pub available_mb: u64,
    pub swap_total_mb: u64,
    pub swap_available_mb: u64,
}

/// Information about a storage device/mount.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageInfo {
    pub name: String,
    pub mount_point: String,
    pub total_mb: u64,
    pub available_mb: u64,
    pub fs_type: String,
    pub is_ssd: bool,
}

/// Hardware virtualization support.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualizationSupport {
    /// VT-x (Intel) or AMD-V is available.
    pub hardware_virt: bool,
    /// Nested virtualization is supported.
    pub nested_virt: bool,
    /// IOMMU (VT-d / AMD-Vi) is available.
    pub iommu: bool,
    /// Already running under a hypervisor.
    pub hypervisor_present: bool,
    /// Name of the platform hypervisor if detected (e.g. "Hyper-V", "KVM").
    pub platform_hypervisor: Option<String>,
}

/// An installed runtime detected on the host.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstalledRuntime {
    /// Backend identifier (e.g., `"docker"`, `"virtualbox"`).
    pub id: String,
    /// Display name.
    pub name: String,
    /// Installed version.
    pub version: String,
    /// Path to the runtime binary.
    pub path: PathBuf,
    /// Whether the runtime is currently operational.
    pub health: HealthStatus,
}

/// Health scoring for a detected runtime.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeHealthReport {
    pub runtime_id: String,
    pub installed: bool,
    pub version: Option<String>,
    /// Health score from 0.0 (broken) to 1.0 (perfect).
    pub health_score: f64,
    /// Specific issues found.
    pub issues: Vec<String>,
    pub gpu_ready: bool,
    pub snapshot_capable: bool,
}


