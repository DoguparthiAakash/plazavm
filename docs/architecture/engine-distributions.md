# Engine Distributions

PlazaVM abstracts the base operating system layout using the `EngineDistribution` trait.

The Engine Distribution translates the semantic `WorkspaceImageSpec` into a logical `WorkspaceImageBuildPlan` (e.g., resolving `packages: [git]` into `apk add git` for Alpine or `dnf install git` for Fedora).

Currently supported engines:
- **Alpine**: Fully supported. Validates specs and emits `apk` commands.
- **Fedora**: (Stubbed) Returns `UnsupportedDistribution`.
- **Arch**: (Stubbed) Returns `UnsupportedDistribution`.

## Distribution Independence
The CLI, `plaza.yaml`, and the `WorkspaceImageSpec` do not leak distribution-specific package manager commands. They rely on the distribution engine to resolve the plan, guaranteeing portability and a declarative specification model.
