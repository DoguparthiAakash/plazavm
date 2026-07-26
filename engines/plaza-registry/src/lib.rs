//! # plaza-registry
//!
//! Separated runtime images registry and workspace templates management (R2 refinement).

pub mod importer;
pub mod pro_image;
pub mod pur_image;
pub mod runtime_images;
pub mod templates;
pub mod auth;
pub mod cache;
pub mod mirror;
pub mod offline;
pub use importer::{ImportedPriResult, RootFsSource, RuntimeImporter};
pub use pro_image::{ProImageLayer, ProImageManager, ProImageManifest, ProImageSignature};
pub use pur_image::{PurImageLayer, PurImageManager, PurImageManifest, PurImageSignature};
pub use runtime_images::{RuntimeImage, RuntimeImageRegistry};
pub use templates::{WorkspaceTemplate, WorkspaceTemplateRegistry};

