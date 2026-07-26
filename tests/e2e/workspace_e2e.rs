//! Root-level E2E workspace lifecycle test.

use plaza_api::bootstrap::BootstrapBuilder;
use plaza_workspace::model::{DesiredState, WorkspaceSpec, WorkspaceState};

#[tokio::test]
async fn test_e2e_workspace_full_lifecycle() {
    let container = BootstrapBuilder::new()
        .with_in_memory_db()
        .build()
        .await
        .expect("Bootstrap failed");

    let ws = container
        .workspace_service
        .create_workspace("e2e-root-ws", WorkspaceSpec::default())
        .await
        .expect("Create workspace failed");

    assert_eq!(ws.status.state, WorkspaceState::Created);

    // Set desired state to Running
    container
        .workspace_service
        .set_desired_state(&ws.id, DesiredState::Running)
        .await
        .expect("Set state failed");

    let updated = container
        .workspace_service
        .get_workspace(&ws.id)
        .await
        .expect("Fetch failed")
        .expect("Workspace missing");

    assert_eq!(updated.status.state, WorkspaceState::Scheduling);
}

