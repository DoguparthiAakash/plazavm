//! # plaza-platform
//!
//! Host platform intelligence for PlazaVM.
//!
//! Detects the host operating system, CPU capabilities, GPU hardware,
//! installed virtualization runtimes, hardware virtualization support,
//! and generates a [`PlatformProfile`] that influences runtime selection,
//! resource scheduling, and workspace defaults.

pub mod capabilities;
pub mod detector;
pub mod gpu;
pub mod kal;
pub mod profile;
pub mod runtime_detection;

pub use capabilities::{
    CpuCapabilities, CpuVendor, GpuCapabilities, GpuCompute, GpuVendor, HostCapabilities, HostOs,
    InstalledRuntime, MemoryInfo, RuntimeHealthReport, StorageInfo, VirtualizationSupport,
};
pub use detector::PlatformDetector;
pub use kal::{KernelAdapter, KernelCapabilities, LinuxKernelAdapter};
pub use profile::PlatformProfile;
