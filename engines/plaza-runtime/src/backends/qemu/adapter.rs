//! Translates `MachineConfig` into QEMU command-line arguments.

use crate::MachineConfig;
use plaza_foundation::core::PlazaResult;
use std::ffi::OsString;
use std::path::PathBuf;

/// Builds arguments for the QEMU executable.
pub struct QemuAdapter {
    binary: PathBuf,
    args: Vec<OsString>,
}

impl QemuAdapter {
    pub fn new(binary: PathBuf) -> Self {
        let args = vec![
            "-accel".into(),
            "tcg,thread=multi".into(),
            "-nodefaults".into(),
            "-display".into(),
            "none".into(),
        ];

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

        // Virtual Block Storage boot device configuration is deferred to the plugin
        // which dynamically maps it to the NBD Unix socket or TCP port.
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

        if let Some(kernel) = &config.kernel_path {
            self.args.push("-kernel".into());
            self.args.push(kernel.into());
        }
        if let Some(initrd) = &config.initrd_path {
            self.args.push("-initrd".into());
            self.args.push(initrd.into());
        }
        if let Some(args) = &config.kernel_args {
            self.args.push("-append".into());
            self.args.push(args.into());
        }
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
