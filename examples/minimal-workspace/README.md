# Minimal Workspace Example

This example demonstrates the absolute minimal configuration required to create a PlazaVM workspace.

---

## 📄 Configuration (`plaza.yaml`)

```yaml
version: "1"
workspace:
  name: "minimal-dev"
runtime:
  kind: "container"
```

---

## 🎯 Expected Resolution & Output

When submitted to PlazaVM CLI:
- **Decision Engine Selection**: `docker` (or `podman` on Linux)
- **Assigned Resources**: Default 2 vCPU, 2048 MB RAM
- **Final State**: `WorkspaceState::Running`
