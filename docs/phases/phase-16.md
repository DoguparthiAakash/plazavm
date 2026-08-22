# Phase 16: Project Workspace Image & Custom Engine Implementation

**Status**: Completed

## Objective
Transform PlazaVM from a basic VM runner into a project-aware workspace engine driven by a `plaza.yaml` configuration file, utilizing content-addressed Workspace Images to guarantee deterministic, isolated project environments.

## Achievements
- Implemented `plaza.yaml` sections (`engine`, `project`, `image`, `environment`).
- Created the `WorkspaceImageSpec` domain model to represent workspace configurations.
- Implemented deterministic `WorkspaceImageID` computation using canonical JSON hashing (SHA-256).
- Abstracted OS construction logic via the `EngineDistribution` trait (with `AlpineEngine`, and stubs for `FedoraEngine`/`ArchEngine`).
- Updated the orchestration pipeline (`pipeline.rs` and `plaza-cli`) to calculate the identity, query the local `plaza-image` registry, and explicitly abort image provisioning if the requested image is not pre-cached. This respects PlazaVM's strict default-deny security model against modifying the host.
- Disabled direct Guest Shell execution in `plaza-cli` returning `UnsupportedCapability` for strong host-environment boundary isolation.

## Architectural Enforcement
- The Host OS cannot run package managers (e.g., `apk`, `apt`) during Workspace Image generation. 
- Image provisioning requests generate a `WorkspaceImageBuildPlan` which encapsulates required instructions without running them.
- Mutability exists purely in a ephemeral session boundary (`CowWritableLayer`), ensuring the `WorkspaceImage` acts only as a pristine, immutable base.
