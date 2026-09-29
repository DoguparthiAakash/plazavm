use plaza_foundation::core::{PlazaError, PlazaResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Protocol for port forwarding rules.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PortProtocol {
    Tcp,
    Udp,
}

impl std::fmt::Display for PortProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Tcp => write!(f, "tcp"),
            Self::Udp => write!(f, "udp"),
        }
    }
}

/// Status of a port forwarding rule.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ForwardStatus {
    Active,
    Paused,
    Failed,
    Closed,
}

/// A single port forwarding rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortForwardRule {
    pub id: String,
    pub workspace_id: String,
    pub host_port: u16,
    pub guest_port: u16,
    pub protocol: PortProtocol,
    pub status: ForwardStatus,
    pub description: Option<String>,
    pub created_at: u64,
    pub bytes_forwarded: u64,
}

/// Summary of port forwarding for a workspace.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortForwardSummary {
    pub workspace_id: String,
    pub total_rules: usize,
    pub active_rules: usize,
    pub total_bytes_forwarded: u64,
    pub rules: Vec<PortForwardRule>,
}

/// Manages host↔guest port forwarding for workspaces.
#[derive(Clone)]
pub struct PortForwarder {
    rules: Arc<RwLock<HashMap<String, PortForwardRule>>>,
}

impl Default for PortForwarder {
    fn default() -> Self {
        Self::new()
    }
}

