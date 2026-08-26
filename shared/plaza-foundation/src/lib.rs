//! # plaza-foundation
//!
//! Absolute bottom-layer dependencies for the PlazaVM ecosystem.

pub mod core;

pub mod config;
pub mod events;
pub mod platform;

// We preserve the existing engine module temporarily if needed by other crates,
// but it should probably move to plaza-workspace.

pub mod engine;
