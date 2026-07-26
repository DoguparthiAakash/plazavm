//! GPU detection module.
//!
//! Phase 2 will implement nvidia-smi, rocm-smi, and Metal queries.
//! For now this is a placeholder that returns an empty list.

use crate::platform::capabilities::GpuCapabilities;

/// Detect GPU devices on the host.
///
/// In Phase 2 this will call platform-specific tools:
/// - `nvidia-smi` for NVIDIA/CUDA
/// - `rocm-smi` for AMD/ROCm
/// - System profiler for Apple Metal
/// - `lspci` / WMI for Intel
pub async fn detect_gpus() -> Vec<GpuCapabilities> {
    // Phase 2: real GPU detection
    Vec::new()
}


