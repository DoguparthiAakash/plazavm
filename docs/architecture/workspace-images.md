# Workspace Images

PlazaVM represents the isolated project environment as a **Workspace Image**.

A Workspace Image is an immutable, content-addressed asset derived from the constraints declared in `plaza.yaml`.

## Identity and Determinism

The identity of a Workspace Image is calculated deterministically by taking the SHA-256 hash of the canonical JSON representation of its `WorkspaceImageSpec`.

The specification includes:
- Engine distribution (`alpine`, `fedora`, etc.)
- Desired packages and tools
- Project language and runtime versions
- Dependency lists
- Isolated environment variables

If two `plaza.yaml` files resolve to the same semantic `WorkspaceImageSpec`, they will share the exact same `WorkspaceImageID` and use the same cached image blob from `.plaza/blobs`.

## Security

PlazaVM operates on a strict "Default Deny" security model.
During Phase 16, PlazaVM **DOES NOT** execute arbitrary commands (`apk add`, `apt-get`, `pip install`) directly on the host machine to assemble these images, as doing so would compromise the host.

If the requested `WorkspaceImageID` does not exist in the local image cache, the system produces an `ImageBuildUnavailable` error containing the required build plan, deferring actual image assembly to a trusted remote builder or a later phase (Phase 17).
