//! `plaza-env` — Environment build pipeline for PlazaVM.
//!
//! Compiles open-source OS environments (Linux, FreeBSD, OpenBSD, NetBSD, DragonFlyBSD)
//! from vendored source trees inside ephemeral Inferno sandbox workspaces.
//!
//! # Design Principle
//!
//! The host system never runs any compiler or build tool directly.
//! Every `make`, `gcc`, and `ld` invocation happens inside an Inferno workspace.
//! This mirrors Rust's ownership model: the toolchain is owned and scoped
//! inside the sandbox, not leaked to the host.
//!
//! # Source Trees
//!
//! | Runtime    | Default Path               | Configurable? |
//! |------------|----------------------------|---------------|
//! | `linux`    | `vendors/linux/`           | Yes           |
//! | `freebsd`  | `inferno-os/FreeBSD/`      | Yes           |
//! | `openbsd`  | `inferno-os/OpenBSD/`      | Yes           |
//! | `netbsd`   | `inferno-os/NetBSD/`       | Yes           |
//! | `dragonfly`| `inferno-os/DragonFly/`    | Yes           |
//!
//! Set `env.source_path` in `plaza.yaml` to override the default for any target.
//!
//! # Reference Platforms (Manual)
//!
//! Windows-compatible environments are built via ReactOS — see `docs/manual/platforms/windows.md`.
//! macOS-compatible environments are built via Darling — see `docs/manual/platforms/macos.md`.

pub mod builder;
pub mod pipeline;
pub mod source;

pub use builder::{EnvBuildConfig, EnvBuildResult, EnvBuilder};
pub use pipeline::BuildPipeline;
pub use source::SourceTree;
