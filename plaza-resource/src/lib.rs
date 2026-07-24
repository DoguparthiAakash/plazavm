//! # plaza-resource
//!
//! Dedicated resource scheduling, allocation tracking, and policies.

pub mod manager;
pub mod priority;
pub mod vhal;

pub use manager::{ResourceAllocation, ResourceManager, ResourcePlan};
pub use priority::WorkspacePriority;
pub use vhal::{HardwareProfileKind, VirtualHardwareProfile};
