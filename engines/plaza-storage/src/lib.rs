//! # plaza-storage
//!
//! SQLite persistence layer, schema migrations, and event store.

pub mod event_store;
pub mod migrations;
pub mod repository;

pub use event_store::SqliteEventStore;
pub use repository::SqliteWorkspaceRepository;

