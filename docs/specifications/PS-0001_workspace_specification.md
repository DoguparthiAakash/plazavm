# PS-0001: Workspace Specification v1.0

## Status
Standard / Frozen (Architecture v1.0)

## Abstract
This specification defines the fundamental unit of computing in PlazaVM: the **Workspace**. A Workspace is an isolated, reproducible, self-contained development environment possessing its own runtime, toolchains, dependencies, network topology, storage volumes, and security policies.

## Workspace Directory Hierarchy
Every workspace conforming to PS-0001 MUST enforce the following directory layout under its root path:
- `src/` — Primary application source code directory.
- `.plaza/` — Scoped PlazaVM internal metadata root directory.
- `.plaza/config/` — Workspace-scoped configuration manifests (`plaza.yaml`, `plaza.lock`).
- `.plaza/services/` — Declarative background service definitions (PostgreSQL, Redis, Ollama).
- `.plaza/logs/` — Process, service, and workspace execution logs.
- `.plaza/cache/` — Build artifact and package manager cache.
- `.plaza/models/` — Scoped AI model weights and vector embeddings.
- `.plaza/datasets/` — Data science and AI test datasets.
- `.plaza/snapshots/` — Layered point-in-time filesystem snapshots.
- `.plaza/secrets/` — Encrypted workspace secrets.
- `.plaza/temp/` — Temporary execution sandbox files.
