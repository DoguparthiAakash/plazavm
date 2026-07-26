//! Workspace aggregate root domain model.

use super::graph::WorkspaceGraph;
use plaza_foundation::config::IntentConfig;
use plaza_foundation::core::id::WorkspaceId;
use plaza_foundation::core::security::SecurityPolicy;
use plaza_foundation::core::types::{Architecture, HealthStatus, OperatingSystem, Timestamp};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The central Workspace aggregate root.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    pub id: WorkspaceId,
    pub name: String,
    pub description: Option<String>,
    pub spec: WorkspaceSpec,
    pub status: WorkspaceStatus,
    pub metadata: WorkspaceMetadata,
    pub graph: WorkspaceGraph,
}

impl Workspace {
    /// Create a new Workspace with sensible defaults.
    pub fn new(name: impl Into<String>, spec: WorkspaceSpec) -> Self {
        let name_str = name.into();
        let id = WorkspaceId::new();
        let primary_spec = spec.runtime.clone();
        let resources = spec.resources.clone();
        let graph = WorkspaceGraph::single_node("main", primary_spec, resources);

        Self {
            id: id.clone(),
            name: name_str,
            description: None,
            spec,
            status: WorkspaceStatus::default(),
            metadata: WorkspaceMetadata {
                created_at: Timestamp::now(),
                updated_at: Timestamp::now(),
                tags: Vec::new(),
            },
            graph,
        }
    }
}

/// Workspace specification — declared desired state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceSpec {
    pub desired_state: DesiredState,
    pub runtime: RuntimeSpec,
    pub resources: ResourceSpec,
    pub networking: NetworkSpec,
    pub storage: Vec<VolumeSpec>,
    pub devices: Vec<DeviceSpec>,
    pub environment: HashMap<String, String>,
    pub security: SecurityPolicy,
    pub intent: Option<IntentConfig>,
    pub extensions: Vec<String>,
}

impl Default for WorkspaceSpec {
    fn default() -> Self {
        Self {
            desired_state: DesiredState::Stopped,
            runtime: RuntimeSpec::default(),
            resources: ResourceSpec::default(),
            networking: NetworkSpec::default(),
            storage: Vec::new(),
            devices: Vec::new(),
            environment: HashMap::new(),
            security: SecurityPolicy::default(),
            intent: None,
            extensions: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum DesiredState {
    Running,
    #[default]
    Stopped,
    Paused,
    Destroyed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeSpec {
    pub kind: RuntimeKind,
    pub image: Option<String>,
    pub backend: RuntimeBackendPreference,
    pub os: OperatingSystem,
    pub arch: Architecture,
}

impl Default for RuntimeSpec {
    fn default() -> Self {
        Self {
            kind: RuntimeKind::Container,
            image: Some("ubuntu:24.04".into()),
            backend: RuntimeBackendPreference::Auto,
            os: OperatingSystem::Linux,
            arch: Architecture::X86_64,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum RuntimeKind {
    #[default]
    Container,
    MicroVM,
    VirtualMachine,
    RemoteHost,
    CloudInstance,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeBackendPreference {
    #[default]
    Auto,
    Preferred(String),
    Pinned(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceSpec {
    pub cpu_cores: u32,
    pub cpu_limit: Option<u32>,
    pub memory_mb: u64,
    pub memory_limit_mb: Option<u64>,
    pub gpu_enabled: bool,
    pub priority: String,
}

impl Default for ResourceSpec {
    fn default() -> Self {
        Self {
            cpu_cores: 2,
            cpu_limit: None,
            memory_mb: 2048,
            memory_limit_mb: None,
            gpu_enabled: false,
            priority: "normal".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkSpec {
    pub mode: String,
    pub ports: Vec<PortMapping>,
    pub dns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortMapping {
    pub host_port: u16,
    pub guest_port: u16,
    pub protocol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeSpec {
    pub name: String,
    pub host_path: Option<String>,
    pub mount_path: String,
    pub size_mb: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceSpec {
    pub device_type: String,
    pub vendor_id: Option<String>,
    pub product_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceStatus {
    pub state: WorkspaceState,
    pub runtime_backend: Option<String>,
    pub runtime_instance_id: Option<String>,
    pub health: HealthStatus,
    pub resources: ResourceUsage,
    pub network: NetworkStatus,
    pub last_transition: Timestamp,
    pub message: Option<String>,
    pub uptime_secs: Option<u64>,
}

impl Default for WorkspaceStatus {
    fn default() -> Self {
        Self {
            state: WorkspaceState::Stopped,
            runtime_backend: None,
            runtime_instance_id: None,
            health: HealthStatus::Unknown,
            resources: ResourceUsage::default(),
            network: NetworkStatus::default(),
            last_transition: Timestamp::now(),
            message: None,
            uptime_secs: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceState {
    Pending,
    Scheduling,
    Creating,
    Starting,
    Running,
    Paused,
    Stopping,
    #[default]
    Stopped,
    Error,
    Destroying,
    Destroyed,
}

impl std::fmt::Display for WorkspaceState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::Scheduling => write!(f, "scheduling"),
            Self::Creating => write!(f, "creating"),
            Self::Starting => write!(f, "starting"),
            Self::Running => write!(f, "running"),
            Self::Paused => write!(f, "paused"),
            Self::Stopping => write!(f, "stopping"),
            Self::Stopped => write!(f, "stopped"),
            Self::Error => write!(f, "error"),
            Self::Destroying => write!(f, "destroying"),
            Self::Destroyed => write!(f, "destroyed"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceUsage {
    pub cpu_usage_pct: f64,
    pub memory_used_mb: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkStatus {
    pub ip_address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceMetadata {
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub tags: Vec<String>,
}

