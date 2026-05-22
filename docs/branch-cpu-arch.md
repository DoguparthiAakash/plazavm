# Branch: feature/cpu-arch

## Scope
- Support extensibility of CPU architectures.
- Dynamically build QEMU command arguments based on `ArchProfile` in the registry.

## How to test without real hardware
- Run QEMU with TCG (Tiny Code Generator) fallback for architectures that don't match the host.

## Roadmap
- [x] x86_64
- [ ] arm64
- [ ] riscv64
- [ ] mips64
