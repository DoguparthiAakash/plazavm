//! Capability model for Inferno guest runtime.
//!
//! Defines which Linux-oriented operations are supported by the
//! Inferno compatibility layer. Each capability has an explicit state
//! indicating whether it's natively supported, emulable, or unsupported.
//!
//! # Design Principles
//!
//! 1. **Explicit capability states**: Never claim a capability works unless tested
//! 2. **Transparency**: Users can query what operations are available
//! 3. **Safety**: The compatibility layer must never provide an escape path
//! 4. **Measurability**: Each capability can be benchmarked independently

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The state of a capability in the Inferno compatibility layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityState {
    /// Available natively in Inferno without any translation.
    Supported,
    /// Can be translated to an Inferno equivalent with full fidelity.
    Emulable,
    /// Some aspects work, some don't. Partial implementation.
    PartiallySupported,
    /// Not available in Inferno. Cannot be provided.
    Unsupported,
    /// Implementation is blocked by a dependency that hasn't been resolved yet.
    Blocked,
}

impl std::fmt::Display for CapabilityState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Supported => write!(f, "✅ Supported"),
            Self::Emulable => write!(f, "🔄 Emulable"),
            Self::PartiallySupported => write!(f, "⚠️  Partially Supported"),
            Self::Unsupported => write!(f, "❌ Unsupported"),
            Self::Blocked => write!(f, "🚧 Blocked"),
        }
    }
}

/// A single capability that the Inferno runtime can provide.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeCapability {
    /// The capability name (e.g. "filesystem.read").
    pub name: String,
    /// Human-readable description.
    pub description: String,
    /// Current implementation state.
    pub state: CapabilityState,
    /// The Inferno mechanism used (if applicable).
    pub mechanism: Option<String>,
    /// Known limitations (if any).
    pub limitations: Vec<String>,
    /// Whether this capability has been tested.
    pub tested: bool,
}

/// The complete capability profile for an Inferno guest runtime.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityProfile {
    /// Runtime identifier.
    pub runtime: String,
    /// All capabilities.
    pub capabilities: Vec<RuntimeCapability>,
    /// Additional metadata.
    pub metadata: HashMap<String, String>,
}

