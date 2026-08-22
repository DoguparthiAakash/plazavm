# Plaza-QEMU Vendor Engine

This directory contains the custom-tailored, stripped-down source code for `plaza-qemu`.

## Why Vendored?

To provide a fully independent **Userspace Execution Engine** without forcing the user to install any system-level dependencies.
If PlazaVM relied on `qemu-system-x86_64` from the host's `PATH`, it would conflict with existing hypervisors (Docker, Hyper-V, standard QEMU) and could fail due to version mismatches or missing features.

By vendoring QEMU, we guarantee:
1. **Zero Host Conflicts**: Our binary is completely isolated.
2. **Minimal Footprint**: We strip out UI (GTK/SDL), audio drivers, and unnecessary emulated hardware, focusing solely on headless TCG execution and VirtIO (9p/virtio-fs).
3. **PlazaVM Specific Patches**: Any custom kernel hooks or optimizations specifically for our `.img` layer blending are patched directly in.

## Build Instructions
(Currently managed by the `plaza-build` toolchain).
The binary is compiled statically and injected into the PlazaVM installation's `bin/` directory as `plaza-qemu.exe` (on Windows) or `plaza-qemu` (on Linux).
