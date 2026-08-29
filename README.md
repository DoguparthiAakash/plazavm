# PlazaVM

PlazaVM is an extremely lightweight, secure, and ultra-portable virtual workspace control plane. By entirely ditching heavy containers (Docker/OCI) and full-blown Linux distributions, PlazaVM provides deterministic, instantaneous development environments built on top of **Inferno OS** running seamlessly inside QEMU.

PlazaVM is under active development. Its core philosophy is absolute isolation and portability without the heavy runtime footprint of modern container engines.

## Core Capabilities

- **Inferno OS Native Architecture**: Workspace environments are powered by customized `inferno.386` kernels.
- **Zero-Dependency Environments**: No Docker, Podman, or third-party container runtimes required. If you have QEMU, you can run PlazaVM.
- **Lightning Fast Boot**: Boots into a fully interactive Limbo/Dis environment in milliseconds.
- **9P / Styx Protocol Support**: Host-to-guest directory sharing is natively supported through Inferno's Styx protocol over virtio-serial or network listeners, providing seamless and secure workspace mounting.
- **Strict Default-Deny Security**: Zero capabilities (network, host mounts) exist unless explicitly declared and approved by the Capability Engine.
- **Linux Compatibility Layer (WIP)**: A specialized Limbo subsystem that translates subset Linux syscalls, allowing crucial static ELF binaries to run natively inside the Inferno environment.

See [the support matrix](docs/support-matrix.md) for exact status and limitations.

## Architecture

```mermaid
graph TD
    CLI[plazavm CLI] --> Engine[PlazaVM Workspace Engine]
    
    subgraph PlazaVM 
        Engine --> Config[plaza.yaml Parser]
        Engine --> Capability[Capability Engine]
        Engine --> Image[Image & Kernel Builder]
        Image --> QEMU[QEMU TCG Backend]
    end

    subgraph QEMU Guest
        QEMU --> |Boot| Inferno[inferno.386 Kernel]
        Inferno --> Limbo[Workspace Init (Limbo)]
        Limbo --> |9P/Styx| Mount[Workspace Mount]
        Limbo --> LinuxCompat[Linux Compatibility Subsystem]
    end
```

Detailed architectural designs can be found in the `docs/architecture` folder:
- [Architecture Overview](docs/architecture/overview.md)
- [Workspace Images](docs/architecture/workspace-images.md)
- [Inferno Runtime Evaluation](docs/architecture/inferno-runtime-evaluation.md)

## Prerequisites

Required for building from source:
- Rust stable and Cargo
- Docker (only temporarily required to compile the custom Inferno kernel artifacts)
- QEMU (`qemu-system-i386`)

## Build & Boot

1. **Build the Custom Inferno Kernel**:
   PlazaVM requires a pre-built custom Inferno kernel equipped with the PlazaVM Limbo boot scripts.
   ```powershell
   .\scripts\build-inferno-kernel.ps1
   ```

2. **Build the Engine**:
   ```powershell
   cargo build --workspace
   ```

3. **Create a Workspace**:
   ```powershell
   cargo run -p plazavm_cli -- workspace create test-ws
   ```
   This command provisions the `.plaza` configuration and automatically boots the `inferno.386` kernel under QEMU, waiting for the Limbo init script to signal readiness over the serial console.

## Project Structure

```text
crates/
  plazavm_core/        Configuration parsing and schema validation
  engines/
    plaza-workspace/   Orchestrates workspace lifecycle, capabilities, and 9P mounts
    plaza-runtime/     Manages the physical QEMU process and serial IPC
scripts/
  build-inferno-kernel.ps1   Automated kernel and Limbo compilation pipeline
inferno-os/            Submodule containing the Inferno OS source tree
docs/                  Architecture and design documents
```

## Development Policy

- **No Third-Party Bloat**: If it can be done natively or in Limbo, we do not add a heavy dependency.
- **Portability First**: The system must run on old hardware (i386 targets, standard IDE drives). Avoid strict requirements on hardware virtualization (KVM/WHPX) for the core workspace flow.
- **Security**: Never grant host capabilities without explicit declarative configuration. 

The staged roadmap is maintained in [ROADMAP.md](ROADMAP.md).
