# Changelog

All notable changes to PlazaVM will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [2.0.0-phase1.5] - 2026-07-24

### Added
- **22-Crate Architecture**: Complete v2 rebuild with locked 5-layer architecture.
- **Composition Root Pattern**: `BootstrapBuilder` in `plaza-api::bootstrap` for DI initialization.
- **Runtime Plugins**: Docker, VirtualBox, QEMU, Podman, and Hyper-V execution plugins.
- **Platform Manager**: Automated host OS, CPU, RAM, and GPU capability detection.
- **Decision Engine**: Matrix scoring for workload intent matching.
- **Evidence-Driven QA Certification Framework**: 16-stage automated validation pipeline in `plaza-cli validate`.
- **Desktop Shell**: Tauri v2 + React 19 frontend workspace manager interface.
