//! Real-time system monitoring using `sysinfo`.

use plaza_events::{EventBus, PlazaEvent};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sysinfo::System;
use tokio_util::sync::CancellationToken;
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetricsSnapshot {
    pub cpu_usage_pct: f64,
    pub memory_used_mb: u64,
    pub memory_total_mb: u64,
    pub memory_usage_pct: f64,
}

pub struct SystemMonitor {
    event_bus: Arc<EventBus>,
}

impl SystemMonitor {
    pub fn new(event_bus: Arc<EventBus>) -> Self {
        Self { event_bus }
    }

    /// Sample current metrics.
    pub fn sample(&self) -> SystemMetricsSnapshot {
        let mut sys = System::new_all();
        sys.refresh_cpu_usage();
        sys.refresh_memory();

        let cpu_usage_pct = sys.global_cpu_usage() as f64;
        let memory_used_mb = sys.used_memory() / (1024 * 1024);
        let memory_total_mb = sys.total_memory() / (1024 * 1024);
        let memory_usage_pct = if memory_total_mb > 0 {
            (memory_used_mb as f64 / memory_total_mb as f64) * 100.0
        } else {
            0.0
        };

        SystemMetricsSnapshot {
            cpu_usage_pct,
            memory_used_mb,
            memory_total_mb,
            memory_usage_pct,
        }
    }

    /// Run continuous background metrics collection loop.
    pub async fn run(&self, token: CancellationToken) {
        info!("system monitor background loop started");
        loop {
            tokio::select! {
                _ = token.cancelled() => break,
                _ = tokio::time::sleep(std::time::Duration::from_secs(2)) => {
                    let snapshot = self.sample();
                    self.event_bus.publish(PlazaEvent::SystemMetricsUpdated {
                        cpu_usage_pct: snapshot.cpu_usage_pct,
                        memory_usage_pct: snapshot.memory_usage_pct,
                        disk_usage_pct: 0.0,
                    }).await;
                }
            }
        }
    }
}
