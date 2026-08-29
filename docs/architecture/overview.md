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
                      Inferno OS (Guest)
                            │
                      QEMU (Host TCG)
```

## Core Engines

### 1. Workspace Engine
The **Workspace Engine** manages the lifecycle, configuration, and state transitions of virtual environments. It parses the declarative `plaza.yaml` configuration, provisions `.plaza/workspace`, and orchestrates the QEMU boot process.

### 2. Image Engine
The **Image Engine** locates, builds, and manages the `inferno.386` kernels. It dynamically injects Limbo boot scripts (`inferno_workspace.b`, `linux_compat.b`) so the kernel boots directly into a managed state.

### 3. Capability Engine
The **Capability Engine** enforces a strict **Default-Deny Security Model**. Every capability (filesystem access, networking, environment variables, etc.) must be explicitly granted in the workspace configuration. During execution, it mediates the 9P/Styx protocol connection to enforce host-side limits.

## Runtime Backend

PlazaVM abstracts execution through a unified backend:

- **Inferno OS over QEMU (TCG)**: Software emulation for native hardware targets (`i386`). Avoids host kernel virtualization (like KVM or Hyper-V) to ensure maximum portability across all platforms (Windows, macOS, Linux). The guest OS is extremely lightweight and starts in milliseconds.
