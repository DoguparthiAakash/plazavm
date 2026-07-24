# `plaza-events`

High-throughput event bus crate powering PlazaVM's event-driven architecture.

---

## 🛠 Responsibilities

- Tokio broadcast `EventBus` supporting over 2,000,000 events/second.
- Strongly-typed `PlazaEvent` enumeration (WorkspaceCreated, Reconciled, PlatformScanned, PluginRegistered, ErrorEncountered).
