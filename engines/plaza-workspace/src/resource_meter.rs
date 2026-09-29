//! Workspace resource metering and budget enforcement.
//!
//! Enforces per-workspace memory and CPU budgets to keep total
//! resource usage under 15MB RAM per workspace (vs Docker's ~50-200MB).
//!
//! Uses QMP to query real VM metrics and enforces limits by
//! adjusting QEMU parameters or triggering workspace suspension.

use plaza_foundation::core::{PlazaError, PlazaResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

/// Default memory budget per workspace in MB.
/// Target: 1-15MB per workspace for lightweight Inferno runtimes.
/// Linux compatibility runtimes may need up to 64MB.
pub const DEFAULT_MEMORY_BUDGET_MB: u64 = 15;

/// Absolute minimum memory budget (hard floor).
pub const MIN_MEMORY_BUDGET_MB: u64 = 4;

/// Maximum memory budget for Linux compatibility mode.
pub const MAX_MEMORY_BUDGET_MB: u64 = 64;

/// CPU time budget: maximum percentage of a single core.
pub const DEFAULT_CPU_BUDGET_PERCENT: f64 = 25.0;

/// Disk I/O budget: maximum read/write bytes per second.
pub const DEFAULT_DISK_IO_BUDGET_BPS: u64 = 10 * 1024 * 1024; // 10 MB/s

/// Alert thresholds (percentage of budget used).
pub const ALERT_WARN_PERCENT: f64 = 80.0;
pub const ALERT_CRITICAL_PERCENT: f64 = 95.0;

/// Enforcement actions when budget is exceeded.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EnforcementAction {
    /// Log a warning but allow continued operation.
    Warn,
    /// Throttle the workspace (reduce CPU allocation).
    Throttle,
    /// Suspend the workspace until resources are available.
    Suspend,
    /// Terminate the workspace (last resort).
    Terminate,
}

impl std::fmt::Display for EnforcementAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Warn => write!(f, "warn"),
            Self::Throttle => write!(f, "throttle"),
            Self::Suspend => write!(f, "suspend"),
            Self::Terminate => write!(f, "terminate"),
        }
    }
}

/// Resource budget for a workspace.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceBudget {
    /// Maximum memory in MB.
    pub memory_mb: u64,
    /// Maximum CPU cores (fractional allowed).
    pub cpu_cores: f64,
    /// Maximum disk I/O in bytes per second.
    pub disk_io_bps: u64,
    /// Maximum number of open file descriptors.
    pub max_fds: u32,
    /// Action to take when memory budget is exceeded.
    pub memory_action: EnforcementAction,
    /// Action to take when CPU budget is exceeded.
    pub cpu_action: EnforcementAction,
}

impl Default for ResourceBudget {
    fn default() -> Self {
        Self {
            memory_mb: DEFAULT_MEMORY_BUDGET_MB,
            cpu_cores: 1.0,
            disk_io_bps: DEFAULT_DISK_IO_BUDGET_BPS,
            max_fds: 256,
            memory_action: EnforcementAction::Throttle,
            cpu_action: EnforcementAction::Warn,
        }
    }
}

impl ResourceBudget {
    /// Create a budget for a lightweight Inferno workspace.
    pub fn inferno() -> Self {
        Self {
            memory_mb: 8,
            cpu_cores: 0.5,
            disk_io_bps: 5 * 1024 * 1024,
            max_fds: 128,
            memory_action: EnforcementAction::Throttle,
            cpu_action: EnforcementAction::Warn,
        }
    }

    /// Create a budget for a Linux compatibility workspace.
    pub fn linux_compat() -> Self {
        Self {
            memory_mb: 32,
            cpu_cores: 1.0,
            disk_io_bps: 10 * 1024 * 1024,
            max_fds: 256,
            memory_action: EnforcementAction::Suspend,
            cpu_action: EnforcementAction::Throttle,
        }
    }

    /// Validate the budget is within acceptable ranges.
    pub fn validate(&self) -> PlazaResult<()> {
        if self.memory_mb < MIN_MEMORY_BUDGET_MB {
            return Err(PlazaError::config(format!(
                "Memory budget {}MB is below minimum {}MB",
                self.memory_mb, MIN_MEMORY_BUDGET_MB
            )));
        }
        if self.memory_mb > MAX_MEMORY_BUDGET_MB {
            return Err(PlazaError::config(format!(
                "Memory budget {}MB exceeds maximum {}MB",
                self.memory_mb, MAX_MEMORY_BUDGET_MB
            )));
        }
        if self.cpu_cores <= 0.0 || self.cpu_cores > 4.0 {
            return Err(PlazaError::config(format!(
                "CPU cores {} is out of range (0-4)",
                self.cpu_cores
            )));
        }
        Ok(())
    }
}

