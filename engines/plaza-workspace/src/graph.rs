//! Workspace Graph domain model (R1: future-proofing multi-node workspaces).

use crate::model::{ResourceSpec, RuntimeSpec};
use serde::{Deserialize, Serialize};

/// Graph representation of a Workspace.
///
/// In Phase 1, every Workspace contains a single primary node.
/// Future phases will support multi-node graphs (e.g. App + Postgres + Redis).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceGraph {
    pub nodes: Vec<RuntimeNode>,
    #[serde(default)]
    pub edges: Vec<NodeConnection>,
}

impl WorkspaceGraph {
    /// Create a single-node graph (default for Phase 1).
    pub fn single_node(
        node_id: impl Into<String>,
        runtime_spec: RuntimeSpec,
        resources: ResourceSpec,
    ) -> Self {
        Self {
            nodes: vec![RuntimeNode {
                id: node_id.into(),
                role: NodeRole::Primary,
                runtime_spec,
                resources,
                status: None,
            }],
            edges: vec![],
        }
    }

    /// Retrieve the primary node.
    pub fn primary_node(&self) -> Option<&RuntimeNode> {
        self.nodes.iter().find(|n| n.role == NodeRole::Primary)
    }
}

/// A node within a Workspace graph representing an execution container/VM.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeNode {
    pub id: String,
    pub role: NodeRole,
    pub runtime_spec: RuntimeSpec,
    pub resources: ResourceSpec,
    pub status: Option<String>,
}

/// Node role within a Workspace graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NodeRole {
    /// Primary entrypoint / development workload.
    Primary,
    /// Supporting sidecar container (e.g., proxy, metrics agent).
    Sidecar,
    /// Background service node (e.g., database, cache).
    Service,
}

/// Connection between two nodes in a Workspace graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConnection {
    pub source_node_id: String,
    pub target_node_id: String,
    pub connection_type: String, // "network", "volume", "link"
}

