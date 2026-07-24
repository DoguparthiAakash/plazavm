use plaza_api::bootstrap::BootstrapBuilder;
use plaza_events::PlazaEvent;
use plaza_workspace::model::{DesiredState, WorkspaceSpec, WorkspaceState};
use std::sync::Arc;

#[tokio::test]
async fn integration_workflow_full_workspace_lifecycle() {
    let container = BootstrapBuilder::new()
        .with_in_memory_db()
        .build()
        .await
        .expect("bootstrap composition root");

    // Register Docker plugin
    container
        .plugin_host
        .register_runtime_plugin(Arc::new(docker_plugin::DockerPlugin::new()))
        .await
        .unwrap();

    // 1. Create Workspace
    let ws = container
        .workspace_service
        .create_workspace("lifecycle-demo", WorkspaceSpec::default())
        .await
        .expect("create workspace");

    assert_eq!(ws.status.state, WorkspaceState::Stopped);

    // 2. Start Workspace (Desired: Running)
    container
        .workspace_service
        .set_desired_state(&ws.id, DesiredState::Running)
        .await
        .unwrap();

    let fetched = container
        .workspace_service
        .get_workspace(&ws.id)
        .await
        .unwrap()
        .unwrap();
    container
        .controller
        .reconcile_workspace(&fetched)
        .await
        .unwrap();

    let running_ws = container
        .workspace_service
        .get_workspace(&ws.id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(running_ws.status.state, WorkspaceState::Running);
    assert_eq!(running_ws.status.runtime_backend.as_deref(), Some("docker"));

    // 3. Stop Workspace (Desired: Stopped)
    container
        .workspace_service
        .set_desired_state(&ws.id, DesiredState::Stopped)
        .await
        .unwrap();

    let stopping_ws = container
        .workspace_service
        .get_workspace(&ws.id)
        .await
        .unwrap()
        .unwrap();
    container
        .controller
        .reconcile_workspace(&stopping_ws)
        .await
        .unwrap();

    let stopped_ws = container
        .workspace_service
        .get_workspace(&ws.id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(stopped_ws.status.state, WorkspaceState::Stopped);

    // 4. Delete Workspace
    container
        .workspace_service
        .delete_workspace(&ws.id)
        .await
        .unwrap();
    assert!(container
        .workspace_service
        .get_workspace(&ws.id)
        .await
        .unwrap()
        .is_none());
}

#[tokio::test]
async fn stress_test_10000_events_throughput() {
    let bus = Arc::new(plaza_events::EventBus::with_capacity(16384));
    let mut rx = bus.subscribe();

    let start = std::time::Instant::now();

    let bus_clone = bus.clone();
    let publisher = tokio::spawn(async move {
        for i in 0..10_000 {
            bus_clone
                .publish(PlazaEvent::PlatformScanned {
                    profile: format!("profile_{i}"),
                })
                .await;
        }
    });

    let mut received = 0;
    while received < 10_000 {
        if rx.recv().await.is_ok() {
            received += 1;
        }
    }

    publisher.await.unwrap();
    let elapsed = start.elapsed();

    println!(
        "Stress Test: 10,000 events processed in {:.2}ms ({:.0} ev/sec)",
        elapsed.as_secs_f64() * 1000.0,
        10_000.0 / elapsed.as_secs_f64()
    );

    assert_eq!(received, 10_000);
}

#[tokio::test]
async fn stress_test_1000_workspaces_scaling() {
    let container = BootstrapBuilder::new()
        .with_in_memory_db()
        .build()
        .await
        .expect("bootstrap composition root");

    let start = std::time::Instant::now();

    for i in 0..1_000 {
        let name = format!("scale-ws-{i}");
        container
            .workspace_service
            .create_workspace(&name, WorkspaceSpec::default())
            .await
            .unwrap();
    }

    let list = container.workspace_service.list_workspaces().await.unwrap();
    let elapsed = start.elapsed();

    println!(
        "Scaling Test: Created and retrieved 1,000 workspaces in {:.2}ms",
        elapsed.as_secs_f64() * 1000.0
    );

    assert_eq!(list.len(), 1_000);
}

#[tokio::test]
async fn failure_test_resource_exhaustion_recovery() {
    let container = BootstrapBuilder::new()
        .with_in_memory_db()
        .build()
        .await
        .expect("bootstrap composition root");

    let mut spec = WorkspaceSpec::default();
    spec.resources.memory_mb = 1_000_000_000; // Over-subscription

    let ws = container
        .workspace_service
        .create_workspace("oversubscribed", spec)
        .await
        .unwrap();

    container
        .workspace_service
        .set_desired_state(&ws.id, DesiredState::Running)
        .await
        .unwrap();

    let fetched = container
        .workspace_service
        .get_workspace(&ws.id)
        .await
        .unwrap()
        .unwrap();
    let result = container.controller.reconcile_workspace(&fetched).await;

    assert!(result.is_err());
    let failed_ws = container
        .workspace_service
        .get_workspace(&ws.id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(failed_ws.status.state, WorkspaceState::Error);
}
