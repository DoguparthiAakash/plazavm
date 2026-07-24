# `plaza-decision`

Intent matching decision engine and runtime scoring crate.

---

## 🛠 Responsibilities

- Multi-factor scoring matrix (`ScoringEngine`).
- Evaluates workload intent against host platform profile and registered plugins.
- Selects the optimal runtime backend strictly within $0.0 .. 1.0$ score range.
