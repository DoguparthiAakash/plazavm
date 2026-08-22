//! Machine OS Manager for handling lifecycle and state of the main engine OS.

pub const OS_READY_MARKER: &str = "SUCCESS_PLAZA_GUEST_READY";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OsLifecycleState {
    Booting,
    Ready,
    ShuttingDown,
    Stopped,
    Error(String),
}

pub struct MachineManager {
    state: OsLifecycleState,
}

impl MachineManager {
    pub fn new() -> Self {
        Self {
            state: OsLifecycleState::Booting,
        }
    }

    pub fn set_state(&mut self, state: OsLifecycleState) {
        self.state = state;
    }

    pub fn get_state(&self) -> &OsLifecycleState {
        &self.state
    }

    /// Checks a chunk of serial output to see if the OS is ready.
    pub fn check_ready_marker(chunk: &str) -> bool {
        chunk.contains(OS_READY_MARKER)
    }
}

impl Default for MachineManager {
    fn default() -> Self {
        Self::new()
    }
}
