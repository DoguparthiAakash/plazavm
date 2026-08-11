# PlazaVM Architecture Overview

PlazaVM is built on a unified virtualization and emulation engine that abstracts away the underlying hypervisor differences, providing a consistent execution environment.

## High-Level Architecture

```text
                         PLAZAVM
                            │
             ┌──────────────┼──────────────┐
             │              │              │
        Workspace       Image Engine    Capability
          Engine                         Engine
             │              │              │
             └──────────────┼──────────────┘
                            │
                      Runtime Manager
                            │
                ┌───────────┴───────────┐
                │                       │
              v86                     QEMU
        (WASM/Browser)          (Native/Hardware)
```

## Core Engines

### 1. Workspace Engine
The **Workspace Engine** manages the lifecycle, configuration, and state transitions of virtual environments. It parses the declarative `plaza.yaml` configuration and orchestrates the reconciliation pipeline.

### 2. Image Engine
The **Image Engine** handles content-addressable storage, OCI-style image layers, and virtual block devices. It composes root filesystems dynamically without relying on host-level mounting (avoiding the need for root privileges).

### 3. Capability Engine
The **Capability Engine** enforces a strict **Default-Deny Security Model**. Every capability (filesystem access, networking, environment variables, etc.) must be explicitly granted in the workspace configuration and validated against host constraints before execution.

## Runtime Backends

PlazaVM abstracts execution through the **Runtime Manager**, supporting multiple backends:

- **QEMU (TCG)**: Software emulation for native hardware targets. Avoids host kernel virtualization (like KVM or Hyper-V) to ensure maximum portability.
- **v86**: WebAssembly-based x86 emulation, allowing workloads to run efficiently in constrained or browser-like environments.

The backend is selected automatically based on the `WorkspaceSpec` and host capabilities, completely transparent to the user.
