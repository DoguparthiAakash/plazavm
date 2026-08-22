# Capability Security Model

PlazaVM implements a strict **Default-Deny** capability-based security model.

## The Problem
Many virtualization environments implicitly grant resources (like the host network, some host mounts, or environment variables) unless explicitly locked down. This violates the principle of least privilege.

## PlazaVM's Solution
In PlazaVM, if a capability is not explicitly defined in `plaza.yaml`, it is absolutely denied.

### Configuration (`plaza.yaml`)
Users define grants in their specification:

```yaml
capabilities:
  filesystem:
    - path: "src"
      mode: read_write
  network:
    enabled: true
    mode: bridged
```

### Resolution and Enforcement
1. The raw YAML is parsed into `CapabilityGrants`.
2. `plaza-workspace` invokes `CapabilityPolicy::resolve()` from `plaza-foundation`. This:
   - Evaluates filesystem paths (ensuring they exist and are canonized).
   - Resolves network intents.
   - If `capabilities` is completely missing from the YAML, it falls back to `CapabilityPolicy::default()`, which implies zero filesystem grants, no network, no clipboard, no devices.
3. The resolved `CapabilityPolicy` is injected into `MachineConfig`.
4. `plaza-runtime` passes `MachineConfig` to the chosen plugin (QEMU, v86, etc.).
5. The plugin is **not allowed** to query `plaza.yaml` directly or infer any defaults. It blindly executes the boundaries defined in the `CapabilityPolicy`.

### I/O Sandboxing
- **Network**: Evaluated securely before VM boot.
- **Storage**: Evaluated in `plaza-image`. `VirtualBlockDevice` handles disk access, entirely preventing the guest from bypassing its defined layers.
