//! # plaza-plugin
//!
//! Plugin system for PlazaVM.
//!
//! Manages plugin lifecycle, discovery, capability advertisement,
//! and provides the [`RuntimePlugin`] trait that combines
//! [`Plugin`] with [`RuntimeBackend`](plaza_runtime::RuntimeBackend).

mod host;
mod manifest;

pub use host::PluginHost;
pub use manifest::{PluginDependency, PluginManifest, PluginType};

use async_trait::async_trait;
use plaza_core::types::HealthStatus;
use plaza_core::PlazaResult;
use plaza_runtime::RuntimeBackend;

/// Base contract for all PlazaVM plugins.
#[async_trait]
pub trait Plugin: Send + Sync {
    /// Plugin manifest with metadata.
    fn manifest(&self) -> &PluginManifest;

    /// Initialize the plugin with runtime context.
    async fn init(&mut self) -> PlazaResult<()>;

    /// Gracefully shut down the plugin.
    async fn shutdown(&mut self) -> PlazaResult<()>;

    /// Current health status.
    fn health(&self) -> HealthStatus;
}

/// A plugin that provides a runtime execution backend.
///
/// Must implement both [`Plugin`] (lifecycle) and
/// [`RuntimeBackend`](plaza_runtime::RuntimeBackend) (execution).
pub trait RuntimePlugin: Plugin + RuntimeBackend {}

// Blanket impl: anything that is both Plugin and RuntimeBackend
// automatically satisfies RuntimePlugin.
impl<T: Plugin + RuntimeBackend> RuntimePlugin for T {}
