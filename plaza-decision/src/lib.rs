//! # plaza-decision
//!
//! Runtime Selection Engine, scoring algorithms, and intent resolution.
//!
//! The Decision Layer evaluates workspace requirements, intent, host capabilities,
//! and available plugins to choose the optimal runtime backend and plan resource
//! allocations.

pub mod engine;
pub mod intent;
pub mod scoring;

pub use engine::{DecisionEngine, SelectedBackend, WorkspaceDecision};
pub use intent::IntentResolver;
pub use scoring::ScoredCandidate;