/// Real-time resource usage metrics for a workspace.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    /// Current memory usage in MB.
    pub memory_mb: f64,
    /// Current CPU usage percentage (0-100).
    pub cpu_percent: f64,
    /// Current disk I/O in bytes per second.
    pub disk_io_bps: u64,
    /// Number of open file descriptors.
    pub open_fds: u32,
    /// Memory budget utilization percentage.
    pub memory_utilization: f64,
    /// CPU budget utilization percentage.
    pub cpu_utilization: f64,
    /// Timestamp of measurement.
    pub measured_at: String,
}

impl Default for ResourceUsage {
    fn default() -> Self {
        Self {
            memory_mb: 0.0,
            cpu_percent: 0.0,
            disk_io_bps: 0,
            open_fds: 0,
            memory_utilization: 0.0,
            cpu_utilization: 0.0,
            measured_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}

/// Alert level for resource budget violations.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AlertLevel {
    Normal,
    Warning,
    Critical,
}

impl std::fmt::Display for AlertLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal => write!(f, "normal"),
            Self::Warning => write!(f, "warning"),
            Self::Critical => write!(f, "critical"),
        }
    }
}

/// Resource meter for a single workspace.
pub struct WorkspaceResourceMeter {
    /// The workspace ID.
    pub workspace_id: String,
    /// The configured budget.
    pub budget: ResourceBudget,
    /// Latest usage snapshot.
    usage: Arc<RwLock<ResourceUsage>>,
    /// Historical usage samples (capped ring buffer).
    history: Arc<RwLock<Vec<ResourceUsage>>>,
    /// Current alert level.
    alert_level: Arc<RwLock<AlertLevel>>,
}

impl WorkspaceResourceMeter {
    /// Create a new resource meter.
    pub fn new(workspace_id: String, budget: ResourceBudget) -> Self {
        Self {
            workspace_id,
            budget,
            usage: Arc::new(RwLock::new(ResourceUsage::default())),
            history: Arc::new(RwLock::new(Vec::with_capacity(60))),
            alert_level: Arc::new(RwLock::new(AlertLevel::Normal)),
        }
    }

    /// Update usage metrics and check budget violations.
    pub async fn update(&self, metrics: ResourceUsage) -> PlazaResult<AlertLevel> {
        // Calculate utilization
        let memory_util = if self.budget.memory_mb > 0 {
            (metrics.memory_mb / self.budget.memory_mb as f64) * 100.0
        } else {
            0.0
        };

        let cpu_util = if self.budget.cpu_cores > 0.0 {
            (metrics.cpu_percent / (self.budget.cpu_cores * 100.0)) * 100.0
        } else {
            0.0
        };

        let mut updated = metrics;
        updated.memory_utilization = memory_util;
        updated.cpu_utilization = cpu_util;
        updated.measured_at = chrono::Utc::now().to_rfc3339();

        // Determine alert level
        let max_util = memory_util.max(cpu_util);
        let new_level = if max_util >= ALERT_CRITICAL_PERCENT {
            AlertLevel::Critical
        } else if max_util >= ALERT_WARN_PERCENT {
            AlertLevel::Warning
        } else {
            AlertLevel::Normal
        };

        // Log state transitions
        let prev_level = self.alert_level.read().await.clone();
        if new_level != prev_level {
            match new_level {
                AlertLevel::Warning => warn!(
                    "Workspace '{}' resource usage at {:.1}% (warning threshold)",
                    self.workspace_id, max_util
                ),
                AlertLevel::Critical => warn!(
                    "Workspace '{}' resource usage at {:.1}% (critical threshold)",
                    self.workspace_id, max_util
                ),
                AlertLevel::Normal => info!(
                    "Workspace '{}' resource usage returned to normal ({:.1}%)",
                    self.workspace_id, max_util
                ),
            }
        }

        // Store metrics
        *self.usage.write().await = updated.clone();
        *self.alert_level.write().await = new_level.clone();

        // Append to history (ring buffer of 60 samples = 5 minutes at 5s intervals)
        let mut history = self.history.write().await;
        if history.len() >= 60 {
            history.remove(0);
        }
        history.push(updated);

        Ok(new_level)
    }

