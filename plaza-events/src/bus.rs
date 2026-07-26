//! Async event bus backed by tokio broadcast channels.

use crate::PlazaEvent;
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use tracing::debug;

/// Default channel capacity — how many events can be buffered before
/// slow subscribers start missing events.
const DEFAULT_CAPACITY: usize = 1024;

/// Maximum number of events retained in the history ring buffer.
const MAX_HISTORY: usize = 512;

/// Async event bus for PlazaVM.
///
/// All subsystems publish events here. Subscribers receive events
/// through tokio broadcast receivers. A bounded history is kept so
/// late subscribers can replay recent events.
#[derive(Clone)]
pub struct EventBus {
    sender: broadcast::Sender<PlazaEvent>,
    history: Arc<RwLock<VecDeque<PlazaEvent>>>,
}

impl EventBus {
    /// Create a new event bus with default capacity.
    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_CAPACITY)
    }

    /// Create a new event bus with a specific channel capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);
        Self {
            sender,
            history: Arc::new(RwLock::new(VecDeque::with_capacity(MAX_HISTORY))),
        }
    }

    /// Publish an event to all subscribers.
    pub async fn publish(&self, event: PlazaEvent) {
        debug!(event_type = event.event_type(), "publishing event");

        // Hold the lock while sending to ensure events are broadcast
        // in the exact same order they are appended to history.
        let mut history = self.history.write().await;
        if history.len() >= MAX_HISTORY {
            history.pop_front();
        }
        history.push_back(event.clone());

        // Broadcast — if nobody is listening that's fine
        if self.sender.send(event).is_err() {
            debug!("event published but no active subscribers");
        }
    }

    /// Subscribe to the event stream.
    pub fn subscribe(&self) -> broadcast::Receiver<PlazaEvent> {
        self.sender.subscribe()
    }

    /// Get the current number of active subscribers.
    pub fn subscriber_count(&self) -> usize {
        self.sender.receiver_count()
    }

    /// Get a snapshot of the event history.
    pub async fn history(&self) -> Vec<PlazaEvent> {
        self.history.read().await.iter().cloned().collect()
    }

    /// Get recent events of a specific type.
    pub async fn history_filtered(&self, event_type: &str) -> Vec<PlazaEvent> {
        self.history
            .read()
            .await
            .iter()
            .filter(|e| e.event_type() == event_type)
            .cloned()
            .collect()
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use plaza_core::WorkspaceId;

    #[tokio::test]
    async fn publish_and_receive() {
        let bus = EventBus::new();
        let mut rx = bus.subscribe();

        let id = WorkspaceId::new();
        bus.publish(PlazaEvent::WorkspaceCreated {
            id: id.clone(),
            name: "test".to_string(),
        })
        .await;

        let event = rx.recv().await.unwrap();
        assert_eq!(event.event_type(), "workspace.created");
    }

    #[tokio::test]
    async fn history_is_retained() {
        let bus = EventBus::new();

        bus.publish(PlazaEvent::PlatformScanned {
            profile: "developer_laptop".to_string(),
        })
        .await;

        let history = bus.history().await;
        assert_eq!(history.len(), 1);
    }

    #[tokio::test]
    async fn publish_without_subscribers_does_not_panic() {
        let bus = EventBus::new();
        bus.publish(PlazaEvent::PlatformScanned {
            profile: "test".to_string(),
        })
        .await;
        // Should not panic
    }
}
