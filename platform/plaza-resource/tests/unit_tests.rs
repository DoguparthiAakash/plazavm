use plaza_foundation::core::id::WorkspaceId;
use plaza_foundation::events::EventBus;
use plaza_foundation::platform::PlatformDetector;
use plaza_resource::{ResourceManager, ResourcePlan, WorkspacePriority};
use std::sync::Arc;

#[tokio::test]
async fn test_resource_allocation_success_and_release() {
    let platform = Arc::new(PlatformDetector::new());
    platform.scan().await.unwrap();
    let event_bus = Arc::new(EventBus::new());

    let mgr = ResourceManager::new(platform, event_bus);

    let plan = ResourcePlan {
        workspace_id: WorkspaceId::new(),
        cpu_cores: 2,
        memory_mb: 1024,
        gpu_enabled: false,
        priority: WorkspacePriority::Normal,
    };

    let alloc = mgr
        .allocate(&plan)
        .await
        .expect("allocation should succeed");
    assert_eq!(alloc.cpu_cores, 2);
    assert_eq!(alloc.memory_mb, 1024);

    let active = mgr.active_allocations().await;
    assert_eq!(active.len(), 1);

    mgr.release(&plan.workspace_id).await.unwrap();
    let active_after = mgr.active_allocations().await;
    assert_eq!(active_after.len(), 0);
}

#[tokio::test]
async fn test_resource_allocation_exceeds_host_memory() {
    let platform = Arc::new(PlatformDetector::new());
    platform.scan().await.unwrap();
    let event_bus = Arc::new(EventBus::new());

    let mgr = ResourceManager::new(platform, event_bus);

    // Host memory is around 32GB; request 1,000,000 MB
    let plan = ResourcePlan {
        workspace_id: WorkspaceId::new(),
        cpu_cores: 64,
        memory_mb: 1_000_000,
        gpu_enabled: false,
        priority: WorkspacePriority::Normal,
    };

    let result = mgr.allocate(&plan).await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(matches!(
        err,
        plaza_foundation::core::PlazaError::ResourceExhausted { .. }
    ));
}

