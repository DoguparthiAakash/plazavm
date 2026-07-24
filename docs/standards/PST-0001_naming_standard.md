# PST-0001: Resource Naming & PURI Standard v1.0

## Status
Standard / Frozen (Architecture v1.0)

## Abstract
This standard defines the format and validation rules for Plaza Universal Resource Identifiers (PURI).

## PURI Syntax Specification
```text
plaza://<namespace>/<resource_id>[?query][#fragment]
```

## Reserved Namespaces
- `workspace`: Isolated developer workspaces
- `runtime`: Execution backends (`docker`, `podman`, `qemu`, `virtualbox`, `hyperv`)
- `provider`: PAL distribution providers (`debian`, `ubuntu`, `arch`, `fedora`, `alpine`)
- `package`: Unified package abstraction entities
- `snapshot`: Layered storage snapshots
- `plugin`: Extension plugins
- `policy`: Zero-trust security policies
- `vhal`: Virtual hardware device specifications
