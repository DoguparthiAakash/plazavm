use plaza_events::EventBus;
use plaza_monitor::SystemMonitor;
use std::sync::Arc;

pub struct EngineMetrics {
    monitor: SystemMonitor,
}

impl EngineMetrics {
    pub fn new(event_bus: Arc<EventBus>) -> Self {
        Self {
            monitor: SystemMonitor::new(event_bus),
        }
    }

    pub fn sample(&self) -> plaza_monitor::SystemMetricsSnapshot {
        self.monitor.sample()
    }
}
