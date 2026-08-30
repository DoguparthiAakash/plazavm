//! Build pipeline definition for environment compilation.
//!
//! Defines the sequential steps executed inside the Inferno build workspace:
//!   1. Configure — inject toolchain, set build parameters
//!   2. Build     — run the actual compilation
//!   3. Package   — produce a bootable image from build output
//!
//! Each step is represented as a `BuildStep` that emits a command string
//! to be executed inside the Inferno sandbox via `plaza workspace exec`.

use plaza_foundation::core::PlazaResult;
use plaza_runtime::runtime::GuestRuntimeKind;
use std::fmt;

/// A single step in the environment build pipeline.
#[derive(Debug, Clone)]
pub struct BuildStep {
    /// Human-readable name shown in progress output.
    pub name: String,
    /// The shell command to run inside the Inferno build workspace.
    pub command: String,
    /// Whether failing this step is fatal (aborts the pipeline).
    pub is_fatal: bool,
}

impl fmt::Display for BuildStep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.name, self.command)
    }
}

/// The full ordered sequence of build steps for a given target runtime.
#[derive(Debug)]
pub struct BuildPipeline {
    pub target: GuestRuntimeKind,
    pub steps: Vec<BuildStep>,
}

impl BuildPipeline {
    /// Construct the build pipeline for a given target runtime.
    ///
    /// The pipeline assumes the Inferno build workspace has:
    /// - The source tree mounted at `/src/<target>` (e.g. `/src/linux`, `/src/freebsd`)
    /// - A cross-compilation toolchain (gcc, make, binutils) available in PATH
    /// - `/output/` as the directory where build artifacts are written
    pub fn for_target(target: &GuestRuntimeKind) -> PlazaResult<Self> {
        let steps = match target {
            GuestRuntimeKind::Linux => linux_steps(),
            GuestRuntimeKind::FreeBsd => freebsd_steps(),
            GuestRuntimeKind::OpenBsd => openbsd_steps(),
            GuestRuntimeKind::NetBsd => netbsd_steps(),
            GuestRuntimeKind::DragonFly => dragonfly_steps(),
            GuestRuntimeKind::Inferno => {
                return Err(plaza_foundation::core::PlazaError::config(
                    "Inferno does not use the env build pipeline — it is a native runtime.",
                ))
            }
        };

        Ok(BuildPipeline {
            target: target.clone(),
            steps,
        })
    }

    /// Pretty-print the pipeline steps for `plaza env status`.
    pub fn describe(&self) -> String {
        let mut out = format!("Build pipeline for '{}':\n", self.target.display_name());
        for (i, step) in self.steps.iter().enumerate() {
            out.push_str(&format!("  {:2}. {}\n", i + 1, step));
        }
        out
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Per-target step generators
// ──────────────────────────────────────────────────────────────────────────────

fn linux_steps() -> Vec<BuildStep> {
    vec![
        BuildStep {
            name: "configure".into(),
            command: "cd /src/linux && make ARCH=x86_64 defconfig".into(),
            is_fatal: true,
        },
        BuildStep {
            name: "build-kernel".into(),
            command: "cd /src/linux && make ARCH=x86_64 -j$(nproc) bzImage".into(),
            is_fatal: true,
        },
        BuildStep {
            name: "build-modules".into(),
            command: "cd /src/linux && make ARCH=x86_64 -j$(nproc) modules".into(),
            is_fatal: false,
        },
        BuildStep {
            name: "build-initrd".into(),
            // musl + BusyBox initrd assembled using a minimal buildroot-style script
            command: "sh /plaza-build/scripts/build-initrd.sh".into(),
            is_fatal: true,
        },
        BuildStep {
            name: "package".into(),
            command: "cp /src/linux/arch/x86_64/boot/bzImage /output/vmlinuz && \
                      cp /tmp/initrd.gz /output/initrd.gz".into(),
            is_fatal: true,
        },
    ]
}

fn freebsd_steps() -> Vec<BuildStep> {
    vec![
        BuildStep {
            name: "configure".into(),
            command: "cd /src/freebsd && make -j$(nproc) buildkernel KERNCONF=GENERIC".into(),
            is_fatal: true,
        },
        BuildStep {
            name: "build-world".into(),
            command: "cd /src/freebsd && make -j$(nproc) buildworld".into(),
            is_fatal: true,
        },
        BuildStep {
            name: "install-world".into(),
            command: "cd /src/freebsd && make installworld DESTDIR=/output/rootfs".into(),
            is_fatal: true,
        },
        BuildStep {
            name: "package".into(),
            command: "sh /plaza-build/scripts/package-bsd.sh freebsd".into(),
            is_fatal: true,
        },
    ]
}

fn openbsd_steps() -> Vec<BuildStep> {
    vec![
        BuildStep {
            name: "configure".into(),
            command: "cd /src/openbsd && ./configure".into(),
            is_fatal: true,
        },
        BuildStep {
            name: "build".into(),
            command: "cd /src/openbsd && make -j$(nproc) build".into(),
            is_fatal: true,
        },
        BuildStep {
            name: "package".into(),
            command: "sh /plaza-build/scripts/package-bsd.sh openbsd".into(),
            is_fatal: true,
        },
    ]
}

fn netbsd_steps() -> Vec<BuildStep> {
    vec![
        BuildStep {
            name: "build".into(),
            // NetBSD uses build.sh as the canonical entry point
            command: "cd /src/netbsd && ./build.sh -U -j$(nproc) tools kernel=GENERIC release".into(),
            is_fatal: true,
        },
        BuildStep {
            name: "package".into(),
            command: "sh /plaza-build/scripts/package-bsd.sh netbsd".into(),
            is_fatal: true,
        },
    ]
}

fn dragonfly_steps() -> Vec<BuildStep> {
    vec![
        BuildStep {
            name: "build".into(),
            command: "cd /src/dragonfly && make -j$(nproc) buildworld buildkernel".into(),
            is_fatal: true,
        },
        BuildStep {
            name: "package".into(),
            command: "sh /plaza-build/scripts/package-bsd.sh dragonfly".into(),
            is_fatal: true,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pipeline_for_all_supported_targets() {
        let targets = [
            GuestRuntimeKind::Linux,
            GuestRuntimeKind::FreeBsd,
            GuestRuntimeKind::OpenBsd,
            GuestRuntimeKind::NetBsd,
            GuestRuntimeKind::DragonFly,
        ];
        for target in &targets {
            let pipeline = BuildPipeline::for_target(target).unwrap();
            assert!(!pipeline.steps.is_empty(), "Pipeline for {} has no steps", target);
            // All steps must have a non-empty command.
            for step in &pipeline.steps {
                assert!(!step.command.is_empty(), "Empty command in step '{}'", step.name);
            }
        }
    }

    #[test]
    fn inferno_returns_error() {
        let result = BuildPipeline::for_target(&GuestRuntimeKind::Inferno);
        assert!(result.is_err());
    }
}
