//! Security traits and types.
//!
//! This module lives in `plaza-core` during Phase 1. It will be extracted
//! into a dedicated `plaza-security` crate when the module grows to
//! require its own dependency set (e.g., keyring integration, RBAC).

use crate::PlazaResult;
use serde::{Deserialize, Serialize};

/// Isolation level for a workspace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum IsolationLevel {
    /// Shared filesystem, minimal separation.
    Minimal,
    /// Standard container/VM isolation (default).
    #[default]
    Standard,
    /// Maximum isolation — read-only root, no privileged ops, strict seccomp.
    Strict,
}

impl std::fmt::Display for IsolationLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Minimal => write!(f, "minimal"),
            Self::Standard => write!(f, "standard"),
            Self::Strict => write!(f, "strict"),
        }
    }
}

/// Security policy attached to a workspace spec.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPolicy {
    /// Isolation level.
    pub isolation: IsolationLevel,
    /// Mount the root filesystem read-only.
    #[serde(default)]
    pub read_only_root: bool,
    /// Allow privileged operations inside the workspace.
    #[serde(default)]
    pub allow_privileged: bool,
}

impl Default for SecurityPolicy {
    fn default() -> Self {
        Self {
            isolation: IsolationLevel::Standard,
            read_only_root: false,
            allow_privileged: false,
        }
    }
}

/// Trait for secret storage backends.
///
/// Phase 1: in-memory or file-based stub.
/// Future: OS keyring, HashiCorp Vault, etc.
pub trait SecretStore: Send + Sync {
    /// Retrieve a secret by key.
    fn get(&self, key: &str) -> PlazaResult<Option<String>>;
    /// Store a secret.
    fn set(&self, key: &str, value: &str) -> PlazaResult<()>;
    /// Delete a secret.
    fn delete(&self, key: &str) -> PlazaResult<()>;
    /// List all secret keys (not values).
    fn list_keys(&self) -> PlazaResult<Vec<String>>;
}

/// Simple in-memory secret store for development/testing.
#[derive(Debug, Default)]
pub struct InMemorySecretStore {
    secrets: std::sync::RwLock<std::collections::HashMap<String, String>>,
}

impl SecretStore for InMemorySecretStore {
    fn get(&self, key: &str) -> PlazaResult<Option<String>> {
        Ok(self.secrets.read().unwrap().get(key).cloned())
    }

    fn set(&self, key: &str, value: &str) -> PlazaResult<()> {
        self.secrets
            .write()
            .unwrap()
            .insert(key.to_string(), value.to_string());
        Ok(())
    }

    fn delete(&self, key: &str) -> PlazaResult<()> {
        self.secrets.write().unwrap().remove(key);
        Ok(())
    }

    fn list_keys(&self) -> PlazaResult<Vec<String>> {
        Ok(self.secrets.read().unwrap().keys().cloned().collect())
    }
}
