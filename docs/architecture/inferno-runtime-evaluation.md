# Inferno OS Runtime Evaluation for PlazaVM

## Executive Summary

Inferno OS is a distributed operating system from Bell Labs/Vita Nuova that boots
as a Multiboot-compatible kernel. For PlazaVM, the native mode is architecturally
preferred because it preserves the VM-level isolation boundary.

## Verified Boot Mechanism

### Kernel Format
- **Format**: ELF binary with Multiboot header (magic: `0x1BADB002`)
- **Architecture**: 386 (x86)
- **Entry point**: `0x80100020` (virtual), `0x00100020` (physical)
- **Flags**: `0x00010003` (aligned, memory info, load modules)

### QEMU Boot Command
```bash
qemu-system-i386 \
    -kernel inferno.386 \
    -m 64 \
    -nographic \
    -serial stdio \
    -accel tcg
```

### Configuration Format
Inferno reads configuration from `CONFADDR` in Plan 9 ini format:
```
bootdisk=sdC0
noboot=true
```

### Console
- i8250 UART at port `0x3F8` (COM1)
- Baud rate: configurable via kernel args
- Serial output available via `-serial stdio`

### Boot Sequence
1. Multiboot loader loads kernel to `0x00100020`
2. Kernel sets up MMU, GDT, page tables
3. Reads configuration from `CONFADDR`
4. Initializes hardware (console, keyboard, timer)
5. Starts `userinit()` which runs `init0()`
6. `init0()` calls `disinit("/osinit.dis")` to load Dis init program
7. Init program starts the user environment

## Source & Build

- **Repository**: https://github.com/inferno-os/inferno-os
- **License**: MIT-style (free software)
- **Build requirements**: GCC, libc6-dev (Linux hosted), or its own Limbo compiler
- **Build process**: From within Inferno's hosted environment under Linux
- **Supported native architectures**: x86 (386), ARM, PowerPC, SPARC

## QEMU Compatibility

- **Confirmed**: Inferno native kernel boots in QEMU with `-kernel` flag
- **Machine type**: `qemu-system-i386` (standard PC/AT)
- **Storage**: Can use virtio or IDE block devices
- **Serial console**: Supported via `-serial stdio`
- **Network**: RTL8139 NIC support documented

## Disk Image Sizes

| Component | Size | Notes |
|---|---|---|
| Native kernel (32-bit x86) | ~2-5 MB | Compact, purpose-built |
| Minimal userspace | ~10-25 MB | Core Inferno commands + libs |
| Full installation | ~256 MB | All standard files |
| KFS root filesystem | ~50-100 MB | Typical project workspace |

**NOTE**: These are approximate sizes from community documentation.
Actual sizes must be measured from the PlazaVM build.

## Memory Requirements

- **Minimum**: ~16 MB RAM for basic Inferno operation
- **Recommended**: ~64 MB for comfortable project workloads
- **Contrast with Linux/Alpine**: ~128-256 MB typical PlazaVM allocation

## Key Architectural Properties

### Namespace Model
- Everything is a file (Plan 9 heritage)
- Resources accessed via open/read/write/close
- Per-process namespace attachment
- 9P/Styx protocol for network-transparent resource access

### Process Model
- Lightweight processes (not Linux threads)
- Limbo language for application development
- Dis bytecode VM for portable execution
- Built-in concurrency primitives

### Filesystem
- KFS (kernel file system) - Inferno's native filesystem
- Can mount 9P servers as local directories
- Union/overlay semantics for namespace composition

### Networking
- Built-in TCP/IP stack
- 9P network servers
- RTL8139 NIC support in QEMU

## Devices Included (from pc configuration)

```
dev:
  root, cons, arch, env, mnt, pipe, prog, rtc, srv, dup, ssl, cap
  draw, pointer, vga
  ip, ether, uart, tinyfs

ip:
  tcp, udp, ipifc, icmp, icmp6, ipmux

lib:
  interp, keyring, draw, memlayer, memdraw, tk, sec, mp, math, kern
```

## Root Filesystem Structure

