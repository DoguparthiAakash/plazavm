//! # plaza-workspace
//!
//! Workspace aggregate root, workspace graph model, builder, and domain services.

pub mod builder;
pub mod capability;
pub mod graph;
pub mod memory;
pub mod model;
pub mod pipeline;
pub mod process;
pub mod service;
pub mod service_manager;
pub mod session;
pub mod wsc;

pub use builder::WorkspaceBuilder;
pub use capability::{CapabilityDatabase, CapabilityDescriptor, CapabilityResolver};
pub use graph::{NodeConnection, NodeRole, RuntimeNode, WorkspaceGraph};
pub use memory::{WorkspaceMemory, WorkspaceMemoryManager};
pub use model::{
    DesiredState, DeviceSpec, NetworkSpec, NetworkStatus, ResourceSpec, ResourceUsage,
    RuntimeBackendPreference, RuntimeKind, RuntimeSpec, VolumeSpec, Workspace, WorkspaceMetadata,
    WorkspaceSpec, WorkspaceState, WorkspaceStatus,
};
pub use pipeline::{BuilderStage, TransactionalPipelineBuilder};
pub use process::{ProcessSpec, ProcessState, WorkspaceProcessManager};
pub use service::WorkspaceService;
pub use service_manager::{ServiceSpec, ServiceStatus, WorkspaceServiceManager};
pub use session::{SessionManager, SessionStatus, StructuredCommandEntry, WorkspaceSession};
pub use wsc::{WorkspaceCommit, WorkspaceTimeline, WscEngine};
