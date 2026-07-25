//! # plaza-registry
//!
//! Separated runtime images registry and workspace templates management (R2 refinement).

pub mod importer;
pub mod runtime_images;
pub mod templates;

pub use importer::{ImportedPriResult, RootFsSource, RuntimeImporter};
pub use runtime_images::{RuntimeImage, RuntimeImageRegistry};
pub use templates::{WorkspaceTemplate, WorkspaceTemplateRegistry};