impl CapabilityProfile {
    /// Create the default Inferno capability profile.
    ///
    /// This represents the current best understanding of what Inferno
    /// can provide for PlazaVM workloads.
    pub fn inferno_default() -> Self {
        let capabilities = vec![
            // === Filesystem Operations ===
            RuntimeCapability {
                name: "filesystem.read".into(),
                description: "Read files from the workspace".into(),
                state: CapabilityState::Supported,
                mechanism: Some("Inferno native file operations".into()),
                limitations: vec![],
                tested: false,
            },
            RuntimeCapability {
                name: "filesystem.write".into(),
                description: "Write files to the workspace".into(),
                state: CapabilityState::Supported,
                mechanism: Some("Inferno native file operations".into()),
                limitations: vec![],
                tested: false,
            },
            RuntimeCapability {
                name: "filesystem.create".into(),
                description: "Create new files and directories".into(),
                state: CapabilityState::Supported,
                mechanism: Some("Inferno native file operations".into()),
                limitations: vec![],
                tested: false,
            },
            RuntimeCapability {
                name: "filesystem.delete".into(),
                description: "Delete files and directories".into(),
                state: CapabilityState::Supported,
                mechanism: Some("Inferno native file operations".into()),
                limitations: vec![],
                tested: false,
            },
            RuntimeCapability {
                name: "filesystem.permissions".into(),
                description: "Unix-style file permissions (chmod/chown)".into(),
                state: CapabilityState::PartiallySupported,
                mechanism: Some("Inferno uses own permission model".into()),
                limitations: vec![
                    "Inferno permissions differ from Unix permissions".into(),
                    "No numeric mode bits".into(),
                ],
                tested: false,
            },
            RuntimeCapability {
                name: "filesystem.symlinks".into(),
                description: "Symbolic links".into(),
                state: CapabilityState::Supported,
                mechanism: Some("Inferno symlinks (symlink)".into()),
                limitations: vec![],
                tested: false,
            },
            RuntimeCapability {
                name: "filesystem.mount".into(),
                description: "Mount filesystems".into(),
                state: CapabilityState::Supported,
                mechanism: Some("Inferno bind/mount operations".into()),
                limitations: vec![],
                tested: false,
            },

            // === Process Execution ===
            RuntimeCapability {
                name: "process.execute".into(),
                description: "Execute programs".into(),
                state: CapabilityState::Supported,
                mechanism: Some("Inferno Dis VM / native execution".into()),
                limitations: vec![
                    "Only Dis bytecode or Inferno native binaries".into(),
                    "Cannot execute Linux ELF binaries".into(),
                ],
                tested: false,
            },
            RuntimeCapability {
                name: "process.linux_binary".into(),
                description: "Execute Linux ELF binaries".into(),
                state: CapabilityState::Unsupported,
                mechanism: None,
                limitations: vec![
                    "Inferno cannot execute Linux binaries".into(),
                    "No Linux syscall emulation".into(),
                ],
                tested: false,
            },
            RuntimeCapability {
                name: "process.fork".into(),
                description: "Fork/process creation".into(),
                state: CapabilityState::Supported,
                mechanism: Some("Inferno rfork".into()),
                limitations: vec![
                    "Inferno rfork semantics differ from Unix fork".into(),
                ],
                tested: false,
            },
            RuntimeCapability {
                name: "process.signals".into(),
                description: "Unix signal handling".into(),
                state: CapabilityState::Unsupported,
                mechanism: None,
                limitations: vec![
                    "Inferno does not have Unix signals".into(),
                    "Uses note mechanism instead".into(),
                ],
                tested: false,
            },

            // === Shell / CLI ===
            RuntimeCapability {
                name: "shell.interactive".into(),
                description: "Interactive shell".into(),
                state: CapabilityState::Supported,
                mechanism: Some("Inferno sh (rc-like shell)".into()),
                limitations: vec![
                    "Different syntax from bash/sh".into(),
                ],
                tested: false,
            },
            RuntimeCapability {
                name: "shell.bash_compat".into(),
                description: "Bash-compatible shell scripting".into(),
                state: CapabilityState::Unsupported,
                mechanism: None,
                limitations: vec![
                    "Inferno sh is not bash-compatible".into(),
                    "Different syntax, no bashisms".into(),
                ],
                tested: false,
            },
            RuntimeCapability {
                name: "shell.pipe".into(),
                description: "Pipe commands together".into(),
                state: CapabilityState::Supported,
                mechanism: Some("Inferno pipe support".into()),
                limitations: vec![],
                tested: false,
            },

            // === Environment ===
            RuntimeCapability {
                name: "env.variables".into(),
                description: "Environment variables".into(),
                state: CapabilityState::Emulable,
                mechanism: Some("Inferno namespace bindings".into()),
                limitations: vec![
                    "Inferno uses namespace, not env vars".into(),
                    "Translation layer required".into(),
                ],
                tested: false,
            },
            RuntimeCapability {
                name: "env.path".into(),
                description: "PATH-style command lookup".into(),
                state: CapabilityState::Emulable,
                mechanism: Some("Inferno namespace + path binding".into()),
                limitations: vec![],
                tested: false,
            },

            // === Networking ===
            RuntimeCapability {
                name: "network.tcp".into(),
                description: "TCP networking".into(),
                state: CapabilityState::Supported,
                mechanism: Some("Inferno native TCP/IP stack".into()),
                limitations: vec![],
                tested: false,
            },
            RuntimeCapability {
                name: "network.udp".into(),
                description: "UDP networking".into(),
                state: CapabilityState::Supported,
                mechanism: Some("Inferno native UDP stack".into()),
                limitations: vec![],
                tested: false,
            },
            RuntimeCapability {
                name: "network.dns".into(),
                description: "DNS resolution".into(),
                state: CapabilityState::Supported,
                mechanism: Some("Inferno dns resolver".into()),
                limitations: vec![],
                tested: false,
            },
            RuntimeCapability {
                name: "network.sockets".into(),
                description: "BSD-style sockets API".into(),
                state: CapabilityState::Unsupported,
                mechanism: None,
                limitations: vec![
                    "Inferno does not have BSD socket API".into(),
                    "Uses its own network model".into(),
                ],
                tested: false,
            },

            // === IPC ===
            RuntimeCapability {
                name: "ipc.pipe".into(),
                description: "Pipe-based IPC".into(),
                state: CapabilityState::Supported,
                mechanism: Some("Inferno pipes".into()),
                limitations: vec![],
                tested: false,
            },
            RuntimeCapability {
                name: "ipc.shared_memory".into(),
                description: "Shared memory".into(),
                state: CapabilityState::Unsupported,
                mechanism: None,
                limitations: vec![
                    "Inferno does not have POSIX shared memory".into(),
                ],
                tested: false,
            },
            RuntimeCapability {
                name: "ipc.9p".into(),
                description: "9P/Styx-based IPC".into(),
                state: CapabilityState::Supported,
                mechanism: Some("Native 9P protocol".into()),
                limitations: vec![],
                tested: false,
            },

            // === Build Tools ===
            RuntimeCapability {
                name: "build.make".into(),
                description: "make/build systems".into(),
                state: CapabilityState::PartiallySupported,
                mechanism: Some("Inferno mk build system".into()),
                limitations: vec![
                    "GNU make not available".into(),
                    "Inferno mk uses different syntax".into(),
                ],
                tested: false,
            },
            RuntimeCapability {
                name: "build.compiler".into(),
                description: "C/C++ compilers".into(),
                state: CapabilityState::PartiallySupported,
                mechanism: Some("Inferno Limbo compiler".into()),
                limitations: vec![
                    "No GCC".into(),
                    "Only Limbo/Dis compilation".into(),
                ],
                tested: false,
            },

            // === Package Management ===
            RuntimeCapability {
                name: "package.apk".into(),
                description: "Alpine apk package manager".into(),
                state: CapabilityState::Unsupported,
                mechanism: None,
                limitations: vec![
                    "Inferno does not have apk".into(),
                    "Inferno has its own package model".into(),
                ],
                tested: false,
            },
            RuntimeCapability {
                name: "package.pip".into(),
                description: "Python pip package manager".into(),
                state: CapabilityState::Unsupported,
                mechanism: None,
                limitations: vec![
                    "No Python runtime in Inferno".into(),
                ],
                tested: false,
            },
            RuntimeCapability {
                name: "package.npm".into(),
                description: "Node.js npm package manager".into(),
                state: CapabilityState::Unsupported,
                mechanism: None,
                limitations: vec![
                    "No Node.js runtime in Inferno".into(),
                ],
                tested: false,
            },

            // === PlazaVM-specific ===
            RuntimeCapability {
                name: "plaza.workspace".into(),
                description: "PlazaVM workspace operations".into(),
                state: CapabilityState::Supported,
                mechanism: Some("Inferno 9P workspace server".into()),
                limitations: vec![],
                tested: false,
            },
            RuntimeCapability {
                name: "plaza.storage".into(),
                description: "PlazaVM persistent storage".into(),
                state: CapabilityState::Supported,
                mechanism: Some("NBD + Inferno block device".into()),
                limitations: vec![],
                tested: false,
            },
            RuntimeCapability {
                name: "plaza.isolation".into(),
                description: "Workspace isolation".into(),
                state: CapabilityState::Supported,
                mechanism: Some("Inferno namespace isolation".into()),
                limitations: vec![],
                tested: false,
            },
        ];

        let mut metadata = HashMap::new();
        metadata.insert("version".into(), "1.0.0".into());
        metadata.insert(
            "description".into(),
            "Capability profile for Inferno OS guest runtime".into(),
        );
        metadata.insert("runtime".into(), "inferno".into());

        Self {
            runtime: "inferno".into(),
            capabilities,
            metadata,
        }
    }

