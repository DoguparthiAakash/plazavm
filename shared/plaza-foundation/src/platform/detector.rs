//! Host platform detection using `sysinfo`.

use crate::platform::capabilities::*;
use crate::platform::profile::PlatformProfile;
use crate::core::types::{Architecture, HealthStatus};
use crate::core::PlazaResult;
use std::sync::Arc;
use sysinfo::System;
use tokio::sync::RwLock;
use tracing::info;

/// Detects and caches host platform capabilities.
pub struct PlatformDetector {
    cached: Arc<RwLock<Option<HostCapabilities>>>,
    profile: Arc<RwLock<PlatformProfile>>,
}

impl PlatformDetector {
    pub fn new() -> Self {
        Self {
            cached: Arc::new(RwLock::new(None)),
            profile: Arc::new(RwLock::new(PlatformProfile::DeveloperLaptop)),
        }
    }

    /// Perform a full host scan. Results are cached.
    pub async fn scan(&self) -> PlazaResult<HostCapabilities> {
        info!("scanning host platform capabilities");

        let mut sys = System::new_all();
        sys.refresh_all();

        let os = self.detect_os(&sys);
        let cpu = self.detect_cpu(&sys);
        let memory = self.detect_memory(&sys);
        let storage = self.detect_storage(&sys);
        let gpu = self.detect_gpu();
        let virtualization = self.detect_virtualization();
        let installed_runtimes = self.detect_runtimes().await;

        let caps = HostCapabilities {
            os,
            cpu,
            gpu,
            memory,
            storage,
            virtualization,
            installed_runtimes,
        };

        let detected_profile = PlatformProfile::detect(&caps);
        info!(profile = %detected_profile, "platform profile classified");

        *self.profile.write().await = detected_profile;
        *self.cached.write().await = Some(caps.clone());

        Ok(caps)
    }

    /// Get cached capabilities, scanning if needed.
    pub async fn capabilities(&self) -> PlazaResult<HostCapabilities> {
        if let Some(caps) = self.cached.read().await.as_ref() {
            return Ok(caps.clone());
        }
        self.scan().await
    }

    /// Get the current platform profile.
    pub async fn profile(&self) -> PlatformProfile {
        self.profile.read().await.clone()
    }

    fn detect_os(&self, _sys: &System) -> HostOs {
        HostOs {
            name: System::name().unwrap_or_else(|| "Unknown".into()),
            version: System::os_version().unwrap_or_else(|| "Unknown".into()),
            arch: if cfg!(target_arch = "x86_64") {
                Architecture::X86_64
            } else if cfg!(target_arch = "aarch64") {
                Architecture::Aarch64
            } else {
                Architecture::X86_64
            },
            kernel: System::kernel_version().unwrap_or_else(|| "Unknown".into()),
            is_headless: std::env::var("DISPLAY").is_err()
                && !cfg!(target_os = "windows")
                && !cfg!(target_os = "macos"),
        }
    }

    fn detect_cpu(&self, sys: &System) -> CpuCapabilities {
        let cpus = sys.cpus();
        let first = cpus.first();

        let model = first.map(|c| c.brand().to_string()).unwrap_or_default();
        let vendor_str = first.map(|c| c.vendor_id().to_string()).unwrap_or_default();
        let vendor = match vendor_str.to_lowercase().as_str() {
            s if s.contains("intel") => CpuVendor::Intel,
            s if s.contains("amd") => CpuVendor::Amd,
            s if s.contains("apple") => CpuVendor::Apple,
            s if s.contains("arm") => CpuVendor::Arm,
            _ => CpuVendor::Unknown,
        };

        CpuCapabilities {
            arch: if cfg!(target_arch = "x86_64") {
                Architecture::X86_64
            } else {
                Architecture::Aarch64
            },
            model,
            vendor,
            cores_physical: sys.physical_core_count().unwrap_or(1) as u32,
            cores_logical: cpus.len() as u32,
            frequency_mhz: first.map(|c| c.frequency()).unwrap_or(0),
            features: Vec::new(), // Populated by platform-specific detection in Phase 2
        }
    }

    fn detect_memory(&self, sys: &System) -> MemoryInfo {
        MemoryInfo {
            total_mb: sys.total_memory() / (1024 * 1024),
            available_mb: sys.available_memory() / (1024 * 1024),
            swap_total_mb: sys.total_swap() / (1024 * 1024),
            swap_available_mb: sys.free_swap() / (1024 * 1024),
        }
    }

    fn detect_storage(&self, _sys: &System) -> Vec<StorageInfo> {
        sysinfo::Disks::new_with_refreshed_list()
            .iter()
            .map(|d| StorageInfo {
                name: d.name().to_string_lossy().into_owned(),
                mount_point: d.mount_point().to_string_lossy().into_owned(),
                total_mb: d.total_space() / (1024 * 1024),
                available_mb: d.available_space() / (1024 * 1024),
                fs_type: d.file_system().to_string_lossy().into_owned(),
                is_ssd: !d.is_removable(), // Heuristic; refined in Phase 2
            })
            .collect()
    }

    fn detect_gpu(&self) -> Vec<GpuCapabilities> {
        // GPU detection is platform-specific and requires external tools.
        // Phase 2 will implement nvidia-smi, rocm-smi, etc.
        Vec::new()
    }

    fn detect_virtualization(&self) -> VirtualizationSupport {
        VirtualizationSupport {
            hardware_virt: cfg!(target_arch = "x86_64"), // Heuristic; refined in Phase 2
            nested_virt: false,
            iommu: false,
            hypervisor_present: false,
            platform_hypervisor: None,
        }
    }

    /// Detect installed runtimes by probing for known binaries.
    async fn detect_runtimes(&self) -> Vec<InstalledRuntime> {
        let mut runtimes = Vec::new();

        // Check for common runtime binaries
        let checks = vec![
            ("docker", "Docker Engine", "docker"),
            ("podman", "Podman", "podman"),
            ("qemu-system-x86_64", "QEMU", "qemu"),
            ("VBoxManage", "VirtualBox", "virtualbox"),
        ];

        for (binary, name, id) in checks {
            if let Ok(output) = tokio::process::Command::new(binary)
                .arg("--version")
                .output()
                .await
            {
                if output.status.success() {
                    let version = String::from_utf8_lossy(&output.stdout)
                        .lines()
                        .next()
                        .unwrap_or("unknown")
                        .to_string();
                    runtimes.push(InstalledRuntime {
                        id: id.to_string(),
                        name: name.to_string(),
                        version,
                        path: std::path::PathBuf::from(binary),
                        health: HealthStatus::Healthy,
                    });
                }
            }
        }

        // Windows-specific: check for Hyper-V
        #[cfg(target_os = "windows")]
        {
            if let Ok(output) = tokio::process::Command::new("powershell")
                .args([
                    "-Command",
                    "(Get-WindowsOptionalFeature -Online -FeatureName Microsoft-Hyper-V).State",
                ])
                .output()
                .await
            {
                let state = String::from_utf8_lossy(&output.stdout);
                if state.trim() == "Enabled" {
                    runtimes.push(InstalledRuntime {
                        id: "hyperv".to_string(),
                        name: "Hyper-V".to_string(),
                        version: "Windows Built-in".to_string(),
                        path: std::path::PathBuf::from("powershell"),
                        health: HealthStatus::Healthy,
                    });
                }
            }
        }

        runtimes
    }
}

impl Default for PlatformDetector {
    fn default() -> Self {
        Self::new()
    }
}



