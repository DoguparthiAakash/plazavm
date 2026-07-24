# ADR 0003: Event-Driven Reconciliation Controller

## Status
Accepted

## Context
Traditional virtual machine / container managers poll execution engines synchronously, creating UI freezes, thread contention, and blocking main event loops.

## Decision
Adopt a non-blocking, event-driven reconciliation loop (`WorkspaceController`) modeled after Kubernetes controllers.
- The state engine compares `spec.desired_state` against `status.state`.
- State transitions publish asynchronous domain events (`PlazaEvent::WorkspaceStateChanged`, `WorkspaceHealthChanged`) to the Tokio broadcast `EventBus`.

## Consequences
- Completely decoupled execution tracking.
- Reactive UI updates without polling overhead.
- Safe async state reconciliation.
