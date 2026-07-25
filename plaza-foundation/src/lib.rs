//! # plaza-foundation
//!
//! Plaza Foundation Core, Service Registry, Provider Registry, Internal Protocol, and Plaza Foundation Engine (PFE).

pub mod engine;
pub mod protocol;
pub mod registry;

pub use engine::{EngineCore, EngineLifecycleState, PfeError, PfeResult};
pub use protocol::{FoundationCommand, FoundationQuery, FoundationResponse, ProtocolEnvelope};
pub use registry::{ProviderCategory, ProviderDescriptor, ProviderRegistry};

use plaza_events::EventBus;
use plaza_platform::PlatformDetector;
use std::sync::Arc;

/// Central Foundation Core Orchestrator and Service Registry container.
pub struct FoundationCore {
    pub engine: Arc<EngineCore>,
    pub event_bus: Arc<EventBus>,
    pub platform: Arc<PlatformDetector>,
    pub provider_registry: Arc<ProviderRegistry>,
}

impl FoundationCore {
    pub async fn initialize() -> plaza_core::PlazaResult<Self> {
        let engine = Arc::new(EngineCore::boot().await.map_err(|e| {
            plaza_core::PlazaError::Config(e.to_string())
        })?);

        let event_bus = Arc::new(EventBus::new());
        let platform = Arc::new(PlatformDetector::new());
        let provider_registry = Arc::new(ProviderRegistry::new());

        // Register default built-in platform provider descriptor
        provider_registry.register(ProviderDescriptor {
            id: "linux-foundation".into(),
            name: "Linux PAL Provider".into(),
            category: ProviderCategory::Platform,
            version: semver::Version::new(1, 0, 0),
            capabilities: vec!["cgroups_v2".into(), "namespaces".into(), "landlock".into()],
        });

        Ok(Self {
            engine,
            event_bus,
            platform,
            provider_registry,
        })
    }
}
