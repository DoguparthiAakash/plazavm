# PlazaVM v2 — Automated Validation & QA Certification Framework

PlazaVM v2 features an **Evidence-Driven QA Certification Pipeline** accessible directly via the CLI:

```bash
cargo run -p plaza-cli -- validate
```

---

## 📋 16 Validation Pipeline Stages

1. **Workspace Build & Quality Check**: Runs `cargo fmt`, `cargo clippy -D warnings`, `cargo build`, `cargo doc`.
2. **Unit Tests Suite**: Runs unit tests across all member crates.
3. **Integration Workflows**: Verifies end-to-end workspace lifecycle state transitions.
4. **Stress Tests & Benchmark Scaling**: Evaluates system behavior under 10,000 events & 1,000 workspaces.
5. **Failure Injection & Automatic Recovery**: Validates graceful error recovery under resource exhaustion.
6. **Decision Engine Matrix Validation**: Assesses intent scoring accuracy.
7. **Platform Profile Validation**: Probes host OS, CPU, RAM, and GPU.
8. **Plugin System Validation**: Validates dynamic loading and manifest checks for all 5 execution plugins.
9. **Security Audit Scan**: Scans for unsafe blocks, SQL injections, and path traversal risks.
10. **Performance Benchmarks**: Records baseline latency metrics.
11. **Desktop UI Snapshot Testing**: Runs TypeScript compilation (`npx tsc --noEmit`) and component graph checks.
12. **CLI Snapshot Audit**: Captures raw CLI output snapshots (`plaza --help`, `platform`, `system`).
13. **Configuration Schema Validation**: Validates `plaza.yaml` parsing and intent resolution.
14. **Documentation & ADR Integrity**: Verifies all Markdown documentation links and ADRs.
15. **Dependency Graph & License Audit**: Verifies acyclic crate dependency graph and license compliance.
16. **Quality Gate Synthesis & Coverage**: Computes final health score (100/100) and exports interactive HTML dashboard.
