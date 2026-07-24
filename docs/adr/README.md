# Architecture Decision Records (ADRs)

This directory contains formal records of architectural decisions made during the design and rebuild of PlazaVM v2.

---

## 📜 ADR Index

- [ADR 0001: Five-Layer Architecture](0001-five-layer-architecture.md) — Locked separation into Platform, Decision, Controller, Runtime, and Resource subsystems.
- [ADR 0002: Composition Root Bootstrap Pattern](0002-composition-root.md) — Dependency injection container in `plaza-api::bootstrap`.
- [ADR 0003: Event-Driven Reconciliation Loop](0003-event-driven-controller.md) — Tokio broadcast event bus and state convergence loop.