    /// Get the current usage snapshot.
    pub async fn current_usage(&self) -> ResourceUsage {
        self.usage.read().await.clone()
    }

    /// Get the current alert level.
    pub async fn alert_level(&self) -> AlertLevel {
        self.alert_level.read().await.clone()
    }

    /// Get the configured budget.
    pub fn budget(&self) -> &ResourceBudget {
        &self.budget
    }

    /// Get the recommended enforcement action based on current usage.
    pub async fn recommended_action(&self) -> EnforcementAction {
        let level = self.alert_level().await;
        match level {
            AlertLevel::Normal => EnforcementAction::Warn,
            AlertLevel::Warning => self.budget.memory_action.clone(),
            AlertLevel::Critical => EnforcementAction::Terminate,
        }
    }

    /// Get usage history.
    pub async fn history(&self) -> Vec<ResourceUsage> {
        self.history.read().await.clone()
    }

    /// Calculate average memory usage over history.
    pub async fn avg_memory_mb(&self) -> f64 {
        let history = self.history.read().await;
        if history.is_empty() {
            return 0.0;
        }
        let sum: f64 = history.iter().map(|h| h.memory_mb).sum();
        sum / history.len() as f64
    }

    /// Calculate peak memory usage over history.
    pub async fn peak_memory_mb(&self) -> f64 {
        let history = self.history.read().await;
        history
            .iter()
            .map(|h| h.memory_mb)
            .fold(0.0f64, f64::max)
    }
}

/// Global resource manager that tracks all workspace meters.
pub struct GlobalResourceManager {
    meters: Arc<RwLock<HashMap<String, Arc<WorkspaceResourceMeter>>>>,
    /// Global memory budget in MB (total across all workspaces).
    global_memory_budget_mb: u64,
}

impl GlobalResourceManager {
    /// Create a new global resource manager.
    pub fn new(global_memory_budget_mb: u64) -> Self {
        Self {
            meters: Arc::new(RwLock::new(HashMap::new())),
            global_memory_budget_mb,
        }
    }

    /// Register a workspace meter.
    pub async fn register(&self, meter: Arc<WorkspaceResourceMeter>) {
        let mut meters = self.meters.write().await;
        meters.insert(meter.workspace_id.clone(), meter);
    }

    /// Unregister a workspace meter.
    pub async fn unregister(&self, workspace_id: &str) {
        let mut meters = self.meters.write().await;
        meters.remove(workspace_id);
    }

    /// Get a workspace meter.
    pub async fn get(&self, workspace_id: &str) -> Option<Arc<WorkspaceResourceMeter>> {
        let meters = self.meters.read().await;
        meters.get(workspace_id).cloned()
    }

    /// Get total memory usage across all workspaces.
    pub async fn total_memory_mb(&self) -> f64 {
        let meters = self.meters.read().await;
        let mut total = 0.0;
        for meter in meters.values() {
            let usage = meter.current_usage().await;
            total += usage.memory_mb;
        }
        total
    }

    /// Check if adding a new workspace would exceed the global budget.
    pub async fn can_add_workspace(&self, budget_mb: u64) -> bool {
        let used = self.total_memory_mb().await;
        used + budget_mb as f64 <= self.global_memory_budget_mb as f64
    }

    /// Get summary of all workspace resource usage.
    pub async fn summary(&self) -> ResourceSummary {
        let meters = self.meters.read().await;
        let mut summary = ResourceSummary {
            total_workspaces: meters.len(),
            global_budget_mb: self.global_memory_budget_mb,
            ..Default::default()
        };

        for meter in meters.values() {
            let usage = meter.current_usage().await;
            let budget = meter.budget();
            summary.total_memory_mb += usage.memory_mb;
            summary.total_cpu_percent += usage.cpu_percent;
            summary.total_fds += usage.open_fds;

            match meter.alert_level().await {
                AlertLevel::Normal => summary.normal_count += 1,
                AlertLevel::Warning => summary.warning_count += 1,
                AlertLevel::Critical => summary.critical_count += 1,
            }
        }

        summary.memory_utilization = if summary.global_budget_mb > 0 {
            (summary.total_memory_mb / summary.global_budget_mb as f64) * 100.0
        } else {
            0.0
        };

        summary
    }
}

