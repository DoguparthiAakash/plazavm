use plaza_core::id::WorkspaceId;
use plaza_core::types::HealthStatus;
use plaza_events::{EventBus, PlazaEvent};
use std::sync::Arc;

#[tokio::test]
async fn test_event_bus_publish_subscribe() {
    let bus = EventBus::new();
    let mut rx = bus.subscribe();

    let ws_id = WorkspaceId::new();
    bus.publish(PlazaEvent::WorkspaceCreated {
        id: ws_id.clone(),
        name: "test-ws".into(),
    })
    .await;

    let received = rx.recv().await.expect("subscriber should receive event");
    assert_eq!(received.event_type(), "workspace.created");
}

#[tokio::test]
async fn test_event_history_and_filtering() {
    let bus = EventBus::new();

    let id = WorkspaceId::new();
    bus.publish(PlazaEvent::WorkspaceCreated {
        id: id.clone(),
        name: "ws1".into(),
    })
    .await;
    bus.publish(PlazaEvent::WorkspaceHealthChanged {
        id: id.clone(),
        health: HealthStatus::Healthy,
    })
    .await;

    let history = bus.history().await;
    assert_eq!(history.len(), 2);

    let created_events = bus.history_filtered("workspace.created").await;
    assert_eq!(created_events.len(), 1);

    let health_events = bus.history_filtered("workspace.health_changed").await;
    assert_eq!(health_events.len(), 1);
}

#[tokio::test]
async fn test_subscriber_count() {
    let bus = EventBus::new();
    assert_eq!(bus.subscriber_count(), 0);

    let _rx1 = bus.subscribe();
    assert_eq!(bus.subscriber_count(), 1);

    let _rx2 = bus.subscribe();
    assert_eq!(bus.subscriber_count(), 2);
}

#[tokio::test]
async fn test_concurrent_event_publishers() {
    let bus = Arc::new(EventBus::new());
    let mut rx = bus.subscribe();

    let mut handles = vec![];
    for i in 0..10 {
        let bus_clone = bus.clone();
        handles.push(tokio::spawn(async move {
            bus_clone
                .publish(PlazaEvent::PlatformScanned {
                    profile: format!("profile_{i}"),
                })
                .await;
        }));
    }

    for h in handles {
        h.await.unwrap();
    }

    let mut count = 0;
    while count < 10 {
        let ev = rx.recv().await.unwrap();
        assert_eq!(ev.event_type(), "platform.scanned");
        count += 1;
    }

    assert_eq!(bus.history().await.len(), 10);
}
