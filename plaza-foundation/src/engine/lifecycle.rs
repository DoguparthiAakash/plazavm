use crate::engine::errors::{PfeError, PfeResult};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU8, Ordering};

/// Deterministic Engine Lifecycle States.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EngineLifecycleState {
    Booting = 0,
    Initializing = 1,
    Discovering = 2,
    Ready = 3,
    Running = 4,
    Paused = 5,
    Maintenance = 6,
    Recovering = 7,
    Stopping = 8,
    Stopped = 9,
    Failed = 10,
}

impl EngineLifecycleState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Booting => "Booting",
            Self::Initializing => "Initializing",
            Self::Discovering => "Discovering",
            Self::Ready => "Ready",
            Self::Running => "Running",
            Self::Paused => "Paused",
            Self::Maintenance => "Maintenance",
            Self::Recovering => "Recovering",
            Self::Stopping => "Stopping",
            Self::Stopped => "Stopped",
            Self::Failed => "Failed",
        }
    }
}

/// Thread-safe Engine Lifecycle State Machine.
pub struct EngineLifecycle {
    current: AtomicU8,
}

impl EngineLifecycle {
    pub fn new() -> Self {
        Self {
            current: AtomicU8::new(EngineLifecycleState::Booting as u8),
        }
    }

    pub fn state(&self) -> EngineLifecycleState {
        match self.current.load(Ordering::SeqCst) {
            0 => EngineLifecycleState::Booting,
            1 => EngineLifecycleState::Initializing,
            2 => EngineLifecycleState::Discovering,
            3 => EngineLifecycleState::Ready,
            4 => EngineLifecycleState::Running,
            5 => EngineLifecycleState::Paused,
            6 => EngineLifecycleState::Maintenance,
            7 => EngineLifecycleState::Recovering,
            8 => EngineLifecycleState::Stopping,
            9 => EngineLifecycleState::Stopped,
            _ => EngineLifecycleState::Failed,
        }
    }

    pub fn transition_to(&self, next: EngineLifecycleState) -> PfeResult<()> {
        let current_state = self.state();
        let valid = matches!(
            (current_state, next),
            (
                EngineLifecycleState::Booting,
                EngineLifecycleState::Initializing
            ) | (
                EngineLifecycleState::Initializing,
                EngineLifecycleState::Discovering
            ) | (
                EngineLifecycleState::Discovering,
                EngineLifecycleState::Ready
            ) | (EngineLifecycleState::Ready, EngineLifecycleState::Running)
                | (EngineLifecycleState::Running, EngineLifecycleState::Paused)
                | (EngineLifecycleState::Paused, EngineLifecycleState::Running)
                | (
                    EngineLifecycleState::Running,
                    EngineLifecycleState::Maintenance
                )
                | (
                    EngineLifecycleState::Maintenance,
                    EngineLifecycleState::Running
                )
                | (
                    EngineLifecycleState::Running,
                    EngineLifecycleState::Recovering
                )
                | (
                    EngineLifecycleState::Recovering,
                    EngineLifecycleState::Ready
                )
                | (
                    EngineLifecycleState::Running,
                    EngineLifecycleState::Stopping
                )
                | (
                    EngineLifecycleState::Stopping,
                    EngineLifecycleState::Stopped
                )
                | (_, EngineLifecycleState::Failed)
        );

        if valid {
            self.current.store(next as u8, Ordering::SeqCst);
            tracing::info!(
                from = current_state.as_str(),
                to = next.as_str(),
                "PFE Lifecycle State Changed"
            );
            Ok(())
        } else {
            Err(PfeError::InvalidLifecycleTransition {
                from: current_state.as_str().into(),
                to: next.as_str().into(),
            })
        }
    }
}

impl Default for EngineLifecycle {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lifecycle_transitions() {
        let lifecycle = EngineLifecycle::new();
        assert_eq!(lifecycle.state(), EngineLifecycleState::Booting);

        assert!(lifecycle
            .transition_to(EngineLifecycleState::Initializing)
            .is_ok());
        assert_eq!(lifecycle.state(), EngineLifecycleState::Initializing);

        assert!(lifecycle
            .transition_to(EngineLifecycleState::Discovering)
            .is_ok());
        assert!(lifecycle.transition_to(EngineLifecycleState::Ready).is_ok());
        assert!(lifecycle
            .transition_to(EngineLifecycleState::Running)
            .is_ok());

        // Invalid transition
        assert!(lifecycle
            .transition_to(EngineLifecycleState::Booting)
            .is_err());
    }
}
