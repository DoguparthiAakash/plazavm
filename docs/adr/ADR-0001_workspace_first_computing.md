# ADR-0001: Workspace-First Computing Paradigm

## Status
Accepted / Frozen

## Context
Traditional developer tooling forces developers to globally install runtime languages (Python, Node, Rust, Java), database servers (PostgreSQL, Redis), and system libraries onto the host OS. This results in host pollution, dependency conflicts, unreproducible builds, and fragile local setups.

## Decision
PlazaVM adopts the **Workspace-First Computing** paradigm.
The **Workspace** is the primary unit of computing for software development.
The host operating system serves solely as a Hardware Provider, while Plaza Foundation owns runtime orchestration, process sandboxing, toolchain isolation, and workspace lifecycle control above the Linux kernel (`Applications -> Workspace -> Workspace API -> Plaza Foundation -> Execution Backend -> Linux Kernel -> Hardware`).

## Consequences
- The host OS remains completely clean.
- All developer environments are 100% reproducible and portable.
- Runtimes (Docker, Podman, QEMU, VirtualBox, Hyper-V) become interchangeable execution backends.
