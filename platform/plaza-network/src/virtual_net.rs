use plaza_foundation::core::{PlazaError, PlazaResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// A virtual network bridge connecting multiple workspaces.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualBridge {
    pub name: String,
    pub subnet: String,
    pub gateway: String,
    pub dns_servers: Vec<String>,
    pub connected_workspaces: Vec<String>,
    pub created_at: u64,
}

/// A subnet allocation within a bridge.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubnetAllocation {
    pub workspace_id: String,
    pub ip_address: String,
    pub netmask: String,
    pub gateway: String,
    pub bridge_name: String,
}

/// Network statistics for a workspace.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkStats {
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub packets_sent: u64,
    pub packets_received: u64,
    pub errors: u64,
}

/// Manages virtual network bridges, subnets, and per-workspace network allocations.
#[derive(Clone)]
pub struct VirtualNetworkManager {
    bridges: Arc<RwLock<HashMap<String, VirtualBridge>>>,
    allocations: Arc<RwLock<HashMap<String, SubnetAllocation>>>,
    stats: Arc<RwLock<HashMap<String, NetworkStats>>>,
}

impl Default for VirtualNetworkManager {
    fn default() -> Self {
        Self::new()
    }
}

impl VirtualNetworkManager {
    pub fn new() -> Self {
        Self {
            bridges: Arc::new(RwLock::new(HashMap::new())),
            allocations: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create a new virtual network bridge with a given name and subnet.
    pub async fn create_bridge(&self, name: &str, subnet: &str) -> PlazaResult<VirtualBridge> {
        let mut bridges = self.bridges.write().await;
        if bridges.contains_key(name) {
            return Err(PlazaError::Network(format!(
                "bridge '{}' already exists",
                name
            )));
        }

        // Strip CIDR suffix if present: "10.0.0.0/24" → "10.0.0.0"
        let cidrless = subnet.split('/').next().unwrap_or(subnet);
        // Derive prefix by removing the last octet: "10.0.0.0" → "10.0.0"
        let prefix = cidrless.rsplit_once('.').map(|(p, _)| p).unwrap_or(cidrless);
        let gateway = format!("{}.1", prefix);
        let bridge = VirtualBridge {
            name: name.to_string(),
            subnet: subnet.to_string(),
            gateway,
            dns_servers: vec!["1.1.1.1".to_string(), "8.8.8.8".to_string()],
            connected_workspaces: Vec::new(),
            created_at: timestamp_now(),
        };

        bridges.insert(name.to_string(), bridge.clone());
        tracing::info!("Created virtual bridge '{}' on subnet {}", name, subnet);
        Ok(bridge)
    }

    /// Remove a virtual network bridge. Fails if workspaces are still connected.
    pub async fn remove_bridge(&self, name: &str) -> PlazaResult<()> {
        let mut bridges = self.bridges.write().await;
        let bridge = bridges
            .get(name)
            .ok_or_else(|| PlazaError::Network(format!("bridge '{}' not found", name)))?;

        if !bridge.connected_workspaces.is_empty() {
            return Err(PlazaError::Network(format!(
                "bridge '{}' still has {} connected workspaces",
                name,
                bridge.connected_workspaces.len()
            )));
        }

        bridges.remove(name);
        tracing::info!("Removed virtual bridge '{}'", name);
        Ok(())
    }

    /// List all active virtual bridges.
    pub async fn list_bridges(&self) -> PlazaResult<Vec<VirtualBridge>> {
        let bridges = self.bridges.read().await;
        Ok(bridges.values().cloned().collect())
    }

    /// Get details of a specific bridge.
    pub async fn get_bridge(&self, name: &str) -> PlazaResult<VirtualBridge> {
        let bridges = self.bridges.read().await;
        bridges
            .get(name)
            .cloned()
            .ok_or_else(|| PlazaError::Network(format!("bridge '{}' not found", name)))
    }

    /// Connect a workspace to a bridge and allocate an IP address.
    pub async fn connect_workspace(
        &self,
        workspace_id: &str,
        bridge_name: &str,
    ) -> PlazaResult<SubnetAllocation> {
        let mut bridges = self.bridges.write().await;
        let bridge = bridges
            .get_mut(bridge_name)
            .ok_or_else(|| PlazaError::Network(format!("bridge '{}' not found", bridge_name)))?;

        if bridge.connected_workspaces.contains(&workspace_id.to_string()) {
            return Err(PlazaError::Network(format!(
                "workspace '{}' is already connected to bridge '{}'",
                workspace_id, bridge_name
            )));
        }

        // Determine next IP: .2, .3, ... based on gateway prefix
        let prefix = bridge.gateway.rsplit_once('.').map(|(p, _)| p).unwrap_or("10.0.0");
        let idx = bridge.connected_workspaces.len() as u8 + 2;
        let ip = format!("{}.{}", prefix, idx);
        let gateway = bridge.gateway.clone();
        bridge
            .connected_workspaces
            .push(workspace_id.to_string());

        let allocation = SubnetAllocation {
            workspace_id: workspace_id.to_string(),
            ip_address: ip.clone(),
            netmask: "255.255.255.0".to_string(),
            gateway,
            bridge_name: bridge_name.to_string(),
        };

        let mut allocations = self.allocations.write().await;
        allocations.insert(workspace_id.to_string(), allocation.clone());

        tracing::info!(
            "Connected workspace '{}' to bridge '{}' with IP {}",
            workspace_id,
            bridge_name,
            ip
        );
        Ok(allocation)
    }

    /// Disconnect a workspace from its bridge.
    pub async fn disconnect_workspace(&self, workspace_id: &str) -> PlazaResult<()> {
        let mut allocations = self.allocations.write().await;
        let allocation = allocations
            .remove(workspace_id)
            .ok_or_else(|| PlazaError::Network(format!(
                "workspace '{}' has no network allocation",
                workspace_id
            )))?;

        let mut bridges = self.bridges.write().await;
        if let Some(bridge) = bridges.get_mut(&allocation.bridge_name) {
            bridge
                .connected_workspaces
                .retain(|ws| ws != workspace_id);
        }

        tracing::info!(
            "Disconnected workspace '{}' from bridge '{}'",
            workspace_id,
            allocation.bridge_name
        );
        Ok(())
    }

    /// Get the network allocation for a workspace.
    pub async fn get_allocation(&self, workspace_id: &str) -> PlazaResult<SubnetAllocation> {
        let allocations = self.allocations.read().await;
        allocations
            .get(workspace_id)
            .cloned()
            .ok_or_else(|| PlazaError::Network(format!(
                "workspace '{}' has no network allocation",
                workspace_id
            )))
    }

    /// Record network statistics for a workspace.
    pub async fn record_stats(
        &self,
        workspace_id: &str,
        stats: NetworkStats,
    ) -> PlazaResult<()> {
        let mut all_stats = self.stats.write().await;
        all_stats.insert(workspace_id.to_string(), stats);
        Ok(())
    }

    /// Get network statistics for a workspace.
    pub async fn get_stats(&self, workspace_id: &str) -> PlazaResult<NetworkStats> {
        let all_stats = self.stats.read().await;
        Ok(all_stats
            .get(workspace_id)
            .cloned()
            .unwrap_or_default())
    }

    /// Create a default "plaza" bridge for general workspace use.
    pub async fn ensure_default_bridge(&self) -> PlazaResult<VirtualBridge> {
        {
            let bridges = self.bridges.read().await;
            if let Some(bridge) = bridges.get("plaza") {
                return Ok(bridge.clone());
            }
        }
        self.create_bridge("plaza", "10.88.0.0").await
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
    async fn create_and_list_bridge() {
        let mgr = VirtualNetworkManager::new();
        let bridge = mgr.create_bridge("test-bridge", "10.0.0.0").await.unwrap();
        assert_eq!(bridge.name, "test-bridge");
        assert_eq!(bridge.subnet, "10.0.0.0");
        assert_eq!(bridge.gateway, "10.0.0.1");

        let list = mgr.list_bridges().await.unwrap();
        assert_eq!(list.len(), 1);
    }

    #[tokio::test]
    async fn duplicate_bridge_fails() {
        let mgr = VirtualNetworkManager::new();
        mgr.create_bridge("dup", "10.0.0.0").await.unwrap();
        let err = mgr.create_bridge("dup", "10.0.1.0").await;
        assert!(err.is_err());
    }

    #[tokio::test]
    async fn connect_and_disconnect_workspace() {
        let mgr = VirtualNetworkManager::new();
        mgr.create_bridge("net1", "10.0.0.0").await.unwrap();

        let alloc = mgr.connect_workspace("ws-1", "net1").await.unwrap();
        assert_eq!(alloc.ip_address, "10.0.0.2");
        assert_eq!(alloc.bridge_name, "net1");

        let alloc2 = mgr.connect_workspace("ws-2", "net1").await.unwrap();
        assert_eq!(alloc2.ip_address, "10.0.0.3");

        mgr.disconnect_workspace("ws-1").await.unwrap();
        let bridge = mgr.get_bridge("net1").await.unwrap();
        assert_eq!(bridge.connected_workspaces.len(), 1);
    }

    #[tokio::test]
    async fn remove_bridge_with_workspaces_fails() {
        let mgr = VirtualNetworkManager::new();
        mgr.create_bridge("locked", "10.0.0.0").await.unwrap();
        mgr.connect_workspace("ws-1", "locked").await.unwrap();
        assert!(mgr.remove_bridge("locked").await.is_err());
    }

    #[tokio::test]
    async fn ensure_default_bridge_is_idempotent() {
        let mgr = VirtualNetworkManager::new();
        let b1 = mgr.ensure_default_bridge().await.unwrap();
        let b2 = mgr.ensure_default_bridge().await.unwrap();
        assert_eq!(b1.name, b2.name);
        assert_eq!(mgr.list_bridges().await.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn network_stats_tracking() {
        let mgr = VirtualNetworkManager::new();
        let stats = NetworkStats {
            bytes_sent: 1024,
            bytes_received: 2048,
            ..Default::default()
        };
        mgr.record_stats("ws-1", stats).await.unwrap();
        let got = mgr.get_stats("ws-1").await.unwrap();
        assert_eq!(got.bytes_sent, 1024);
        assert_eq!(got.bytes_received, 2048);
    }
}
