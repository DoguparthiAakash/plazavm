use plaza_foundation::core::{PlazaError, PlazaResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// A DNS record entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsRecord {
    pub name: String,
    pub record_type: DnsRecordType,
    pub value: String,
    pub ttl: u32,
}

/// Supported DNS record types.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum DnsRecordType {
    A,
    AAAA,
    CNAME,
    MX,
    TXT,
}

impl std::fmt::Display for DnsRecordType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::A => write!(f, "A"),
            Self::AAAA => write!(f, "AAAA"),
            Self::CNAME => write!(f, "CNAME"),
            Self::MX => write!(f, "MX"),
            Self::TXT => write!(f, "TXT"),
        }
    }
}

/// Configuration for a workspace's DNS resolution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsConfig {
    pub workspace_id: String,
    pub upstream_servers: Vec<String>,
    pub custom_records: Vec<DnsRecord>,
    pub search_domains: Vec<String>,
    pub host_dns_enabled: bool,
}

impl Default for DnsConfig {
    fn default() -> Self {
        Self {
            workspace_id: String::new(),
            upstream_servers: vec!["1.1.1.1".to_string(), "8.8.8.8".to_string()],
            custom_records: Vec::new(),
            search_domains: vec!["plaza.local".to_string()],
            host_dns_enabled: true,
        }
    }
}

/// DNS resolution result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsResolution {
    pub name: String,
    pub resolved_ip: String,
    pub record_type: DnsRecordType,
    pub ttl: u32,
    pub source: DnsSource,
}

/// Where a DNS resolution came from.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DnsSource {
    CustomRecord,
    UpstreamServer,
    HostDns,
}

/// Workspace-aware DNS resolver with custom record support.
#[derive(Clone)]
pub struct DnsResolver {
    configs: Arc<RwLock<HashMap<String, DnsConfig>>>,
    cache: Arc<RwLock<HashMap<String, DnsResolution>>>,
}

impl Default for DnsResolver {
    fn default() -> Self {
        Self::new()
    }
}