    /// Get a capability by name.
    pub fn get_capability(&self, name: &str) -> Option<&RuntimeCapability> {
        self.capabilities.iter().find(|c| c.name == name)
    }

    /// Check if a specific capability is supported or emulable.
    pub fn is_capable(&self, name: &str) -> bool {
        self.get_capability(name)
            .map(|c| matches!(c.state, CapabilityState::Supported | CapabilityState::Emulable))
            .unwrap_or(false)
    }

    /// Get all capabilities in a given state.
    pub fn capabilities_in_state(&self, state: CapabilityState) -> Vec<&RuntimeCapability> {
        self.capabilities.iter().filter(|c| c.state == state).collect()
    }

    /// Get a summary of capability states.
    pub fn summary(&self) -> CapabilitySummary {
        let mut summary = CapabilitySummary::default();
        for cap in &self.capabilities {
            match cap.state {
                CapabilityState::Supported => summary.supported += 1,
                CapabilityState::Emulable => summary.emulable += 1,
                CapabilityState::PartiallySupported => summary.partially_supported += 1,
                CapabilityState::Unsupported => summary.unsupported += 1,
                CapabilityState::Blocked => summary.blocked += 1,
            }
            if cap.tested {
                summary.tested += 1;
            }
        }
        summary.total = self.capabilities.len();
        summary
    }

