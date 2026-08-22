# v86 WebAssembly Emulation

PlazaVM includes experimental support for `v86`, a PC emulator written in C and compiled to WebAssembly.

## WASM Execution

Currently, `plaza_plugin::v86` utilizes **Wasmtime** as the native WebAssembly engine to execute `v86.wasm`.

## Integration

The `v86` emulator exposes an API boundary via WebAssembly imports and exports (the `env` module). PlazaVM intercepts these calls in `V86Environment`. 

### VirtualBlockDevice Mapping

While full block storage mapping via memory-mapped read/write intercepts (`mmap_read8`, `mmap_write8`) is the desired architecture, **the current implementation is incomplete and acts as a stub.** 

The `v86` runner explicitly rejects execution with `UnsupportedCapability` because the complex memory translation between Wasmtime linear memory and PlazaVM's asynchronous `VirtualBlockDevice` trait is pending implementation in future phases.
