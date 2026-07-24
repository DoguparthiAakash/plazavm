//! # plaza-registry
//!
//! Separated runtime images registry and workspace templates management (R2 refinement).

pub mod runtime_images;
pub mod templates;

pub use runtime_images::{RuntimeImage, RuntimeImageRegistry};
pub use templates::{WorkspaceTemplate, WorkspaceTemplateRegistry};
