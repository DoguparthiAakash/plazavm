//! Virtual Hardware Abstraction Layer (VHAL) resource models & profiles.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum HardwareProfileKind {
    #[default]
    Desktop,
    Server,
    AiWorkstation,
    Minimal,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualHardwareProfile {
    pub kind: HardwareProfileKind,
    pub vcpu_cores: u32,
    pub memory_mb: u64,
    pub gpu_vram_mb: Option<u64>,
    pub disk_size_gb: u64,
    pub enable_audio: bool,
    pub enable_display: bool,
}

impl VirtualHardwareProfile {
    pub fn for_kind(kind: HardwareProfileKind) -> Self {
        match kind {
            HardwareProfileKind::Desktop => Self {
                kind,
                vcpu_cores: 4,
                memory_mb: 8192,
                gpu_vram_mb: None,
                disk_size_gb: 50,
                enable_audio: true,
                enable_display: true,
            },
            HardwareProfileKind::Server => Self {
                kind,
                vcpu_cores: 8,
                memory_mb: 16384,
                gpu_vram_mb: None,
                disk_size_gb: 100,
                enable_audio: false,
                enable_display: false,
            },
            HardwareProfileKind::AiWorkstation => Self {
                kind,
                vcpu_cores: 16,
                memory_mb: 32768,
                gpu_vram_mb: Some(16384),
                disk_size_gb: 250,
                enable_audio: false,
                enable_display: true,
            },
            HardwareProfileKind::Minimal => Self {
                kind,
                vcpu_cores: 1,
                memory_mb: 1024,
                gpu_vram_mb: None,
                disk_size_gb: 10,
                enable_audio: false,
                enable_display: false,
            },
            HardwareProfileKind::Custom => Self::default(),
        }
    }
}

impl Default for VirtualHardwareProfile {
    fn default() -> Self {
        Self::for_kind(HardwareProfileKind::Desktop)
    }
}

