//! Workspace resource priority levels.

use serde::{Deserialize, Serialize};

/// Priority classification for resource scheduling and auto-suspension.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum WorkspacePriority {
    /// Critical workload — never auto-suspend under memory pressure.
    Critical,
    /// High priority — last to be suspended.
    High,
    /// Normal priority (default).
    #[default]
    Normal,
    /// Low priority — first candidate for auto-suspension.
    Low,
    /// Background priority — best-effort resource allocation.
    Background,
}

impl std::fmt::Display for WorkspacePriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Critical => write!(f, "critical"),
            Self::High => write!(f, "high"),
            Self::Normal => write!(f, "normal"),
            Self::Low => write!(f, "low"),
            Self::Background => write!(f, "background"),
        }
    }
}

