# Inferno Runtime Evaluation

This document details the verified behavior of the real Inferno OS kernel and its compatibility with PlazaVM under QEMU-TCG.

## 1. Supported Architecture
Inferno OS natively supports the `386` architecture (`x86` 32-bit). It is compiled targeting `OBJTYPE=386` with `SYSTARG=Linux` or similar host environments.

## 2. QEMU-Compatible Kernel Target
The target for QEMU booting is the PC kernel, compiled in `os/pc`. The configuration file is `os/pc/pc` (or a custom variant) producing the `inferno.386` binary. This binary is bootable via QEMU's `-kernel` flag.

## 3. Kernel Binary Format
The `inferno.386` file is an ELF binary. It includes not only the kernel code but also the initial root filesystem embedded directly inside the executable, as defined in the `root` section of the `os/pc/pc` configuration file.

## 4. Boot Arguments
Unlike Linux, Inferno does not use the standard `cmdline`. It expects its configuration to be in a specific memory location (`CONFADDR`). However, QEMU multiboot can pass arguments to Inferno. Common arguments for automated booting include:
- `bootdisk=sdC0` (if booting from an external IDE/ATA drive)
- `noboot=true` (skips the interactive boot menu)
- `nobreak=true` (prevents breaking into the kernel debugger)
- `console=0` (directs console output to the serial port, i.e., COM1)

## 5. Root Filesystem Expectations
Inferno does not use an external `initrd`. The minimal root filesystem is compiled into the kernel. It includes essential Dis binaries (like `/dis/sh.dis`, `/dis/ls.dis`) and libraries. The initial filesystem is mounted as `/`. Any external persistent storage needs to be mounted explicitly after boot (e.g., KFS, FAT, or via 9P/Styx).

## 6. Console Configuration
Inferno supports serial console output natively. The `uarti8250` device driver provides serial communication. By configuring QEMU with `-serial stdio` (or a unix socket) and passing the correct console boot arguments, we can interact with the Inferno shell over serial.

## 7. Startup/Init Mechanism
The kernel starts the first Dis process, typically `/osinit.dis` or a window manager init (`wminit`). For PlazaVM headless operation, a custom `init` script should be executed to set up the environment, start the 9P server for workspace mounting, and emit the readiness marker (`PLAZA_INFERNO_READY`).

## 8. Available Filesystem Drivers
Inferno supports several filesystems natively:
- `tinyfs`: An in-memory filesystem.
- `kfs`: The native persistent filesystem.
- `dos`: For FAT16/FAT32 partitions.
- `cdfs`: ISO9660.
- `tarfs`: Read-only tar archives.

## 9. Available 9P/Styx Functionality
9P/Styx is a first-class citizen in Inferno. The `exportfs` and `import` commands allow serving and mounting filesystems over network or local pipes. This is highly advantageous for providing the persistent `/workspace` directory directly from the host.

## 10. How Inferno Exposes Devices
Devices are exposed as files in the namespace. For example, `#t` (serial port) is mounted to `/dev/eia0`. Storage devices are exposed under `#S` (e.g., `/dev/sdC0/data`).

## 11. How Commands/Processes are Started
Commands are executed in the Dis virtual machine. The shell (`/dis/sh.dis`) reads input and runs `.dis` binaries. There are no Linux standard syscalls (`fork`/`execve` in the POSIX sense); instead, Limbo modules are loaded and their `init` functions are called.

## 12. Conclusion & Adoption

Based on the above evaluation, **Inferno OS has been officially adopted as the primary workspace engine for PlazaVM**. Its inherent portability, microscopic footprint, 9P natively-integrated filesystem model, and zero-reliance on OCI tooling perfectly aligns with PlazaVM's goals of lightweight, instantaneous workspace execution.

The evaluation proved successful. The integration phase is completed, and workspaces now boot dynamically compiled `.dis` binaries injected into the `inferno.386` ELF kernel payload over QEMU TCG.
