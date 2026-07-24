# VirtualBox Full VM Workspace Example

This example demonstrates deploying a full desktop x86_64 guest VM via VirtualBox.

---

## 📄 Configuration (`plaza.yaml`)

```yaml
version: "1"
workspace:
  name: "win11-desktop-workspace"
runtime:
  kind: "virtual_machine"
  os: "windows"
resources:
  cpu_cores: 4
  memory_mb: 8192
```

---

## 🎯 Decision Engine Resolution

Because `kind: virtual_machine` and `os: windows` are requested, the Decision Engine scores VirtualBox at `0.80`, outscoring container runtimes (`0.00`).
