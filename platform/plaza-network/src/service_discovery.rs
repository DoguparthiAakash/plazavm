use plaza_foundation::core::{PlazaError, PlazaResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Health status of a discovered service.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ServiceHealth {
    Healthy,
    Unhealthy,
    Unknown,
}

/// A registered service instance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInstance {
    pub name: String,
    pub workspace_id: String,
    pub address: String,
    pub port: u16,
    pub protocol: String,
    pub health: ServiceHealth,
    pub tags: Vec<String>,
    pub metadata: HashMap<String, String>,
    pub registered_at: u64,
    pub last_heartbeat: u64,
}

/// A discovered service match from lookup.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceDiscoveryResult {
    pub service_name: String,
    pub instances: Vec<ServiceInstance>,
    pub total: usize,
}

/// Manages service registration, discovery, and health for workspace services.
#[derive(Clone)]
pub struct ServiceDiscovery {
    services: Arc<RwLock<HashMap<String, Vec<ServiceInstance>>>>,
}

impl Default for ServiceDiscovery {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceDiscovery {
    pub fn new() -> Self {
        Self {
            services: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a new service instance.
    pub async fn register_service(
        &self,
        name: &str,
        workspace_id: &str,
        address: &str,
        port: u16,
        protocol: &str,
        tags: Vec<String>,
        metadata: HashMap<String, String>,
    ) -> PlazaResult<ServiceInstance> {
        let now = timestamp_now();
        let instance = ServiceInstance {
            name: name.to_string(),
            workspace_id: workspace_id.to_string(),
            address: address.to_string(),
            port,
            protocol: protocol.to_string(),
            health: ServiceHealth::Healthy,
            tags,
            metadata,
            registered_at: now,
            last_heartbeat: now,
        };

        let mut services = self.services.write().await;
        services
            .entry(name.to_string())
            .or_insert_with(Vec::new)
            .push(instance.clone());

        tracing::info!(
            "Registered service '{}' at {}:{} for workspace '{}'",
            name,
            address,
            port,
            workspace_id
        );
        Ok(instance)
    }

    /// Deregister a specific service instance.
    pub async fn deregister_service(
        &self,
        name: &str,
        workspace_id: &str,
    ) -> PlazaResult<bool> {
        let mut services = self.services.write().await;
        if let Some(instances) = services.get_mut(name) {
            let before = instances.len();
            instances.retain(|i| i.workspace_id != workspace_id);
            let removed = before > instances.len();
            if removed {
                tracing::info!(
                    "Deregistered service '{}' for workspace '{}'",
                    name,
                    workspace_id
                );
            }
            // Clean up empty entries
            if instances.is_empty() {
                services.remove(name);
            }
            return Ok(removed);
        }
        Ok(false)
    }

    /// Look up all instances of a service by name.
    pub async fn lookup_service(&self, name: &str) -> PlazaResult<ServiceDiscoveryResult> {
        let services = self.services.read().await;
        let instances = services
            .get(name)
            .cloned()
            .unwrap_or_default();
        let total = instances.len();

        Ok(ServiceDiscoveryResult {
            service_name: name.to_string(),
            instances,
            total,
        })
    }

    /// Look up services by tag within a workspace.
    pub async fn lookup_by_tag(
        &self,
        workspace_id: &str,
        tag: &str,
    ) -> PlazaResult<Vec<ServiceInstance>> {
        let services = self.services.read().await;
        let results: Vec<ServiceInstance> = services
            .values()
            .flatten()
            .filter(|i| i.workspace_id == workspace_id && i.tags.contains(&tag.to_string()))
            .cloned()
            .collect();
        Ok(results)
    }

    /// List all services registered by a workspace.
    pub async fn list_workspace_services(
        &self,
        workspace_id: &str,
    ) -> PlazaResult<Vec<ServiceInstance>> {
        let services = self.services.read().await;
        Ok(services
            .values()
            .flatten()
            .filter(|i| i.workspace_id == workspace_id)
            .cloned()
            .collect())
    }

    /// Send a heartbeat to keep a service instance alive and update its health.
    pub async fn heartbeat(
        &self,
        name: &str,
        workspace_id: &str,
        health: ServiceHealth,
    ) -> PlazaResult<()> {
        let mut services = self.services.write().await;
        if let Some(instances) = services.get_mut(name) {
            for instance in instances.iter_mut() {
                if instance.workspace_id == workspace_id {
                    instance.last_heartbeat = timestamp_now();
                    instance.health = health;
                    return Ok(());
                }
            }
        }
        Err(PlazaError::Network(format!(
            "service '{}' not found for workspace '{}'",
            name, workspace_id
        )))
    }

    /// Remove all services for a workspace (used during workspace teardown).
    pub async fn remove_all_for_workspace(&self, workspace_id: &str) -> PlazaResult<u32> {
        let mut services = self.services.write().await;
        let mut total_removed = 0u32;
        for instances in services.values_mut() {
            let before = instances.len();
            instances.retain(|i| i.workspace_id != workspace_id);
            total_removed += (before - instances.len()) as u32;
        }
        // Clean up empty entries
        services.retain(|_, v| !v.is_empty());

        tracing::info!(
            "Removed {} service registrations for workspace '{}'",
            total_removed,
            workspace_id
        );
        Ok(total_removed)
    }

    /// Clean up stale services that haven't sent a heartbeat within the timeout.
    pub async fn cleanup_stale(&self, timeout_secs: u64) -> PlazaResult<u32> {
        let now = timestamp_now();
        let mut services = self.services.write().await;
        let mut total_removed = 0u32;

        for instances in services.values_mut() {
            let before = instances.len();
            instances
                .retain(|i| now.saturating_sub(i.last_heartbeat) < timeout_secs);
            total_removed += (before - instances.len()) as u32;
        }
        services.retain(|_, v| !v.is_empty());

        if total_removed > 0 {
            tracing::info!("Cleaned up {} stale service registrations", total_removed);
        }
        Ok(total_removed)
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
    async fn register_and_lookup() {
        let sd = ServiceDiscovery::new();
        sd.register_service(
            "postgres",
            "ws-1",
            "10.0.0.2",
            5432,
            "tcp",
            vec!["database".into()],
            HashMap::new(),
        )
        .await
        .unwrap();

        let result = sd.lookup_service("postgres").await.unwrap();
        assert_eq!(result.total, 1);
        assert_eq!(result.instances[0].port, 5432);
    }

    #[tokio::test]
    async fn deregister_service() {
        let sd = ServiceDiscovery::new();
        sd.register_service("redis", "ws-1", "10.0.0.2", 6379, "tcp", vec![], HashMap::new())
            .await
            .unwrap();

        let removed = sd.deregister_service("redis", "ws-1").await.unwrap();
        assert!(removed);
        assert!(sd.lookup_service("redis").await.unwrap().instances.is_empty());
    }

    #[tokio::test]
    async fn lookup_by_tag() {
        let sd = ServiceDiscovery::new();
        sd.register_service(
            "api",
            "ws-1",
            "10.0.0.2",
            8080,
            "tcp",
            vec!["web".into()],
            HashMap::new(),
        )
        .await
        .unwrap();
        sd.register_service(
            "worker",
            "ws-1",
            "10.0.0.2",
            9090,
            "tcp",
            vec!["background".into()],
            HashMap::new(),
        )
        .await
        .unwrap();

        let web_services = sd.lookup_by_tag("ws-1", "web").await.unwrap();
        assert_eq!(web_services.len(), 1);
        assert_eq!(web_services[0].name, "api");
    }

    #[tokio::test]
    async fn heartbeat_updates_health() {
        let sd = ServiceDiscovery::new();
        sd.register_service("svc", "ws-1", "10.0.0.2", 80, "tcp", vec![], HashMap::new())
            .await
            .unwrap();

        sd.heartbeat("svc", "ws-1", ServiceHealth::Unhealthy)
            .await
            .unwrap();
        let result = sd.lookup_service("svc").await.unwrap();
        assert_eq!(result.instances[0].health, ServiceHealth::Unhealthy);
    }

    #[tokio::test]
    async fn remove_all_for_workspace() {
        let sd = ServiceDiscovery::new();
        sd.register_service("svc-a", "ws-1", "10.0.0.2", 80, "tcp", vec![], HashMap::new())
            .await
            .unwrap();
        sd.register_service("svc-b", "ws-1", "10.0.0.2", 81, "tcp", vec![], HashMap::new())
            .await
            .unwrap();
        sd.register_service("svc-c", "ws-2", "10.0.0.3", 80, "tcp", vec![], HashMap::new())
            .await
            .unwrap();

        let removed = sd.remove_all_for_workspace("ws-1").await.unwrap();
        assert_eq!(removed, 2);
        assert!(sd.list_workspace_services("ws-1").await.unwrap().is_empty());
        assert_eq!(sd.list_workspace_services("ws-2").await.unwrap().len(), 1);
    }
}
