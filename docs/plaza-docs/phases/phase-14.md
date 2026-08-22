# Phase 14: Runtime Execution Completion & End-to-End Integration

## Mission
Phase 14 transitions PlazaVM from architecturally correct abstractions to real, observable, end-to-end runtime execution for QEMU-TCG, while progressively advancing the v86-WASM runtime.

## Accomplishments

### QEMU-TCG Integration
The QEMU plugin was updated to be a fully functional runtime execution engine.

1. **Process Lifecycle Management**
   - The QEMU process is now spawned robustly and managed safely (`plugins/qemu/src/process.rs`).
   - Generic `Ok(())` stubs were replaced with actual Tokio child process monitoring.
   - The NBD server task is aborted safely when QEMU shuts down or crashes.

2. **QMP (QEMU Machine Protocol) JSON-RPC**
   - The QEMU Plugin connects to QEMU via a QMP JSON-RPC socket (`plugins/qemu/src/qmp.rs`).
   - Guest readiness detection, graceful shutdown (`system_powerdown`, `quit`), and system status (`query-status`) are supported.

3. **Storage Boundary Enforcement (Userspace NBD)**
   - To uphold the isolation boundary, QEMU is supplied storage via a local userspace NBD proxy (`plugins/qemu/src/storage/nbd.rs`).
   - QEMU mounts the image as a raw network block device over a Unix/TCP socket.
   - QEMU itself performs no direct writes to the backing block devices; all I/O is vetted through the `RuntimeStorage` abstraction (and subsequently the COW Layer).

4. **Argument Adapter**
   - Implemented `QemuAdapter` to enforce Capability policies.
   - Enforces TCG hardware-agnostic emulation (`-accel tcfg,thread=multi`).
   - Enforces Default-Deny isolation (`-nodefaults`, headless, selective network attachment).

### Workspace & CLI Integration
1. **Engine Reconciliation**
   - `WorkspaceEngine` (`engines/plaza-workspace/src/engine.rs`) was expanded to actively start, monitor, and transition the states of `RuntimeInstance`.
   - Workspace states transition natively (`Stopped` -> `Starting` -> `Running`).

2. **Diagnostic Feedback**
   - The `plaza workspace inspect` CLI command now reports detailed `WorkspaceStatus` diagnostics to users.

### v86-WASM Emulation
- **Staged Expansion**: v86 WASM engine loading via `wasmtime` has been validated.
- **Storage Bridge Abstraction**: Implemented `WasmMemoryBridge` (`plugins/v86/src/storage.rs`) to translate WASM linear memory I/O operations (e.g., `mmap_read8`, `mmap_write8`) directly to the underlying `VirtualBlockDevice`.
- **Reporting Boundaries**: Explicitly marked the v86 backend's execution layer as suspended pending full async translation of the memory bridge in Wasmtime. Returns `NotImplemented` error dynamically when started. 

## Technical Details

The final runtime execution path is as follows:

```
plaza.yaml -> WorkspaceEngine (Reconciliation Loop) -> QemuPlugin
    ↓
VirtualBlockDevice (COW Layer) -> NbdServer
    ↓
QemuProcess (QMP connected)
    ↓
TCG (Guest OS)
```
