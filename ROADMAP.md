# PlazaVM v2 — Project Roadmap

---

## 📍 Completed: Phase 1.5 (Quality, Verification & Infrastructure)

- [x] Rebuild 22-crate modular workspace architecture
- [x] Composition Root dependency injection pattern
- [x] Intent scoring decision matrix
- [x] 16-Stage Evidence-Driven QA Certification Pipeline
- [x] Repository organization, schemas, devcontainer & documentation suite

---

## 📍 Completed: Phase 2 (Inferno OS Kernel Integration)

- [x] Adopt Inferno OS as the primary workspace runtime
- [x] Remove Docker, VirtualBox, and OCI container dependencies
- [x] Set up automated `inferno.386` kernel compilation via QEMU
- [x] Implement Limbo boot scripts (`inferno_workspace.b`)
- [x] Implement QEMU-serial IPC for readiness markers

---

## 🚀 Current: Phase 3 (Workspace 9P Mounting & Styx Protocol)

- [ ] Establish 9P/Styx server listener within Limbo
- [ ] Implement host-side 9P client in Rust to mount `/workspace`
- [ ] Enforce Capability Engine constraints over the 9P channel
- [ ] Bidirectional file synchronization without host root privileges

---

## 🌐 Phase 4: Linux Compatibility Layer

- [ ] Develop `linux_compat.b` Limbo subsystem
- [ ] Translate minimal POSIX subset (open, read, write, exec) to Inferno native calls
- [ ] Run basic statically linked Linux ELF binaries (like `gcc` or `git`) natively inside the Inferno environment
