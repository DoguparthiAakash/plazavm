//! Workspace Reconciliation Engine.
//!
//! Implements a Kubernetes-style reconciliation loop that continuously
//! converges the *observed* workspace state toward the *desired* state
//! declared in the `WorkspaceSpec`.
//!
//! The reconciler is the single source of truth for state transitions.
//! No direct state mutation is allowed outside the reconciler's control.

use crate::model::{DesiredState, Workspace, WorkspaceState};
use plaza_foundation::core::{PlazaError, PlazaResult};
use tracing::{debug, info, warn};

/// The result of a single reconciliation pass.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReconcileAction {
    /// No action needed — observed state matches desired state.
    Noop,
    /// Workspace should be started.
    Start,
    /// Workspace should be stopped.
    Stop,
    /// Workspace should be paused.
    Pause,
    /// Workspace should be resumed from pause.
    Resume,
    /// Workspace should be destroyed and cleaned up.
    Destroy,
    /// Workspace is in an error state and needs recovery.
    Recover,
}

/// Validates whether a given state transition is legal.
pub fn validate_transition(from: WorkspaceState, to: WorkspaceState) -> PlazaResult<()> {
    let allowed = match from {
        WorkspaceState::Stopped => matches!(
            to,
            WorkspaceState::Pending
                | WorkspaceState::Starting
                | WorkspaceState::Destroying
                | WorkspaceState::Destroyed
        ),
        WorkspaceState::Pending => matches!(
            to,
            WorkspaceState::Scheduling | WorkspaceState::Error | WorkspaceState::Stopped
        ),
        WorkspaceState::Scheduling => matches!(
            to,
            WorkspaceState::Creating | WorkspaceState::Error | WorkspaceState::Stopped
        ),
        WorkspaceState::Creating => matches!(
            to,
            WorkspaceState::Starting | WorkspaceState::Error | WorkspaceState::Stopped
        ),
        WorkspaceState::Starting => matches!(
            to,
            WorkspaceState::Running | WorkspaceState::Error | WorkspaceState::Stopped
        ),
        WorkspaceState::Running => matches!(
            to,
            WorkspaceState::Paused
                | WorkspaceState::Stopping
                | WorkspaceState::Error
        ),
        WorkspaceState::Paused => matches!(
            to,
            WorkspaceState::Running
                | WorkspaceState::Stopping
                | WorkspaceState::Error
        ),
        WorkspaceState::Stopping => matches!(
            to,
            WorkspaceState::Stopped | WorkspaceState::Error
        ),
        WorkspaceState::Error => matches!(
            to,
            WorkspaceState::Stopped | WorkspaceState::Destroying
        ),
        WorkspaceState::Destroying => matches!(to, WorkspaceState::Destroyed | WorkspaceState::Error),
        WorkspaceState::Destroyed => false, // Terminal state — no transitions out.
    };

    if allowed {
        Ok(())
    } else {
        Err(PlazaError::InvalidStateTransition {
            from: from.to_string(),
            to: to.to_string(),
        })
    }
}

/// The workspace reconciler.
pub struct Reconciler;

