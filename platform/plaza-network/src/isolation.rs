use plaza_foundation::core::{PlazaError, PlazaResult};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Isolation policy for a workspace's network.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum IsolationPolicy {
    /// Fully isolated — no network access to other workspaces or host.
    Strict,
    /// Isolated from other workspaces but allowed to reach the internet.
    InternetOnly,
    /// Can communicate with workspaces in the same group.
    GroupShared,
    /// Full network access (development only, never default).
    Permissive,
}

impl std::fmt::Display for IsolationPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Strict => write!(f, "strict"),
            Self::InternetOnly => write!(f, "internet_only"),
            Self::GroupShared => write!(f, "group_shared"),
            Self::Permissive => write!(f, "permissive"),
        }
    }
}

/// The network isolation state for a workspace.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceIsolation {
    pub workspace_id: String,
    pub policy: IsolationPolicy,
    pub group: Option<String>,
    pub allowed_inbound: HashSet<String>,
    pub allowed_outbound: HashSet<String>,
    pub blocked_ports: Vec<u16>,
    pub created_at: u64,
}

/// A record of a denied cross-workspace access attempt.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IsolationViolation {
    pub source_workspace: String,
    pub target_workspace: String,
    pub violation_type: String,
    pub timestamp: u64,
}

/// Enforces per-workspace network isolation.
#[derive(Clone)]
pub struct NetworkIsolation {
    isolations: Arc<RwLock<HashMap<String, WorkspaceIsolation>>>,
    violations: Arc<RwLock<Vec<IsolationViolation>>>,
}

impl Default for NetworkIsolation {
    fn default() -> Self {
        Self::new()
    }
}

