# ADR 0001: 5-Layer Modular Architecture

## Status
Accepted

## Context
PlazaVM v1 suffered from tight coupling between execution technologies (Docker/VirtualBox) and workspace domain logic, making expansion fragile.

## Decision
Adopt a strict 5-layer modular Cargo workspace architecture:
1. **Core & Events**: `plaza-core`, `plaza-events`, `plaza-config`, `plaza-storage`
2. **Platform & Discovery**: `plaza-platform`, `plaza-monitor`, `plaza-registry`
3. **Domain Abstraction**: `plaza-workspace`, `plaza-runtime`, `plaza-plugin`
4. **Intelligence & Scheduling**: `plaza-decision`, `plaza-resource`, `plaza-controller`, `plaza-ai`
5. **Presentation & Application**: `plaza-api`, `plaza-cli`, `plaza-desktop`, `plugins/*`

## Consequences
- Clean separation of concerns.
- Runtimes interact only via trait contracts (`RuntimeBackend` and `RuntimePlugin`).
- Zero circular dependencies across crates.
