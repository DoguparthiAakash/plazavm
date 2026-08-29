# Workspace Images

PlazaVM represents the isolated project environment as a **Workspace Image**.

A Workspace Image is an immutable, content-addressed asset derived from the constraints declared in `plaza.yaml`.

## Identity and Determinism

The identity of a Workspace Image is calculated deterministically by taking the SHA-256 hash of the canonical JSON representation of its `WorkspaceImageSpec`.

The specification includes:
- Kernel arguments and specific Limbo boot scripts
- Desired packages and tools (compiled as `.dis` Limbo binaries or statically linked Linux ELFs via the compat layer)
- Project language and runtime versions
- Isolated environment variables

If two `plaza.yaml` files resolve to the same semantic `WorkspaceImageSpec`, they will share the exact same `WorkspaceImageID` and use the same compiled `inferno.386` kernel from the cache.

## Security

PlazaVM operates on a strict "Default Deny" security model.
During workspace assembly, PlazaVM **DOES NOT** execute arbitrary commands (`apk add`, `apt-get`, `pip install`) directly on the host machine. Instead, it relies on pre-compiled native Limbo binaries and static ELF binaries embedded in the `.386` kernel image.

When the workspace boots, the `inferno.386` kernel runs in complete isolation within a QEMU TCG sandbox. The host directory (`.plaza/workspace`) is explicitly mounted into the guest via the **9P/Styx** protocol. The host-side 9P server enforces the Capability Engine rules, preventing any unauthorized filesystem traversal or network access outside the declared scope.