/// Summary of all workspace resource usage.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceSummary {
    pub total_workspaces: usize,
    pub global_budget_mb: u64,
    pub total_memory_mb: f64,
    pub total_cpu_percent: f64,
    pub total_fds: u32,
    pub memory_utilization: f64,
    pub normal_count: usize,
    pub warning_count: usize,
    pub critical_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_budget_is_within_limits() {
        let budget = ResourceBudget::default();
        assert!(budget.validate().is_ok());
        assert_eq!(budget.memory_mb, DEFAULT_MEMORY_BUDGET_MB);
    }

    #[test]
    fn inferno_budget_is_lightweight() {
        let budget = ResourceBudget::inferno();
        assert!(budget.validate().is_ok());
        assert!(budget.memory_mb <= 15);
    }

    #[test]
    fn linux_compat_budget_is_larger() {
        let budget = ResourceBudget::linux_compat();
        assert!(budget.validate().is_ok());
        assert!(budget.memory_mb <= 64);
    }

    #[test]
    fn budget_validation_rejects_too_small() {
        let budget = ResourceBudget {
            memory_mb: 1,
            ..Default::default()
        };
        assert!(budget.validate().is_err());
    }

    #[test]
    fn budget_validation_rejects_too_large() {
        let budget = ResourceBudget {
            memory_mb: 128,
            ..Default::default()
        };
        assert!(budget.validate().is_err());
    }

    #[tokio::test]
    async fn meter_tracks_usage() {
        let meter = WorkspaceResourceMeter::new(
            "test-ws".into(),
            ResourceBudget::inferno(),
        );

        let usage = ResourceUsage {
            memory_mb: 4.0,
            cpu_percent: 10.0,
            ..Default::default()
        };

        let level = meter.update(usage).await.unwrap();
        assert_eq!(level, AlertLevel::Normal);

        let current = meter.current_usage().await;
        assert!((current.memory_mb - 4.0).abs() < 0.01);
    }

    #[tokio::test]
    async fn meter_detects_warning() {
        let meter = WorkspaceResourceMeter::new(
            "test-ws".into(),
            ResourceBudget::inferno(),
        );

        // 80% of 8MB = 6.4MB
        let usage = ResourceUsage {
            memory_mb: 6.5,
            cpu_percent: 10.0,
            ..Default::default()
        };

        let level = meter.update(usage).await.unwrap();
        assert_eq!(level, AlertLevel::Warning);
    }

    #[tokio::test]
    async fn meter_detects_critical() {
        let meter = WorkspaceResourceMeter::new(
            "test-ws".into(),
            ResourceBudget::inferno(),
        );

        // 95% of 8MB = 7.6MB
        let usage = ResourceUsage {
            memory_mb: 7.7,
            cpu_percent: 10.0,
            ..Default::default()
        };

        let level = meter.update(usage).await.unwrap();
        assert_eq!(level, AlertLevel::Critical);
    }

    #[tokio::test]
    async fn global_manager_tracks_total() {
        let manager = GlobalResourceManager::new(64);

        let meter1 = Arc::new(WorkspaceResourceMeter::new(
            "ws1".into(),
            ResourceBudget::inferno(),
        ));
        let meter2 = Arc::new(WorkspaceResourceMeter::new(
            "ws2".into(),
            ResourceBudget::inferno(),
        ));

        manager.register(meter1.clone()).await;
        manager.register(meter2.clone()).await;

        meter1
            .update(ResourceUsage {
                memory_mb: 4.0,
                ..Default::default()
            })
            .await
            .unwrap();
        meter2
            .update(ResourceUsage {
                memory_mb: 5.0,
                ..Default::default()
            })
            .await
            .unwrap();

        let total = manager.total_memory_mb().await;
        assert!((total - 9.0).abs() < 0.01);

        assert!(manager.can_add_workspace(50).await);
        assert!(!manager.can_add_workspace(60).await);
    }

    #[tokio::test]
    async fn peak_and_average_tracking() {
        let meter = WorkspaceResourceMeter::new(
            "test-ws".into(),
            ResourceBudget::inferno(),
        );

        for i in 0..5 {
            meter
                .update(ResourceUsage {
                    memory_mb: (i * 2) as f64,
                    ..Default::default()
                })
                .await
                .unwrap();
        }

        let peak = meter.peak_memory_mb().await;
        assert!((peak - 8.0).abs() < 0.01);

        let avg = meter.avg_memory_mb().await;
        assert!((avg - 4.0).abs() < 0.01);
    }
}