impl PortForwarder {
    pub fn new() -> Self {
        Self {
            rules: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create a port forwarding rule for a workspace.
    pub async fn forward_port(
        &self,
        workspace_id: &str,
        host_port: u16,
        guest_port: u16,
        protocol: PortProtocol,
        description: Option<String>,
    ) -> PlazaResult<PortForwardRule> {
        // Check for host port conflicts
        {
            let rules = self.rules.read().await;
            for existing in rules.values() {
                if existing.host_port == host_port
                    && existing.protocol == protocol
                    && existing.status == ForwardStatus::Active
                    && existing.workspace_id != workspace_id
                {
                    return Err(PlazaError::Network(format!(
                        "host port {} ({}) is already forwarded by workspace '{}'",
                        host_port, protocol, existing.workspace_id
                    )));
                }
            }
        }

        let rule_id = format!("{}-{}-{}", workspace_id, host_port, guest_port);
        let rule = PortForwardRule {
            id: rule_id.clone(),
            workspace_id: workspace_id.to_string(),
            host_port,
            guest_port,
            protocol,
            status: ForwardStatus::Active,
            description,
            created_at: timestamp_now(),
            bytes_forwarded: 0,
        };

        let mut rules = self.rules.write().await;
        rules.insert(rule_id.clone(), rule.clone());

        tracing::info!(
            "Port forward created: host:{} → guest:{} ({}) for workspace '{}'",
            host_port,
            guest_port,
            rule.protocol,
            workspace_id
        );
        Ok(rule)
    }

    /// Remove a port forwarding rule by ID.
    pub async fn remove_rule(&self, rule_id: &str) -> PlazaResult<bool> {
        let mut rules = self.rules.write().await;
        if let Some(mut rule) = rules.remove(rule_id) {
            rule.status = ForwardStatus::Closed;
            tracing::info!(
                "Port forward removed: host:{} → guest:{} for workspace '{}'",
                rule.host_port,
                rule.guest_port,
                rule.workspace_id
            );
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Pause a port forwarding rule (stop forwarding without removing it).
    pub async fn pause_rule(&self, rule_id: &str) -> PlazaResult<()> {
        let mut rules = self.rules.write().await;
        let rule = rules
            .get_mut(rule_id)
            .ok_or_else(|| PlazaError::Network(format!("rule '{}' not found", rule_id)))?;
        rule.status = ForwardStatus::Paused;
        tracing::info!("Port forward '{}' paused", rule_id);
        Ok(())
    }

    /// Resume a paused port forwarding rule.
    pub async fn resume_rule(&self, rule_id: &str) -> PlazaResult<()> {
        let mut rules = self.rules.write().await;
        let rule = rules
            .get_mut(rule_id)
            .ok_or_else(|| PlazaError::Network(format!("rule '{}' not found", rule_id)))?;
        rule.status = ForwardStatus::Active;
        tracing::info!("Port forward '{}' resumed", rule_id);
        Ok(())
    }

    /// List all port forwarding rules for a workspace.
    pub async fn list_rules(&self, workspace_id: &str) -> PlazaResult<Vec<PortForwardRule>> {
        let rules = self.rules.read().await;
        Ok(rules
            .values()
            .filter(|r| r.workspace_id == workspace_id)
            .cloned()
            .collect())
    }

    /// Get a summary of port forwarding for a workspace.
    pub async fn get_summary(&self, workspace_id: &str) -> PlazaResult<PortForwardSummary> {
        let all_rules = self.list_rules(workspace_id).await?;
        let active_rules = all_rules.iter().filter(|r| r.status == ForwardStatus::Active).count();
        let total_bytes = all_rules.iter().map(|r| r.bytes_forwarded).sum();

        Ok(PortForwardSummary {
            workspace_id: workspace_id.to_string(),
            total_rules: all_rules.len(),
            active_rules,
            total_bytes_forwarded: total_bytes,
            rules: all_rules,
        })
    }

    /// Remove all port forwarding rules for a workspace.
    pub async fn remove_all_for_workspace(&self, workspace_id: &str) -> PlazaResult<u32> {
        let mut rules = self.rules.write().await;
        let before = rules.len();
        rules.retain(|_, r| r.workspace_id != workspace_id);
        let removed = (before - rules.len()) as u32;
        tracing::info!(
            "Removed {} port forward rules for workspace '{}'",
            removed,
            workspace_id
        );
        Ok(removed)
    }

    /// Check if a specific host port is available.
    pub async fn is_port_available(&self, host_port: u16, protocol: &PortProtocol) -> PlazaResult<bool> {
        let rules = self.rules.read().await;
        let taken = rules.values().any(|r| {
            r.host_port == host_port && r.protocol == *protocol && r.status == ForwardStatus::Active
        });
        Ok(!taken)
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
    async fn create_forward_rule() {
        let pf = PortForwarder::new();
        let rule = pf
            .forward_port("ws-1", 8080, 80, PortProtocol::Tcp, Some("web".into()))
            .await
            .unwrap();
        assert_eq!(rule.host_port, 8080);
        assert_eq!(rule.guest_port, 80);
        assert_eq!(rule.status, ForwardStatus::Active);
    }

    #[tokio::test]
    async fn host_port_conflict_detected() {
        let pf = PortForwarder::new();
        pf.forward_port("ws-1", 3000, 3000, PortProtocol::Tcp, None)
            .await
            .unwrap();
        let err = pf
            .forward_port("ws-2", 3000, 3000, PortProtocol::Tcp, None)
            .await;
        assert!(err.is_err());
    }

    #[tokio::test]
    async fn pause_and_resume() {
        let pf = PortForwarder::new();
        let rule = pf
            .forward_port("ws-1", 9090, 90, PortProtocol::Tcp, None)
            .await
            .unwrap();
        pf.pause_rule(&rule.id).await.unwrap();
        let rules = pf.list_rules("ws-1").await.unwrap();
        assert_eq!(rules[0].status, ForwardStatus::Paused);

        pf.resume_rule(&rule.id).await.unwrap();
        let rules = pf.list_rules("ws-1").await.unwrap();
        assert_eq!(rules[0].status, ForwardStatus::Active);
    }

    #[tokio::test]
    async fn remove_all_for_workspace() {
        let pf = PortForwarder::new();
        pf.forward_port("ws-1", 8080, 80, PortProtocol::Tcp, None)
            .await
            .unwrap();
        pf.forward_port("ws-1", 8443, 443, PortProtocol::Tcp, None)
            .await
            .unwrap();
        pf.forward_port("ws-2", 3000, 3000, PortProtocol::Tcp, None)
            .await
            .unwrap();

        let removed = pf.remove_all_for_workspace("ws-1").await.unwrap();
        assert_eq!(removed, 2);
        assert!(pf.list_rules("ws-1").await.unwrap().is_empty());
        assert_eq!(pf.list_rules("ws-2").await.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn port_availability_check() {
        let pf = PortForwarder::new();
        assert!(pf.is_port_available(8080, &PortProtocol::Tcp).await.unwrap());
        pf.forward_port("ws-1", 8080, 80, PortProtocol::Tcp, None)
            .await
            .unwrap();
        assert!(!pf.is_port_available(8080, &PortProtocol::Tcp).await.unwrap());
        // Different protocol on same port is fine
        assert!(pf.is_port_available(8080, &PortProtocol::Udp).await.unwrap());
    }

    #[tokio::test]
    async fn summary() {
        let pf = PortForwarder::new();
        pf.forward_port("ws-1", 8080, 80, PortProtocol::Tcp, None)
            .await
            .unwrap();
        pf.forward_port("ws-1", 8443, 443, PortProtocol::Tcp, None)
            .await
            .unwrap();

        let summary = pf.get_summary("ws-1").await.unwrap();
        assert_eq!(summary.total_rules, 2);
        assert_eq!(summary.active_rules, 2);
    }
}
