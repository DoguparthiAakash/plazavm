use crate::traits::ExecutableCommand;
use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::sync::Arc;

/// Central registry for all ExecutableCommands in the platform.
/// Allows dynamic discovery and execution of commands by ID.
pub struct CommandRegistry {
    commands: HashMap<String, Arc<dyn ExecutableCommand>>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
        }
    }

    /// Registers a command into the registry using its explicit command_id (e.g. `workspace.create`).
    pub fn register(&mut self, command_id: impl Into<String>, cmd: Arc<dyn ExecutableCommand>) {
        self.commands.insert(command_id.into(), cmd);
    }

    /// Resolves an executable command by its ID.
    pub fn resolve(&self, command_id: &str) -> Result<Arc<dyn ExecutableCommand>> {
        self.commands
            .get(command_id)
            .cloned()
            .ok_or_else(|| anyhow!("Command '{}' not found in registry", command_id))
    }

    /// Returns a list of all registered command IDs for introspection/discovery.
    pub fn list_commands(&self) -> Vec<String> {
        let mut cmds: Vec<String> = self.commands.keys().cloned().collect();
        cmds.sort();
        cmds
    }
}

