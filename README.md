# PlazaVM

PlazaVM is a desktop virtual machine manager designed for zero-friction VM creation, powered by QEMU. It provides an intuitive GUI and powerful headless CLI for running Windows, Linux, and BSD guests with native performance.

## Prerequisites
- [QEMU](https://www.qemu.org/) installed and available in `$PATH`
- Node.js (v20+)
- pnpm (v8+)

## Quickstart
```bash
# 1. Install dependencies
pnpm install

# 2. Run the desktop app and backend in dev mode
pnpm dev
```

## Branch Strategy

| Branch | Purpose |
| ------ | ------- |
| `main` | Stable, tagged releases only |
| `dev`  | Integration branch, all features merge here first |
| `feature/core` | QEMU/libvirt wiring, VM lifecycle |
| `feature/windows` | Windows guest support |
| `feature/linux` | Linux guest support |
| `feature/bsd` | BSD guest support |
| `feature/ui` | Electron shell, React components |
| `feature/networking` | NAT, bridged, host-only modes |
| `feature/storage` | Virtual disk management |
| `feature/cpu-arch` | Custom CPU architecture profiles |
| `feature/cli` | Headless CLI interface |
