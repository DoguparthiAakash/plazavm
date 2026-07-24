# `plaza-controller`

Kubernetes-inspired declarative reconciliation engine crate.

---

## 🛠 Responsibilities

- Declarative state convergence loop (`Reconciler`).
- Continuously aligns current workspace state with `DesiredState`.
- Emits lifecycle events over `EventBus`.
