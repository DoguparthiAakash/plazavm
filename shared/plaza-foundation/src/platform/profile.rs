//! Platform profile classification.

use super::capabilities::*;
use serde::{Deserialize, Serialize};

/// Auto-detected platform profile influencing runtime selection,
/// resource defaults, and scheduling behaviour.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlatformProfile {
    GamingDesktop,
    AiWorkstation,
    DeveloperLaptop,
    LowEndLaptop,
    Server,
    EmbeddedDevice,
    CloudInstance,
    Custom(String),
}

impl PlatformProfile {
    /// Classify the host based on detected capabilities.
    pub fn detect(caps: &HostCapabilities) -> Self {
        let has_powerful_gpu = caps.gpu.iter().any(|g| g.vram_mb >= 8192);
        let has_cuda = caps
            .gpu
            .iter()
            .any(|g| matches!(&g.compute, GpuCompute::Cuda { .. }));
        let high_ram = caps.memory.total_mb >= 32768;
        let many_cores = caps.cpu.cores_logical >= 12;
        let is_headless = caps.os.is_headless;
        let low_ram = caps.memory.total_mb < 8192;

        match () {
            _ if is_headless => Self::Server,
            _ if has_cuda && high_ram => Self::AiWorkstation,
            _ if has_powerful_gpu && many_cores && !has_cuda => Self::GamingDesktop,
            _ if low_ram => Self::LowEndLaptop,
            _ if many_cores || high_ram => Self::DeveloperLaptop,
            _ => Self::DeveloperLaptop,
        }
    }
}

impl std::fmt::Display for PlatformProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GamingDesktop => write!(f, "Gaming Desktop"),
            Self::AiWorkstation => write!(f, "AI Workstation"),
            Self::DeveloperLaptop => write!(f, "Developer Laptop"),
            Self::LowEndLaptop => write!(f, "Low-End Laptop"),
            Self::Server => write!(f, "Server"),
            Self::EmbeddedDevice => write!(f, "Embedded Device"),
            Self::CloudInstance => write!(f, "Cloud Instance"),
            Self::Custom(s) => write!(f, "{s}"),
        }
    }
}
