# Getting Started with PlazaVM v2

This quickstart guide walks you through installing PlazaVM and creating your first virtual workspace.

---

## Prerequisites

- **Operating System**: Windows 10/11 64-bit, Linux (Ubuntu 22.04+), or macOS 13+
- **Rust Toolchain**: Rust 1.75+ (for compiling from source)
- **Node.js**: Node 18+ & npm (for desktop frontend development)

---

## 🚀 1. Build PlazaVM CLI

Clone the repository and build using Cargo:

```bash
git clone https://github.com/plazavm/plazavm.git
cd plazavm
cargo build --release -p plaza-cli
```

The binary will be generated at `target/release/plaza-cli.exe` (or `target/release/plaza-cli` on Linux/macOS).

---

## 🔍 2. Verify Host Platform Hardware

Run the platform diagnostic command:

```bash
cargo run -p plaza-cli -- platform
```

**Expected Output**:
```text
System Platform Capability Audit
--------------------------------
Host Operating System : Windows (x86_64)
CPU Cores             : 16 Logical Cores
System Memory         : 32768 MB Total
GPU Acceleration      : Detected
Classified Profile    : HighPerformanceDesktop
```

---

## 🛠 3. Run the Automated QA Certification Pipeline

Validate that your host environment passes all 16 QA certification stages:

```bash
cargo run -p plaza-cli -- validate
```

The command will execute the full QA certification pipeline and output raw evidence reports to `~/.plazavm/artifacts/validation/latest/REPORT.html`.