    /// Generate a human-readable report.
    pub fn report(&self) -> String {
        let summary = self.summary();
        let mut report = format!(
            "Inferno Runtime Capability Report\n\
             ================================\n\
             Total capabilities: {}\n\
             Supported:          {}\n\
             Emulable:           {}\n\
             Partially Supported:{}\n\
             Unsupported:        {}\n\
             Blocked:            {}\n\
             Tested:             {}/{}\n\n\
             Capabilities:\n",
            summary.total,
            summary.supported,
            summary.emulable,
            summary.partially_supported,
            summary.unsupported,
            summary.blocked,
            summary.tested,
            summary.total,
        );

        for cap in &self.capabilities {
            report.push_str(&format!(
                "  {} {} - {}\n",
                cap.state, cap.name, cap.description
            ));
            if !cap.limitations.is_empty() {
                for lim in &cap.limitations {
                    report.push_str(&format!("    ⚠️  {}\n", lim));
                }
            }
        }

        report
    }
}

/// Summary of capability states.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CapabilitySummary {
    pub total: usize,
    pub supported: usize,
    pub emulable: usize,
    pub partially_supported: usize,
    pub unsupported: usize,
    pub blocked: usize,
    pub tested: usize,
}

/// Linux ↔ Inferno compatibility layer.
///
/// Translates Linux-oriented operations to Inferno primitives.
/// This is NOT a complete Linux emulation - it provides only the
/// specific operations that PlazaVM workloads actually need.
pub struct LinuxCompatibilityLayer {
    capability_profile: CapabilityProfile,
}

impl LinuxCompatibilityLayer {
    /// Create a new compatibility layer with the default profile.
    pub fn new() -> Self {
        Self {
            capability_profile: CapabilityProfile::inferno_default(),
        }
    }

    /// Check if a Linux operation is supported in Inferno.
    pub fn is_supported(&self, operation: &str) -> bool {
        self.capability_profile.is_capable(operation)
    }

    /// Get the capability profile.
    pub fn profile(&self) -> &CapabilityProfile {
        &self.capability_profile
    }

    /// Translate a Linux-style file path to Inferno namespace path.
    ///
    /// This is a key function for the compatibility layer.
    /// Linux paths like `/home/user/project` become Inferno namespace
    /// paths like `/workspace/project`.
    pub fn translate_path(&self, linux_path: &str) -> String {
        // Map Linux filesystem paths to Inferno namespace paths
        if linux_path.starts_with("/workspace/") {
            // Already a workspace path - use directly in Inferno namespace
            linux_path.to_string()
        } else if linux_path.starts_with("/home/") {
            // Map /home/user/... to /workspace/...
            let relative = linux_path
                .splitn(4, '/')
                .nth(3)
                .unwrap_or("");
            if relative.is_empty() {
                "/workspace".to_string()
            } else {
                format!("/workspace/{}", relative)
            }
        } else if linux_path.starts_with("/tmp/") {
            // Map /tmp/ to Inferno's /tmp
            linux_path.to_string()
        } else if linux_path.starts_with("/usr/") {
            // Map /usr/ to Inferno's /usr (or /Dis for Dis binaries)
            linux_path.to_string()
        } else {
            // Unknown path - return as-is with a warning
            linux_path.to_string()
        }
    }