impl Reconciler {
    /// Determine the action required to converge the observed state
    /// toward the desired state.
    pub fn reconcile(workspace: &Workspace) -> ReconcileAction {
        let observed = workspace.status.state;
        let desired = workspace.spec.desired_state;

        debug!(
            workspace = %workspace.name,
            observed = %observed,
            desired = ?desired,
            "reconciling workspace"
        );

        // If we're in an error state, always recommend recovery first.
        if observed == WorkspaceState::Error {
            warn!(workspace = %workspace.name, "workspace in error state, recommending recovery");
            return ReconcileAction::Recover;
        }

        // If we're in a transient state (Creating, Starting, Stopping, etc.),
        // do nothing — wait for the transition to complete.
        if matches!(
            observed,
            WorkspaceState::Pending
                | WorkspaceState::Scheduling
                | WorkspaceState::Creating
                | WorkspaceState::Starting
                | WorkspaceState::Stopping
                | WorkspaceState::Destroying
        ) {
            debug!(workspace = %workspace.name, "workspace in transient state, waiting");
            return ReconcileAction::Noop;
        }

        match (observed, desired) {
            // Already in desired state.
            (WorkspaceState::Running, DesiredState::Running) => ReconcileAction::Noop,
            (WorkspaceState::Stopped, DesiredState::Stopped) => ReconcileAction::Noop,
            (WorkspaceState::Paused, DesiredState::Paused) => ReconcileAction::Noop,
            (WorkspaceState::Destroyed, DesiredState::Destroyed) => ReconcileAction::Noop,

            // Convergence actions.
            (WorkspaceState::Stopped, DesiredState::Running) => {
                info!(workspace = %workspace.name, "reconciler: stopped -> starting");
                ReconcileAction::Start
            }
            (WorkspaceState::Stopped, DesiredState::Destroyed) => {
                info!(workspace = %workspace.name, "reconciler: stopped -> destroying");
                ReconcileAction::Destroy
            }
            (WorkspaceState::Running, DesiredState::Stopped) => {
                info!(workspace = %workspace.name, "reconciler: running -> stopping");
                ReconcileAction::Stop
            }
            (WorkspaceState::Running, DesiredState::Paused) => {
                info!(workspace = %workspace.name, "reconciler: running -> pausing");
                ReconcileAction::Pause
            }
            (WorkspaceState::Running, DesiredState::Destroyed) => {
                info!(workspace = %workspace.name, "reconciler: running -> stopping (for destroy)");
                ReconcileAction::Stop
            }
            (WorkspaceState::Paused, DesiredState::Running) => {
                info!(workspace = %workspace.name, "reconciler: paused -> resuming");
                ReconcileAction::Resume
            }
            (WorkspaceState::Paused, DesiredState::Stopped) => {
                info!(workspace = %workspace.name, "reconciler: paused -> stopping");
                ReconcileAction::Stop
            }
            (WorkspaceState::Paused, DesiredState::Destroyed) => {
                info!(workspace = %workspace.name, "reconciler: paused -> stopping (for destroy)");
                ReconcileAction::Stop
            }

            // Catch-all: should not normally happen, treat as noop.
            _ => {
                warn!(
                    workspace = %workspace.name,
                    observed = %observed,
                    desired = ?desired,
                    "reconciler: unhandled state combination, no action"
                );
                ReconcileAction::Noop
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::WorkspaceSpec;

    #[test]
    fn test_valid_transitions() {
        assert!(validate_transition(WorkspaceState::Stopped, WorkspaceState::Starting).is_ok());
        assert!(validate_transition(WorkspaceState::Starting, WorkspaceState::Running).is_ok());
        assert!(validate_transition(WorkspaceState::Running, WorkspaceState::Stopping).is_ok());
        assert!(validate_transition(WorkspaceState::Stopping, WorkspaceState::Stopped).is_ok());
        assert!(validate_transition(WorkspaceState::Running, WorkspaceState::Paused).is_ok());
        assert!(validate_transition(WorkspaceState::Paused, WorkspaceState::Running).is_ok());
        assert!(validate_transition(WorkspaceState::Error, WorkspaceState::Stopped).is_ok());
    }

    #[test]
    fn test_invalid_transitions() {
        assert!(validate_transition(WorkspaceState::Stopped, WorkspaceState::Running).is_err());
        assert!(validate_transition(WorkspaceState::Running, WorkspaceState::Creating).is_err());
        assert!(validate_transition(WorkspaceState::Destroyed, WorkspaceState::Running).is_err());
        assert!(validate_transition(WorkspaceState::Destroyed, WorkspaceState::Stopped).is_err());
    }

    #[test]
    fn test_reconcile_noop_when_converged() {
        let mut ws = test_workspace();
        ws.status.state = WorkspaceState::Running;
        ws.spec.desired_state = DesiredState::Running;
        assert_eq!(Reconciler::reconcile(&ws), ReconcileAction::Noop);
    }

    #[test]
    fn test_reconcile_start() {
        let mut ws = test_workspace();
        ws.status.state = WorkspaceState::Stopped;
        ws.spec.desired_state = DesiredState::Running;
        assert_eq!(Reconciler::reconcile(&ws), ReconcileAction::Start);
    }

    #[test]
    fn test_reconcile_stop() {
        let mut ws = test_workspace();
        ws.status.state = WorkspaceState::Running;
        ws.spec.desired_state = DesiredState::Stopped;
        assert_eq!(Reconciler::reconcile(&ws), ReconcileAction::Stop);
    }

    #[test]
    fn test_reconcile_recover_on_error() {
        let mut ws = test_workspace();
        ws.status.state = WorkspaceState::Error;
        ws.spec.desired_state = DesiredState::Running;
        assert_eq!(Reconciler::reconcile(&ws), ReconcileAction::Recover);
    }

    #[test]
    fn test_reconcile_noop_during_transient() {
        let mut ws = test_workspace();
        ws.status.state = WorkspaceState::Starting;
        ws.spec.desired_state = DesiredState::Running;
        assert_eq!(Reconciler::reconcile(&ws), ReconcileAction::Noop);
    }

    fn test_workspace() -> Workspace {
        Workspace::new("test-ws", WorkspaceSpec::default())
    }
}