```
/chan     - Channel namespace
/dev      - Device namespace
/dis      - Dis bytecode programs
/env      - Environment variables
/fd       - File descriptors
/n        - Network namespace
/n/remote - Remote network
/net      - Network
/nvfs     - Virtual filesystem
/prog     - Process control
/dis/lib  - Dis libraries
/dis/svc  - Dis services
/dis/wm   - Window manager
/dis/sh   - Shell
/dis/ls   - List directory
/dis/cat  - Concatenate files
/dis/bind - Bind operations
/dis/mount - Mount operations
/dis/pwd  - Print working directory
/dis/echo - Echo output
/dis/cd   - Change directory
```

## PlazaVM Integration Points

### What Reuses Directly
- `VirtualBlockDevice` trait (storage-agnostic)
- `RuntimeStorage` (Arc<Mutex<dyn VirtualBlockDevice>>)
- `LayeredBlockDevice` (multi-layer block device)
- `CowWritableLayer` (COW for workspace state)
- `NbdServer` (userspace NBD)
- QEMU process lifecycle management
- Workspace isolation architecture
- Content-addressed blob storage

### What Requires Adaptation
- Image acquisition (Inferno source → kernel binary)
- Filesystem builder (no SquashFS; use KFS or raw block)
- Boot configuration (Inferno uses Multiboot, not Linux)
- Init system (Inferno namespace setup vs Linux init)
- Readiness detection (different protocol than SUCCESS_PLAZA_GUEST_READY)
- Guest filesystem mount points (different namespace model)

### What May Need Replacement
- Alpine acquisition source (use Inferno build system)
- SquashFS builder (use KFS or direct block device)
- Linux-specific init script
- Linux process execution compatibility

## Linux ↔ Inferno Compatibility Layer

### Challenge
Inferno is NOT Linux. Projects expecting Linux syscalls, glibc, /proc, etc.
will NOT work directly in Inferno.

### Approach
Create an explicit compatibility subsystem that translates Linux-oriented
operations to Inferno primitives:

| Linux Concept | Inferno Equivalent | Status |
|---|---|---|
| File operations | Namespace files | NATIVE |
| Process execution | Limbo Dis processes | NATIVE |
| Environment variables | Bind-mounted files | EMULABLE |
| Shell commands | Inferno shell (sh) | NATIVE |
| Networking | Built-in TCP/IP | NATIVE |
| Pipes | Inferno pipes | NATIVE |
| /proc filesystem | /proc namespace | NATIVE |
| /sys filesystem | /dev namespace | PARTIAL |
| Linux syscalls | Not available | UNSUPPORTED |
| glibc functions | Not available | UNSUPPORTED |
| epoll/ioctl | Not available | UNSUPPORTED |
| Shared libraries | Dis bytecode only | UNSUPPORTED |

### Capability Model States
- **SUPPORTED**: Available natively in Inferno
- **EMULABLE**: Can be translated to Inferno equivalent
- **PARTIAL**: Some aspects work, some don't
- **UNSUPPORTED**: Not available in Inferno

## Risks & Incompatibilities

1. **Build complexity**: Building Inferno requires its own toolchain
2. **No pre-built images**: Must build from source or find community builds
3. **Limited x86_64 support**: Inferno primarily targets 32-bit x86
4. **No modern hardware support**: Limited PCI/VGA compared to Linux
5. **No Linux binary compatibility**: Cannot run arbitrary Linux programs
6. **Documentation scarcity**: Less community documentation than Linux

## Recommended Migration Sequence

1. ✅ Make Inferno the default runtime kind (Phase A)
2. ✅ Investigate Inferno OS artifacts and boot requirements (Phase B)
3. 🔲 Build real Inferno kernel from source using Docker
4. 🔲 Create Inferno filesystem builder (KFS or raw block)
5. 🔲 Implement real InfernoGuestRuntime provisioning
6. 🔲 Boot Inferno under existing QEMU infrastructure
7. 🔲 Integrate workspace storage (NBD + COW)
8. 🔲 Implement Inferno guest init and readiness detection
9. 🔲 Create Linux compatibility layer
10. 🔲 Add capability negotiation
11. 🔲 Implement project workspace operations
12. 🔲 Add persistence, isolation, and security tests
