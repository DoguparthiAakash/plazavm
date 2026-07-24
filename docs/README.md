# PlazaVM v2 Documentation

Welcome to the official documentation for **PlazaVM v2**, the universal workspace virtualization platform.

PlazaVM abstracts execution backends (Docker, VirtualBox, QEMU, Podman, Hyper-V) behind a unified, declarative workspace model.

---

## 📚 Documentation Navigation

- [🚀 Getting Started](getting-started/quickstart.md) — Install PlazaVM and create your first workspace.
- [💻 Installation](installation/windows.md) — OS-specific setup guides for Windows, Linux, and macOS.
- [💡 Concepts](concepts/workspace-graph.md) — Core concepts: Workspace Graph, Declarative Intent Model, Reconciliation.
- [🏗 Architecture](architecture/overview.md) — Layered architecture, crate breakdown, and sequence diagrams.
- [⚙️ Configuration](configuration/plaza-yaml.md) — `plaza.yaml` schema reference and settings documentation.
- [🔌 Plugin System](plugins/development.md) — Build custom runtime execution backends.
- [🔒 Security Audit](security/audit.md) — Memory safety, SQL parameterized isolation, and security review.
- [🧪 Testing & QA](testing/guide.md) — Automated testing suite, QA certification framework, and benchmarks.
- [📋 Architecture Decision Records (ADRs)](adr/README.md) — Formal architectural decisions.

---

## 🛠 Core Architectural Layers

1. **Platform Manager (`plaza-platform`)**: Host OS, CPU, RAM, and GPU capabilities.
2. **Decision Layer (`plaza-decision`)**: Workload intent scoring & optimal backend selection.
3. **Controller Layer (`plaza-controller`)**: Event-driven reconciliation loop.
4. **Runtime Layer (`plaza-runtime`, `plaza-plugin`)**: Extensible execution backends.
5. **Resource Manager (`plaza-resource`)**: Dynamic quota allocation & tracking.
