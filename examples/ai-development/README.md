# AI Development Workspace Example

This example demonstrates configuring a high-performance workspace for AI model training and PyTorch/CUDA workloads.

---

## 📄 Configuration (`plaza.yaml`)

```yaml
version: "1"
workspace:
  name: "ai-llm-fine-tuning"
  description: "PyTorch 2.2 + CUDA 12.1 Workstation"
runtime:
  kind: "container"
  os: "linux"
  image: "pytorch/pytorch:2.2.0-cuda12.1-cudnn8-runtime"
resources:
  cpu_cores: 8
  memory_mb: 32768
intent:
  purpose: "AI Research"
  gpu: "required"
```

---

## 🎯 Expected Output

PlazaVM automatically verifies CUDA hardware capabilities via `plaza-platform` and selects GPU passthrough container execution.
