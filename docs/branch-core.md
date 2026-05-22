# Branch: feature/core

## Scope
- Implement the `VMEngine` class.
- Manage VM lifecycles: create, start, stop, pause, resume.
- Parse and save VM configurations as JSON in `~/.plazavm/vms/`.

## Merge Criteria
- 100% unit test coverage for state transitions and configuration parsing.
- Functional QEMU subprocess spawning for at least one architecture (x86_64).
