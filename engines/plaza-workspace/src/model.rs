//! Workspace aggregate root domain model.

use super::graph::WorkspaceGraph;
use plaza_foundation::config::IntentConfig;
use plaza_foundation::core::id::WorkspaceId;
use plaza_foundation::core::capability_policy::CapabilityPolicy;
use plaza_foundation::core::types::{Architecture, HealthStatus, OperatingSystem, Timestamp};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
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
                tags: vec![],
                project_path: None,
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
    pub capabilities: CapabilityPolicy,
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
            capabilities: CapabilityPolicy::default(),
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
    pub execution_mode: Option<String>,
    pub storage_backend: Option<String>,
    pub pid: Option<u32>,
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
            execution_mode: None,
            storage_backend: None,
            pid: None,
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
    #[serde(default)]
    pub project_path: Option<String>,
}

/// The independent environment configuration specifying the workspace image to build.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceImageSpec {
    pub engine_distribution: String,
    pub engine_version: Option<String>,
    pub base_image: Option<String>,
    pub packages: Vec<String>,
    pub tools: Vec<String>,
    pub project_language: Option<String>,
    pub project_runtime_version: Option<String>,
    pub project_dependencies: Vec<String>,
    pub environment: HashMap<String, String>,
}

impl WorkspaceImageSpec {
    pub fn from_yaml(yaml: &plaza_foundation::config::PlazaYaml) -> Self {
        let engine = yaml.engine.as_ref();
        let project = yaml.project.as_ref();
        let image = yaml.image.as_ref();

        Self {
            engine_distribution: engine.map(|e| e.distribution.clone()).unwrap_or_else(|| "alpine".to_string()),
            engine_version: engine.and_then(|e| e.version.clone()),
            base_image: image.and_then(|i| i.base.clone()),
            packages: image.map(|i| i.packages.clone()).unwrap_or_default(),
            tools: image.map(|i| i.tools.clone()).unwrap_or_default(),
            project_language: project.map(|p| p.language.clone()),
            project_runtime_version: project.and_then(|p| p.runtime.as_ref().map(|r| r.version.clone())),
            project_dependencies: project.map(|p| p.dependencies.clone()).unwrap_or_default(),
            environment: yaml.environment.clone(),
        }
    }

    /// Compute the deterministic Content ID for this workspace image specification.
    pub fn compute_identity(&self) -> String {
        // We use JSON canonicalization implicitly by serializing an ordered version of the struct.
        // BTreeMap would guarantee key ordering for the environment map if we needed it, but
        // for now we sort the lists to ensure determinism regardless of YAML order.
        
        let mut packages = self.packages.clone();
        packages.sort();
        
        let mut tools = self.tools.clone();
        tools.sort();
        
        let mut project_deps = self.project_dependencies.clone();
        project_deps.sort();

        // Convert HashMap to a sorted Vec of pairs to ensure deterministic JSON serialization
        let mut env: Vec<(&String, &String)> = self.environment.iter().collect();
        env.sort_by(|a, b| a.0.cmp(b.0));

        // Create a canonical representation
        #[derive(Serialize)]
        struct CanonicalSpec<'a> {
            engine_distribution: &'a str,
            engine_version: Option<&'a str>,
            base_image: Option<&'a str>,
            packages: Vec<String>,
            tools: Vec<String>,
            project_language: Option<&'a str>,
            project_runtime_version: Option<&'a str>,
            project_dependencies: Vec<String>,
            environment: Vec<(&'a String, &'a String)>,
        }

        let canonical = CanonicalSpec {
            engine_distribution: &self.engine_distribution,
            engine_version: self.engine_version.as_deref(),
            base_image: self.base_image.as_deref(),
            packages,
            tools,
            project_language: self.project_language.as_deref(),
            project_runtime_version: self.project_runtime_version.as_deref(),
            project_dependencies: project_deps,
            environment: env,
        };

        let serialized = serde_json::to_string(&canonical).expect("Failed to serialize WorkspaceImageSpec");
        let mut hasher = Sha256::new();
        hasher.update(serialized.as_bytes());
        let result = hasher.finalize();
        result.iter().map(|b| format!("{:02x}", b)).collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn image_identity_deterministic() {
        let spec1 = WorkspaceImageSpec {
            engine_distribution: "alpine".into(),
            engine_version: Some("stable".into()),
            base_image: None,
            packages: vec!["git".into(), "curl".into()],
            tools: vec![],
            project_language: None,
            project_runtime_version: None,
            project_dependencies: vec![],
            environment: HashMap::new(),
        };

        let spec2 = WorkspaceImageSpec {
            engine_distribution: "alpine".into(),
            engine_version: Some("stable".into()),
            base_image: None,
            packages: vec!["curl".into(), "git".into()], // Reordered
            tools: vec![],
            project_language: None,
            project_runtime_version: None,
            project_dependencies: vec![],
            environment: HashMap::new(),
        };

        assert_eq!(spec1.compute_identity(), spec2.compute_identity());
    }

    #[test]
    fn image_identity_changes() {
        let spec1 = WorkspaceImageSpec {
            engine_distribution: "alpine".into(),
            engine_version: Some("stable".into()),
            base_image: None,
            packages: vec!["git".into()],
            tools: vec![],
            project_language: None,
            project_runtime_version: None,
            project_dependencies: vec![],
            environment: HashMap::new(),
        };

        let mut spec2 = spec1.clone();
        spec2.engine_distribution = "fedora".into();

        assert_ne!(spec1.compute_identity(), spec2.compute_identity());
    }
}
