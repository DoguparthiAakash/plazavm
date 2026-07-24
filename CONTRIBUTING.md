# Contributing to PlazaVM v2

Thank you for your interest in contributing to PlazaVM!

---

## 🛠 Development Setup

1. Install Rust (1.75+):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. Clone repository & check workspace:
   ```bash
   git clone https://github.com/plazavm/plazavm.git
   cd plazavm
   cargo check --workspace
   ```
3. Run the complete QA validation suite:
   ```bash
   cargo run -p plaza-cli -- validate
   ```

---

## 📏 Engineering Directives

- **Zero Clippy Warnings**: All code must compile with `cargo clippy --workspace -- -D warnings`.
- **Code Formatting**: Format code using `cargo fmt --all`.
- **Tests**: Add unit tests for core domain logic in `src/` or `tests/`.
- **Layer Separation**: Maintain clean architecture separation across member crates.