impl NetworkIsolation {
    pub fn new() -> Self {
        Self {
            isolations: Arc::new(RwLock::new(HashMap::new())),
            violations: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Apply an isolation policy to a workspace.
    pub async fn isolate(
        &self,
        workspace_id: &str,
        policy: IsolationPolicy,
        group: Option<String>,
    ) -> PlazaResult<WorkspaceIsolation> {
        let isolation = WorkspaceIsolation {
            workspace_id: workspace_id.to_string(),
            policy,
            group,
            allowed_inbound: HashSet::new(),
            allowed_outbound: HashSet::new(),
            blocked_ports: Vec::new(),
            created_at: timestamp_now(),
        };

        let mut isolations = self.isolations.write().await;
        isolations.insert(workspace_id.to_string(), isolation.clone());
        tracing::info!(
            "Applied '{}' isolation policy to workspace '{}'",
            isolation.policy,
            workspace_id
        );
        Ok(isolation)
    }

    /// Check if workspace A is allowed to communicate with workspace B.
    pub async fn can_communicate(
        &self,
        source: &str,
        target: &str,
    ) -> PlazaResult<bool> {
        // Same workspace can always talk to itself
        if source == target {
            return Ok(true);
        }

        let isolations = self.isolations.read().await;
        let source_iso = isolations.get(source);

        match source_iso {
            None => {
                // No isolation configured — deny by default
                self.record_violation(source, target, "no_isolation_configured")
                    .await;
                Ok(false)
            }
            Some(iso) => match iso.policy {
                IsolationPolicy::Strict => {
                    self.record_violation(source, target, "strict_isolation")
                        .await;
                    Ok(false)
                }
                IsolationPolicy::InternetOnly => {
                    // Can only reach the internet, not other workspaces
                    self.record_violation(source, target, "internet_only_policy")
                        .await;
                    Ok(false)
                }
                IsolationPolicy::GroupShared => {
                    if let Some(target_iso) = isolations.get(target) {
                        let same_group =
                            iso.group.is_some() && iso.group == target_iso.group;
                        if !same_group {
                            self.record_violation(source, target, "different_group")
                                .await;
                        }
                        Ok(same_group)
                    } else {
                        self.record_violation(source, target, "target_not_isolated")
                            .await;
                        Ok(false)
                    }
                }
                IsolationPolicy::Permissive => Ok(true),
            },
        }
    }

    /// Check if a workspace can access a specific external address.
    pub async fn can_reach_external(
        &self,
        workspace_id: &str,
        address: &str,
    ) -> PlazaResult<bool> {
        let isolations = self.isolations.read().await;
        match isolations.get(workspace_id) {
            None => Ok(false),
            Some(iso) => match iso.policy {
                IsolationPolicy::Strict => Ok(false),
                IsolationPolicy::InternetOnly | IsolationPolicy::GroupShared => {
                    // Allow unless explicitly blocked
                    Ok(!iso.blocked_ports.is_empty()
                        || !iso.allowed_outbound.contains(address))
                }
                IsolationPolicy::Permissive => Ok(true),
            },
        }
    }

    /// Add an explicit allow-rule for inbound communication.
    pub async fn allow_inbound(
        &self,
        workspace_id: &str,
        source_workspace: &str,
    ) -> PlazaResult<()> {
        let mut isolations = self.isolations.write().await;
        let iso = isolations
            .get_mut(workspace_id)
            .ok_or_else(|| PlazaError::Network(format!(
                "no isolation configured for workspace '{}'",
                workspace_id
            )))?;
        iso.allowed_inbound.insert(source_workspace.to_string());
        Ok(())
    }

    /// Add an explicit allow-rule for outbound communication.
    pub async fn allow_outbound(
        &self,
        workspace_id: &str,
        target: &str,
    ) -> PlazaResult<()> {
        let mut isolations = self.isolations.write().await;
        let iso = isolations
            .get_mut(workspace_id)
            .ok_or_else(|| PlazaError::Network(format!(
                "no isolation configured for workspace '{}'",
                workspace_id
            )))?;
        iso.allowed_outbound.insert(target.to_string());
        Ok(())
    }

    /// Block a specific port for a workspace.
    pub async fn block_port(
        &self,
        workspace_id: &str,
        port: u16,
    ) -> PlazaResult<()> {
        let mut isolations = self.isolations.write().await;
        let iso = isolations
            .get_mut(workspace_id)
            .ok_or_else(|| PlazaError::Network(format!(
                "no isolation configured for workspace '{}'",
                workspace_id
            )))?;
        if !iso.blocked_ports.contains(&port) {
            iso.blocked_ports.push(port);
        }
        Ok(())
    }

    /// Get the isolation state for a workspace.
    pub async fn get_isolation(
        &self,
        workspace_id: &str,
    ) -> PlazaResult<WorkspaceIsolation> {
        let isolations = self.isolations.read().await;
        isolations
            .get(workspace_id)
            .cloned()
            .ok_or_else(|| PlazaError::Network(format!(
                "no isolation configured for workspace '{}'",
                workspace_id
            )))
    }

    /// List all isolation records.
    pub async fn list_isolations(&self) -> PlazaResult<Vec<WorkspaceIsolation>> {
        let isolations = self.isolations.read().await;
        Ok(isolations.values().cloned().collect())
    }

    /// Record an isolation violation.
    async fn record_violation(
        &self,
        source: &str,
        target: &str,
        violation_type: &str,
    ) {
        let violation = IsolationViolation {
            source_workspace: source.to_string(),
            target_workspace: target.to_string(),
            violation_type: violation_type.to_string(),
            timestamp: timestamp_now(),
        };
        let mut violations = self.violations.write().await;
        violations.push(violation);
        tracing::warn!(
            "Isolation violation: workspace '{}' attempted to reach workspace '{}' ({})",
            source,
            target,
            violation_type
        );
    }

    /// Get all recorded isolation violations.
    pub async fn get_violations(&self) -> PlazaResult<Vec<IsolationViolation>> {
        let violations = self.violations.read().await;
        Ok(violations.clone())
    }

    /// Remove all isolation records for a workspace.
    pub async fn remove_isolation(&self, workspace_id: &str) -> PlazaResult<bool> {
        let mut isolations = self.isolations.write().await;
        Ok(isolations.remove(workspace_id).is_some())
    }
}

fn timestamp_now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn strict_isolation_blocks_communication() {
        let ni = NetworkIsolation::new();
        ni.isolate("ws-a", IsolationPolicy::Strict, None)
            .await
            .unwrap();
        ni.isolate("ws-b", IsolationPolicy::Strict, None)
            .await
            .unwrap();

        assert!(!ni.can_communicate("ws-a", "ws-b").await.unwrap());
    }

    #[tokio::test]
    async fn same_workspace_always_communicates() {
        let ni = NetworkIsolation::new();
        ni.isolate("ws-a", IsolationPolicy::Strict, None)
            .await
            .unwrap();
        assert!(ni.can_communicate("ws-a", "ws-a").await.unwrap());
    }

    #[tokio::test]
    async fn group_shared_within_same_group() {
        let ni = NetworkIsolation::new();
        ni.isolate("ws-a", IsolationPolicy::GroupShared, Some("team-1".into()))
            .await
            .unwrap();
        ni.isolate("ws-b", IsolationPolicy::GroupShared, Some("team-1".into()))
            .await
            .unwrap();
        ni.isolate("ws-c", IsolationPolicy::GroupShared, Some("team-2".into()))
            .await
            .unwrap();

        assert!(ni.can_communicate("ws-a", "ws-b").await.unwrap());
        assert!(!ni.can_communicate("ws-a", "ws-c").await.unwrap());
    }

    #[tokio::test]
    async fn permissive_allows_all() {
        let ni = NetworkIsolation::new();
        ni.isolate("ws-a", IsolationPolicy::Permissive, None)
            .await
            .unwrap();
        ni.isolate("ws-b", IsolationPolicy::Strict, None)
            .await
            .unwrap();

        assert!(ni.can_communicate("ws-a", "ws-b").await.unwrap());
    }

    #[tokio::test]
    async fn violations_recorded() {
        let ni = NetworkIsolation::new();
        ni.isolate("ws-a", IsolationPolicy::Strict, None)
            .await
            .unwrap();
        ni.isolate("ws-b", IsolationPolicy::Strict, None)
            .await
            .unwrap();

        let _ = ni.can_communicate("ws-a", "ws-b").await;
        let violations = ni.get_violations().await.unwrap();
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].source_workspace, "ws-a");
        assert_eq!(violations[0].target_workspace, "ws-b");
    }

    #[tokio::test]
    async fn remove_isolation() {
        let ni = NetworkIsolation::new();
        ni.isolate("ws-a", IsolationPolicy::Strict, None)
            .await
            .unwrap();
        assert!(ni.remove_isolation("ws-a").await.unwrap());
        assert!(ni.get_isolation("ws-a").await.is_err());
    }
}
