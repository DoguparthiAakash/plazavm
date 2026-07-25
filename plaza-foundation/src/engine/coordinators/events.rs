use plaza_events::EventBus;
use std::sync::Arc;

/// Thin coordinator delegating pub-sub event operations to `plaza-events`.
pub struct EventCoordinator {
    event_bus: Arc<EventBus>,
}

impl EventCoordinator {
    pub fn new(event_bus: Arc<EventBus>) -> Self {
        Self { event_bus }
    }

    pub fn event_bus(&self) -> &Arc<EventBus> {
        &self.event_bus
    }
}
