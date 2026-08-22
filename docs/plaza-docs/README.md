# PlazaVM Documentation

Welcome to the official documentation for PlazaVM. 

PlazaVM is an isolated, capability-based virtual machine manager and workspace orchestration engine. It allows you to run containers, microVMs, and webassembly environments with strict security constraints and a unified lifecycle.

## Documentation Structure

* `architecture/`: Technical design documents explaining how PlazaVM subsystems interact.
* `phases/`: Historical documentation detailing the evolution of PlazaVM development phases.
* `development/`: Guides on how to build, test, and contribute to PlazaVM.

## Key Subsystems

* **plaza-workspace**: The domain aggregate root for the workspace model, managing declarative `WorkspaceSpec` configurations.
* **plaza-runtime**: The abstract runtime manager that routes execution commands to specific plugins via `MachineConfig`.
* **plaza-image**: Virtual block device synthesis and image layer management.
* **plaza-foundation**: Shared abstractions (error handling, capability models, events).
* **plaza-api / plaza-cli**: User-facing interfaces for interacting with the workspace state.

*(Note: PlazaVM is currently in development under Phase 13).*
