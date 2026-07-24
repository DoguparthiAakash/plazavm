//! # plaza-workspace
//!
//! Workspace aggregate root, workspace graph model, and domain service.

pub mod graph;
pub mod model;
pub mod service;

pub use graph::{NodeConnection, NodeRole, RuntimeNode, WorkspaceGraph};
pub use model::{
    DesiredState, DeviceSpec, NetworkSpec, NetworkStatus, ResourceSpec, ResourceUsage,
    RuntimeBackendPreference, RuntimeKind, RuntimeSpec, VolumeSpec, Workspace, WorkspaceMetadata,
    WorkspaceSpec, WorkspaceState, WorkspaceStatus,
};
pub use service::WorkspaceService;
