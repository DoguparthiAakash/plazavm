# Intent-Driven Virtualization Model

In PlazaVM, developers specify **what** their workload requires (the "Intent"), while PlazaVM's Decision Engine determines **how** to execute it (the "Backend Selection").

---

## 🎯 Declarative Intent vs Hardware Specifics

Traditional virtualization forces developers to choose specific hypervisors, disk image paths, or network bridge parameters.

PlazaVM introduces **Intent Declarations**:

| Developer Intent | PlazaVM Decision Engine Resolution | Selected Backend |
|---|---|---|
| Purpose: "Microservice API" | High container score (`0.85`), low overhead | `Docker` or `Podman` |
| Purpose: "Windows Desktop App" | Requires full x86_64 guest kernel | `VirtualBox` or `Hyper-V` |
| Purpose: "Kernel Testing" | Requires raw QEMU emulation | `QEMU` |
| GPU: "CUDA Acceleration" | Matches host CUDA GPU capability | `Docker (nvidia)` or `QEMU (vfio)` |

---

## 🔍 Decision Engine Scoring Matrix

The `plaza-decision` crate evaluates all registered runtime plugins against host hardware profile and workload intent using calibrated scoring rules:

$$\text{Score}(P) = W_{\text{kind}} \cdot S_{\text{kind}} + W_{\text{OS}} \cdot S_{\text{OS}} + W_{\text{GPU}} \cdot S_{\text{GPU}}$$

The backend with the highest score strictly within $0.0 .. 1.0$ is selected for execution.
