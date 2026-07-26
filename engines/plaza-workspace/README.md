# `plaza-workspace`

Workspace model definitions, spec structures, and workspace lifecycle service logic.

---

## 🛠 Responsibilities

- Core `Workspace`, `WorkspaceSpec`, and `WorkspaceStatus` models.
- Workspace state machine (`Created`, `Scheduling`, `Running`, `Stopping`, `Stopped`, `Error`, `Destroyed`).
- Workspace service CRUD operations (`WorkspaceService`).
