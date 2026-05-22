# Contributing to PlazaVM

## Branching Convention
- All feature work must be done in feature branches (e.g. `feature/windows`, `feature/ui`).
- No direct commits to `dev` or `main`.
- Open a Pull Request targeting `dev`.

## Commit Message Format
We use [Conventional Commits](https://www.conventionalcommits.org/):
- `feat: added arm64 support`
- `fix: resolved virtio disk detection issue`
- `docs: updated readme`

## How to add a new guest OS preset
1. Identify the OS category (Windows, Linux, BSD)
2. Open the corresponding preset file in `packages/core/src/presets/`
3. Add a new object implementing the `Preset` interface.
4. Ensure default config specifies sufficient cores, RAM, and disk space.

## How to add a new CPU architecture
1. Open `packages/core/src/cpu-arch/registry.ts`
2. Add a new `ArchProfile` to the `archRegistry` array.
3. Example:
```ts
{
  id: 'mips64',
  label: 'MIPS64',
  qemuBinary: 'qemu-system-mips64el',
  defaultMachine: 'malta',
  defaultCPU: '5Kf',
  supportedGuests: ['linux']
}
```
