//! Guest runtime implementations.
//!
//! Each module provides a concrete [`plaza_runtime::GuestRuntime`] implementation
//! for a specific guest operating system.

pub mod capabilities;
pub mod inferno;
pub mod linux;

pub use capabilities::{CapabilityProfile, CapabilityState, LinuxCompatibilityLayer};
pub use inferno::InfernoGuestRuntime;
pub use linux::LinuxGuestRuntime;

use plaza_runtime::GuestRuntimeKind;

/// Create a GuestRuntime instance for the specified kind.
pub fn create_guest_runtime(kind: GuestRuntimeKind) -> Box<dyn plaza_runtime::GuestRuntime> {
    match kind {
        GuestRuntimeKind::Linux => Box::new(LinuxGuestRuntime::new()),
        GuestRuntimeKind::Inferno => Box::new(InfernoGuestRuntime::new()),
    }
}