    /// Translate a Linux command to Inferno equivalent.
    ///
    /// Returns (inferno_command, is_direct_translation).
    /// If is_direct_translation is false, the command cannot be
    /// directly executed and requires manual intervention.
    pub fn translate_command(&self, linux_cmd: &str) -> (String, bool) {
        let parts: Vec<&str> = linux_cmd.split_whitespace().collect();
        if parts.is_empty() {
            return (linux_cmd.to_string(), false);
        }

        match parts[0] {
            // Direct equivalents (return full command with arguments)
            "ls" | "cat" | "echo" | "mkdir" | "rm" | "cp" | "mv" |
            "chmod" | "chown" | "ps" | "kill" | "env" | "which" |
            "wc" | "head" | "tail" | "grep" | "find" | "sort" |
            "uniq" | "diff" | "sed" | "awk" => (linux_cmd.to_string(), true),
            "pwd" => ("cd".to_string(), true),

            // No direct equivalent
            "sudo" => (
                "# Inferno does not use sudo - all operations run as system".to_string(),
                false,
            ),
            "apt" | "apt-get" | "apk" | "yum" | "dnf" | "pacman" => (
                "# Inferno uses its own package management".to_string(),
                false,
            ),
            "systemctl" | "service" => (
                "# Inferno does not have systemd - services are managed via namespace".to_string(),
                false,
            ),
            "docker" | "podman" | "kubectl" => (
                "# Container tools not available in Inferno guest".to_string(),
                false,
            ),
            "python" | "python3" | "pip" | "pip3" => (
                "# Python not available in Inferno guest".to_string(),
                false,
            ),
            "node" | "npm" | "npx" | "yarn" | "pnpm" => (
                "# Node.js not available in Inferno guest".to_string(),
                false,
            ),
            "java" | "javac" | "gradle" | "mvn" => (
                "# JVM not available in Inferno guest".to_string(),
                false,
            ),
            "gcc" | "g++" | "make" | "cmake" => (
                "# C/C++ build tools not available in Inferno guest".to_string(),
                false,
            ),
            "git" => (
                "# Git not available in Inferno guest (use PlazaVM workspace features)".to_string(),
                false,
            ),
            "curl" | "wget" => (
                "# Use Inferno's httpget for HTTP requests".to_string(),
                false,
            ),

            // Default: return as-is with a note
            _ => (linux_cmd.to_string(), true),
        }
    }
}

impl Default for LinuxCompatibilityLayer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capability_profile_has_capabilities() {
        let profile = CapabilityProfile::inferno_default();
        assert!(!profile.capabilities.is_empty());
    }

    #[test]
    fn capability_profile_summary() {
        let profile = CapabilityProfile::inferno_default();
        let summary = profile.summary();
        assert!(summary.total > 0);
        assert!(summary.supported > 0);
        assert!(summary.unsupported > 0);
    }

    #[test]
    fn capability_profile_get_capability() {
        let profile = CapabilityProfile::inferno_default();
        let cap = profile.get_capability("filesystem.read");
        assert!(cap.is_some());
        assert_eq!(cap.unwrap().state, CapabilityState::Supported);
    }

    #[test]
    fn capability_profile_is_capable() {
        let profile = CapabilityProfile::inferno_default();
        assert!(profile.is_capable("filesystem.read"));
        assert!(profile.is_capable("network.tcp"));
        assert!(!profile.is_capable("process.linux_binary"));
    }

    #[test]
    fn capability_profile_report() {
        let profile = CapabilityProfile::inferno_default();
        let report = profile.report();
        assert!(report.contains("Inferno Runtime Capability Report"));
        assert!(report.contains("Supported:"));
    }

    #[test]
    fn compatibility_layer_path_translation() {
        let layer = LinuxCompatibilityLayer::new();
        assert_eq!(layer.translate_path("/workspace/foo"), "/workspace/foo");
        assert_eq!(layer.translate_path("/home/user/project"), "/workspace/project");
        assert_eq!(layer.translate_path("/tmp/test"), "/tmp/test");
    }

    #[test]
    fn compatibility_layer_command_translation() {
        let layer = LinuxCompatibilityLayer::new();
        let (cmd, direct) = layer.translate_command("ls -la");
        assert_eq!(cmd, "ls -la");
        assert!(direct);

        let (cmd, direct) = layer.translate_command("python3 script.py");
        assert!(cmd.contains("Python not available"));
        assert!(!direct);
    }

    #[test]
    fn capability_state_display() {
        assert_eq!(CapabilityState::Supported.to_string(), "✅ Supported");
        assert_eq!(CapabilityState::Unsupported.to_string(), "❌ Unsupported");
    }
}
