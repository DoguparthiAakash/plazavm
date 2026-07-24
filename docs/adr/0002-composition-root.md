# ADR 0002: Centralized Composition Root & Dependency Injection

## Status
Accepted

## Context
Manual object creation throughout application handlers or UI commands leads to duplicated state initialization and prevents test mocking.

## Decision
Introduce `BootstrapBuilder` in `plaza-api::bootstrap` as the single Composition Root for constructing the complete application dependency graph.

## Consequences
- Enforces Dependency Injection (DI).
- Subsystems receive `Arc<T>` handles to shared dependencies.
- Unit/integration tests construct in-memory test graphs cleanly via `BootstrapBuilder::new().with_in_memory_db().build().await`.