impl DnsResolver {
    pub fn new() -> Self {
        Self {
            configs: Arc::new(RwLock::new(HashMap::new())),
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Configure DNS for a workspace.
    pub async fn configure_dns(&self, config: DnsConfig) -> PlazaResult<()> {
        let workspace_id = config.workspace_id.clone();
        let mut configs = self.configs.write().await;
        configs.insert(workspace_id.clone(), config);
        tracing::info!("Configured DNS for workspace '{}'", workspace_id);
        Ok(())
    }

    /// Add a custom DNS record for a workspace.
    pub async fn add_record(
        &self,
        workspace_id: &str,
        record: DnsRecord,
    ) -> PlazaResult<()> {
        let mut configs = self.configs.write().await;
        let config = configs
            .entry(workspace_id.to_string())
            .or_insert_with(|| DnsConfig {
                workspace_id: workspace_id.to_string(),
                ..Default::default()
            });
        config.custom_records.push(record);
        tracing::debug!(
            "Added DNS record '{}' for workspace '{}'",
            config.custom_records.last().unwrap().name,
            workspace_id
        );
        Ok(())
    }

    /// Remove a custom DNS record by name for a workspace.
    pub async fn remove_record(&self, workspace_id: &str, record_name: &str) -> PlazaResult<bool> {
        let mut configs = self.configs.write().await;
        if let Some(config) = configs.get_mut(workspace_id) {
            let before = config.custom_records.len();
            config
                .custom_records
                .retain(|r| r.name != record_name);
            return Ok(config.custom_records.len() < before);
        }
        Ok(false)
    }

    /// Resolve a hostname for a workspace — checks custom records first, then cache.
    pub async fn resolve(&self, workspace_id: &str, hostname: &str) -> PlazaResult<DnsResolution> {
        // Check custom records first
        {
            let configs = self.configs.read().await;
            if let Some(config) = configs.get(workspace_id) {
                for record in &config.custom_records {
                    if record.name == hostname {
                        return Ok(DnsResolution {
                            name: hostname.to_string(),
                            resolved_ip: record.value.clone(),
                            record_type: record.record_type.clone(),
                            ttl: record.ttl,
                            source: DnsSource::CustomRecord,
                        });
                    }
                }
            }
        }

        // Check cache
        {
            let cache = self.cache.read().await;
            let cache_key = format!("{}:{}", workspace_id, hostname);
            if let Some(cached) = cache.get(&cache_key) {
                return Ok(cached.clone());
            }
        }

        // For workspaces that have host DNS enabled, simulate resolution
        // In a real implementation this would query upstream servers
        {
            let configs = self.configs.read().await;
            if let Some(config) = configs.get(workspace_id) {
                if config.host_dns_enabled {
                    // Simulate external resolution — in production this would use
                    // a real DNS client to query upstream servers
                    let resolution = DnsResolution {
                        name: hostname.to_string(),
                        resolved_ip: format!("10.88.{}", hash_simple(hostname) % 254 + 1),
                        record_type: DnsRecordType::A,
                        ttl: 300,
                        source: DnsSource::UpstreamServer,
                    };
                    let mut cache = self.cache.write().await;
                    let cache_key = format!("{}:{}", workspace_id, hostname);
                    cache.insert(cache_key, resolution.clone());
                    return Ok(resolution);
                }
            }
        }

        Err(PlazaError::Network(format!(
            "DNS resolution failed for '{}' in workspace '{}': no DNS configured",
            hostname, workspace_id
        )))
    }

    /// Get all DNS configuration for a workspace.
    pub async fn get_config(&self, workspace_id: &str) -> PlazaResult<Option<DnsConfig>> {
        let configs = self.configs.read().await;
        Ok(configs.get(workspace_id).cloned())
    }

    /// List all custom DNS records for a workspace.
    pub async fn list_records(&self, workspace_id: &str) -> PlazaResult<Vec<DnsRecord>> {
        let configs = self.configs.read().await;
        Ok(configs
            .get(workspace_id)
            .map(|c| c.custom_records.clone())
            .unwrap_or_default())
    }

    /// Clear the DNS cache for a workspace.
    pub async fn clear_cache(&self, workspace_id: &str) -> PlazaResult<u64> {
        let mut cache = self.cache.write().await;
        let prefix = format!("{}:", workspace_id);
        let before = cache.len();
        cache.retain(|k, _| !k.starts_with(&prefix));
        Ok((before - cache.len()) as u64)
    }
}

/// Simple hash for deterministic IP assignment from hostname.
fn hash_simple(s: &str) -> u32 {
    let mut h: u32 = 5381;
    for b in s.bytes() {
        h = h.wrapping_mul(33).wrapping_add(b as u32);
    }
    h
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn add_and_resolve_custom_record() {
        let resolver = DnsResolver::new();
        resolver
            .configure_dns(DnsConfig {
                workspace_id: "ws-1".into(),
                ..Default::default()
            })
            .await
            .unwrap();

        resolver
            .add_record(
                "ws-1",
                DnsRecord {
                    name: "myapp.local".into(),
                    record_type: DnsRecordType::A,
                    value: "192.168.1.100".into(),
                    ttl: 600,
                },
            )
            .await
            .unwrap();

        let res = resolver.resolve("ws-1", "myapp.local").await.unwrap();
        assert_eq!(res.resolved_ip, "192.168.1.100");
        assert_eq!(res.source, DnsSource::CustomRecord);
    }

    #[tokio::test]
    async fn remove_record() {
        let resolver = DnsResolver::new();
        resolver
            .configure_dns(DnsConfig {
                workspace_id: "ws-1".into(),
                ..Default::default()
            })
            .await
            .unwrap();
        resolver
            .add_record(
                "ws-1",
                DnsRecord {
                    name: "to-remove.local".into(),
                    record_type: DnsRecordType::A,
                    value: "10.0.0.1".into(),
                    ttl: 300,
                },
            )
            .await
            .unwrap();

        let removed = resolver.remove_record("ws-1", "to-remove.local").await.unwrap();
        assert!(removed);
        assert!(resolver.list_records("ws-1").await.unwrap().is_empty());
    }

    #[tokio::test]
    async fn unconfigured_workspace_fails() {
        let resolver = DnsResolver::new();
        let err = resolver.resolve("unknown", "test.com").await;
        assert!(err.is_err());
    }

    #[tokio::test]
    async fn cache_clear() {
        let resolver = DnsResolver::new();
        resolver
            .configure_dns(DnsConfig {
                workspace_id: "ws-1".into(),
                ..Default::default()
            })
            .await
            .unwrap();

        // Trigger a cache entry
        let _ = resolver.resolve("ws-1", "cached.local").await;
        let cleared = resolver.clear_cache("ws-1").await.unwrap();
        assert_eq!(cleared, 1);
    }
}
