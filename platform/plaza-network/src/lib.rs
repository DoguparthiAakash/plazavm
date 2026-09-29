pub mod dns;
pub mod isolation;
pub mod port_forward;
pub mod service_discovery;
pub mod virtual_net;

// Re-export primary types for convenience
pub use dns::{DnsConfig, DnsRecord, DnsRecordType, DnsResolution, DnsResolver, DnsSource};
pub use isolation::{IsolationPolicy, NetworkIsolation, WorkspaceIsolation};
pub use port_forward::{PortForwarder, PortForwardRule, PortForwardSummary, PortProtocol};
pub use service_discovery::{ServiceDiscovery, ServiceHealth, ServiceInstance};
pub use virtual_net::{SubnetAllocation, VirtualBridge, VirtualNetworkManager};
