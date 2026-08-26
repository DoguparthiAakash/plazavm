use plaza_workspace::graph::NodeRole;
use plaza_workspace::model::{
    DesiredState, RuntimeBackendPreference, RuntimeKind, Workspace, WorkspaceSpec, WorkspaceState,
};

#[test]
fn test_workspace_aggregate_construction() {
    let spec = WorkspaceSpec::default();
    let ws = Workspace::new("dev-env", spec);

    assert_eq!(ws.name, "dev-env");
    assert_eq!(ws.status.state, WorkspaceState::Stopped);
    assert_eq!(ws.spec.desired_state, DesiredState::Stopped);

    let primary = ws.graph.primary_node().expect("primary node should exist");
    assert_eq!(primary.role, NodeRole::Primary);
}

#[test]
fn test_workspace_fsm_display() {
    assert_eq!(WorkspaceState::Pending.to_string(), "pending");
    assert_eq!(WorkspaceState::Running.to_string(), "running");
    assert_eq!(WorkspaceState::Stopped.to_string(), "stopped");
    assert_eq!(WorkspaceState::Error.to_string(), "error");
}

#[test]
fn test_runtime_spec_defaults() {
    let spec = WorkspaceSpec::default();
    assert_eq!(spec.runtime.kind, RuntimeKind::Container);
    assert_eq!(spec.runtime.backend, RuntimeBackendPreference::Auto);
}
