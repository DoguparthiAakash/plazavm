# PlazaVM Engineering Tools — Validator Subsystem

The validator engineering tool operates as PlazaVM's **Evidence-Driven QA Certification Framework**.

---

## 🏗 Architecture

- **Entrypoint**: `cargo run -p plaza-cli -- validate`
- **Engine Source**: `plaza-cli/src/validator/` (`runner.rs`, `stages.rs`, `evidence.rs`, `reporter.rs`, `dashboard.rs`)
- **Evidence Output**: `~/.plazavm/artifacts/validation/YYYY-MM-DD_HH-MM-SS/`
