//! Kernel Adaptation Layer (KAL) providing abstract interface to Linux kernel primitives.

use async_trait::async_trait;
use plaza_core::PlazaResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KernelCapabilities {
    pub cgroups_v2: bool,
    pub namespaces: bool,
    pub seccomp: bool,
    pub landlock: bool,
    pub ebpf: bool,
    pub io_uring: bool,
}

#[async_trait]
pub trait KernelAdapter: Send + Sync {
    async fn detect_capabilities(&self) -> PlazaResult<KernelCapabilities>;
    async fn apply_cgroup_limits(&self, cgroup_path: &str, cpu_shares: u32, memory_bytes: u64) -> PlazaResult<()>;
}

pub struct LinuxKernelAdapter;

#[async_trait]
impl KernelAdapter for LinuxKernelAdapter {
    async fn detect_capabilities(&self) -> PlazaResult<KernelCapabilities> {
        Ok(KernelCapabilities {
            cgroups_v2: true,
            namespaces: true,
            seccomp: true,
            landlock: true,
            ebpf: false,
            io_uring: true,
        })
    }

    async fn apply_cgroup_limits(&self, _cgroup_path: &str, _cpu_shares: u32, _memory_bytes: u64) -> PlazaResult<()> {
        Ok(())
    }
}
