# QEMU and TCG Integration

PlazaVM leverages QEMU's hypervisor capabilities for full system emulation (`qemu-system-x86_64`, `qemu-system-aarch64`).

## VirtualBlockDevice and NBD

A significant architecture constraint in PlazaVM is that the guest operating system **must not** directly touch raw files on the host file system for its root block device, as the image layers are managed centrally by `plaza-image`.

To solve this, PlazaVM implements a userspace NBD (Network Block Device) server located in `plaza-plugin/src/qemu/storage/nbd.rs`.

1. `plaza-image` constructs a `VirtualBlockDevice` (a layered, Copy-on-Write overlay abstraction of chunks).
2. The QEMU plugin launches an embedded async NBD server bound to a local TCP port or Unix Domain Socket.
3. The NBD server takes a handle to the `VirtualBlockDevice`.
4. The QEMU process is launched with `-drive file=nbd:127.0.0.1:<port>,format=raw`.
5. QEMU sends raw sector read/write commands over the NBD protocol.
6. The PlazaVM NBD server translates these commands into `VirtualBlockDevice::read_at()` and `VirtualBlockDevice::write_at()`.

This guarantees completely sandboxed block storage without exposing internal layered implementations to QEMU.
