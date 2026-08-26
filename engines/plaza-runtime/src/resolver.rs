//! Runtime Resolution Logic.
//!
//! Handles capability negotiation by checking the requirements of a `MachineConfig`
//! against the `RuntimeCapabilities` exposed by registered backends.

use crate::backend::RuntimeBackend;
use crate::machine::MachineConfig;
use std::sync::Arc;
use tracing::debug;

/// Evaluates if a given backend supports the requirements of the given MachineConfig.
pub fn supports(
    backend: &dyn RuntimeBackend,
    config: &MachineConfig,
    backend_override: Option<&str>,
) -> bool {
    let caps = backend.capabilities();

    // 1. Architecture Check
    if !caps.supported_arch.contains(&config.machine.architecture) {
        debug!(
            "Backend {} rejected: unsupported architecture {}",
            backend.id(),
            config.machine.architecture
        );
        return false;
    }

    // 2. Capability Checks (Network)
    if config.capabilities.network.enabled && !caps.can_bridge_network {
        debug!("Backend {} rejected: cannot bridge network", backend.id());
        return false;
    }

    // 3. Runtime Override Check
    if let Some(requested) = backend_override {
        if requested != "auto" && requested != backend.id() {
            debug!(
                "Backend {} rejected: does not match explicitly requested backend {}",
                backend.id(),
                requested
            );
            return false;
        }
    }

    true
}

/// Resolves the best backend for the given configuration from the list of candidates.
pub fn resolve_backend<'a, I>(
    candidates: I,
    config: &MachineConfig,
    backend_override: Option<&str>,
) -> Option<Arc<dyn RuntimeBackend>>
where
    I: Iterator<Item = &'a Arc<dyn RuntimeBackend>>,
{
    // Prioritize QEMU for non-WASM targets if available and supported.
    // In a mature implementation, we might score backends based on acceleration support,
    // memory overhead, etc. For now, we take the first matching backend.

    for backend in candidates {
        if supports(backend.as_ref(), config, backend_override) {
            debug!("Resolved backend {} for configuration", backend.id());
            return Some(backend.clone());
        }
    }

    None
}
