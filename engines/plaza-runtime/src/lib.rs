//! # plaza-runtime
//!
//! Runtime abstraction layer for PlazaVM.
//!
//! This crate defines the **traits** that every execution backend must
//! implement. It contains **zero** implementation code — all real work
//! lives in plugin crates (`plugins/docker`, `plugins/qemu`, etc.).
//!
//! The core system interacts with runtimes exclusively through these traits,
//! making it possible to add new backends without modifying core code.

mod backend;
pub mod backends;
mod capabilities;
mod instance;
mod machine;
mod manager;
pub mod resolver;
pub mod runtime;
pub mod storage;

pub use backend::RuntimeBackend;
pub use capabilities::RuntimeCapabilities;
pub use instance::{ConsoleStream, RuntimeInstance, RuntimeMetrics, RuntimeStatus, SnapshotInfo};
pub use machine::{MachineConfig, OperatingSystemTarget};
pub use manager::RuntimeManager;
pub use runtime::{GuestRuntime, GuestRuntimeKind, GuestRuntimeParams, ResolvedArtifacts};
pub use storage::RuntimeStorage;
