//! Runtime detection module.
//!
//! Re-exports detection logic from [`PlatformDetector`](super::detector::PlatformDetector).
//! This module will grow in Phase 2 with deeper version parsing, health
//! scoring, and dependency checking.

pub use super::capabilities::{InstalledRuntime, RuntimeHealthReport};

