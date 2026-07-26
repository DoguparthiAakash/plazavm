//! Domain events emitted throughout the PlazaVM platform.

use plaza_core::id::{PluginId, WorkspaceId};
use plaza_core::types::HealthStatus;
use serde::{Deserialize, Serialize};

/// All domain events in the PlazaVM system.
///
/// Events are immutable records of something that happened.
/// They flow through the [`EventBus`](super::EventBus) and drive
/// the controller's reconciliation loop.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum PlazaEvent {
    // ── Workspace lifecycle ─────────────────────────────────────────────────
    WorkspaceCreated {
        id: WorkspaceId,
        name: String,
    },
    WorkspaceOpened {
        id: WorkspaceId,
        path: String,
    },
    ExecutionPlanReady {
        workspace_id: WorkspaceId,
        target: String,
        driver_id: String,
    },
    ImageBuilt {
        workspace_id: WorkspaceId,
        image_id: String,
        layer_count: usize,
    },
    PackageInstalled {
        workspace_id: WorkspaceId,
        package_name: String,
    },
    WorkspaceDesiredStateChanged {
        id: WorkspaceId,
        desired: String,
    },
    WorkspaceStarting {
        id: WorkspaceId,
    },
    WorkspaceStarted {
        id: WorkspaceId,
        runtime_backend: String,
    },
    WorkspaceStopping {
        id: WorkspaceId,
    },
    WorkspaceStopped {
        id: WorkspaceId,
    },
    WorkspacePaused {
        id: WorkspaceId,
    },
    WorkspaceResumed {
        id: WorkspaceId,
    },
    WorkspaceError {
        id: WorkspaceId,
        error: String,
    },
    WorkspaceHealthChanged {
        id: WorkspaceId,
        health: HealthStatus,
    },
    WorkspaceDeleted {
        id: WorkspaceId,
    },

    // ── Decision events ─────────────────────────────────────────────────────
    RuntimeSelected {
        workspace_id: WorkspaceId,
        backend: String,
        reason: String,
    },
    ResourceAllocated {
        workspace_id: WorkspaceId,
        cpu_cores: u32,
        memory_mb: u64,
    },
    ResourceReleased {
        workspace_id: WorkspaceId,
    },

    // ── Controller events ───────────────────────────────────────────────────
    ReconciliationStarted {
        workspace_id: WorkspaceId,
    },
    ReconciliationCompleted {
        workspace_id: WorkspaceId,
        action: String,
    },
    ReconciliationFailed {
        workspace_id: WorkspaceId,
        error: String,
    },
    AutoSuspended {
        workspace_id: WorkspaceId,
        reason: String,
    },
    AutoResumed {
        workspace_id: WorkspaceId,
    },

    // ── Platform events ─────────────────────────────────────────────────────
    PlatformScanned {
        profile: String,
    },
    RuntimeDiscovered {
        runtime_id: String,
        version: String,
    },
    RuntimeUnavailable {
        runtime_id: String,
        reason: String,
    },

    // ── Plugin events ───────────────────────────────────────────────────────
    PluginLoaded {
        id: PluginId,
        name: String,
    },
    PluginUnloaded {
        id: PluginId,
    },
    PluginError {
        id: PluginId,
        error: String,
    },

    // ── AI events ───────────────────────────────────────────────────────────
    AiRecommendation {
        workspace_id: WorkspaceId,
        category: String,
        recommendation: String,
    },

    // ── Command events ──────────────────────────────────────────────────────
    CommandReceived {
        command: String,
    },
    CommandExecutionStarted {
        command: String,
        target: String,
    },
    CommandExecutionCompleted {
        command: String,
        target: String,
        duration_ms: u64,
    },
    CommandExecutionFailed {
        command: String,
        target: String,
        error: String,
    },

    // ── Engine events ───────────────────────────────────────────────────────
    EngineStarting {
        name: String,
    },
    EngineStarted {
        name: String,
    },
    EngineStopping {
        name: String,
    },
    EngineStopped {
        name: String,
    },

    // ── System events ───────────────────────────────────────────────────────
    SystemMetricsUpdated {
        cpu_usage_pct: f64,
        memory_usage_pct: f64,
        disk_usage_pct: f64,
    },
}

impl PlazaEvent {
    /// Returns the event type name for logging and filtering.
    pub fn event_type(&self) -> &'static str {
        match self {
            Self::WorkspaceCreated { .. } => "workspace.created",
            Self::WorkspaceOpened { .. } => "workspace.opened",
            Self::ExecutionPlanReady { .. } => "workspace.execution_plan_ready",
            Self::ImageBuilt { .. } => "workspace.image_built",
            Self::PackageInstalled { .. } => "workspace.package_installed",
            Self::WorkspaceDesiredStateChanged { .. } => "workspace.desired_state_changed",
            Self::WorkspaceStarting { .. } => "workspace.starting",
            Self::WorkspaceStarted { .. } => "workspace.started",
            Self::WorkspaceStopping { .. } => "workspace.stopping",
            Self::WorkspaceStopped { .. } => "workspace.stopped",
            Self::WorkspacePaused { .. } => "workspace.paused",
            Self::WorkspaceResumed { .. } => "workspace.resumed",
            Self::WorkspaceError { .. } => "workspace.error",
            Self::WorkspaceHealthChanged { .. } => "workspace.health_changed",
            Self::WorkspaceDeleted { .. } => "workspace.deleted",
            Self::RuntimeSelected { .. } => "decision.runtime_selected",
            Self::ResourceAllocated { .. } => "decision.resource_allocated",
            Self::ResourceReleased { .. } => "decision.resource_released",
            Self::ReconciliationStarted { .. } => "controller.reconciliation_started",
            Self::ReconciliationCompleted { .. } => "controller.reconciliation_completed",
            Self::ReconciliationFailed { .. } => "controller.reconciliation_failed",
            Self::AutoSuspended { .. } => "controller.auto_suspended",
            Self::AutoResumed { .. } => "controller.auto_resumed",
            Self::PlatformScanned { .. } => "platform.scanned",
            Self::RuntimeDiscovered { .. } => "platform.runtime_discovered",
            Self::RuntimeUnavailable { .. } => "platform.runtime_unavailable",
            Self::PluginLoaded { .. } => "plugin.loaded",
            Self::PluginUnloaded { .. } => "plugin.unloaded",
            Self::PluginError { .. } => "plugin.error",
            Self::AiRecommendation { .. } => "ai.recommendation",
            Self::CommandReceived { .. } => "command.received",
            Self::CommandExecutionStarted { .. } => "command.started",
            Self::CommandExecutionCompleted { .. } => "command.completed",
            Self::CommandExecutionFailed { .. } => "command.failed",
            Self::EngineStarting { .. } => "engine.starting",
            Self::EngineStarted { .. } => "engine.started",
            Self::EngineStopping { .. } => "engine.stopping",
            Self::EngineStopped { .. } => "engine.stopped",
            Self::SystemMetricsUpdated { .. } => "system.metrics_updated",
        }
    }
}
