//! Translates `MachineConfig` into QEMU command-line arguments.

use plaza_foundation::core::PlazaResult;
use plaza_runtime::MachineConfig;
use std::ffi::OsString;
use std::path::PathBuf;

/// Builds arguments for the QEMU executable.
pub struct QemuAdapter {
    binary: PathBuf,
    args: Vec<OsString>,
}

impl QemuAdapter {
    /// Create a new adapter using the discovered QEMU binary.
    pub fn new(binary: PathBuf) -> Self {
        let mut args = Vec::new();

        // Security Policy Amendment: Always force TCG (software emulation)
        // No KVM, no HVF, no WHPX allowed in PlazaVM architecture.
        args.push("-accel".into());
        args.push("tcg,thread=multi".into());

        // Default-deny implies no network by default, no unnecessary devices.
        args.push("-nodefaults".into());
        args.push("-display".into());
        args.push("none".into()); // Headless by default

        Self { binary, args }
    }

    /// Convert a PlazaVM `MachineConfig` into QEMU arguments.
    pub fn apply_config(mut self, config: &MachineConfig) -> PlazaResult<Self> {
        // CPU configuration
        self.args.push("-smp".into());
        self.args.push(config.machine.cpu.cores.to_string().into());

        // Memory configuration
        let mem_mb = plaza_foundation::core::types::ByteSize::parse(&config.machine.memory.size)
            .unwrap_or_else(|_| plaza_foundation::core::types::ByteSize::from_mb(256))
            .as_mb();
            
        self.args.push("-m".into());
        self.args.push(format!("{}M", mem_mb).into());

        // Virtual Block Storage boot device
        self.args.push("-drive".into());
        self.args.push(format!("file={},format=raw,if=virtio", config.boot_device.display()).into());

        // Network capability check
        if config.capabilities.network.enabled {
            self.args.push("-netdev".into());
            self.args.push("user,id=net0".into());
            self.args.push("-device".into());
            self.args.push("virtio-net-pci,netdev=net0".into());
        }

        // QMP (QEMU Machine Protocol) socket for lifecycle management
        // The process.rs module will bind this and connect to it.
        // We defer QMP socket arguments to the process spawning logic.

        Ok(self)
    }

    /// Add a raw argument to the adapter.
    pub fn add_arg(&mut self, arg: OsString) {
        self.args.push(arg);
    }

    /// Get the final binary and arguments for `std::process::Command`.
    pub fn into_command(self) -> (PathBuf, Vec<OsString>) {
        (self.binary, self.args)
    }
}
