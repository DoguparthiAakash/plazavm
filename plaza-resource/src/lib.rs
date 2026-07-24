//! # plaza-resource
//!
//! Dedicated resource scheduling, allocation tracking, and policies.

pub mod manager;
pub mod priority;

pub use manager::{ResourceAllocation, ResourceManager, ResourcePlan};
pub use priority::WorkspacePriority;
