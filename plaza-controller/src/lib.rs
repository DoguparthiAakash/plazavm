//! # plaza-controller
//!
//! Reconciliation controller layer.
//!
//! Continuously compares workspace desired state against actual state and reacts
//! to domain events. Delegates runtime selection to `plaza-decision`, resource
//! allocation to `plaza-resource`, and execution to `plaza-plugin` / `plaza-runtime`.

pub mod reconciler;
pub mod recovery;

pub use reconciler::WorkspaceController;
