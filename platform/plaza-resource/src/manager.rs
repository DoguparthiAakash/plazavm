//! ResourceManager implementation for tracking and granting resource allocations.

use super::priority::WorkspacePriority;
use plaza_foundation::core::id::WorkspaceId;
use plaza_foundation::core::types::Timestamp;
use plaza_foundation::core::{PlazaError, PlazaResult};
use plaza_foundation::events::{EventBus, PlazaEvent};
use plaza_foundation::platform::PlatformDetector;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

/// Dynamic plan requested for a workspace before allocation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourcePlan {
    pub workspace_id: WorkspaceId,
    pub cpu_cores: u32,
    pub memory_mb: u64,
    pub gpu_enabled: bool,
    pub priority: WorkspacePriority,
}

/// An active resource allocation granted to a running workspace.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceAllocation {
    pub workspace_id: WorkspaceId,
    pub cpu_cores: u32,
    pub memory_mb: u64,
    pub gpu_enabled: bool,
    pub allocated_at: Timestamp,
}

/// Central resource manager for PlazaVM.
pub struct ResourceManager {
    platform: Arc<PlatformDetector>,
    event_bus: Arc<EventBus>,
    allocations: RwLock<HashMap<WorkspaceId, ResourceAllocation>>,
}

impl ResourceManager {
    pub fn new(platform: Arc<PlatformDetector>, event_bus: Arc<EventBus>) -> Self {
        Self {
            platform,
            event_bus,
            allocations: RwLock::new(HashMap::new()),
        }
    }

    /// Allocate resources for a workspace.
    pub async fn allocate(&self, plan: &ResourcePlan) -> PlazaResult<ResourceAllocation> {
        let caps = self.platform.capabilities().await?;
        let current_allocs = self.allocations.read().await;

        let total_allocated_mem: u64 = current_allocs.values().map(|a| a.memory_mb).sum();
        let _total_allocated_cpu: u32 = current_allocs.values().map(|a| a.cpu_cores).sum();

        // Check if allocation would exceed host limits
        if total_allocated_mem + plan.memory_mb > caps.memory.total_mb {
            return Err(PlazaError::ResourceExhausted {
                resource: format!(
                    "Memory limit exceeded: requested {}Mi, available {}Mi",
                    plan.memory_mb,
                    caps.memory.total_mb.saturating_sub(total_allocated_mem)
                ),
            });
        }

        drop(current_allocs);

        let allocation = ResourceAllocation {
            workspace_id: plan.workspace_id.clone(),
            cpu_cores: plan.cpu_cores,
            memory_mb: plan.memory_mb,
            gpu_enabled: plan.gpu_enabled,
            allocated_at: Timestamp::now(),
        };

        self.allocations
            .write()
            .await
            .insert(plan.workspace_id.clone(), allocation.clone());

        self.event_bus
            .publish(PlazaEvent::ResourceAllocated {
                workspace_id: plan.workspace_id.clone(),
                cpu_cores: plan.cpu_cores,
                memory_mb: plan.memory_mb,
            })
            .await;

        info!(
            workspace_id = %plan.workspace_id,
            cpu = plan.cpu_cores,
            mem = plan.memory_mb,
            "resource allocation granted"
        );

        Ok(allocation)
    }

    /// Release resources held by a workspace.
    pub async fn release(&self, workspace_id: &WorkspaceId) -> PlazaResult<()> {
        if self
            .allocations
            .write()
            .await
            .remove(workspace_id)
            .is_some()
        {
            self.event_bus
                .publish(PlazaEvent::ResourceReleased {
                    workspace_id: workspace_id.clone(),
                })
                .await;
            info!(workspace_id = %workspace_id, "resources released");
        }
        Ok(())
    }

    /// Get all active allocations.
    pub async fn active_allocations(&self) -> Vec<ResourceAllocation> {
        self.allocations.read().await.values().cloned().collect()
    }
}

